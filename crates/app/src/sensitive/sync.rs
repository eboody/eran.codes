use super::*;

impl Service {
    #[tracing::instrument(skip(self))]
    pub async fn refresh_provider_token(&self) -> Result<sensitive::TokenStatus> {
        let now = self.clock.now();
        let current_token = self.load_refreshable_token().await?;
        let refreshed = self
            .refresh_provider_token_inner(
                now,
                current_token.as_ref().map(|value| &value.access_token),
            )
            .await?;
        Ok(refreshed.status)
    }

    #[tracing::instrument(skip(self))]
    pub async fn run_sync(&self) -> Result<sensitive::SyncRun> {
        let started_at = self.clock.now();
        let boundary_meta = self.provider.boundary_meta(PROVIDER);
        let previous_state = self.repo.load_integration_state(PROVIDER).await?;
        match self
            .run_sync_success(started_at, &boundary_meta, previous_state.as_ref())
            .await
        {
            Ok(success) => {
                self.repo
                    .upsert_integration_state(&success.integration_state)
                    .await?;
                self.repo.record_sync_run(&success.run).await?;
                tracing::info!(
                    target: "demo.sensitive",
                    records_seen = success.run.records_seen,
                    records_upserted = success.run.records_upserted,
                    "sensitive runtime sync completed",
                );
                Ok(success.run)
            }
            Err(failure) => {
                let failed_run = sensitive::SyncRun::builder()
                    .provider(PROVIDER)
                    .outcome(sensitive::SyncOutcome::Failed)
                    .records_seen(0)
                    .records_upserted(0)
                    .detail(sync_detail_text(failure.error.to_string()))
                    .started_at(started_at)
                    .finished_at(self.clock.now())
                    .build();
                if let Err(state_error) = self
                    .repo
                    .upsert_integration_state(&failure.integration_state)
                    .await
                {
                    tracing::warn!(
                        target: "demo.sensitive",
                        ?state_error,
                        "failed to persist failed integration state",
                    );
                }
                if let Err(record_error) = self.repo.record_sync_run(&failed_run).await {
                    tracing::warn!(
                        target: "demo.sensitive",
                        ?record_error,
                        "failed to persist failed sync run",
                    );
                }
                Err(failure.error)
            }
        }
    }

    async fn ensure_fresh_token(
        &self,
        now: SystemTime,
    ) -> core::result::Result<
        (
            ProviderToken,
            sensitive::TokenStrategy,
            sensitive::FetchOutcome,
        ),
        AttemptFailure,
    > {
        let current_token = self.load_refreshable_token().await.map_err(|error| {
            sync_attempt_failure(
                error,
                sensitive::TokenStrategy::RefreshedToken,
                Some(sensitive::FetchOutcome::Failed),
            )
        })?;

        if let Some(token) = current_token {
            if !token_is_stale(&token, now) {
                return Ok((
                    token,
                    sensitive::TokenStrategy::CachedToken,
                    sensitive::FetchOutcome::Success,
                ));
            }

            let refreshed = self
                .refresh_provider_token_inner(now, Some(&token.access_token))
                .await
                .map_err(|error| {
                    sync_attempt_failure(
                        error,
                        sensitive::TokenStrategy::RefreshedToken,
                        Some(sensitive::FetchOutcome::Failed),
                    )
                })?;
            return Ok((
                refreshed,
                sensitive::TokenStrategy::RefreshedToken,
                sensitive::FetchOutcome::Success,
            ));
        }

        let refreshed =
            self.refresh_provider_token_inner(now, None)
                .await
                .map_err(|error| {
                    sync_attempt_failure(
                        error,
                        sensitive::TokenStrategy::RefreshedToken,
                        Some(sensitive::FetchOutcome::Failed),
                    )
                })?;
        Ok((
            refreshed,
            sensitive::TokenStrategy::RefreshedToken,
            sensitive::FetchOutcome::Success,
        ))
    }

