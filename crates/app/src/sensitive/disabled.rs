use super::*;

pub(super) struct Repository;

#[async_trait]
impl super::Repository for Repository {
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
            .detail(super::rotation::rotation_detail_text(
                "no ciphertext rows required rewrap",
            ))
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

pub(super) struct Provider;

#[async_trait]
impl super::ProviderClient for Provider {
    fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
        ProviderBoundaryMeta::builder()
            .mode(sensitive::ProviderMode::LocalStub)
            .endpoint(
                sensitive::DetailText::try_new("disabled local boundary")
                    .expect("detail text"),
            )
            .maybe_auth_mode(None)
            .maybe_retry_backoff_secs(None)
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

pub(super) struct Clock;

impl super::Clock for Clock {
    fn now(&self) -> SystemTime {
        UNIX_EPOCH
    }
}
