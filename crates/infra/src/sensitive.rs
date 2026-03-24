use std::time::SystemTime;

use app::sensitive::{self, RepositoryOperation};
use async_trait::async_trait;
use domain::{sensitive as sensitive_domain, user as user_domain};
use secrecy::{ExposeSecret, SecretString};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row, types::time::OffsetDateTime};

use crate::crypto::{Keyring, SealedValue};

pub struct Repository {
    pg: PgPool,
    crypto: Keyring,
}

impl Repository {
    pub fn new(pg: PgPool, crypto: Keyring) -> Self {
        Self { pg, crypto }
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
            sensitive::Error::query_repository(RepositoryOperation::LoadSnapshot, source)
        })?;

        row.map(|row| {
            let provider_raw = row.get::<String, _>("provider");
            let provider =
                provider_raw
                    .parse::<sensitive_domain::Provider>()
                    .map_err(|_| sensitive::Error::InvalidStoredProvider {
                        provider: provider_raw.clone(),
                    })?;
            Ok(sensitive::TokenProof::builder()
                .status(
                    sensitive_domain::TokenStatus::builder()
                        .provider(provider)
                        .expires_at(from_offset_datetime(row.get("expires_at")))
                        .refreshed_at(from_offset_datetime(row.get("refreshed_at")))
                        .build(),
                )
                .ciphertext(ciphertext_evidence(
                    parse_key_id(row.get::<String, _>("token_key_id"))?,
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
        .map_err(|source| sensitive::Error::query_repository(
            RepositoryOperation::LoadSnapshot,
            source,
        ))?;

        row.map(sync_run_from_row).transpose()
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
            sensitive::Error::query_repository(
                RepositoryOperation::LoadIntegrationState,
                source,
            )
        })?;

        row.map(integration_state_from_row).transpose()
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
            sensitive::Error::query_repository(RepositoryOperation::LoadKeyCustody, source)
        })?;

        row.map(rotation_run_from_row).transpose()
    }

    async fn query_keyed_counts(
        &self,
        query: &str,
        operation: RepositoryOperation,
    ) -> sensitive::Result<Vec<sensitive_domain::KeyedCiphertextCount>> {
        let rows = sqlx::query(query)
            .fetch_all(&self.pg)
            .await
            .map_err(|source| sensitive::Error::query_repository(operation, source))?;

        rows.into_iter()
            .map(|row| {
                Ok(sensitive_domain::KeyedCiphertextCount::builder()
                    .key_id(parse_key_id(row.get::<String, _>("key_id"))?)
                    .count(parse_u32_count(
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
                sensitive::Error::query_repository(
                    RepositoryOperation::LoadKeyCustody,
                    source,
                )
            })?;

        parse_u32_count(count, "stale_count")
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
            sensitive::Error::query_repository(RepositoryOperation::LoadSnapshot, source)
        })?;

        rows.into_iter().map(record_proof_from_row).collect()
    }
}

#[async_trait]
impl sensitive::Repository for Repository {
    async fn load_snapshot(&self) -> sensitive::Result<sensitive::StoredSnapshot> {
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

    async fn load_authorized_record(
        &self,
        record_id: &sensitive_domain::Id,
    ) -> sensitive::Result<Option<sensitive_domain::AuthorizedRecord>> {
        let row = sqlx::query(
            r#"
            SELECT id, redacted_label, redacted_last4, authorized_key_id,
                   authorized_ciphertext, authorized_nonce, synced_at
            FROM sensitive_records
            WHERE id = $1
            "#,
        )
        .bind(record_id.as_ref())
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(
                RepositoryOperation::LoadAuthorizedRecord,
                source,
            )
        })?;