    async fn load_refreshable_token(&self) -> Result<Option<ProviderToken>> {
        match self.repo.load_token(PROVIDER).await {
            Ok(token) => Ok(token),
            Err(error) if stored_token_requires_refresh(&error) => {
                tracing::warn!(
                    target: "demo.sensitive",
                    ?error,
                    "stored provider token could not be decrypted; refreshing provider token",
                );
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }

    async fn refresh_provider_token_inner(
        &self,
        now: SystemTime,
        current_token: Option<&SecretString>,
    ) -> Result<ProviderToken> {
        let refreshed = self
            .provider
            .refresh_token(PROVIDER, now, current_token)
            .await?;
        self.repo.upsert_token(&refreshed).await?;
        tracing::info!(
            target: "demo.sensitive",
            provider = %refreshed.status.provider,
            expires_at = debug_timestamp(refreshed.status.expires_at),
            "sensitive provider token refreshed",
        );
        Ok(refreshed)
    }

    async fn fetch_records_with_retry(
        &self,
        token: &ProviderToken,
        cursor: Option<&sensitive::SyncCursor>,
        now: SystemTime,
        token_strategy: sensitive::TokenStrategy,
    ) -> core::result::Result<(ProviderRecords, sensitive::TokenStrategy), AttemptFailure>
    {
        match self
            .provider
            .fetch_records(PROVIDER, token, cursor, now)
            .await
        {
            Ok(records) => Ok((records, token_strategy)),
            Err(error)
                if provider_error_category(&error)
                    == Some(sensitive::RemoteErrorCategory::Unauthorized) =>
            {
                let refreshed = self
                    .refresh_provider_token_inner(now, Some(&token.access_token))
                    .await
                    .map_err(|error| {
                        sync_attempt_failure(
                            error,
                            sensitive::TokenStrategy::RetryAfterUnauthorized,
                            Some(sensitive::FetchOutcome::Failed),
                        )
                    })?;
                self.provider
                    .fetch_records(PROVIDER, &refreshed, cursor, now)
                    .await
                    .map(|records| {
                        (records, sensitive::TokenStrategy::RetryAfterUnauthorized)
                    })
                    .map_err(|error| {
                        sync_attempt_failure(
                            error,
                            sensitive::TokenStrategy::RetryAfterUnauthorized,
                            Some(sensitive::FetchOutcome::Success),
                        )
                    })
            }
            Err(error) => Err(sync_attempt_failure(
                error,
                token_strategy,
                Some(sensitive::FetchOutcome::Success),
            )),
        }
    }

    async fn run_sync_success(
        &self,
        started_at: SystemTime,
        boundary_meta: &ProviderBoundaryMeta,
        previous_state: Option<&sensitive::IntegrationState>,
    ) -> core::result::Result<Success, Failure> {
        let previous_cursor = previous_state.and_then(|state| state.cursor.clone());
        let (token, token_strategy, auth_outcome) = self
            .ensure_fresh_token(started_at)
            .await
            .map_err(|failure| Failure {
                integration_state: failed_integration_state(
                    boundary_meta,
                    previous_state,
                    previous_cursor.clone(),
                    failure.token_strategy,
                    failure.error_category,
                    failure.status_code,
                    failure.auth_outcome,
                    self.clock.now(),
                ),
                error: failure.error,
            })?;
        let (provider_records, token_strategy) = self
            .fetch_records_with_retry(
                &token,
                previous_cursor.as_ref(),
                started_at,
                token_strategy,
            )
            .await
            .map_err(|failure| Failure {
                integration_state: failed_integration_state(
                    boundary_meta,
                    previous_state,
                    previous_cursor.clone(),
                    failure.token_strategy,
                    failure.error_category,
                    failure.status_code,
                    failure.auth_outcome.or(Some(auth_outcome)),
                    self.clock.now(),
                ),
                error: failure.error,
            })?;
        let records_seen = provider_records.records.len() as u32;
        let upserted = self
            .repo
            .upsert_records(&provider_records.records, started_at)
            .await
            .map_err(|error| Failure {
                integration_state: failed_integration_state(
                    boundary_meta,
                    previous_state,
                    previous_cursor,
                    token_strategy,
                    provider_error_category(&error),
                    provider_error_status_code(&error),
                    Some(auth_outcome),
                    self.clock.now(),
                ),
                error,
            })?;
        let finished_at = self.clock.now();
        let integration_state = successful_integration_state(
            boundary_meta,
            provider_records.cursor,
            token_strategy,
            auth_outcome,
            finished_at,
        );
        let run = sensitive::SyncRun::builder()
            .provider(PROVIDER)
            .outcome(sensitive::SyncOutcome::Success)
            .records_seen(records_seen)
            .records_upserted(upserted as u32)
            .detail(sync_detail_text(format!(
                "{} provider records processed through the {} boundary",
                records_seen,
                boundary_meta.mode.as_ref()
            )))
            .started_at(started_at)
            .finished_at(finished_at)
            .build();

        Ok(Success {
            run,
            integration_state,
        })
    }
}

struct Success {
    run: sensitive::SyncRun,
    integration_state: sensitive::IntegrationState,
}

struct Failure {
    error: failure::Error,
    integration_state: sensitive::IntegrationState,
}

struct AttemptFailure {
    error: failure::Error,
    token_strategy: sensitive::TokenStrategy,
    error_category: Option<sensitive::RemoteErrorCategory>,
    status_code: Option<u16>,
    auth_outcome: Option<sensitive::FetchOutcome>,
}

fn sync_attempt_failure(
    error: failure::Error,
    token_strategy: sensitive::TokenStrategy,
    auth_outcome: Option<sensitive::FetchOutcome>,
) -> AttemptFailure {
    let error_category = provider_error_category(&error);
    let status_code = provider_error_status_code(&error);
    AttemptFailure {
        error,
        token_strategy,
        error_category,
        status_code,
        auth_outcome,
    }
}

fn token_is_stale(token: &ProviderToken, now: SystemTime) -> bool {
    let refresh_cutoff = now
        .checked_add(TOKEN_REFRESH_SKEW)
        .unwrap_or_else(|| now + TOKEN_REFRESH_SKEW);
    token.status.expires_at <= refresh_cutoff
}

fn stored_token_requires_refresh(error: &failure::Error) -> bool {
    matches!(
        error,
        failure::Error::Repository {
            source: failure::Repository::DecryptToken { .. },
        }
    )
}

fn provider_error_category(
    error: &failure::Error,
) -> Option<sensitive::RemoteErrorCategory> {
    match error {
        failure::Error::Provider { kind, .. } => Some(match kind {
            failure::ProviderFailureKind::Configuration => {
                sensitive::RemoteErrorCategory::Configuration
            }
            failure::ProviderFailureKind::Unauthorized => {
                sensitive::RemoteErrorCategory::Unauthorized
            }
            failure::ProviderFailureKind::Forbidden => {
                sensitive::RemoteErrorCategory::Forbidden
            }
            failure::ProviderFailureKind::RateLimited => {
                sensitive::RemoteErrorCategory::RateLimited
            }
            failure::ProviderFailureKind::MalformedPayload => {
                sensitive::RemoteErrorCategory::MalformedPayload
            }
            failure::ProviderFailureKind::Timeout => {
                sensitive::RemoteErrorCategory::Timeout
            }
            failure::ProviderFailureKind::ServerError => {
                sensitive::RemoteErrorCategory::ServerError
            }
            failure::ProviderFailureKind::Transport => {
                sensitive::RemoteErrorCategory::Transport
            }
        }),
        _ => None,
    }
}

fn provider_error_status_code(error: &failure::Error) -> Option<u16> {
    match error {
        failure::Error::Provider { status_code, .. } => *status_code,
        _ => None,
    }
}

fn successful_integration_state(
    boundary_meta: &ProviderBoundaryMeta,
    cursor: Option<sensitive::SyncCursor>,
    token_strategy: sensitive::TokenStrategy,
    auth_outcome: sensitive::FetchOutcome,
    finished_at: SystemTime,
) -> sensitive::IntegrationState {
    sensitive::IntegrationState::builder()
        .provider(PROVIDER)
        .mode(boundary_meta.mode)
        .endpoint(boundary_meta.endpoint.clone())
        .maybe_auth_mode(boundary_meta.auth_mode)
        .maybe_cursor(cursor)
        .last_fetch_outcome(sensitive::FetchOutcome::Success)
        .token_strategy(token_strategy)
        .maybe_last_error_category(None)
        .last_auth_outcome(auth_outcome)
        .maybe_last_remote_status_code(None)
        .maybe_retry_backoff_secs(None)
        .last_successful_mode(boundary_meta.mode)
        .last_successful_fetch_at(finished_at)
        .last_attempted_fetch_at(finished_at)
        .failure_count(0)
        .build()
}

fn failed_integration_state(
    boundary_meta: &ProviderBoundaryMeta,
    previous_state: Option<&sensitive::IntegrationState>,
    cursor: Option<sensitive::SyncCursor>,
    token_strategy: sensitive::TokenStrategy,
    error_category: Option<sensitive::RemoteErrorCategory>,
    status_code: Option<u16>,
    auth_outcome: Option<sensitive::FetchOutcome>,
    attempted_at: SystemTime,
) -> sensitive::IntegrationState {
    sensitive::IntegrationState::builder()
        .provider(PROVIDER)
        .mode(boundary_meta.mode)
        .endpoint(boundary_meta.endpoint.clone())
        .maybe_auth_mode(boundary_meta.auth_mode)
        .maybe_cursor(cursor)
        .last_fetch_outcome(sensitive::FetchOutcome::Failed)
        .token_strategy(token_strategy)
        .maybe_last_error_category(error_category)
        .maybe_last_auth_outcome(auth_outcome)
        .maybe_last_remote_status_code(status_code)
        .maybe_retry_backoff_secs(retry_backoff_for(boundary_meta, error_category))
        .maybe_last_successful_mode(previous_successful_mode(previous_state))
        .maybe_last_successful_fetch_at(
            previous_state.and_then(|state| state.last_successful_fetch_at),
        )
        .last_attempted_fetch_at(attempted_at)
        .failure_count(
            previous_state
                .map(|state| state.failure_count.saturating_add(1))
                .unwrap_or(1),
        )
        .build()
}

fn previous_successful_mode(
    previous_state: Option<&sensitive::IntegrationState>,
) -> Option<sensitive::ProviderMode> {
    previous_state.and_then(|state| state.last_successful_mode.or(Some(state.mode)))
}

fn retry_backoff_for(
    boundary_meta: &ProviderBoundaryMeta,
    error_category: Option<sensitive::RemoteErrorCategory>,
) -> Option<u32> {
    match error_category {
        Some(
            sensitive::RemoteErrorCategory::RateLimited
            | sensitive::RemoteErrorCategory::Timeout
            | sensitive::RemoteErrorCategory::ServerError
            | sensitive::RemoteErrorCategory::Transport,
        ) => boundary_meta.retry_backoff_secs,
        _ => None,
    }
}

fn debug_timestamp(value: SystemTime) -> u64 {
    value
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub(super) fn sync_detail_text(message: impl Into<String>) -> sensitive::DetailText {
    super::bounded_detail_text(message, "runtime sync status")
}
