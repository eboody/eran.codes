use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use secrecy::SecretString;

use super::super::*;

pub(super) struct RepoState {
    pub(super) snapshot: StoredSnapshot,
    pub(super) key_custody: sensitive::KeyCustodyState,
    pub(super) authorized_record: Option<sensitive::AuthorizedRecord>,
    pub(super) token: Option<ProviderToken>,
    pub(super) token_load_failures_remaining: usize,
    pub(super) grants: Vec<sensitive::AccessGrant>,
    pub(super) authorized_loads: usize,
    pub(super) upserted_records: Vec<Vec<sensitive::Record>>,
    pub(super) recorded_runs: Vec<sensitive::SyncRun>,
    pub(super) recorded_rotation_runs: Vec<sensitive::KeyRotationRun>,
    pub(super) access_events: Vec<sensitive::AccessEvent>,
    pub(super) access_grant_writes: usize,
    pub(super) token_writes: usize,
    pub(super) external_ids: Vec<String>,
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

pub(super) struct TestRepository {
    pub(super) state: Mutex<RepoState>,
}

impl TestRepository {
    pub(super) fn new(state: RepoState) -> Self {
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
            return Err(failure::Error::decrypt_token(std::io::Error::other(
                "stored token ciphertext could not be decrypted",
            )));
        }

        Ok(state.token.clone())
    }

    async fn upsert_token(&self, token: &ProviderToken) -> Result<()> {
        let mut state = self.state.lock().expect("repo state");
        state.token = Some(token.clone());
        state.token_writes += 1;
        state.key_custody.token_counts = vec![
            sensitive::KeyedCiphertextCount::builder()
                .key_id(active_key_id())
                .count(1)
                .build(),
        ];
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
        state.key_custody.record_counts = vec![
            sensitive::KeyedCiphertextCount::builder()
                .key_id(active_key_id())
                .count(state.snapshot.records.len() as u32)
                .build(),
        ];
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
        let rows_scanned = stale_token_count
            .saturating_add(stale_record_count)
            .min(limit as u32);
        let rows_rewrapped = rows_scanned;
        state.key_custody.stale_token_count = stale_token_count.saturating_sub(1);
        state.key_custody.stale_record_count = stale_record_count
            .saturating_sub(rows_rewrapped.saturating_sub(stale_token_count.min(1)));
        state.key_custody.token_counts = vec![
            sensitive::KeyedCiphertextCount::builder()
                .key_id(active_key_id())
                .count(1)
                .build(),
        ];
        state.key_custody.record_counts = vec![
            sensitive::KeyedCiphertextCount::builder()
                .key_id(active_key_id())
                .count(state.snapshot.records.len() as u32)
                .build(),
        ];

        Ok(KeyRotationProgress::builder()
            .active_key_id(active_key_id())
            .rows_scanned(rows_scanned)
            .rows_rewrapped(rows_rewrapped)
            .rows_already_current(0)
            .rows_failed(0)
            .detail(super::super::rotation::rotation_detail_text(
                "stale ciphertext rewrapped to the active key",
            ))
            .build())
    }

    async fn record_sync_run(&self, run: &sensitive::SyncRun) -> Result<()> {
        let mut state = self.state.lock().expect("repo state");
        state.snapshot.latest_sync = Some(run.clone());
        state.recorded_runs.push(run.clone());
        Ok(())
    }

    async fn record_key_rotation_run(&self, run: &sensitive::KeyRotationRun) -> Result<()> {
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
            if state
                .grants
                .iter()
                .any(|grant| grant.user_id == *user_id && grant.capability == *capability)
            {
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

pub(super) struct TestProvider {
    pub(super) token_refreshes: Mutex<usize>,
    pub(super) records: Vec<sensitive::Record>,
}

#[async_trait]
impl ProviderClient for TestProvider {
    fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
        local_stub_meta()
    }

    async fn refresh_token(
        &self,
        provider: sensitive::Provider,
        now: SystemTime,
        _current_token: Option<&SecretString>,
    ) -> Result<ProviderToken> {
        let mut refreshes = self.token_refreshes.lock().expect("refresh count");
        *refreshes += 1;
        Ok(provider_token(
            provider,
            now,
            format!("token-{}", *refreshes),
        ))
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
            .cursor(sensitive::SyncCursor::try_new("cursor-alpha").expect("cursor"))
            .build())
    }
}

pub(super) struct FixedClock {
    pub(super) now: SystemTime,
}

impl Clock for FixedClock {
    fn now(&self) -> SystemTime {
        self.now
    }
}

pub(super) struct FailingProvider;

#[async_trait]
impl ProviderClient for FailingProvider {
    fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
        local_stub_meta()
    }

    async fn refresh_token(
        &self,
        provider: sensitive::Provider,
        now: SystemTime,
        _current_token: Option<&SecretString>,
    ) -> Result<ProviderToken> {
        Ok(provider_token(provider, now, "token-fail"))
    }

    async fn fetch_records(
        &self,
        _provider: sensitive::Provider,
        _token: &ProviderToken,
        _cursor: Option<&sensitive::SyncCursor>,
        _now: SystemTime,
    ) -> Result<ProviderRecords> {
        Err(failure::Error::provider_request(
            ProviderOperation::FetchRecords,
            std::io::Error::other("synthetic provider offline"),
        ))
    }
}

pub(super) struct RetryAfterUnauthorizedProvider {
    pub(super) unauthorized_remaining: Mutex<usize>,
    pub(super) records: Vec<sensitive::Record>,
}

#[async_trait]
impl ProviderClient for RetryAfterUnauthorizedProvider {
    fn boundary_meta(&self, _provider: sensitive::Provider) -> ProviderBoundaryMeta {
        local_stub_meta()
    }