        row.map(|row| {
            let payload = self
                .crypto
                .decrypt(&SealedValue {
                    key_id: parse_key_id(row.get::<String, _>("authorized_key_id"))?,
                    nonce: row.get("authorized_nonce"),
                    ciphertext: row.get("authorized_ciphertext"),
                })
                .map_err(sensitive::Error::decrypt_record)?;
            let authorized: sensitive_domain::AuthorizedFields =
                serde_json::from_slice(&payload)
                    .map_err(sensitive::Error::decode_authorized_fields)?;

            Ok(sensitive_domain::AuthorizedRecord::builder()
                .id(sensitive_domain::Id::from(row.get::<uuid::Uuid, _>("id")))
                .label(
                    sensitive_domain::Label::try_new(
                        row.get::<String, _>("redacted_label"),
                    )
                    .map_err(sensitive::Error::decode_label)?,
                )
                .last4(
                    sensitive_domain::Last4::try_new(
                        row.get::<String, _>("redacted_last4"),
                    )
                    .map_err(sensitive::Error::decode_last4)?,
                )
                .authorized(authorized)
                .synced_at(from_offset_datetime(row.get("synced_at")))
                .build())
        })
        .transpose()
    }

    async fn load_integration_state(
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
            sensitive::Error::query_repository(
                RepositoryOperation::LoadIntegrationState,
                source,
            )
        })?;

        row.map(integration_state_from_row).transpose()
    }

    async fn load_key_custody(
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

    async fn load_access_grants(
        &self,
        user_id: &user_domain::Id,
    ) -> sensitive::Result<Vec<sensitive_domain::AccessGrant>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, capability, granted_at
            FROM sensitive_access_grants
            WHERE user_id = $1
            ORDER BY capability ASC
            "#,
        )
        .bind(user_id.as_ref())
        .fetch_all(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(
                RepositoryOperation::LoadAccessGrants,
                source,
            )
        })?;

        rows.into_iter().map(access_grant_from_row).collect()
    }

    async fn load_token(
        &self,
        provider: sensitive_domain::Provider,
    ) -> sensitive::Result<Option<sensitive::ProviderToken>> {
        let row = sqlx::query(
            r#"
            SELECT provider, token_key_id, token_ciphertext, token_nonce, expires_at, refreshed_at
            FROM integration_credentials
            WHERE provider = $1
            "#,
        )
        .bind(provider.as_ref())
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(RepositoryOperation::LoadToken, source)
        })?;

        row.map(|row| {
            let provider_raw = row.get::<String, _>("provider");
            let provider =
                provider_raw
                    .parse::<sensitive_domain::Provider>()
                    .map_err(|_| sensitive::Error::InvalidStoredProvider {
                        provider: provider_raw.clone(),
                    })?;
            let decrypted = self
                .crypto
                .decrypt(&SealedValue {
                    key_id: parse_key_id(row.get::<String, _>("token_key_id"))?,
                    nonce: row.get("token_nonce"),
                    ciphertext: row.get("token_ciphertext"),
                })
                .map_err(sensitive::Error::decrypt_token)?;
            let token_text =
                String::from_utf8(decrypted).map_err(sensitive::Error::decrypt_token)?;

            Ok(sensitive::ProviderToken::builder()
                .status(
                    sensitive_domain::TokenStatus::builder()
                        .provider(provider)
                        .expires_at(from_offset_datetime(row.get("expires_at")))
                        .refreshed_at(from_offset_datetime(row.get("refreshed_at")))
                        .build(),
                )
                .access_token(SecretString::new(token_text.into_boxed_str()))
                .build())
        })
        .transpose()
    }

    async fn upsert_token(
        &self,
        token: &sensitive::ProviderToken,
    ) -> sensitive::Result<()> {
        let encrypted = self
            .crypto
            .encrypt(token.access_token.expose_secret())
            .map_err(sensitive::Error::encrypt_token)?;

        sqlx::query(
            r#"
            INSERT INTO integration_credentials (
                provider,
                token_key_id,
                token_ciphertext,
                token_nonce,
                expires_at,
                refreshed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (provider) DO UPDATE
            SET token_key_id = EXCLUDED.token_key_id,
                token_ciphertext = EXCLUDED.token_ciphertext,
                token_nonce = EXCLUDED.token_nonce,
                expires_at = EXCLUDED.expires_at,
                refreshed_at = EXCLUDED.refreshed_at,
                updated_at = now()
            "#,
        )
        .bind(token.status.provider.as_ref())
        .bind(encrypted.key_id.to_string())
        .bind(encrypted.ciphertext)
        .bind(encrypted.nonce)
        .bind(to_offset_datetime(token.status.expires_at))
        .bind(to_offset_datetime(token.status.refreshed_at))
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(RepositoryOperation::UpsertToken, source)
        })?;

        Ok(())
    }

    async fn upsert_records(
        &self,
        records: &[sensitive_domain::Record],
        synced_at: SystemTime,
    ) -> sensitive::Result<usize> {
        let mut upserted = 0;
        for record in records {
            let authorized_json = serde_json::to_vec(&record.authorized)
                .map_err(sensitive::Error::encode_authorized_fields)?;
            let payload_fingerprint = payload_fingerprint(record, &authorized_json);
            let external_id = record.external_id.to_string();
            let changed = sqlx::query_scalar::<_, String>(
                r#"
                SELECT payload_fingerprint
                FROM sensitive_records
                WHERE external_id = $1
                "#,
            )
            .bind(&external_id)
            .fetch_optional(&self.pg)
            .await
            .map_err(|source| {
                sensitive::Error::query_repository(
                    RepositoryOperation::UpsertRecords,
                    source,
                )
            })?
            .as_deref()
            .map(|existing| existing != payload_fingerprint.as_str())
            .unwrap_or(true);
            let encrypted = self
                .crypto
                .encrypt(&authorized_json)
                .map_err(sensitive::Error::encrypt_record)?;

            sqlx::query(
                r#"
                INSERT INTO sensitive_records (
                    external_id,
                    redacted_label,
                    redacted_last4,
                    authorized_key_id,
                    authorized_ciphertext,
                    authorized_nonce,
                    payload_fingerprint,
                    synced_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (external_id) DO UPDATE
                SET redacted_label = EXCLUDED.redacted_label,
                    redacted_last4 = EXCLUDED.redacted_last4,
                    authorized_key_id = EXCLUDED.authorized_key_id,
                    authorized_ciphertext = EXCLUDED.authorized_ciphertext,
                    authorized_nonce = EXCLUDED.authorized_nonce,
                    payload_fingerprint = EXCLUDED.payload_fingerprint,
                    synced_at = EXCLUDED.synced_at,
                    updated_at = now()
                "#,
            )
            .bind(external_id)
            .bind(record.label.to_string())
            .bind(record.last4.to_string())
            .bind(encrypted.key_id.to_string())
            .bind(encrypted.ciphertext)
            .bind(encrypted.nonce)
            .bind(payload_fingerprint)
            .bind(to_offset_datetime(synced_at))
            .execute(&self.pg)
            .await
            .map_err(|source| {
                sensitive::Error::query_repository(
                    RepositoryOperation::UpsertRecords,
                    source,
                )
            })?;

            if changed {
                upserted += 1;
            }
        }

        Ok(upserted)
    }

    async fn upsert_integration_state(
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
        .bind(state.last_successful_fetch_at.map(to_offset_datetime))
        .bind(to_offset_datetime(state.last_attempted_fetch_at))
        .bind(state.failure_count as i64)
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(
                RepositoryOperation::UpsertIntegrationState,
                source,
            )
        })?;

        Ok(())
    }

    async fn rotate_ciphertext_to_active_key(
        &self,
        limit: usize,
        rotated_at: SystemTime,
    ) -> sensitive::Result<sensitive::KeyRotationProgress> {
        let mut rows_scanned = 0_u32;
        let mut rows_rewrapped = 0_u32;
        let mut rows_already_current = 0_u32;
        let mut rows_failed = 0_u32;
        let mut detail = "no stale ciphertext required rewrap".to_string();
        let active_key_id = self.crypto.active_key_id().clone();

        if limit == 0 {
            return Ok(sensitive::KeyRotationProgress::builder()
                .active_key_id(active_key_id)
                .rows_scanned(0)
                .rows_rewrapped(0)
                .rows_already_current(0)
                .rows_failed(0)
                .detail(
                    sensitive_domain::DetailText::try_new(detail)
                        .map_err(sensitive::Error::decode_detail_text)?,
                )
                .build());
        }

        if let Some(token_row) = sqlx::query(
            r#"
            SELECT provider, token_key_id, token_ciphertext, token_nonce
            FROM integration_credentials
            ORDER BY CASE WHEN token_key_id = $1 THEN 1 ELSE 0 END, updated_at ASC
            LIMIT 1
            "#,
        )
        .bind(active_key_id.to_string())
        .fetch_optional(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(
                RepositoryOperation::RotateCiphertext,
                source,
            )
        })? {
            rows_scanned += 1;
            let row_key_id = parse_key_id(token_row.get::<String, _>("token_key_id"))?;
            if row_key_id == active_key_id {
                rows_already_current += 1;
            } else {
                match self
                    .crypto
                    .decrypt(&SealedValue {
                        key_id: row_key_id.clone(),
                        nonce: token_row.get("token_nonce"),
                        ciphertext: token_row.get("token_ciphertext"),
                    })
                    .and_then(|plaintext| self.crypto.encrypt(plaintext))
                {
                    Ok(resealed) => {
                        sqlx::query(
                            r#"
                            UPDATE integration_credentials
                            SET token_key_id = $2,
                                token_ciphertext = $3,
                                token_nonce = $4,
                                updated_at = $5
                            WHERE provider = $1
                            "#,
                        )
                        .bind(token_row.get::<String, _>("provider"))
                        .bind(resealed.key_id.to_string())
                        .bind(resealed.ciphertext)
                        .bind(resealed.nonce)
                        .bind(to_offset_datetime(rotated_at))
                        .execute(&self.pg)
                        .await
                        .map_err(|source| {
                            sensitive::Error::query_repository(
                                RepositoryOperation::RotateCiphertext,
                                source,
                            )
                        })?;
                        rows_rewrapped += 1;
                        detail = format!(
                            "provider token rewrapped from {} to {}",
                            row_key_id, active_key_id
                        );
                    }
                    Err(error) => {
                        rows_failed += 1;
                        detail = format!("provider token rewrap failed for {}", row_key_id);
                        tracing::warn!(target: "demo.sensitive", ?error, %row_key_id, "provider token rewrap failed");
                    }
                }
            }
        }

        if rows_scanned < limit as u32 {
            let remaining = limit.saturating_sub(rows_scanned as usize);
            let rows = sqlx::query(
                r#"
                SELECT id, authorized_key_id, authorized_ciphertext, authorized_nonce
                FROM sensitive_records
                ORDER BY CASE WHEN authorized_key_id = $1 THEN 1 ELSE 0 END,
                         updated_at ASC,
                         id ASC
                LIMIT $2
                "#,
            )
            .bind(active_key_id.to_string())
            .bind(remaining as i64)
            .fetch_all(&self.pg)
            .await
            .map_err(|source| {
                sensitive::Error::query_repository(
                    RepositoryOperation::RotateCiphertext,
                    source,
                )
            })?;

            for row in rows {
                rows_scanned += 1;
                let row_key_id = parse_key_id(row.get::<String, _>("authorized_key_id"))?;
                if row_key_id == active_key_id {
                    rows_already_current += 1;
                    continue;
                }

                match self
                    .crypto
                    .decrypt(&SealedValue {
                        key_id: row_key_id.clone(),
                        nonce: row.get("authorized_nonce"),
                        ciphertext: row.get("authorized_ciphertext"),
                    })
                    .and_then(|plaintext| self.crypto.encrypt(plaintext))
                {
                    Ok(resealed) => {
                        sqlx::query(
                            r#"
                            UPDATE sensitive_records
                            SET authorized_key_id = $2,
                                authorized_ciphertext = $3,
                                authorized_nonce = $4,
                                updated_at = $5
                            WHERE id = $1
                            "#,
                        )
                        .bind(row.get::<uuid::Uuid, _>("id"))
                        .bind(resealed.key_id.to_string())
                        .bind(resealed.ciphertext)
                        .bind(resealed.nonce)
                        .bind(to_offset_datetime(rotated_at))
                        .execute(&self.pg)
                        .await
                        .map_err(|source| {
                            sensitive::Error::query_repository(
                                RepositoryOperation::RotateCiphertext,
                                source,
                            )
                        })?;
                        rows_rewrapped += 1;
                        detail = format!(
                            "sensitive record ciphertext rewrapped to {}",
                            active_key_id
                        );
                    }
                    Err(error) => {
                        rows_failed += 1;
                        detail =
                            format!("record ciphertext rewrap failed for {}", row_key_id);
                        tracing::warn!(target: "demo.sensitive", ?error, %row_key_id, "record ciphertext rewrap failed");
                    }
                }
            }
        }

        Ok(sensitive::KeyRotationProgress::builder()
            .active_key_id(active_key_id)
            .rows_scanned(rows_scanned)
            .rows_rewrapped(rows_rewrapped)
            .rows_already_current(rows_already_current)
            .rows_failed(rows_failed)
            .detail(
                sensitive_domain::DetailText::try_new(
                    detail.chars().take(120).collect::<String>(),
                )
                .map_err(sensitive::Error::decode_detail_text)?,
            )
            .build())
    }

    async fn record_sync_run(
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
        .bind(to_offset_datetime(run.started_at))
        .bind(to_offset_datetime(run.finished_at))
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(RepositoryOperation::RecordSyncRun, source)
        })?;

        Ok(())
    }

    async fn record_key_rotation_run(
        &self,
        run: &sensitive_domain::KeyRotationRun,
    ) -> sensitive::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO key_rotation_runs (
                active_key_id,
                outcome,
                rows_scanned,
                rows_rewrapped,
                rows_already_current,
                rows_failed,
                detail,
                started_at,
                finished_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(run.active_key_id.to_string())
        .bind(run.outcome.as_ref())
        .bind(run.rows_scanned as i32)
        .bind(run.rows_rewrapped as i32)
        .bind(run.rows_already_current as i32)
        .bind(run.rows_failed as i32)
        .bind(run.detail.to_string())
        .bind(to_offset_datetime(run.started_at))
        .bind(to_offset_datetime(run.finished_at))
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(
                RepositoryOperation::RecordKeyRotationRun,
                source,
            )
        })?;

        Ok(())
    }

    async fn upsert_access_grants(
        &self,
        user_id: &user_domain::Id,
        capabilities: &[sensitive_domain::AccessCapability],
        granted_at: SystemTime,
    ) -> sensitive::Result<()> {
        for capability in capabilities {
            sqlx::query(
                r#"
                INSERT INTO sensitive_access_grants (
                    user_id,
                    capability,
                    granted_at
                )
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id, capability) DO NOTHING
                "#,
            )
            .bind(user_id.as_ref())
            .bind(capability.as_ref())
            .bind(to_offset_datetime(granted_at))
            .execute(&self.pg)
            .await
            .map_err(|source| {
                sensitive::Error::query_repository(
                    RepositoryOperation::UpsertAccessGrants,
                    source,
                )
            })?;
        }

        Ok(())
    }

    async fn record_access_event(
        &self,
        event: &sensitive_domain::AccessEvent,
    ) -> sensitive::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sensitive_access_events (
                user_id,
                capability,
                target_record_id,
                outcome,
                detail,
                occurred_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(event.user_id.map(uuid::Uuid::from))
        .bind(event.capability.as_ref())
        .bind(event.record_id.map(uuid::Uuid::from))
        .bind(event.outcome.as_ref())
        .bind(event.detail.to_string())
        .bind(to_offset_datetime(event.occurred_at))
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(
                RepositoryOperation::RecordAccessEvent,
                source,
            )
        })?;

        Ok(())
    }

    async fn list_recent_access_events(
        &self,
        limit: usize,
    ) -> sensitive::Result<Vec<sensitive_domain::AccessEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, capability, target_record_id, outcome, detail, occurred_at
            FROM sensitive_access_events
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(
                RepositoryOperation::ListAccessEvents,
                source,
            )
        })?;

        rows.into_iter().map(access_event_from_row).collect()
    }
}

