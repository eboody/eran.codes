use super::*;

impl Repository {
    pub(super) async fn rotate_ciphertext_to_active_key_impl(
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
            let row_key_id =
                mapping::parse_key_id(token_row.get::<String, _>("token_key_id"))?;
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
                        .bind(mapping::to_offset_datetime(rotated_at))
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
                let row_key_id =
                    mapping::parse_key_id(row.get::<String, _>("authorized_key_id"))?;
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
                        .bind(mapping::to_offset_datetime(rotated_at))
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

    pub(super) async fn record_key_rotation_run_impl(
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
        .bind(mapping::to_offset_datetime(run.started_at))
        .bind(mapping::to_offset_datetime(run.finished_at))
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
}