    async fn refresh_token(
        &self,
        provider: sensitive::Provider,
        now: SystemTime,
        _current_token: Option<&SecretString>,
    ) -> Result<ProviderToken> {
        Ok(provider_token(provider, now, "token-retry"))
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
            return Err(failure::Error::provider_failure(
                ProviderOperation::FetchRecords,
                ProviderFailureKind::Unauthorized,
                std::io::Error::other("provider forced unauthorized retry"),
            ));
        }

        Ok(ProviderRecords::builder()
            .records(self.records.clone())
            .cursor(sensitive::SyncCursor::try_new("cursor-retried").expect("cursor"))
            .build())
    }
}

pub(super) fn example_record() -> sensitive::Record {
    sensitive::Record::builder()
        .external_id(
            sensitive::ExternalId::try_new("synthetic-alpha").expect("external id"),
        )
        .label(sensitive::Label::try_new("Alpha file").expect("label"))
        .last4(sensitive::Last4::try_new("1001").expect("last4"))
        .authorized(
            sensitive::AuthorizedFields::builder()
                .subject_name(sensitive::DetailText::try_new("Case alpha").expect("detail"))
                .classification(
                    sensitive::DetailText::try_new("synthetic_record").expect("detail"),
                )
                .note(
                    sensitive::DetailText::try_new("Authorized path only.")
                        .expect("detail"),
                )
                .build(),
        )
        .build()
}

pub(super) fn active_key_id() -> sensitive::KeyId {
    sensitive::KeyId::try_new("active_data_key").expect("key id")
}

pub(super) fn legacy_key_id() -> sensitive::KeyId {
    sensitive::KeyId::try_new("legacy_data_key").expect("key id")
}

pub(super) fn key_custody_state() -> sensitive::KeyCustodyState {
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
        .token_counts(vec![
            sensitive::KeyedCiphertextCount::builder()
                .key_id(legacy_key_id())
                .count(1)
                .build(),
        ])
        .record_counts(vec![
            sensitive::KeyedCiphertextCount::builder()
                .key_id(legacy_key_id())
                .count(1)
                .build(),
        ])
        .stale_token_count(1)
        .stale_record_count(1)
        .maybe_last_rotation_run(None)
        .build()
}

pub(super) fn record_proof(record_id: sensitive::Id) -> RecordProof {
    RecordProof::builder()
        .id(record_id)
        .label(sensitive::Label::try_new("Alpha file").expect("label"))
        .last4(sensitive::Last4::try_new("1001").expect("last4"))
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

pub(super) fn authorized_record(record_id: sensitive::Id) -> sensitive::AuthorizedRecord {
    sensitive::AuthorizedRecord::builder()
        .id(record_id)
        .label(sensitive::Label::try_new("Alpha file").expect("label"))
        .last4(sensitive::Last4::try_new("1001").expect("last4"))
        .authorized(example_record().authorized)
        .synced_at(UNIX_EPOCH)
        .build()
}

pub(super) fn test_user_id() -> user::Id {
    user::Id::from(uuid::Uuid::from_u128(0xfeed))
}

pub(super) fn authenticated_viewer(email: &str) -> Viewer {
    Viewer::Authenticated(
        AuthenticatedViewer::builder()
            .user_id(test_user_id())
            .email(user::Email::try_new(email).expect("email"))
            .build(),
    )
}

pub(super) fn access_grant(
    capability: sensitive::AccessCapability,
) -> sensitive::AccessGrant {
    sensitive::AccessGrant::builder()
        .user_id(test_user_id())
        .capability(capability)
        .granted_at(UNIX_EPOCH)
        .build()
}

pub(super) fn token_proof() -> TokenProof {
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

pub(super) fn latest_sync() -> sensitive::SyncRun {
    sensitive::SyncRun::builder()
        .provider(PROVIDER)
        .outcome(sensitive::SyncOutcome::Success)
        .records_seen(1)
        .records_upserted(1)
        .detail(super::super::sync::sync_detail_text(
            "1 synthetic record processed for runtime proof",
        ))
        .started_at(UNIX_EPOCH)
        .finished_at(UNIX_EPOCH + Duration::from_secs(1))
        .build()
}

pub(super) fn service_with_state(state: RepoState) -> (Service, Arc<TestRepository>) {
    let repo = Arc::new(TestRepository::new(state));
    let provider = Arc::new(TestProvider {
        token_refreshes: Mutex::new(0),
        records: vec![example_record()],
    });
    let service = Service::builder()
        .with_repo(repo.clone())
        .with_provider(provider)
        .with_clock(Arc::new(FixedClock {
            now: UNIX_EPOCH + Duration::from_secs(100),
        }))
        .build();
    (service, repo)
}

fn local_stub_meta() -> ProviderBoundaryMeta {
    ProviderBoundaryMeta::builder()
        .mode(sensitive::ProviderMode::LocalStub)
        .endpoint(
            sensitive::DetailText::try_new("http://127.0.0.1:4001").expect("detail text"),
        )
        .auth_mode(sensitive::ProviderAuthMode::StubIssuedToken)
        .maybe_retry_backoff_secs(None)
        .build()
}

fn provider_token(
    provider: sensitive::Provider,
    now: SystemTime,
    access_token: impl Into<String>,
) -> ProviderToken {
    ProviderToken::builder()
        .status(
            sensitive::TokenStatus::builder()
                .provider(provider)
                .expires_at(now + Duration::from_secs(120))
                .refreshed_at(now)
                .build(),
        )
        .access_token(SecretString::new(access_token.into().into()))
        .build()
}