#[derive(Default)]
pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self {
        Self
    }
}

impl sensitive::Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

fn parse_key_id(value: String) -> sensitive::Result<sensitive_domain::KeyId> {
    sensitive_domain::KeyId::try_new(value.clone())
        .map_err(|_| sensitive::Error::InvalidStoredKeyId { key_id: value })
}

fn parse_u32_count(value: i64, field: &'static str) -> sensitive::Result<u32> {
    u32::try_from(value)
        .map_err(|_| sensitive::Error::InvalidStoredRotationCount { field, value })
}

fn ciphertext_evidence(
    key_id: sensitive_domain::KeyId,
    ciphertext: &[u8],
) -> sensitive::CiphertextEvidence {
    let preview = utils::b64::b64u_encode(ciphertext);
    let preview = preview.chars().take(18).collect::<String>();
    sensitive::CiphertextEvidence::builder()
        .key_id(key_id)
        .preview(preview)
        .bytes(ciphertext.len())
        .build()
}

fn payload_fingerprint(
    record: &sensitive_domain::Record,
    authorized_json: &[u8],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(record.external_id.to_string().as_bytes());
    hasher.update(record.label.to_string().as_bytes());
    hasher.update(record.last4.to_string().as_bytes());
    hasher.update(authorized_json);
    format!("{:x}", hasher.finalize())
}

