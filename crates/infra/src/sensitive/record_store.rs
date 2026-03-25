use super::*;

impl Repository {
    pub(super) async fn load_authorized_record_impl(
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
            sensitive::failure::Error::query_repository(
                RepositoryOperation::LoadAuthorizedRecord,
                source,
            )
        })?;

        row.map(|row| {
            let payload = self
                .crypto
                .decrypt(&SealedValue {
                    key_id: mapping::parse_key_id(
                        row.get::<String, _>("authorized_key_id"),
                    )?,
                    nonce: row.get("authorized_nonce"),
                    ciphertext: row.get("authorized_ciphertext"),
                })
                .map_err(sensitive::failure::Error::decrypt_record)?;
            let authorized: sensitive_domain::AuthorizedFields =
                serde_json::from_slice(&payload)
                    .map_err(sensitive::failure::Error::decode_authorized_fields)?;

            Ok(sensitive_domain::AuthorizedRecord::builder()
                .id(sensitive_domain::Id::from(row.get::<uuid::Uuid, _>("id")))
                .label(
                    sensitive_domain::Label::try_new(
                        row.get::<String, _>("redacted_label"),
                    )
                    .map_err(sensitive::failure::Error::decode_label)?,
                )
                .last4(
                    sensitive_domain::Last4::try_new(
                        row.get::<String, _>("redacted_last4"),
                    )
                    .map_err(sensitive::failure::Error::decode_last4)?,
                )
                .authorized(authorized)
                .synced_at(mapping::from_offset_datetime(row.get("synced_at")))
                .build())
        })
        .transpose()
    }

    pub(super) async fn upsert_records_impl(
        &self,
        records: &[sensitive_domain::Record],
        synced_at: SystemTime,
    ) -> sensitive::Result<usize> {
        let mut upserted = 0;
        for record in records {
            let authorized_json = serde_json::to_vec(&record.authorized)
                .map_err(sensitive::failure::Error::encode_authorized_fields)?;
            let payload_fingerprint =
                mapping::payload_fingerprint(record, &authorized_json);
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
                sensitive::failure::Error::query_repository(
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
                .map_err(sensitive::failure::Error::encrypt_record)?;

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
            .bind(mapping::to_offset_datetime(synced_at))
            .execute(&self.pg)
            .await
            .map_err(|source| {
                sensitive::failure::Error::query_repository(
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
}
