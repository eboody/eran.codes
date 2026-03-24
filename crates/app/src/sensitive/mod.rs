mod error;

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use bon::Builder;
use secrecy::SecretString;
use strum_macros::{AsRefStr, Display};

use domain::{sensitive, user};
pub use error::{
    Error, ProviderFailureKind, ProviderOperation, RepositoryOperation, Result,
};

const PROVIDER: sensitive::Provider = sensitive::Provider::SyntheticSecureFeed;
const TOKEN_REFRESH_SKEW: Duration = Duration::from_secs(30);
const ACCESS_EVENT_LIMIT: usize = 6;

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct AuthenticatedViewer {
    pub user_id: user::Id,
    pub email: user::Email,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Viewer {
    Guest,
    Authenticated(AuthenticatedViewer),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum ViewerTier {
    Guest,
    Authenticated,
    SensitiveReader,
    SensitiveOperator,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct ViewerState {
    pub tier: ViewerTier,
    pub capabilities: Vec<sensitive::AccessCapability>,
}

impl ViewerState {
    pub fn guest() -> Self {
        Self::builder()
            .tier(ViewerTier::Guest)
            .capabilities(Vec::new())
            .build()
    }

    pub fn authenticated(capabilities: Vec<sensitive::AccessCapability>) -> Self {
        Self::builder()
            .tier(viewer_tier_for_capabilities(&capabilities))
            .capabilities(capabilities)
            .build()
    }

    pub fn is_authenticated(&self) -> bool {
        self.tier != ViewerTier::Guest
    }

    pub fn has_capability(&self, capability: sensitive::AccessCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn allows_authorized_record(&self) -> bool {
        self.has_capability(sensitive::AccessCapability::AuthorizedRecordRead)
    }

    pub fn allows_token_status(&self) -> bool {
        self.has_capability(sensitive::AccessCapability::TokenStatusRead)
    }

    pub fn allows_access_audit(&self) -> bool {
        self.has_capability(sensitive::AccessCapability::AccessAuditRead)
    }
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct CiphertextEvidence {
    pub key_id: sensitive::KeyId,
    pub preview: String,
    pub bytes: usize,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct RecordProof {
    pub id: sensitive::Id,
    pub label: sensitive::Label,
    pub last4: sensitive::Last4,
    pub synced_at: SystemTime,
    pub ciphertext: CiphertextEvidence,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct TokenProof {
    pub status: sensitive::TokenStatus,
    pub ciphertext: CiphertextEvidence,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct ProviderBoundaryMeta {
    pub mode: sensitive::ProviderMode,
    pub endpoint: sensitive::DetailText,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct ProviderRecords {
    pub records: Vec<sensitive::Record>,
    pub cursor: Option<sensitive::SyncCursor>,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct StoredSnapshot {
    pub token: Option<TokenProof>,
    pub latest_sync: Option<sensitive::SyncRun>,
    pub integration_state: Option<sensitive::IntegrationState>,
    pub records: Vec<RecordProof>,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct KeyRotationProgress {
    pub active_key_id: sensitive::KeyId,
    pub rows_scanned: u32,
    pub rows_rewrapped: u32,
    pub rows_already_current: u32,
    pub rows_failed: u32,
    pub detail: sensitive::DetailText,
}

#[derive(Clone, Debug, Builder)]
pub struct ProviderToken {
    pub status: sensitive::TokenStatus,
    pub access_token: SecretString,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct Snapshot {
    pub viewer: ViewerState,
    pub token: Option<TokenProof>,
    pub latest_sync: Option<sensitive::SyncRun>,
    pub integration_state: Option<sensitive::IntegrationState>,
    pub key_custody: Option<sensitive::KeyCustodyState>,
    pub records: Vec<RecordProof>,
    pub authorized_record: Option<sensitive::AuthorizedRecord>,
    pub access_events: Vec<sensitive::AccessEvent>,
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn load_snapshot(&self) -> Result<StoredSnapshot>;
    async fn load_authorized_record(
        &self,
        record_id: &sensitive::Id,
    ) -> Result<Option<sensitive::AuthorizedRecord>>;
    async fn load_integration_state(
        &self,
        provider: sensitive::Provider,
    ) -> Result<Option<sensitive::IntegrationState>>;
    async fn load_key_custody(&self) -> Result<sensitive::KeyCustodyState>;
    async fn load_access_grants(
        &self,
        user_id: &user::Id,
    ) -> Result<Vec<sensitive::AccessGrant>>;
    async fn load_token(
        &self,
        provider: sensitive::Provider,
    ) -> Result<Option<ProviderToken>>;
    async fn upsert_token(&self, token: &ProviderToken) -> Result<()>;
    async fn upsert_records(
        &self,
        records: &[sensitive::Record],
        synced_at: SystemTime,
    ) -> Result<usize>;
    async fn upsert_integration_state(
        &self,
        state: &sensitive::IntegrationState,
    ) -> Result<()>;
    async fn rotate_ciphertext_to_active_key(
        &self,
        limit: usize,
        rotated_at: SystemTime,
    ) -> Result<KeyRotationProgress>;
    async fn record_sync_run(&self, run: &sensitive::SyncRun) -> Result<()>;
    async fn record_key_rotation_run(
        &self,
        run: &sensitive::KeyRotationRun,
    ) -> Result<()>;
    async fn upsert_access_grants(
        &self,
        user_id: &user::Id,
        capabilities: &[sensitive::AccessCapability],
        granted_at: SystemTime,
    ) -> Result<()>;
    async fn record_access_event(&self, event: &sensitive::AccessEvent) -> Result<()>;
    async fn list_recent_access_events(
        &self,
        limit: usize,
    ) -> Result<Vec<sensitive::AccessEvent>>;
}

#[async_trait]
pub trait ProviderClient: Send + Sync {
    fn boundary_meta(&self, provider: sensitive::Provider) -> ProviderBoundaryMeta;

    async fn refresh_token(
        &self,
        provider: sensitive::Provider,
        now: SystemTime,
        current_token: Option<&SecretString>,
    ) -> Result<ProviderToken>;

    async fn fetch_records(
        &self,
        provider: sensitive::Provider,
        token: &ProviderToken,
        cursor: Option<&sensitive::SyncCursor>,
        now: SystemTime,
    ) -> Result<ProviderRecords>;
}

pub trait Clock: Send + Sync {
    fn now(&self) -> SystemTime;
}

#[derive(Clone, Debug, Default)]
pub struct BootstrapGrants {
    reader_emails: Vec<user::Email>,
    operator_emails: Vec<user::Email>,
}

impl BootstrapGrants {
    pub fn new(reader_emails: Vec<user::Email>, operator_emails: Vec<user::Email>) -> Self {
        Self {
            reader_emails,
            operator_emails,
        }
    }

    pub fn configured_emails(&self) -> Vec<user::Email> {
        let mut emails = self.reader_emails.clone();
        for email in &self.operator_emails {
            if !emails.contains(email) {
                emails.push(email.clone());
            }
        }
        emails
    }

    fn capabilities_for(&self, email: &user::Email) -> Vec<sensitive::AccessCapability> {
        let mut capabilities = Vec::new();
        if self.reader_emails.contains(email) || self.operator_emails.contains(email) {
            capabilities.push(sensitive::AccessCapability::AuthorizedRecordRead);
        }
        if self.operator_emails.contains(email) {
            capabilities.push(sensitive::AccessCapability::TokenStatusRead);
            capabilities.push(sensitive::AccessCapability::AccessAuditRead);
        }
        sorted_capabilities(capabilities)
    }
}

#[derive(Clone)]
pub struct Service {
    repo: Arc<dyn Repository>,
    provider: Arc<dyn ProviderClient>,
    clock: Arc<dyn Clock>,
    bootstrap: Arc<BootstrapGrants>,
}

impl Service {
    pub fn new(
        repo: Arc<dyn Repository>,
        provider: Arc<dyn ProviderClient>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            repo,
            provider,
            clock,
            bootstrap: Arc::new(BootstrapGrants::default()),
        }
    }

    pub fn with_bootstrap_grants(self, bootstrap: BootstrapGrants) -> Self {
        Self {
            repo: self.repo,
            provider: self.provider,
            clock: self.clock,
            bootstrap: Arc::new(bootstrap),
        }
    }

    pub fn disabled() -> Self {
        Self::new(
            Arc::new(DisabledRepository),
            Arc::new(DisabledProvider),
            Arc::new(DisabledClock),
        )
    }

    pub fn bootstrap_grants(&self) -> &BootstrapGrants {
        &self.bootstrap
    }

    pub async fn reconcile_bootstrap_grants_for_user(
        &self,
        user_id: user::Id,
        email: &user::Email,
    ) -> Result<Vec<sensitive::AccessCapability>> {
        let capabilities = self.bootstrap.capabilities_for(email);
        if !capabilities.is_empty() {
            self.repo
                .upsert_access_grants(&user_id, &capabilities, self.clock.now())
                .await?;
        }
        Ok(capabilities)
    }

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
    pub async fn run_key_rotation_pass(
        &self,
        limit: usize,
    ) -> Result<sensitive::KeyRotationRun> {
        let started_at = self.clock.now();
        let progress = self
            .repo
            .rotate_ciphertext_to_active_key(limit, started_at)
            .await?;
        let finished_at = self.clock.now();
        let outcome = if progress.rows_failed == 0 {
            sensitive::RotationOutcome::Success
        } else {
            sensitive::RotationOutcome::Failed
        };
        let run = sensitive::KeyRotationRun::builder()
            .active_key_id(progress.active_key_id)
            .outcome(outcome)
            .rows_scanned(progress.rows_scanned)
            .rows_rewrapped(progress.rows_rewrapped)
            .rows_already_current(progress.rows_already_current)
            .rows_failed(progress.rows_failed)
            .detail(progress.detail)
            .started_at(started_at)
            .finished_at(finished_at)
            .build();
        self.repo.record_key_rotation_run(&run).await?;
        tracing::info!(
            target: "demo.sensitive",
            outcome = %run.outcome,
            rows_scanned = run.rows_scanned,
            rows_rewrapped = run.rows_rewrapped,
            rows_failed = run.rows_failed,
            active_key_id = %run.active_key_id,
            "sensitive key rotation pass completed",
        );
        Ok(run)
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

    #[tracing::instrument(skip(self))]
    pub async fn snapshot(&self, viewer: Viewer) -> Result<Snapshot> {
        let stored = self.repo.load_snapshot().await?;
        let viewer_state = self.resolve_viewer_state(&viewer).await?;
        let first_record_id = stored.records.first().map(|record| record.id);
        let authorized_record = self
            .load_authorized_record_for_viewer(&viewer, &viewer_state, first_record_id)
            .await?;
        let key_custody = if viewer_state.allows_token_status() {
            Some(self.repo.load_key_custody().await?)
        } else {
            None
        };
        let token = if viewer_state.allows_token_status() {
            stored.token
        } else {
            None
        };
        let latest_sync = if viewer_state.allows_token_status() {
            stored.latest_sync
        } else {
            None
        };
        let integration_state = if viewer_state.allows_token_status() {
            stored.integration_state
        } else {
            None
        };
        let access_events = if viewer_state.allows_access_audit() {
            self.repo
                .list_recent_access_events(ACCESS_EVENT_LIMIT)
                .await?
        } else {
            Vec::new()
        };

        Ok(Snapshot::builder()
            .viewer(viewer_state)
            .maybe_token(token)
            .maybe_latest_sync(latest_sync)
            .maybe_integration_state(integration_state)
            .maybe_key_custody(key_custody)
            .records(stored.records)
            .maybe_authorized_record(authorized_record)
            .access_events(access_events)
            .build())
    }

    async fn resolve_viewer_state(&self, viewer: &Viewer) -> Result<ViewerState> {
        match viewer {
            Viewer::Guest => Ok(ViewerState::guest()),
            Viewer::Authenticated(viewer) => {
                let _ = self
                    .reconcile_bootstrap_grants_for_user(viewer.user_id, &viewer.email)
                    .await?;
                let grants = self.repo.load_access_grants(&viewer.user_id).await?;
                let capabilities = sorted_capabilities(
                    grants.into_iter().map(|grant| grant.capability).collect(),
                );
                Ok(ViewerState::authenticated(capabilities))
            }
        }
    }

    async fn load_authorized_record_for_viewer(
        &self,
        viewer: &Viewer,
        viewer_state: &ViewerState,
        record_id: Option<sensitive::Id>,
    ) -> Result<Option<sensitive::AuthorizedRecord>> {
        let Some(record_id) = record_id else {
            return Ok(None);
        };

        if viewer_state.allows_authorized_record() {
            let authorized_record = match self.repo.load_authorized_record(&record_id).await {
                Ok(record) => record,
                Err(error) if authorized_record_requires_denied_fallback(&error) => {
                    tracing::warn!(
                        target: "demo.sensitive",
                        ?error,
                        record_id = %record_id.as_ref(),
                        "authorized record could not be decrypted under the configured keyring",
                    );
                    None
                }
                Err(error) => return Err(error),
            };
            let (outcome, detail) = if authorized_record.is_some() {
                (
                    sensitive::AccessOutcome::Allowed,
                    "authorized record decrypted under persisted grant",
                )
            } else {
                (
                    sensitive::AccessOutcome::Denied,
                    "authorized record was not available at decrypt time",
                )
            };
            self.record_access_decision(viewer, outcome, Some(record_id), detail)
                .await?;
            return Ok(authorized_record);
        }

        self.record_access_decision(
            viewer,
            sensitive::AccessOutcome::Denied,
            Some(record_id),
            access_denial_detail(viewer),
        )
        .await?;
        Ok(None)
    }

    async fn record_access_decision(
        &self,
        viewer: &Viewer,
        outcome: sensitive::AccessOutcome,
        record_id: Option<sensitive::Id>,
        detail: impl Into<String>,
    ) -> Result<()> {
        let user_id = match viewer {
            Viewer::Guest => None,
            Viewer::Authenticated(viewer) => Some(viewer.user_id),
        };
        let event = sensitive::AccessEvent::builder()
            .maybe_user_id(user_id)
            .capability(sensitive::AccessCapability::AuthorizedRecordRead)
            .maybe_record_id(record_id)
            .outcome(outcome)
            .detail(access_detail_text(detail))
            .occurred_at(self.clock.now())
            .build();
        self.repo.record_access_event(&event).await
    }

    async fn ensure_fresh_token(
        &self,
        now: SystemTime,
    ) -> core::result::Result<(ProviderToken, sensitive::TokenStrategy), SyncAttemptFailure>
    {
        let current_token = self.load_refreshable_token().await.map_err(|error| {
            sync_attempt_failure(error, sensitive::TokenStrategy::RefreshedToken)
        })?;

        if let Some(token) = current_token {
            if !token_is_stale(&token, now) {
                return Ok((token, sensitive::TokenStrategy::CachedToken));
            }

            let refreshed = self
                .refresh_provider_token_inner(now, Some(&token.access_token))
                .await
                .map_err(|error| {
                    sync_attempt_failure(error, sensitive::TokenStrategy::RefreshedToken)
                })?;
            return Ok((refreshed, sensitive::TokenStrategy::RefreshedToken));
        }

        let refreshed =
            self.refresh_provider_token_inner(now, None)
                .await
                .map_err(|error| {
                    sync_attempt_failure(error, sensitive::TokenStrategy::RefreshedToken)
                })?;
        Ok((refreshed, sensitive::TokenStrategy::RefreshedToken))
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
    ) -> core::result::Result<(ProviderRecords, sensitive::TokenStrategy), SyncAttemptFailure>
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
                        )
                    })
            }
            Err(error) => Err(sync_attempt_failure(error, token_strategy)),
        }
    }

    async fn run_sync_success(
        &self,
        started_at: SystemTime,
        boundary_meta: &ProviderBoundaryMeta,
        previous_state: Option<&sensitive::IntegrationState>,
    ) -> core::result::Result<SyncSuccess, SyncFailure> {
        let previous_cursor = previous_state.and_then(|state| state.cursor.clone());
        let (token, token_strategy) =
            self.ensure_fresh_token(started_at)
                .await
                .map_err(|failure| SyncFailure {
                    integration_state: failed_integration_state(
                        boundary_meta,
                        previous_state,
                        previous_cursor.clone(),
                        failure.token_strategy,
                        failure.error_category,
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
            .map_err(|failure| SyncFailure {
                integration_state: failed_integration_state(
                    boundary_meta,
                    previous_state,
                    previous_cursor.clone(),
                    failure.token_strategy,
                    failure.error_category,
                    self.clock.now(),
                ),
                error: failure.error,
            })?;
        let records_seen = provider_records.records.len() as u32;
        let upserted = self
            .repo
            .upsert_records(&provider_records.records, started_at)
            .await
            .map_err(|error| SyncFailure {
                integration_state: failed_integration_state(
                    boundary_meta,
                    previous_state,
                    previous_cursor,
                    token_strategy,
                    provider_error_category(&error),
                    self.clock.now(),
                ),
                error,
            })?;
        let finished_at = self.clock.now();
        let integration_state = successful_integration_state(
            boundary_meta,
            provider_records.cursor,
            token_strategy,
            finished_at,
        );
        let run = sensitive::SyncRun::builder()
            .provider(PROVIDER)
            .outcome(sensitive::SyncOutcome::Success)
            .records_seen(records_seen)
            .records_upserted(upserted as u32)
            .detail(sync_detail_text(format!(
                "{} provider records processed through the local HTTP boundary",
                records_seen
            )))
            .started_at(started_at)
            .finished_at(finished_at)
            .build();

        Ok(SyncSuccess {
            run,
            integration_state,
        })
    }
}

struct SyncSuccess {
    run: sensitive::SyncRun,
    integration_state: sensitive::IntegrationState,
}

struct SyncFailure {
    error: Error,
    integration_state: sensitive::IntegrationState,
}

struct SyncAttemptFailure {
    error: Error,
    token_strategy: sensitive::TokenStrategy,
    error_category: Option<sensitive::RemoteErrorCategory>,
}

fn sync_attempt_failure(
    error: Error,
    token_strategy: sensitive::TokenStrategy,
) -> SyncAttemptFailure {
    let error_category = provider_error_category(&error);
    SyncAttemptFailure {
        error,
        token_strategy,
        error_category,
    }
}

fn viewer_tier_for_capabilities(
    capabilities: &[sensitive::AccessCapability],
) -> ViewerTier {
    if capabilities.contains(&sensitive::AccessCapability::TokenStatusRead)
        || capabilities.contains(&sensitive::AccessCapability::AccessAuditRead)
    {
        return ViewerTier::SensitiveOperator;
    }
    if capabilities.contains(&sensitive::AccessCapability::AuthorizedRecordRead) {
        return ViewerTier::SensitiveReader;
    }
    ViewerTier::Authenticated
}

fn sorted_capabilities(
    mut capabilities: Vec<sensitive::AccessCapability>,
) -> Vec<sensitive::AccessCapability> {
    capabilities.sort();
    capabilities.dedup();
    capabilities
}

fn token_is_stale(token: &ProviderToken, now: SystemTime) -> bool {
    let refresh_cutoff = now
        .checked_add(TOKEN_REFRESH_SKEW)
        .unwrap_or_else(|| now + TOKEN_REFRESH_SKEW);
    token.status.expires_at <= refresh_cutoff
}

fn stored_token_requires_refresh(error: &Error) -> bool {
    matches!(
        error,
        Error::Repository {
            source: error::RepositoryError::DecryptToken { .. },
        }
    )
}

fn authorized_record_requires_denied_fallback(error: &Error) -> bool {
    matches!(
        error,
        Error::Repository {
            source: error::RepositoryError::DecryptRecord { .. },
        }
    )
}

fn provider_error_category(error: &Error) -> Option<sensitive::RemoteErrorCategory> {
    match error {
        Error::Provider { kind, .. } => Some(match kind {
            error::ProviderFailureKind::Unauthorized => {
                sensitive::RemoteErrorCategory::Unauthorized
            }
            error::ProviderFailureKind::RateLimited => {
                sensitive::RemoteErrorCategory::RateLimited
            }
            error::ProviderFailureKind::MalformedPayload => {
                sensitive::RemoteErrorCategory::MalformedPayload
            }
            error::ProviderFailureKind::Transport => {
                sensitive::RemoteErrorCategory::Transport
            }
        }),
        _ => None,
    }
}

fn successful_integration_state(
    boundary_meta: &ProviderBoundaryMeta,
    cursor: Option<sensitive::SyncCursor>,
    token_strategy: sensitive::TokenStrategy,
    finished_at: SystemTime,
) -> sensitive::IntegrationState {
    sensitive::IntegrationState::builder()
        .provider(PROVIDER)
        .mode(boundary_meta.mode)
        .endpoint(boundary_meta.endpoint.clone())
        .maybe_cursor(cursor)
        .last_fetch_outcome(sensitive::FetchOutcome::Success)
        .token_strategy(token_strategy)
        .maybe_last_error_category(None)
        .maybe_last_successful_fetch_at(Some(finished_at))
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
    attempted_at: SystemTime,
) -> sensitive::IntegrationState {
    sensitive::IntegrationState::builder()
        .provider(PROVIDER)
        .mode(boundary_meta.mode)
        .endpoint(boundary_meta.endpoint.clone())
        .maybe_cursor(cursor)
        .last_fetch_outcome(sensitive::FetchOutcome::Failed)
        .token_strategy(token_strategy)
        .maybe_last_error_category(error_category)
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

fn debug_timestamp(value: SystemTime) -> u64 {
    value
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn sync_detail_text(message: impl Into<String>) -> sensitive::DetailText {
    bounded_detail_text(message, "runtime sync status")
}

fn access_detail_text(message: impl Into<String>) -> sensitive::DetailText {
    bounded_detail_text(message, "sensitive access decision")
}

fn rotation_detail_text(message: impl Into<String>) -> sensitive::DetailText {
    bounded_detail_text(message, "sensitive key rotation status")
}

fn bounded_detail_text(
    message: impl Into<String>,
    fallback: &'static str,
) -> sensitive::DetailText {
    let mut detail = message.into().trim().chars().take(120).collect::<String>();
    if detail.is_empty() {
        detail = fallback.to_string();
    }

    sensitive::DetailText::try_new(detail)
        .expect("detail text should be normalized to a valid bounded string")
}

fn access_denial_detail(viewer: &Viewer) -> &'static str {
    match viewer {
        Viewer::Guest => "sign in required before authorized record read",
        Viewer::Authenticated(_) => "viewer lacks authorized_record_read grant",
    }
}

struct DisabledRepository;

#[async_trait]
impl Repository for DisabledRepository {
    async fn load_snapshot(&self) -> Result<StoredSnapshot> {
        Ok(StoredSnapshot::builder()
            .maybe_token(None)
            .maybe_latest_sync(None)
            .maybe_integration_state(None)
            .records(Vec::new())
            .build())
    }

    async fn load_authorized_record(
        &self,
        _record_id: &sensitive::Id,
    ) -> Result<Option<sensitive::AuthorizedRecord>> {
        Ok(None)
    }

    async fn load_integration_state(
        &self,
        _provider: sensitive::Provider,
    ) -> Result<Option<sensitive::IntegrationState>> {
        Ok(None)
    }

    async fn load_key_custody(&self) -> Result<sensitive::KeyCustodyState> {
        Ok(sensitive::KeyCustodyState::builder()
            .active_key_id(sensitive::KeyId::try_new("disabled_data_key").unwrap())
            .configured_keys(Vec::new())
            .token_counts(Vec::new())
            .record_counts(Vec::new())
            .stale_token_count(0)
            .stale_record_count(0)
            .maybe_last_rotation_run(None)
            .build())
    }

    async fn load_access_grants(
        &self,
        _user_id: &user::Id,
    ) -> Result<Vec<sensitive::AccessGrant>> {
        Ok(Vec::new())
    }

    async fn load_token(
        &self,
        _provider: sensitive::Provider,
    ) -> Result<Option<ProviderToken>> {
        Ok(None)
    }

    async fn upsert_token(&self, _token: &ProviderToken) -> Result<()> {
        Ok(())
    }

    async fn upsert_records(
        &self,
        _records: &[sensitive::Record],
        _synced_at: SystemTime,
    ) -> Result<usize> {
        Ok(0)
    }

    async fn upsert_integration_state(
        &self,
        _state: &sensitive::IntegrationState,
    ) -> Result<()> {
        Ok(())
    }

    async fn rotate_ciphertext_to_active_key(
        &self,
        _limit: usize,
        _rotated_at: SystemTime,
    ) -> Result<KeyRotationProgress> {
        Ok(KeyRotationProgress::builder()
            .active_key_id(sensitive::KeyId::try_new("disabled_data_key").unwrap())
            .rows_scanned(0)
            .rows_rewrapped(0)
            .rows_already_current(0)
            .rows_failed(0)
            .detail(rotation_detail_text("no ciphertext rows required rewrap"))
            .build())
    }

    async fn record_sync_run(&self, _run: &sensitive::SyncRun) -> Result<()> {
        Ok(())
    }

    async fn record_key_rotation_run(
        &self,
        _run: &sensitive::KeyRotationRun,
    ) -> Result<()> {
        Ok(())
    }

    async fn upsert_access_grants(
        &self,
        _user_id: &user::Id,
        _capabilities: &[sensitive::AccessCapability],
        _granted_at: SystemTime,
    ) -> Result<()> {
        Ok(())
    }

    async fn record_access_event(&self, _event: &sensitive::AccessEvent) -> Result<()> {
        Ok(())
    }

    async fn list_recent_access_events(
        &self,
        _limit: usize,
    ) -> Result<Vec<sensitive::AccessEvent>> {
        Ok(Vec::new())
    }
}

struct DisabledProvider;

#[async_trait]
impl ProviderClient for DisabledProvider {
    fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
        ProviderBoundaryMeta::builder()
            .mode(sensitive::ProviderMode::LocalStub)
            .endpoint(
                sensitive::DetailText::try_new("disabled local boundary")
                    .expect("detail text"),
            )
            .build()
    }

    async fn refresh_token(
        &self,
        provider: sensitive::Provider,
        now: SystemTime,
        _current_token: Option<&SecretString>,
    ) -> Result<ProviderToken> {
        Ok(ProviderToken::builder()
            .status(
                sensitive::TokenStatus::builder()
                    .provider(provider)
                    .expires_at(now + Duration::from_secs(300))
                    .refreshed_at(now)
                    .build(),
            )
            .access_token(SecretString::new(
                "disabled-sensitive-token".to_string().into(),
            ))
            .build())
    }

    async fn fetch_records(
        &self,
        _provider: sensitive::Provider,
        _token: &ProviderToken,
        _cursor: Option<&sensitive::SyncCursor>,
        _now: SystemTime,
    ) -> Result<ProviderRecords> {
        Ok(ProviderRecords::builder()
            .records(Vec::new())
            .maybe_cursor(None)
            .build())
    }
}

struct DisabledClock;

impl Clock for DisabledClock {
    fn now(&self) -> SystemTime {
        UNIX_EPOCH
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct RepoState {
        snapshot: StoredSnapshot,
        key_custody: sensitive::KeyCustodyState,
        authorized_record: Option<sensitive::AuthorizedRecord>,
        token: Option<ProviderToken>,
        token_load_failures_remaining: usize,
        grants: Vec<sensitive::AccessGrant>,
        authorized_loads: usize,
        upserted_records: Vec<Vec<sensitive::Record>>,
        recorded_runs: Vec<sensitive::SyncRun>,
        recorded_rotation_runs: Vec<sensitive::KeyRotationRun>,
        access_events: Vec<sensitive::AccessEvent>,
        access_grant_writes: usize,
        token_writes: usize,
        external_ids: Vec<String>,
    }

    impl Default for RepoState {
        fn default() -> Self {
            Self {
                snapshot: StoredSnapshot::builder()
                    .maybe_token(None)
                    .maybe_latest_sync(None)
                    .maybe_integration_state(None)
                    .records(Vec::new())
                    .build(),
                key_custody: key_custody_state(),
                authorized_record: None,
                token: None,
                token_load_failures_remaining: 0,
                grants: Vec::new(),
                authorized_loads: 0,
                upserted_records: Vec::new(),
                recorded_runs: Vec::new(),
                recorded_rotation_runs: Vec::new(),
                access_events: Vec::new(),
                access_grant_writes: 0,
                token_writes: 0,
                external_ids: Vec::new(),
            }
        }
    }

    struct TestRepository {
        state: Mutex<RepoState>,
    }

    impl TestRepository {
        fn new(state: RepoState) -> Self {
            Self {
                state: Mutex::new(state),
            }
        }
    }

    #[async_trait]
    impl Repository for TestRepository {
        async fn load_snapshot(&self) -> Result<StoredSnapshot> {
            Ok(self.state.lock().expect("repo state").snapshot.clone())
        }

        async fn load_authorized_record(
            &self,
            _record_id: &sensitive::Id,
        ) -> Result<Option<sensitive::AuthorizedRecord>> {
            let mut state = self.state.lock().expect("repo state");
            state.authorized_loads += 1;
            Ok(state.authorized_record.clone())
        }

        async fn load_integration_state(
            &self,
            _provider: sensitive::Provider,
        ) -> Result<Option<sensitive::IntegrationState>> {
            Ok(self
                .state
                .lock()
                .expect("repo state")
                .snapshot
                .integration_state
                .clone())
        }

        async fn load_key_custody(&self) -> Result<sensitive::KeyCustodyState> {
            Ok(self.state.lock().expect("repo state").key_custody.clone())
        }

        async fn load_access_grants(
            &self,
            user_id: &user::Id,
        ) -> Result<Vec<sensitive::AccessGrant>> {
            let state = self.state.lock().expect("repo state");
            Ok(state
                .grants
                .iter()
                .filter(|grant| &grant.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn load_token(
            &self,
            _provider: sensitive::Provider,
        ) -> Result<Option<ProviderToken>> {
            let mut state = self.state.lock().expect("repo state");
            if state.token_load_failures_remaining > 0 {
                state.token_load_failures_remaining -= 1;
                return Err(Error::decrypt_token(std::io::Error::other(
                    "stored token ciphertext could not be decrypted",
                )));
            }

            Ok(state.token.clone())
        }

        async fn upsert_token(&self, token: &ProviderToken) -> Result<()> {
            let mut state = self.state.lock().expect("repo state");
            state.token = Some(token.clone());
            state.token_writes += 1;
            state.key_custody.token_counts =
                vec![sensitive::KeyedCiphertextCount::builder()
                    .key_id(active_key_id())
                    .count(1)
                    .build()];
            state.key_custody.stale_token_count = 0;
            Ok(())
        }

        async fn upsert_records(
            &self,
            records: &[sensitive::Record],
            synced_at: SystemTime,
        ) -> Result<usize> {
            let mut state = self.state.lock().expect("repo state");
            state.upserted_records.push(records.to_vec());

            let mut upserted = 0;
            for record in records {
                let external_id = record.external_id.to_string();
                if !state.external_ids.contains(&external_id) {
                    state.external_ids.push(external_id);
                    upserted += 1;
                }
            }

            state.snapshot.records = records
                .iter()
                .enumerate()
                .map(|(index, record)| {
                    RecordProof::builder()
                        .id(sensitive::Id::from(uuid::Uuid::from_u128(
                            index as u128 + 1,
                        )))
                        .label(record.label.clone())
                        .last4(record.last4.clone())
                        .synced_at(synced_at)
                        .ciphertext(
                            CiphertextEvidence::builder()
                                .key_id(active_key_id())
                                .preview("ciphertext-preview".to_string())
                                .bytes(32)
                                .build(),
                        )
                        .build()
                })
                .collect();
            state.key_custody.record_counts =
                vec![sensitive::KeyedCiphertextCount::builder()
                    .key_id(active_key_id())
                    .count(state.snapshot.records.len() as u32)
                    .build()];
            state.key_custody.stale_record_count = 0;

            Ok(upserted)
        }

        async fn upsert_integration_state(
            &self,
            integration_state: &sensitive::IntegrationState,
        ) -> Result<()> {
            let mut state = self.state.lock().expect("repo state");
            state.snapshot.integration_state = Some(integration_state.clone());
            Ok(())
        }

        async fn rotate_ciphertext_to_active_key(
            &self,
            limit: usize,
            _rotated_at: SystemTime,
        ) -> Result<KeyRotationProgress> {
            let mut state = self.state.lock().expect("repo state");
            let stale_token_count = state.key_custody.stale_token_count;
            let stale_record_count = state.key_custody.stale_record_count;
            let rows_scanned =
                stale_token_count.saturating_add(stale_record_count).min(limit as u32);
            let rows_rewrapped = rows_scanned;
            state.key_custody.stale_token_count = stale_token_count.saturating_sub(1);
            state.key_custody.stale_record_count =
                stale_record_count.saturating_sub(rows_rewrapped.saturating_sub(stale_token_count.min(1)));
            state.key_custody.token_counts =
                vec![sensitive::KeyedCiphertextCount::builder()
                    .key_id(active_key_id())
                    .count(1)
                    .build()];
            state.key_custody.record_counts =
                vec![sensitive::KeyedCiphertextCount::builder()
                    .key_id(active_key_id())
                    .count(state.snapshot.records.len() as u32)
                    .build()];

            Ok(KeyRotationProgress::builder()
                .active_key_id(active_key_id())
                .rows_scanned(rows_scanned)
                .rows_rewrapped(rows_rewrapped)
                .rows_already_current(0)
                .rows_failed(0)
                .detail(rotation_detail_text("stale ciphertext rewrapped to the active key"))
                .build())
        }

        async fn record_sync_run(&self, run: &sensitive::SyncRun) -> Result<()> {
            let mut state = self.state.lock().expect("repo state");
            state.snapshot.latest_sync = Some(run.clone());
            state.recorded_runs.push(run.clone());
            Ok(())
        }

        async fn record_key_rotation_run(
            &self,
            run: &sensitive::KeyRotationRun,
        ) -> Result<()> {
            let mut state = self.state.lock().expect("repo state");
            state.key_custody.last_rotation_run = Some(run.clone());
            state.recorded_rotation_runs.push(run.clone());
            Ok(())
        }

        async fn upsert_access_grants(
            &self,
            user_id: &user::Id,
            capabilities: &[sensitive::AccessCapability],
            granted_at: SystemTime,
        ) -> Result<()> {
            let mut state = self.state.lock().expect("repo state");
            for capability in capabilities {
                if state.grants.iter().any(|grant| {
                    grant.user_id == *user_id && grant.capability == *capability
                }) {
                    continue;
                }
                state.grants.push(
                    sensitive::AccessGrant::builder()
                        .user_id(*user_id)
                        .capability(*capability)
                        .granted_at(granted_at)
                        .build(),
                );
            }
            state.access_grant_writes += 1;
            Ok(())
        }

        async fn record_access_event(&self, event: &sensitive::AccessEvent) -> Result<()> {
            let mut state = self.state.lock().expect("repo state");
            state.access_events.push(event.clone());
            Ok(())
        }

        async fn list_recent_access_events(
            &self,
            limit: usize,
        ) -> Result<Vec<sensitive::AccessEvent>> {
            let state = self.state.lock().expect("repo state");
            Ok(state
                .access_events
                .iter()
                .rev()
                .take(limit)
                .cloned()
                .collect())
        }
    }

    struct TestProvider {
        token_refreshes: Mutex<usize>,
        records: Vec<sensitive::Record>,
    }

    #[async_trait]
    impl ProviderClient for TestProvider {
        fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
            ProviderBoundaryMeta::builder()
                .mode(sensitive::ProviderMode::LocalStub)
                .endpoint(
                    sensitive::DetailText::try_new("http://127.0.0.1:4001")
                        .expect("detail text"),
                )
                .build()
        }

        async fn refresh_token(
            &self,
            provider: sensitive::Provider,
            now: SystemTime,
            _current_token: Option<&SecretString>,
        ) -> Result<ProviderToken> {
            let mut refreshes = self.token_refreshes.lock().expect("refresh count");
            *refreshes += 1;
            Ok(ProviderToken::builder()
                .status(
                    sensitive::TokenStatus::builder()
                        .provider(provider)
                        .expires_at(now + Duration::from_secs(120))
                        .refreshed_at(now)
                        .build(),
                )
                .access_token(SecretString::new(format!("token-{}", *refreshes).into()))
                .build())
        }

        async fn fetch_records(
            &self,
            _provider: sensitive::Provider,
            _token: &ProviderToken,
            _cursor: Option<&sensitive::SyncCursor>,
            _now: SystemTime,
        ) -> Result<ProviderRecords> {
            Ok(ProviderRecords::builder()
                .records(self.records.clone())
                .maybe_cursor(Some(
                    sensitive::SyncCursor::try_new("cursor-alpha").unwrap(),
                ))
                .build())
        }
    }

    struct FixedClock {
        now: SystemTime,
    }

    impl Clock for FixedClock {
        fn now(&self) -> SystemTime {
            self.now
        }
    }

    fn example_record() -> sensitive::Record {
        sensitive::Record::builder()
            .external_id(sensitive::ExternalId::try_new("synthetic-alpha").unwrap())
            .label(sensitive::Label::try_new("Alpha file").unwrap())
            .last4(sensitive::Last4::try_new("1001").unwrap())
            .authorized(
                sensitive::AuthorizedFields::builder()
                    .subject_name(sensitive::DetailText::try_new("Case alpha").unwrap())
                    .classification(
                        sensitive::DetailText::try_new("synthetic_record").unwrap(),
                    )
                    .note(sensitive::DetailText::try_new("Authorized path only.").unwrap())
                    .build(),
            )
            .build()
    }

    fn active_key_id() -> sensitive::KeyId {
        sensitive::KeyId::try_new("active_data_key").unwrap()
    }

    fn legacy_key_id() -> sensitive::KeyId {
        sensitive::KeyId::try_new("legacy_data_key").unwrap()
    }

    fn key_custody_state() -> sensitive::KeyCustodyState {
        sensitive::KeyCustodyState::builder()
            .active_key_id(active_key_id())
            .configured_keys(vec![
                sensitive::ConfiguredKey::builder()
                    .key_id(active_key_id())
                    .status(sensitive::CipherKeyStatus::Active)
                    .build(),
                sensitive::ConfiguredKey::builder()
                    .key_id(legacy_key_id())
                    .status(sensitive::CipherKeyStatus::ReadOnlyLegacy)
                    .build(),
            ])
            .token_counts(vec![sensitive::KeyedCiphertextCount::builder()
                .key_id(legacy_key_id())
                .count(1)
                .build()])
            .record_counts(vec![sensitive::KeyedCiphertextCount::builder()
                .key_id(legacy_key_id())
                .count(1)
                .build()])
            .stale_token_count(1)
            .stale_record_count(1)
            .maybe_last_rotation_run(None)
            .build()
    }

    fn record_proof(record_id: sensitive::Id) -> RecordProof {
        RecordProof::builder()
            .id(record_id)
            .label(sensitive::Label::try_new("Alpha file").unwrap())
            .last4(sensitive::Last4::try_new("1001").unwrap())
            .synced_at(UNIX_EPOCH)
            .ciphertext(
                CiphertextEvidence::builder()
                    .key_id(legacy_key_id())
                    .preview("abc123".to_string())
                    .bytes(24)
                    .build(),
            )
            .build()
    }

    fn authorized_record(record_id: sensitive::Id) -> sensitive::AuthorizedRecord {
        sensitive::AuthorizedRecord::builder()
            .id(record_id)
            .label(sensitive::Label::try_new("Alpha file").unwrap())
            .last4(sensitive::Last4::try_new("1001").unwrap())
            .authorized(example_record().authorized)
            .synced_at(UNIX_EPOCH)
            .build()
    }

    fn test_user_id() -> user::Id {
        user::Id::from(uuid::Uuid::from_u128(0xfeed))
    }

    fn authenticated_viewer(email: &str) -> Viewer {
        Viewer::Authenticated(
            AuthenticatedViewer::builder()
                .user_id(test_user_id())
                .email(user::Email::try_new(email).unwrap())
                .build(),
        )
    }

    fn access_grant(capability: sensitive::AccessCapability) -> sensitive::AccessGrant {
        sensitive::AccessGrant::builder()
            .user_id(test_user_id())
            .capability(capability)
            .granted_at(UNIX_EPOCH)
            .build()
    }

    fn token_proof() -> TokenProof {
        TokenProof::builder()
            .status(
                sensitive::TokenStatus::builder()
                    .provider(PROVIDER)
                    .expires_at(UNIX_EPOCH + Duration::from_secs(600))
                    .refreshed_at(UNIX_EPOCH)
                    .build(),
            )
            .ciphertext(
                CiphertextEvidence::builder()
                    .key_id(legacy_key_id())
                    .preview("token-preview".to_string())
                    .bytes(64)
                    .build(),
            )
            .build()
    }

    fn latest_sync() -> sensitive::SyncRun {
        sensitive::SyncRun::builder()
            .provider(PROVIDER)
            .outcome(sensitive::SyncOutcome::Success)
            .records_seen(1)
            .records_upserted(1)
            .detail(sync_detail_text(
                "1 synthetic record processed for runtime proof",
            ))
            .started_at(UNIX_EPOCH)
            .finished_at(UNIX_EPOCH + Duration::from_secs(1))
            .build()
    }

    fn service_with_state(state: RepoState) -> (Service, Arc<TestRepository>) {
        let repo = Arc::new(TestRepository::new(state));
        let provider = Arc::new(TestProvider {
            token_refreshes: Mutex::new(0),
            records: vec![example_record()],
        });
        let service = Service::new(
            repo.clone(),
            provider,
            Arc::new(FixedClock {
                now: UNIX_EPOCH + Duration::from_secs(100),
            }),
        );
        (service, repo)
    }

    struct FailingProvider;

    #[async_trait]
    impl ProviderClient for FailingProvider {
        fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
            ProviderBoundaryMeta::builder()
                .mode(sensitive::ProviderMode::LocalStub)
                .endpoint(
                    sensitive::DetailText::try_new("http://127.0.0.1:4001")
                        .expect("detail text"),
                )
                .build()
        }

        async fn refresh_token(
            &self,
            provider: sensitive::Provider,
            now: SystemTime,
            _current_token: Option<&SecretString>,
        ) -> Result<ProviderToken> {
            Ok(ProviderToken::builder()
                .status(
                    sensitive::TokenStatus::builder()
                        .provider(provider)
                        .expires_at(now + Duration::from_secs(120))
                        .refreshed_at(now)
                        .build(),
                )
                .access_token(SecretString::new("token-fail".to_string().into()))
                .build())
        }

        async fn fetch_records(
            &self,
            _provider: sensitive::Provider,
            _token: &ProviderToken,
            _cursor: Option<&sensitive::SyncCursor>,
            _now: SystemTime,
        ) -> Result<ProviderRecords> {
            Err(Error::provider_request(
                ProviderOperation::FetchRecords,
                std::io::Error::other("synthetic provider offline"),
            ))
        }
    }

    struct RetryAfterUnauthorizedProvider {
        unauthorized_remaining: Mutex<usize>,
        records: Vec<sensitive::Record>,
    }

    #[async_trait]
    impl ProviderClient for RetryAfterUnauthorizedProvider {
        fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
            ProviderBoundaryMeta::builder()
                .mode(sensitive::ProviderMode::LocalStub)
                .endpoint(
                    sensitive::DetailText::try_new("http://127.0.0.1:4001")
                        .expect("detail text"),
                )
                .build()
        }

        async fn refresh_token(
            &self,
            provider: sensitive::Provider,
            now: SystemTime,
            _current_token: Option<&SecretString>,
        ) -> Result<ProviderToken> {
            Ok(ProviderToken::builder()
                .status(
                    sensitive::TokenStatus::builder()
                        .provider(provider)
                        .expires_at(now + Duration::from_secs(120))
                        .refreshed_at(now)
                        .build(),
                )
                .access_token(SecretString::new("token-retry".to_string().into()))
                .build())
        }

        async fn fetch_records(
            &self,
            _provider: sensitive::Provider,
            _token: &ProviderToken,
            _cursor: Option<&sensitive::SyncCursor>,
            _now: SystemTime,
        ) -> Result<ProviderRecords> {
            let mut unauthorized_remaining =
                self.unauthorized_remaining.lock().expect("retry state");
            if *unauthorized_remaining > 0 {
                *unauthorized_remaining -= 1;
                return Err(Error::provider_failure(
                    ProviderOperation::FetchRecords,
                    ProviderFailureKind::Unauthorized,
                    std::io::Error::other("provider forced unauthorized retry"),
                ));
            }

            Ok(ProviderRecords::builder()
                .records(self.records.clone())
                .maybe_cursor(Some(
                    sensitive::SyncCursor::try_new("cursor-retried").unwrap(),
                ))
                .build())
        }
    }

    #[tokio::test]
    async fn guest_snapshot_records_denied_access_without_loading_authorized_record() {
        let record_id = sensitive::Id::new_v4();
        let (service, repo) = service_with_state(RepoState {
            snapshot: StoredSnapshot::builder()
                .maybe_token(None)
                .maybe_latest_sync(None)
                .records(vec![record_proof(record_id)])
                .build(),
            authorized_record: Some(authorized_record(record_id)),
            ..RepoState::default()
        });

        let snapshot = service.snapshot(Viewer::Guest).await.expect("snapshot");

        assert!(snapshot.authorized_record.is_none());
        assert_eq!(snapshot.viewer.tier, ViewerTier::Guest);
        let state = repo.state.lock().unwrap();
        assert_eq!(state.authorized_loads, 0);
        assert_eq!(state.access_events.len(), 1);
        assert_eq!(
            state.access_events[0].outcome,
            sensitive::AccessOutcome::Denied
        );
        assert_eq!(state.access_events[0].user_id, None);
    }

    #[tokio::test]
    async fn authenticated_snapshot_without_grant_records_denied_access() {
        let record_id = sensitive::Id::new_v4();
        let (service, repo) = service_with_state(RepoState {
            snapshot: StoredSnapshot::builder()
                .maybe_token(None)
                .maybe_latest_sync(None)
                .records(vec![record_proof(record_id)])
                .build(),
            authorized_record: Some(authorized_record(record_id)),
            ..RepoState::default()
        });

        let snapshot = service
            .snapshot(authenticated_viewer("reader@example.com"))
            .await
            .expect("snapshot");

        assert!(snapshot.authorized_record.is_none());
        assert_eq!(snapshot.viewer.tier, ViewerTier::Authenticated);
        let state = repo.state.lock().unwrap();
        assert_eq!(state.authorized_loads, 0);
        assert_eq!(state.access_events.len(), 1);
        assert_eq!(
            state.access_events[0].outcome,
            sensitive::AccessOutcome::Denied
        );
        assert_eq!(state.access_events[0].user_id, Some(test_user_id()));
    }

    #[tokio::test]
    async fn authorized_record_grant_loads_authorized_record_and_records_allowed_event() {
        let record_id = sensitive::Id::new_v4();
        let authorized = authorized_record(record_id);
        let (service, repo) = service_with_state(RepoState {
            snapshot: StoredSnapshot::builder()
                .maybe_token(None)
                .maybe_latest_sync(None)
                .records(vec![record_proof(record_id)])
                .build(),
            authorized_record: Some(authorized.clone()),
            grants: vec![access_grant(
                sensitive::AccessCapability::AuthorizedRecordRead,
            )],
            ..RepoState::default()
        });

        let snapshot = service
            .snapshot(authenticated_viewer("reader@example.com"))
            .await
            .expect("snapshot");

        assert_eq!(snapshot.viewer.tier, ViewerTier::SensitiveReader);
        assert_eq!(snapshot.authorized_record, Some(authorized));
        let state = repo.state.lock().unwrap();
        assert_eq!(state.authorized_loads, 1);
        assert_eq!(state.access_events.len(), 1);
        assert_eq!(
            state.access_events[0].outcome,
            sensitive::AccessOutcome::Allowed
        );
    }

    #[tokio::test]
    async fn authorized_grant_records_denied_when_record_disappears_after_snapshot() {
        let record_id = sensitive::Id::new_v4();
        let (service, repo) = service_with_state(RepoState {
            snapshot: StoredSnapshot::builder()
                .maybe_token(None)
                .maybe_latest_sync(None)
                .records(vec![record_proof(record_id)])
                .build(),
            authorized_record: None,
            grants: vec![access_grant(
                sensitive::AccessCapability::AuthorizedRecordRead,
            )],
            ..RepoState::default()
        });

        let snapshot = service
            .snapshot(authenticated_viewer("reader@example.com"))
            .await
            .expect("snapshot");

        assert!(snapshot.authorized_record.is_none());
        let state = repo.state.lock().unwrap();
        assert_eq!(state.authorized_loads, 1);
        assert_eq!(state.access_events.len(), 1);
        assert_eq!(
            state.access_events[0].outcome,
            sensitive::AccessOutcome::Denied
        );
    }

    #[tokio::test]
    async fn operator_snapshot_exposes_token_sync_and_audit() {
        let record_id = sensitive::Id::new_v4();
        let (service, repo) = service_with_state(RepoState {
            snapshot: StoredSnapshot::builder()
                .maybe_token(Some(token_proof()))
                .maybe_latest_sync(Some(latest_sync()))
                .records(vec![record_proof(record_id)])
                .build(),
            authorized_record: Some(authorized_record(record_id)),
            grants: vec![
                access_grant(sensitive::AccessCapability::AuthorizedRecordRead),
                access_grant(sensitive::AccessCapability::TokenStatusRead),
                access_grant(sensitive::AccessCapability::AccessAuditRead),
            ],
            access_events: vec![
                sensitive::AccessEvent::builder()
                    .maybe_user_id(Some(test_user_id()))
                    .capability(sensitive::AccessCapability::AuthorizedRecordRead)
                    .maybe_record_id(Some(record_id))
                    .outcome(sensitive::AccessOutcome::Denied)
                    .detail(access_detail_text(
                        "viewer lacks authorized_record_read grant",
                    ))
                    .occurred_at(UNIX_EPOCH)
                    .build(),
            ],
            ..RepoState::default()
        });

        let snapshot = service
            .snapshot(authenticated_viewer("operator@example.com"))
            .await
            .expect("snapshot");

        assert_eq!(snapshot.viewer.tier, ViewerTier::SensitiveOperator);
        assert!(snapshot.token.is_some());
        assert!(snapshot.latest_sync.is_some());
        assert!(snapshot.key_custody.is_some());
        assert_eq!(snapshot.access_events.len(), 2);
        assert_eq!(
            snapshot.access_events[0].outcome,
            sensitive::AccessOutcome::Allowed
        );
        assert_eq!(
            snapshot.access_events[1].outcome,
            sensitive::AccessOutcome::Denied
        );
        assert_eq!(repo.state.lock().unwrap().authorized_loads, 1);
    }

    #[tokio::test]
    async fn reader_snapshot_hides_token_and_access_audit() {
        let record_id = sensitive::Id::new_v4();
        let (service, _repo) = service_with_state(RepoState {
            snapshot: StoredSnapshot::builder()
                .maybe_token(Some(token_proof()))
                .maybe_latest_sync(Some(latest_sync()))
                .records(vec![record_proof(record_id)])
                .build(),
            authorized_record: Some(authorized_record(record_id)),
            grants: vec![access_grant(
                sensitive::AccessCapability::AuthorizedRecordRead,
            )],
            ..RepoState::default()
        });

        let snapshot = service
            .snapshot(authenticated_viewer("reader@example.com"))
            .await
            .expect("snapshot");

        assert_eq!(snapshot.viewer.tier, ViewerTier::SensitiveReader);
        assert!(snapshot.token.is_none());
        assert!(snapshot.latest_sync.is_none());
        assert!(snapshot.access_events.is_empty());
    }

    #[tokio::test]
    async fn bootstrap_operator_email_upserts_expected_grants() {
        let (service, repo) = service_with_state(RepoState::default());
        let service = service.with_bootstrap_grants(BootstrapGrants::new(
            Vec::new(),
            vec![user::Email::try_new("operator@example.com").unwrap()],
        ));

        let capabilities = service
            .reconcile_bootstrap_grants_for_user(
                test_user_id(),
                &user::Email::try_new("operator@example.com").unwrap(),
            )
            .await
            .expect("bootstrap grants");

        assert_eq!(
            capabilities,
            vec![
                sensitive::AccessCapability::AuthorizedRecordRead,
                sensitive::AccessCapability::TokenStatusRead,
                sensitive::AccessCapability::AccessAuditRead,
            ]
        );
        let state = repo.state.lock().unwrap();
        assert_eq!(state.access_grant_writes, 1);
        assert_eq!(state.grants.len(), 3);
    }

    #[tokio::test]
    async fn refresh_provider_token_writes_new_token() {
        let (service, repo) = service_with_state(RepoState::default());

        let token = service
            .refresh_provider_token()
            .await
            .expect("token refresh");

        assert_eq!(token.provider, PROVIDER);
        assert_eq!(repo.state.lock().unwrap().token_writes, 1);
    }

    #[tokio::test]
    async fn refresh_provider_token_recovers_from_undecryptable_stored_token() {
        let (service, repo) = service_with_state(RepoState {
            token_load_failures_remaining: 1,
            ..RepoState::default()
        });

        let token = service
            .refresh_provider_token()
            .await
            .expect("token refresh should recover");

        assert_eq!(token.provider, PROVIDER);
        let state = repo.state.lock().unwrap();
        assert_eq!(state.token_writes, 1);
        assert_eq!(state.token_load_failures_remaining, 0);
    }

    #[tokio::test]
    async fn run_sync_refreshes_missing_token_and_records_idempotent_runs() {
        let (service, repo) = service_with_state(RepoState::default());

        let first = service.run_sync().await.expect("first sync");
        let second = service.run_sync().await.expect("second sync");

        assert_eq!(first.records_seen, 1);
        assert_eq!(first.records_upserted, 1);
        assert_eq!(second.records_seen, 1);
        assert_eq!(second.records_upserted, 0);
        assert_eq!(repo.state.lock().unwrap().recorded_runs.len(), 2);
    }

    #[tokio::test]
    async fn failed_sync_records_failed_outcome() {
        let repo = Arc::new(TestRepository::new(RepoState::default()));
        let service = Service::new(
            repo.clone(),
            Arc::new(FailingProvider),
            Arc::new(FixedClock {
                now: UNIX_EPOCH + Duration::from_secs(100),
            }),
        );

        let error = service.run_sync().await.expect_err("sync should fail");

        assert!(matches!(
            error,
            Error::Provider {
                operation: ProviderOperation::FetchRecords,
                ..
            }
        ));
        let state = repo.state.lock().unwrap();
        assert_eq!(state.recorded_runs.len(), 1);
        assert_eq!(
            state.recorded_runs[0].outcome,
            sensitive::SyncOutcome::Failed
        );
    }

    #[tokio::test]
    async fn unauthorized_fetch_retries_with_refreshed_token_and_records_boundary_state() {
        let repo = Arc::new(TestRepository::new(RepoState::default()));
        let service = Service::new(
            repo.clone(),
            Arc::new(RetryAfterUnauthorizedProvider {
                unauthorized_remaining: Mutex::new(1),
                records: vec![example_record()],
            }),
            Arc::new(FixedClock {
                now: UNIX_EPOCH + Duration::from_secs(100),
            }),
        );

        let run = service.run_sync().await.expect("sync should recover");

        assert_eq!(run.records_seen, 1);
        assert_eq!(run.records_upserted, 1);
        let state = repo.state.lock().unwrap();
        assert_eq!(state.token_writes, 2);
        assert_eq!(
            state
                .snapshot
                .integration_state
                .as_ref()
                .map(|value| value.token_strategy),
            Some(sensitive::TokenStrategy::RetryAfterUnauthorized)
        );
        assert_eq!(
            state
                .snapshot
                .integration_state
                .as_ref()
                .and_then(|value| value.cursor.as_ref().map(ToString::to_string)),
            Some("cursor-retried".to_string())
        );
    }

    #[tokio::test]
    async fn key_rotation_pass_rewraps_stale_ciphertext_and_records_run() {
        let (service, repo) = service_with_state(RepoState::default());

        let run = service
            .run_key_rotation_pass(10)
            .await
            .expect("rotation pass should succeed");

        assert_eq!(run.outcome, sensitive::RotationOutcome::Success);
        assert_eq!(run.rows_rewrapped, 2);
        let state = repo.state.lock().unwrap();
        assert_eq!(state.recorded_rotation_runs.len(), 1);
        assert_eq!(state.key_custody.stale_token_count, 0);
        assert_eq!(state.key_custody.stale_record_count, 0);
        assert_eq!(state.key_custody.active_key_id, active_key_id());
    }
}