fn access_grant_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::AccessGrant> {
    let capability_raw = row.get::<String, _>("capability");
    let capability = capability_raw
        .parse::<sensitive_domain::AccessCapability>()
        .map_err(|_| sensitive::Error::InvalidStoredAccessCapability {
            capability: capability_raw.clone(),
        })?;

    Ok(sensitive_domain::AccessGrant::builder()
        .user_id(user_domain::Id::from(row.get::<uuid::Uuid, _>("user_id")))
        .capability(capability)
        .granted_at(from_offset_datetime(row.get("granted_at")))
        .build())
}

fn access_event_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::AccessEvent> {
    let capability_raw = row.get::<String, _>("capability");
    let outcome_raw = row.get::<String, _>("outcome");
    let capability = capability_raw
        .parse::<sensitive_domain::AccessCapability>()
        .map_err(|_| sensitive::Error::InvalidStoredAccessCapability {
            capability: capability_raw.clone(),
        })?;
    let outcome = outcome_raw
        .parse::<sensitive_domain::AccessOutcome>()
        .map_err(|_| sensitive::Error::InvalidStoredAccessOutcome {
            outcome: outcome_raw.clone(),
        })?;

    Ok(sensitive_domain::AccessEvent::builder()
        .maybe_user_id(
            row.get::<Option<uuid::Uuid>, _>("user_id")
                .map(user_domain::Id::from),
        )
        .capability(capability)
        .maybe_record_id(
            row.get::<Option<uuid::Uuid>, _>("target_record_id")
                .map(sensitive_domain::Id::from),
        )
        .outcome(outcome)
        .detail(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("detail"))
                .map_err(sensitive::Error::decode_detail_text)?,
        )
        .occurred_at(from_offset_datetime(row.get("occurred_at")))
        .build())
}

