use super::*;

impl Repository {
    pub(super) async fn load_snapshot_impl(
        &self,
    ) -> sensitive::Result<sensitive::StoredSnapshot> {
        let token = self.query_token_proof().await?;
        let latest_sync = self.query_latest_sync_run().await?;
        let integration_state = self.query_integration_state().await?;
        let records = self.query_record_proofs().await?;

        Ok(sensitive::StoredSnapshot::builder()
            .maybe_token(token)
            .maybe_latest_sync(latest_sync)
            .maybe_integration_state(integration_state)
            .records(records)
            .build())
    }

    pub(super) async fn load_integration_state_impl(
        &self,
        provider: sensitive_domain::Provider,
    ) -> sensitive::Result<Option<sensitive_domain::IntegrationState>> {
        let row = sqlx::query(
            r#"
            SELECT provider, mode, endpoint, auth_mode, cursor, last_fetch_outcome,
                   token_strategy, last_error_category, last_auth_outcome,
                   last_remote_status_code, retry_backoff_secs, last_successful_mode,
                   last_successful_fetch_at, last_attempted_fetch_at, failure_count
            FROM integration_sync_state
            WHERE provider = $1
            "#,
        )
        .bind(provider.as_ref())
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::LoadIntegrationState,
                source,
            )
        })?;

        row.map(mapping::integration_state_from_row).transpose()
    }

    pub(super) async fn load_key_custody_impl(
        &self,
    ) -> sensitive::Result<sensitive_domain::KeyCustodyState> {
        let active_key_id = self.crypto.active_key_id().clone();
        let token_counts = self.query_token_counts_by_key().await?;
        let record_counts = self.query_record_counts_by_key().await?;
        let stale_token_count = self.query_stale_token_count(&active_key_id).await?;
        let stale_record_count = self.query_stale_record_count(&active_key_id).await?;
        let last_rotation_run = self.query_latest_rotation_run().await?;

        Ok(sensitive_domain::KeyCustodyState::builder()
            .active_key_id(active_key_id)
            .configured_keys(self.crypto.configured_keys())
            .token_counts(token_counts)
            .record_counts(record_counts)
            .stale_token_count(stale_token_count)
            .stale_record_count(stale_record_count)
            .maybe_last_rotation_run(last_rotation_run)
            .build())
    }

    pub(super) async fn upsert_integration_state_impl(
        &self,
        state: &sensitive_domain::IntegrationState,
    ) -> sensitive::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO integration_sync_state (
                provider,
                mode,
                endpoint,
                auth_mode,
                cursor,
                last_fetch_outcome,
                token_strategy,
                last_error_category,
                last_auth_outcome,
                last_remote_status_code,
                retry_backoff_secs,
                last_successful_mode,
                last_successful_fetch_at,
                last_attempted_fetch_at,
                failure_count
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (provider) DO UPDATE
            SET mode = EXCLUDED.mode,
                endpoint = EXCLUDED.endpoint,
                auth_mode = EXCLUDED.auth_mode,
                cursor = EXCLUDED.cursor,
                last_fetch_outcome = EXCLUDED.last_fetch_outcome,
                token_strategy = EXCLUDED.token_strategy,
                last_error_category = EXCLUDED.last_error_category,
                last_auth_outcome = EXCLUDED.last_auth_outcome,
                last_remote_status_code = EXCLUDED.last_remote_status_code,
                retry_backoff_secs = EXCLUDED.retry_backoff_secs,
                last_successful_mode = EXCLUDED.last_successful_mode,
                last_successful_fetch_at = EXCLUDED.last_successful_fetch_at,
                last_attempted_fetch_at = EXCLUDED.last_attempted_fetch_at,
                failure_count = EXCLUDED.failure_count,
                updated_at = now()
            "#,
        )
        .bind(state.provider.as_ref())
        .bind(state.mode.as_ref())
        .bind(state.endpoint.to_string())
        .bind(state.auth_mode.map(|mode| mode.as_ref().to_string()))
        .bind(state.cursor.as_ref().map(ToString::to_string))
        .bind(state.last_fetch_outcome.as_ref())
        .bind(state.token_strategy.as_ref())
        .bind(
            state
                .last_error_category
                .map(|category| category.as_ref().to_string()),
        )
        .bind(
            state
                .last_auth_outcome
                .map(|outcome| outcome.as_ref().to_string()),
        )
        .bind(state.last_remote_status_code.map(i64::from))
        .bind(state.retry_backoff_secs.map(i64::from))
        .bind(
            state
                .last_successful_mode
                .map(|mode| mode.as_ref().to_string()),
        )
        .bind(
            state
                .last_successful_fetch_at
                .map(mapping::to_offset_datetime),
        )
        .bind(mapping::to_offset_datetime(state.last_attempted_fetch_at))
        .bind(state.failure_count as i64)
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::UpsertIntegrationState,
                source,
            )
        })?;

        Ok(())
    }

    pub(super) async fn record_sync_run_impl(
        &self,
        run: &sensitive_domain::SyncRun,
    ) -> sensitive::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sync_runs (
                provider,
                outcome,
                records_seen,
                records_upserted,
                detail,
                started_at,
                finished_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(run.provider.as_ref())
        .bind(run.outcome.as_ref())
        .bind(run.records_seen as i32)
        .bind(run.records_upserted as i32)
        .bind(run.detail.to_string())
        .bind(mapping::to_offset_datetime(run.started_at))
        .bind(mapping::to_offset_datetime(run.finished_at))
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::RecordSyncRun,
                source,
            )
        })?;

        Ok(())
    }

    async fn query_token_proof(&self) -> sensitive::Result<Option<sensitive::TokenProof>> {
        let row = sqlx::query(
            r#"
            SELECT provider, token_key_id, token_ciphertext, expires_at, refreshed_at
            FROM integration_credentials
            WHERE provider = $1
            "#,
        )
        .bind(sensitive_domain::Provider::SyntheticSecureFeed.as_ref())
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::LoadSnapshot,
                source,
            )
        })?;

        row.map(|row| {
            let provider_raw = row.get::<String, _>("provider");
            let provider =
                provider_raw
                    .parse::<sensitive_domain::Provider>()
                    .map_err(|_| sensitive::failure::Error::InvalidStoredProvider {
                        provider: provider_raw.clone(),
                    })?;
            Ok(sensitive::TokenProof::builder()
                .status(
                    sensitive_domain::TokenStatus::builder()
                        .provider(provider)
                        .expires_at(mapping::from_offset_datetime(row.get("expires_at")))
                        .refreshed_at(mapping::from_offset_datetime(
                            row.get("refreshed_at"),
                        ))
                        .build(),
                )
                .ciphertext(mapping::ciphertext_evidence(
                    mapping::parse_key_id(row.get::<String, _>("token_key_id"))?,
                    &row.get::<Vec<u8>, _>("token_ciphertext"),
                ))
                .build())
        })
        .transpose()
    }

    async fn query_latest_sync_run(
        &self,
    ) -> sensitive::Result<Option<sensitive_domain::SyncRun>> {
        let row = sqlx::query(
            r#"
            SELECT provider, outcome, records_seen, records_upserted, detail, started_at, finished_at
            FROM sync_runs
            WHERE provider = $1
            ORDER BY finished_at DESC
            LIMIT 1
            "#,
        )
        .bind(sensitive_domain::Provider::SyntheticSecureFeed.as_ref())
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| sensitive::failure::Error::query_repository(
            RepositoryOperation::LoadSnapshot,
            source,
        ))?;

        row.map(mapping::sync_run_from_row).transpose()
    }

    async fn query_integration_state(
        &self,
    ) -> sensitive::Result<Option<sensitive_domain::IntegrationState>> {
        let row = sqlx::query(
            r#"
            SELECT provider, mode, endpoint, auth_mode, cursor, last_fetch_outcome,
                   token_strategy, last_error_category, last_auth_outcome,
                   last_remote_status_code, retry_backoff_secs, last_successful_mode,
                   last_successful_fetch_at, last_attempted_fetch_at, failure_count
            FROM integration_sync_state
            WHERE provider = $1
            "#,
        )
        .bind(sensitive_domain::Provider::SyntheticSecureFeed.as_ref())
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::LoadIntegrationState,
                source,
            )
        })?;

        row.map(mapping::integration_state_from_row).transpose()
    }

    async fn query_latest_rotation_run(
        &self,
    ) -> sensitive::Result<Option<sensitive_domain::KeyRotationRun>> {
        let row = sqlx::query(
            r#"
            SELECT active_key_id, outcome, rows_scanned, rows_rewrapped,
                   rows_already_current, rows_failed, detail, started_at, finished_at
            FROM key_rotation_runs
            ORDER BY finished_at DESC, created_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::LoadKeyCustody,
                source,
            )
        })?;

        row.map(mapping::rotation_run_from_row).transpose()
    }

    async fn query_keyed_counts(
        &self,
        query: &str,
        operation: RepositoryOperation,
    ) -> sensitive::Result<Vec<sensitive_domain::KeyedCiphertextCount>> {
        let rows = sqlx::query(query)
            .fetch_all(&self.pg)
            .await
            .map_err(|source| {
                sensitive::failure::Error::query_repository(operation, source)
            })?;

        rows.into_iter()
            .map(|row| {
                Ok(sensitive_domain::KeyedCiphertextCount::builder()
                    .key_id(mapping::parse_key_id(row.get::<String, _>("key_id"))?)
                    .count(mapping::parse_u32_count(
                        row.get::<i64, _>("ciphertext_count"),
                        "ciphertext_count",
                    )?)
                    .build())
            })
            .collect()
    }

    async fn query_token_counts_by_key(
        &self,
    ) -> sensitive::Result<Vec<sensitive_domain::KeyedCiphertextCount>> {
        self.query_keyed_counts(
            r#"
            SELECT token_key_id AS key_id, COUNT(*)::BIGINT AS ciphertext_count
            FROM integration_credentials
            GROUP BY token_key_id
            ORDER BY token_key_id ASC
            "#,
            RepositoryOperation::LoadKeyCustody,
        )
        .await
    }

    async fn query_record_counts_by_key(
        &self,
    ) -> sensitive::Result<Vec<sensitive_domain::KeyedCiphertextCount>> {
        self.query_keyed_counts(
            r#"
            SELECT authorized_key_id AS key_id, COUNT(*)::BIGINT AS ciphertext_count
            FROM sensitive_records
            GROUP BY authorized_key_id
            ORDER BY authorized_key_id ASC
            "#,
            RepositoryOperation::LoadKeyCustody,
        )
        .await
    }

    async fn query_stale_count(
        &self,
        query: &str,
        active_key_id: &sensitive_domain::KeyId,
    ) -> sensitive::Result<u32> {
        let count = sqlx::query_scalar::<_, i64>(query)
            .bind(active_key_id.to_string())
            .fetch_one(&self.pg)
            .await
            .map_err(|source| {
                sensitive::failure::Error::query_repository(
                    RepositoryOperation::LoadKeyCustody,
                    source,
                )
            })?;

        mapping::parse_u32_count(count, "stale_count")
    }

    async fn query_stale_token_count(
        &self,
        active_key_id: &sensitive_domain::KeyId,
    ) -> sensitive::Result<u32> {
        self.query_stale_count(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM integration_credentials
            WHERE token_key_id <> $1
            "#,
            active_key_id,
        )
        .await
    }

    async fn query_stale_record_count(
        &self,
        active_key_id: &sensitive_domain::KeyId,
    ) -> sensitive::Result<u32> {
        self.query_stale_count(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM sensitive_records
            WHERE authorized_key_id <> $1
            "#,
            active_key_id,
        )
        .await
    }

    async fn query_record_proofs(&self) -> sensitive::Result<Vec<sensitive::RecordProof>> {
        let rows = sqlx::query(
            r#"
            SELECT id, redacted_label, redacted_last4, authorized_key_id, authorized_ciphertext, synced_at
            FROM sensitive_records
            ORDER BY synced_at DESC, redacted_label ASC
            "#,
        )
        .fetch_all(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(RepositoryOperation::LoadSnapshot, source)
        })?;

        rows.into_iter()
            .map(mapping::record_proof_from_row)
            .collect()
    }
}
