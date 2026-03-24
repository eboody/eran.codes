use super::*;

impl Repository {
    pub(super) async fn load_token_impl(
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
                    key_id: mapping::parse_key_id(row.get::<String, _>("token_key_id"))?,
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
                        .expires_at(mapping::from_offset_datetime(row.get("expires_at")))
                        .refreshed_at(mapping::from_offset_datetime(
                            row.get("refreshed_at"),
                        ))
                        .build(),
                )
                .access_token(SecretString::new(token_text.into_boxed_str()))
                .build())
        })
        .transpose()
    }

    pub(super) async fn upsert_token_impl(
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
        .bind(mapping::to_offset_datetime(token.status.expires_at))
        .bind(mapping::to_offset_datetime(token.status.refreshed_at))
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::Error::query_repository(RepositoryOperation::UpsertToken, source)
        })?;

        Ok(())
    }
}