fn record_proof_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive::RecordProof> {
    Ok(sensitive::RecordProof::builder()
        .id(sensitive_domain::Id::from(row.get::<uuid::Uuid, _>("id")))
        .label(
            sensitive_domain::Label::try_new(row.get::<String, _>("redacted_label"))
                .map_err(sensitive::Error::decode_label)?,
        )
        .last4(
            sensitive_domain::Last4::try_new(row.get::<String, _>("redacted_last4"))
                .map_err(sensitive::Error::decode_last4)?,
        )
        .synced_at(from_offset_datetime(row.get("synced_at")))
        .ciphertext(ciphertext_evidence(
            parse_key_id(row.get::<String, _>("authorized_key_id"))?,
            &row.get::<Vec<u8>, _>("authorized_ciphertext"),
        ))
        .build())
}

fn sync_run_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::SyncRun> {
    let provider_raw = row.get::<String, _>("provider");
    let outcome_raw = row.get::<String, _>("outcome");
    let provider = provider_raw
        .parse::<sensitive_domain::Provider>()
        .map_err(|_| sensitive::Error::InvalidStoredProvider {
            provider: provider_raw.clone(),
        })?;
    let outcome = outcome_raw
        .parse::<sensitive_domain::SyncOutcome>()
        .map_err(|_| sensitive::Error::InvalidStoredSyncOutcome {
            outcome: outcome_raw.clone(),
        })?;

    Ok(sensitive_domain::SyncRun::builder()
        .provider(provider)
        .outcome(outcome)
        .records_seen(row.get::<i32, _>("records_seen") as u32)
        .records_upserted(row.get::<i32, _>("records_upserted") as u32)
        .detail(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("detail"))
                .map_err(sensitive::Error::decode_detail_text)?,
        )
        .started_at(from_offset_datetime(row.get("started_at")))
        .finished_at(from_offset_datetime(row.get("finished_at")))
        .build())
}

