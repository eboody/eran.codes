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
            .detail(rotation::rotation_detail_text(
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
            .maybe_auth_mode(Some(sensitive::ProviderAuthMode::StubIssuedToken))
            .maybe_retry_backoff_secs(None)
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
                .classification(sensitive::DetailText::try_new("synthetic_record").unwrap())
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
        .detail(sync::sync_detail_text(
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
            .maybe_auth_mode(Some(sensitive::ProviderAuthMode::StubIssuedToken))
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
            .maybe_auth_mode(Some(sensitive::ProviderAuthMode::StubIssuedToken))
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
                .detail(snapshot::access_detail_text(
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