fn rotation_run_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::KeyRotationRun> {
    let outcome_raw = row.get::<String, _>("outcome");
    let rows_scanned = row.get::<i32, _>("rows_scanned") as i64;
    let rows_rewrapped = row.get::<i32, _>("rows_rewrapped") as i64;
    let rows_already_current = row.get::<i32, _>("rows_already_current") as i64;
    let rows_failed = row.get::<i32, _>("rows_failed") as i64;

    Ok(sensitive_domain::KeyRotationRun::builder()
        .active_key_id(parse_key_id(row.get::<String, _>("active_key_id"))?)
        .outcome(
            outcome_raw
                .parse::<sensitive_domain::RotationOutcome>()
                .map_err(|_| sensitive::Error::InvalidStoredRotationOutcome {
                    outcome: outcome_raw.clone(),
                })?,
        )
        .rows_scanned(parse_u32_count(rows_scanned, "rows_scanned")?)
        .rows_rewrapped(parse_u32_count(rows_rewrapped, "rows_rewrapped")?)
        .rows_already_current(parse_u32_count(
            rows_already_current,
            "rows_already_current",
        )?)
        .rows_failed(parse_u32_count(rows_failed, "rows_failed")?)
        .detail(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("detail"))
                .map_err(sensitive::Error::decode_detail_text)?,
        )
        .started_at(from_offset_datetime(row.get("started_at")))
        .finished_at(from_offset_datetime(row.get("finished_at")))
        .build())
}

fn integration_state_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::IntegrationState> {
    let provider_raw = row.get::<String, _>("provider");
    let mode_raw = row.get::<String, _>("mode");
    let auth_mode_raw = row.get::<Option<String>, _>("auth_mode");
    let cursor_raw = row.get::<Option<String>, _>("cursor");
    let fetch_outcome_raw = row.get::<String, _>("last_fetch_outcome");
    let token_strategy_raw = row.get::<String, _>("token_strategy");
    let error_category_raw = row.get::<Option<String>, _>("last_error_category");
    let auth_outcome_raw = row.get::<Option<String>, _>("last_auth_outcome");
    let remote_status_code = row.get::<Option<i64>, _>("last_remote_status_code");
    let retry_backoff_secs = row.get::<Option<i64>, _>("retry_backoff_secs");
    let last_successful_mode_raw = row.get::<Option<String>, _>("last_successful_mode");
    let failure_count = row.get::<i64, _>("failure_count");

    let provider = provider_raw
        .parse::<sensitive_domain::Provider>()
        .map_err(|_| sensitive::Error::InvalidStoredProvider {
            provider: provider_raw.clone(),
        })?;
    let mode = mode_raw
        .parse::<sensitive_domain::ProviderMode>()
        .map_err(|_| sensitive::Error::InvalidStoredProviderMode {
            mode: mode_raw.clone(),
        })?;
    let cursor = cursor_raw
        .map(|value| {
            sensitive_domain::SyncCursor::try_new(value.clone())
                .map_err(|_| sensitive::Error::InvalidStoredSyncCursor { cursor: value })
        })
        .transpose()?;
    let auth_mode = auth_mode_raw
        .map(|value| {
            value
                .parse::<sensitive_domain::ProviderAuthMode>()
                .map_err(|_| sensitive::Error::InvalidStoredProviderAuthMode {
                    mode: value.clone(),
                })
        })
        .transpose()?;
    let last_fetch_outcome = fetch_outcome_raw
        .parse::<sensitive_domain::FetchOutcome>()
        .map_err(|_| sensitive::Error::InvalidStoredFetchOutcome {
            outcome: fetch_outcome_raw.clone(),
        })?;
    let token_strategy = token_strategy_raw
        .parse::<sensitive_domain::TokenStrategy>()
        .map_err(|_| sensitive::Error::InvalidStoredTokenStrategy {
            strategy: token_strategy_raw.clone(),
        })?;
    let last_error_category = error_category_raw
        .map(|value| {
            value
                .parse::<sensitive_domain::RemoteErrorCategory>()
                .map_err(|_| sensitive::Error::InvalidStoredRemoteErrorCategory {
                    category: value.clone(),
                })
        })
        .transpose()?;
    let last_auth_outcome = auth_outcome_raw
        .map(|value| {
            value
                .parse::<sensitive_domain::FetchOutcome>()
                .map_err(|_| sensitive::Error::InvalidStoredFetchOutcome {
                    outcome: value.clone(),
                })
        })
        .transpose()?;
    let last_remote_status_code = remote_status_code
        .map(|value| {
            u16::try_from(value).map_err(|_| {
                sensitive::Error::InvalidStoredRemoteStatusCode { status_code: value }
            })
        })
        .transpose()?;
    let retry_backoff_secs = retry_backoff_secs
        .map(|value| parse_u32_count(value, "retry_backoff_secs"))
        .transpose()?;
    let last_successful_mode = last_successful_mode_raw
        .map(|value| {
            value
                .parse::<sensitive_domain::ProviderMode>()
                .map_err(|_| sensitive::Error::InvalidStoredProviderMode {
                    mode: value.clone(),
                })
        })
        .transpose()?;
    let failure_count = u32::try_from(failure_count)
        .map_err(|_| sensitive::Error::InvalidStoredFailureCount { failure_count })?;

    Ok(sensitive_domain::IntegrationState::builder()
        .provider(provider)
        .mode(mode)
        .endpoint(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("endpoint"))
                .map_err(sensitive::Error::decode_detail_text)?,
        )
        .maybe_auth_mode(auth_mode)
        .maybe_cursor(cursor)
        .last_fetch_outcome(last_fetch_outcome)
        .token_strategy(token_strategy)
        .maybe_last_error_category(last_error_category)
        .maybe_last_auth_outcome(last_auth_outcome)
        .maybe_last_remote_status_code(last_remote_status_code)
        .maybe_retry_backoff_secs(retry_backoff_secs)
        .maybe_last_successful_mode(last_successful_mode)
        .maybe_last_successful_fetch_at(
            row.get::<Option<OffsetDateTime>, _>("last_successful_fetch_at")
                .map(from_offset_datetime),
        )
        .last_attempted_fetch_at(from_offset_datetime(row.get("last_attempted_fetch_at")))
        .failure_count(failure_count)
        .build())
}

fn to_offset_datetime(value: SystemTime) -> OffsetDateTime {
    OffsetDateTime::from(value)
}

fn from_offset_datetime(value: OffsetDateTime) -> SystemTime {
    SystemTime::from(value)
}
