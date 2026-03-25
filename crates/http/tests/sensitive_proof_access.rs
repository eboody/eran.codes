use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use secrecy::{ExposeSecret, SecretString};
use tower::ServiceExt;
use tower_cookies::Key;
use tower_sessions::MemoryStore;

use app::{auth, user};
use domain::{chat as domain_chat, sensitive as domain_sensitive, user as domain_user};
use http as app_http;

const USER_ID: uuid::Uuid = uuid::Uuid::from_u128(0xd358d153_19a1_4a4c_8c52_73ff1a1f44d3);

struct TestUserRepo;

#[async_trait]
impl user::Repository for TestUserRepo {
    async fn find_by_email(
        &self,
        _email: &domain_user::Email,
    ) -> user::Result<Option<domain_user::User>> {
        Ok(None)
    }

    async fn create_with_credentials(
        &self,
        _user: &domain_user::User,
        _password_hash: &auth::password::Hash,
    ) -> user::Result<()> {
        Ok(())
    }
}

struct TestHasher;

impl auth::password::Hasher for TestHasher {
    fn hash(&self, _password: &str) -> auth::Result<auth::password::Hash> {
        Ok(auth::password::Hash::new("hash"))
    }

    fn verify(
        &self,
        _password: &str,
        _password_hash: &auth::password::Hash,
    ) -> auth::Result<bool> {
        Ok(true)
    }
}

struct TestAuthProvider;

#[derive(Clone, Copy, Debug)]
enum TestCredential {
    Demo,
}

impl TestCredential {
    fn email(self) -> domain_user::Email {
        match self {
            TestCredential::Demo => {
                domain_user::Email::try_new("demo@example.com").expect("email")
            }
        }
    }

    fn password(self) -> SecretString {
        match self {
            TestCredential::Demo => SecretString::new("password".into()),
        }
    }
}

#[async_trait]
impl auth::Provider for TestAuthProvider {
    async fn authenticate(
        &self,
        credentials: auth::Credentials,
    ) -> auth::Result<Option<auth::AuthenticatedUser>> {
        let demo = TestCredential::Demo;
        if credentials.email == demo.email()
            && credentials.password.expose_secret() == demo.password().expose_secret()
        {
            return Ok(Some(test_user()));
        }
        Ok(None)
    }

    async fn get_user(
        &self,
        user_id: &domain_user::Id,
    ) -> auth::Result<Option<auth::AuthenticatedUser>> {
        if *user_id == domain_user::Id::from(USER_ID) {
            return Ok(Some(test_user()));
        }
        Ok(None)
    }
}

fn test_user() -> auth::AuthenticatedUser {
    auth::AuthenticatedUser::builder()
        .id(domain_user::Id::from(USER_ID))
        .username(domain_user::Username::try_new("Demo").expect("username"))
        .email(TestCredential::Demo.email())
        .session_hash(auth::SessionHash::new("hash"))
        .build()
}

#[derive(Default)]
struct ChatRepo;

#[async_trait]
impl app::chat::Repository for ChatRepo {
    async fn create_room(&self, _room: &domain_chat::Room) -> app::chat::Result<()> {
        Ok(())
    }

    async fn find_room(
        &self,
        _room_id: &domain_chat::room::Id,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(None)
    }

    async fn find_room_by_name(
        &self,
        _name: &domain_chat::room::Name,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(None)
    }

    async fn list_messages(
        &self,
        _room_id: &domain_chat::room::Id,
        _limit: usize,
    ) -> app::chat::Result<Vec<domain_chat::Message>> {
        Ok(Vec::new())
    }

    async fn find_message(
        &self,
        _message_id: &domain_chat::message::Id,
    ) -> app::chat::Result<Option<domain_chat::Message>> {
        Ok(None)
    }

    async fn insert_message(
        &self,
        _message: &domain_chat::Message,
    ) -> app::chat::Result<()> {
        Ok(())
    }

    async fn add_membership(
        &self,
        _room_id: &domain_chat::room::Id,
        _user_id: &domain_chat::UserId,
        _role: app::chat::RoomRole,
    ) -> app::chat::Result<()> {
        Ok(())
    }

    async fn is_member(
        &self,
        _room_id: &domain_chat::room::Id,
        _user_id: &domain_chat::UserId,
    ) -> app::chat::Result<bool> {
        Ok(true)
    }

    async fn update_message_status(
        &self,
        _message_id: &domain_chat::message::Id,
        _status: domain_chat::message::Status,
    ) -> app::chat::Result<app::chat::PendingMutationResult> {
        Ok(app::chat::PendingMutationResult::Applied)
    }
}

struct ModerationQueue;

#[async_trait]
impl app::chat::moderation::Queue for ModerationQueue {
    async fn enqueue(
        &self,
        _message_id: &domain_chat::message::Id,
        _reason: &app::chat::moderation::Reason,
    ) -> app::chat::Result<()> {
        Ok(())
    }

    async fn list_pending(
        &self,
        _limit: usize,
    ) -> app::chat::Result<Vec<app::chat::moderation::Item>> {
        Ok(Vec::new())
    }

    async fn complete_if_pending(
        &self,
        _message_id: &domain_chat::message::Id,
        _reviewer_id: &domain_chat::UserId,
        _decision: app::chat::moderation::Decision,
        _reason: Option<app::chat::moderation::Reason>,
    ) -> app::chat::Result<app::chat::PendingMutationResult> {
        Ok(app::chat::PendingMutationResult::Applied)
    }
}

struct RateLimiter;

#[async_trait]
impl app::chat::RateLimiter for RateLimiter {
    async fn check(
        &self,
        _room_id: &domain_chat::room::Id,
        _user_id: &domain_chat::UserId,
    ) -> app::chat::Result<()> {
        Ok(())
    }
}

struct AuditLog;

#[async_trait]
impl app::chat::audit::Log for AuditLog {
    async fn record(&self, _entry: app::chat::audit::Entry) -> app::chat::Result<()> {
        Ok(())
    }
}

struct Clock;

impl app::chat::Clock for Clock {
    fn now(&self) -> SystemTime {
        UNIX_EPOCH
    }
}

struct Ids;

impl app::chat::IdGenerator for Ids {
    fn new_room_id(&self) -> domain_chat::room::Id {
        domain_chat::room::Id::new_v4()
    }

    fn new_message_id(&self) -> domain_chat::message::Id {
        domain_chat::message::Id::new_v4()
    }
}

#[derive(Clone)]
struct FixedSensitiveClock;

impl app::sensitive::Clock for FixedSensitiveClock {
    fn now(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(100)
    }
}

struct SensitiveProvider;

#[async_trait]
impl app::sensitive::ProviderClient for SensitiveProvider {
    fn boundary_meta(
        &self,
        _provider: domain_sensitive::Provider,
    ) -> app::sensitive::ProviderBoundaryMeta {
        app::sensitive::ProviderBoundaryMeta::builder()
            .mode(domain_sensitive::ProviderMode::LocalStub)
            .endpoint(
                domain_sensitive::DetailText::try_new("http://127.0.0.1:4002/")
                    .expect("detail"),
            )
            .maybe_auth_mode(Some(domain_sensitive::ProviderAuthMode::StubIssuedToken))
            .maybe_retry_backoff_secs(None)
            .build()
    }

    async fn refresh_token(
        &self,
        provider: domain_sensitive::Provider,
        now: SystemTime,
        _current_token: Option<&SecretString>,
    ) -> app::sensitive::Result<app::sensitive::ProviderToken> {
        Ok(app::sensitive::ProviderToken::builder()
            .status(
                domain_sensitive::TokenStatus::builder()
                    .provider(provider)
                    .expires_at(now + Duration::from_secs(300))
                    .refreshed_at(now)
                    .build(),
            )
            .access_token(SecretString::new("token".into()))
            .build())
    }

    async fn fetch_records(
        &self,
        _provider: domain_sensitive::Provider,
        _token: &app::sensitive::ProviderToken,
        _cursor: Option<&domain_sensitive::SyncCursor>,
        _now: SystemTime,
    ) -> app::sensitive::Result<app::sensitive::ProviderRecords> {
        Ok(app::sensitive::ProviderRecords::builder()
            .records(Vec::new())
            .maybe_cursor(None)
            .build())
    }
}

struct SensitiveRepoState {
    snapshot: app::sensitive::StoredSnapshot,
    key_custody: domain_sensitive::KeyCustodyState,
    authorized_record: Option<domain_sensitive::AuthorizedRecord>,
    grants: Vec<domain_sensitive::AccessGrant>,
    access_events: Vec<domain_sensitive::AccessEvent>,
}

struct SensitiveRepo {
    state: Mutex<SensitiveRepoState>,
}

impl SensitiveRepo {
    fn new(capabilities: Vec<domain_sensitive::AccessCapability>) -> Self {
        let record_id = domain_sensitive::Id::from(uuid::Uuid::from_u128(0x1111));
        let snapshot = app::sensitive::StoredSnapshot::builder()
            .maybe_token(Some(token_proof()))
            .maybe_latest_sync(Some(latest_sync()))
            .maybe_integration_state(Some(integration_state()))
            .records(vec![record_proof(record_id)])
            .build();
        let grants = capabilities
            .into_iter()
            .map(access_grant)
            .collect::<Vec<_>>();

        Self {
            state: Mutex::new(SensitiveRepoState {
                snapshot,
                key_custody: key_custody_state(),
                authorized_record: Some(authorized_record(record_id)),
                grants,
                access_events: vec![
                    domain_sensitive::AccessEvent::builder()
                        .maybe_user_id(None)
                        .capability(
                            domain_sensitive::AccessCapability::AuthorizedRecordRead,
                        )
                        .maybe_record_id(Some(record_id))
                        .outcome(domain_sensitive::AccessOutcome::Denied)
                        .detail(
                            domain_sensitive::DetailText::try_new(
                                "sign in required before authorized record read",
                            )
                            .expect("detail"),
                        )
                        .occurred_at(UNIX_EPOCH)
                        .build(),
                ],
            }),
        }
    }
}

#[async_trait]
impl app::sensitive::Repository for SensitiveRepo {
    async fn load_snapshot(
        &self,
    ) -> app::sensitive::Result<app::sensitive::StoredSnapshot> {
        Ok(self.state.lock().expect("repo state").snapshot.clone())
    }

    async fn load_authorized_record(
        &self,
        _record_id: &domain_sensitive::Id,
    ) -> app::sensitive::Result<Option<domain_sensitive::AuthorizedRecord>> {
        Ok(self
            .state
            .lock()
            .expect("repo state")
            .authorized_record
            .clone())
    }

    async fn load_access_grants(
        &self,
        user_id: &domain_user::Id,
    ) -> app::sensitive::Result<Vec<domain_sensitive::AccessGrant>> {
        let state = self.state.lock().expect("repo state");
        Ok(state
            .grants
            .iter()
            .filter(|grant| &grant.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn load_key_custody(
        &self,
    ) -> app::sensitive::Result<domain_sensitive::KeyCustodyState> {
        Ok(self.state.lock().expect("repo state").key_custody.clone())
    }

    async fn load_integration_state(
        &self,
        _provider: domain_sensitive::Provider,
    ) -> app::sensitive::Result<Option<domain_sensitive::IntegrationState>> {
        Ok(self
            .state
            .lock()
            .expect("repo state")
            .snapshot
            .integration_state
            .clone())
    }

    async fn load_token(
        &self,
        _provider: domain_sensitive::Provider,
    ) -> app::sensitive::Result<Option<app::sensitive::ProviderToken>> {
        Ok(None)
    }

    async fn upsert_token(
        &self,
        _token: &app::sensitive::ProviderToken,
    ) -> app::sensitive::Result<()> {
        Ok(())
    }

    async fn upsert_records(
        &self,
        _records: &[domain_sensitive::Record],
        _synced_at: SystemTime,
    ) -> app::sensitive::Result<usize> {
        Ok(0)
    }

    async fn upsert_integration_state(
        &self,
        state: &domain_sensitive::IntegrationState,
    ) -> app::sensitive::Result<()> {
        self.state
            .lock()
            .expect("repo state")
            .snapshot
            .integration_state = Some(state.clone());
        Ok(())
    }

    async fn rotate_ciphertext_to_active_key(
        &self,
        _limit: usize,
        _rotated_at: SystemTime,
    ) -> app::sensitive::Result<app::sensitive::KeyRotationProgress> {
        Ok(app::sensitive::KeyRotationProgress::builder()
            .active_key_id(domain_sensitive::KeyId::try_new("active_data_key").unwrap())
            .rows_scanned(2)
            .rows_rewrapped(2)
            .rows_already_current(0)
            .rows_failed(0)
            .detail(
                domain_sensitive::DetailText::try_new(
                    "stale ciphertext rewrapped to the active key",
                )
                .expect("detail"),
            )
            .build())
    }

    async fn record_sync_run(
        &self,
        _run: &domain_sensitive::SyncRun,
    ) -> app::sensitive::Result<()> {
        Ok(())
    }

    async fn record_key_rotation_run(
        &self,
        run: &domain_sensitive::KeyRotationRun,
    ) -> app::sensitive::Result<()> {
        self.state
            .lock()
            .expect("repo state")
            .key_custody
            .last_rotation_run = Some(run.clone());
        Ok(())
    }

    async fn upsert_access_grants(
        &self,
        user_id: &domain_user::Id,
        capabilities: &[domain_sensitive::AccessCapability],
        granted_at: SystemTime,
    ) -> app::sensitive::Result<()> {
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
                domain_sensitive::AccessGrant::builder()
                    .user_id(*user_id)
                    .capability(*capability)
                    .granted_at(granted_at)
                    .build(),
            );
        }
        Ok(())
    }

    async fn record_access_event(
        &self,
        event: &domain_sensitive::AccessEvent,
    ) -> app::sensitive::Result<()> {
        self.state
            .lock()
            .expect("repo state")
            .access_events
            .push(event.clone());
        Ok(())
    }

    async fn list_recent_access_events(
        &self,
        limit: usize,
    ) -> app::sensitive::Result<Vec<domain_sensitive::AccessEvent>> {
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

fn test_app(capabilities: Vec<domain_sensitive::AccessCapability>) -> axum::Router {
    let user_repo = Arc::new(TestUserRepo);
    let hasher = Arc::new(TestHasher);
    let user_service = user::Service::new(user_repo, hasher);
    let auth_service = auth::Service::new(Arc::new(TestAuthProvider));
    let sse_registry = app_http::sse::Registry::new();
    let trace_log = app_http::trace_log::Store::builder()
        .with_sse(sse_registry.clone())
        .build();
    let cookie_key = Key::generate();
    let chat = app::chat::Service::builder()
        .with_repo(Arc::new(ChatRepo))
        .with_moderation_queue(Arc::new(ModerationQueue))
        .with_rate_limiter(Arc::new(RateLimiter))
        .with_audit_log(Arc::new(AuditLog))
        .with_clock(Arc::new(Clock))
        .with_id_generator(Arc::new(Ids))
        .build();
    let sensitive_repo = Arc::new(SensitiveRepo::new(capabilities));
    let sensitive = app::sensitive::Service::builder()
        .with_repo(sensitive_repo)
        .with_provider(Arc::new(SensitiveProvider))
        .with_clock(Arc::new(FixedSensitiveClock))
        .build();
    let state = app_http::State::builder()
        .with_user(user_service)
        .with_auth(auth_service)
        .with_chat(chat)
        .with_sensitive(sensitive)
        .with_sse(sse_registry)
        .with_cookie_key(cookie_key)
        .with_trace_log(trace_log)
        .build();
    let session_store = MemoryStore::default();
    app_http::router(state, session_store)
}

async fn login_cookie(app: axum::Router) -> String {
    let body = "email=demo%40example.com&password=password&next=%2Flab%23sensitive-proof";
    let response = app
        .oneshot(
            Request::post("/login")
                .header(
                    axum::http::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    response
        .headers()
        .get_all(axum::http::header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| value.starts_with("eran.sid="))
        .and_then(|set_cookie| set_cookie.split(';').next())
        .map(str::to_string)
        .expect("eran.sid cookie")
}

fn raw_session_id(cookie_header: &str) -> &str {
    cookie_header
        .strip_prefix("eran.sid=")
        .expect("session cookie prefix")
}

fn assert_redacted_support_trace(body: &str, hidden_values: &[&str]) {
    assert!(body.contains("Redacted trace log"));
    assert!(!body.contains("session_id="));
    assert!(!body.contains("user_id="));
    assert!(!body.contains("sse_tab_id="));
    for value in hidden_values {
        assert!(
            !body.contains(value),
            "support trace should redact `{value}`\n{body}"
        );
    }
}

fn record_proof(record_id: domain_sensitive::Id) -> app::sensitive::RecordProof {
    app::sensitive::RecordProof::builder()
        .id(record_id)
        .label(domain_sensitive::Label::try_new("Alpha file").unwrap())
        .last4(domain_sensitive::Last4::try_new("1001").unwrap())
        .synced_at(UNIX_EPOCH)
        .ciphertext(
            app::sensitive::CiphertextEvidence::builder()
                .key_id(domain_sensitive::KeyId::try_new("legacy_data_key").unwrap())
                .preview("ciphertext-preview".to_string())
                .bytes(32)
                .build(),
        )
        .build()
}

fn authorized_record(
    record_id: domain_sensitive::Id,
) -> domain_sensitive::AuthorizedRecord {
    domain_sensitive::AuthorizedRecord::builder()
        .id(record_id)
        .label(domain_sensitive::Label::try_new("Alpha file").unwrap())
        .last4(domain_sensitive::Last4::try_new("1001").unwrap())
        .authorized(
            domain_sensitive::AuthorizedFields::builder()
                .subject_name(domain_sensitive::DetailText::try_new("Case alpha").unwrap())
                .classification(
                    domain_sensitive::DetailText::try_new("synthetic_record").unwrap(),
                )
                .note(
                    domain_sensitive::DetailText::try_new("Authorized path only.").unwrap(),
                )
                .build(),
        )
        .synced_at(UNIX_EPOCH)
        .build()
}

fn access_grant(
    capability: domain_sensitive::AccessCapability,
) -> domain_sensitive::AccessGrant {
    domain_sensitive::AccessGrant::builder()
        .user_id(domain_user::Id::from(USER_ID))
        .capability(capability)
        .granted_at(UNIX_EPOCH)
        .build()
}

fn token_proof() -> app::sensitive::TokenProof {
    app::sensitive::TokenProof::builder()
        .status(
            domain_sensitive::TokenStatus::builder()
                .provider(domain_sensitive::Provider::SyntheticSecureFeed)
                .expires_at(UNIX_EPOCH + Duration::from_secs(600))
                .refreshed_at(UNIX_EPOCH)
                .build(),
        )
        .ciphertext(
            app::sensitive::CiphertextEvidence::builder()
                .key_id(domain_sensitive::KeyId::try_new("legacy_data_key").unwrap())
                .preview("token-preview".to_string())
                .bytes(64)
                .build(),
        )
        .build()
}

fn key_custody_state() -> domain_sensitive::KeyCustodyState {
    domain_sensitive::KeyCustodyState::builder()
        .active_key_id(domain_sensitive::KeyId::try_new("active_data_key").unwrap())
        .configured_keys(vec![
            domain_sensitive::ConfiguredKey::builder()
                .key_id(domain_sensitive::KeyId::try_new("active_data_key").unwrap())
                .status(domain_sensitive::CipherKeyStatus::Active)
                .build(),
            domain_sensitive::ConfiguredKey::builder()
                .key_id(domain_sensitive::KeyId::try_new("legacy_data_key").unwrap())
                .status(domain_sensitive::CipherKeyStatus::ReadOnlyLegacy)
                .build(),
        ])
        .token_counts(vec![
            domain_sensitive::KeyedCiphertextCount::builder()
                .key_id(domain_sensitive::KeyId::try_new("legacy_data_key").unwrap())
                .count(1)
                .build(),
        ])
        .record_counts(vec![
            domain_sensitive::KeyedCiphertextCount::builder()
                .key_id(domain_sensitive::KeyId::try_new("legacy_data_key").unwrap())
                .count(1)
                .build(),
        ])
        .stale_token_count(1)
        .stale_record_count(1)
        .maybe_last_rotation_run(Some(
            domain_sensitive::KeyRotationRun::builder()
                .active_key_id(domain_sensitive::KeyId::try_new("active_data_key").unwrap())
                .outcome(domain_sensitive::RotationOutcome::Success)
                .rows_scanned(2)
                .rows_rewrapped(2)
                .rows_already_current(0)
                .rows_failed(0)
                .detail(
                    domain_sensitive::DetailText::try_new(
                        "stale ciphertext rewrapped to the active key",
                    )
                    .unwrap(),
                )
                .started_at(UNIX_EPOCH)
                .finished_at(UNIX_EPOCH + Duration::from_secs(1))
                .build(),
        ))
        .build()
}

fn latest_sync() -> domain_sensitive::SyncRun {
    domain_sensitive::SyncRun::builder()
        .provider(domain_sensitive::Provider::SyntheticSecureFeed)
        .outcome(domain_sensitive::SyncOutcome::Success)
        .records_seen(1)
        .records_upserted(1)
        .detail(
            domain_sensitive::DetailText::try_new(
                "1 synthetic record processed for runtime proof",
            )
            .unwrap(),
        )
        .started_at(UNIX_EPOCH)
        .finished_at(UNIX_EPOCH + Duration::from_secs(1))
        .build()
}

fn integration_state() -> domain_sensitive::IntegrationState {
    domain_sensitive::IntegrationState::builder()
        .provider(domain_sensitive::Provider::SyntheticSecureFeed)
        .mode(domain_sensitive::ProviderMode::LocalStub)
        .endpoint(domain_sensitive::DetailText::try_new("http://127.0.0.1:4002/").unwrap())
        .maybe_auth_mode(Some(domain_sensitive::ProviderAuthMode::StubIssuedToken))
        .maybe_cursor(Some(
            domain_sensitive::SyncCursor::try_new("cursor-gamma").unwrap(),
        ))
        .last_fetch_outcome(domain_sensitive::FetchOutcome::Success)
        .maybe_last_auth_outcome(Some(domain_sensitive::FetchOutcome::Success))
        .token_strategy(domain_sensitive::TokenStrategy::RetryAfterUnauthorized)
        .maybe_last_error_category(Some(
            domain_sensitive::RemoteErrorCategory::Unauthorized,
        ))
        .maybe_last_remote_status_code(Some(401))
        .maybe_retry_backoff_secs(Some(45))
        .maybe_last_successful_mode(Some(domain_sensitive::ProviderMode::LocalStub))
        .maybe_last_successful_fetch_at(Some(UNIX_EPOCH + Duration::from_secs(1)))
        .last_attempted_fetch_at(UNIX_EPOCH + Duration::from_secs(2))
        .failure_count(1)
        .build()
}

#[tokio::test]
async fn guest_sensitive_proof_shows_sign_in_gate() {
    let app = test_app(Vec::new());
    let response = app
        .oneshot(
            Request::get("/partials/sensitive-proof?sseTabId=proof-tab")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("viewer_tier"));
    assert!(body.contains("guest"));
    assert!(body.contains("Sign in to request the authorized sample record path."));
    assert!(!body.contains("Authorized path only."));
    assert_redacted_support_trace(&body, &["proof-tab"]);
}

#[tokio::test]
async fn signed_in_viewer_without_grant_shows_denied_state() {
    let app = test_app(Vec::new());
    let cookie_header = login_cookie(app.clone()).await;
    let session_id = raw_session_id(&cookie_header).to_string();
    let user_id = USER_ID.to_string();
    let response = app
        .oneshot(
            Request::get("/partials/sensitive-proof?sseTabId=proof-tab")
                .header(axum::http::header::COOKIE, cookie_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("authenticated"));
    assert!(body.contains(
        "Signed-in viewer is denied until an authorized_record_read grant exists."
    ));
    assert!(!body.contains("Authorized path only."));
    assert_redacted_support_trace(&body, &["proof-tab", &session_id, &user_id]);
}

#[tokio::test]
async fn reader_sensitive_proof_shows_authorized_record_but_hides_operator_panels() {
    let app = test_app(vec![
        domain_sensitive::AccessCapability::AuthorizedRecordRead,
    ]);
    let cookie_header = login_cookie(app.clone()).await;
    let session_id = raw_session_id(&cookie_header).to_string();
    let user_id = USER_ID.to_string();
    let response = app
        .oneshot(
            Request::get("/partials/sensitive-proof?sseTabId=proof-tab")
                .header(axum::http::header::COOKIE, cookie_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("Case alpha"));
    assert!(body.contains("Authorized path only."));
    assert!(body.contains("Token lifecycle state is restricted to sensitive operators."));
    assert!(body.contains("Provider boundary state is restricted to sensitive operators."));
    assert!(
        body.contains("Recent access-audit evidence is restricted to sensitive operators.")
    );
    assert_redacted_support_trace(&body, &["proof-tab", &session_id, &user_id]);
}

#[tokio::test]
async fn operator_sensitive_proof_shows_token_and_audit_evidence() {
    let app = test_app(vec![
        domain_sensitive::AccessCapability::AuthorizedRecordRead,
        domain_sensitive::AccessCapability::TokenStatusRead,
        domain_sensitive::AccessCapability::AccessAuditRead,
    ]);
    let cookie_header = login_cookie(app.clone()).await;
    let session_id = raw_session_id(&cookie_header).to_string();
    let user_id = USER_ID.to_string();
    let response = app
        .oneshot(
            Request::get("/partials/sensitive-proof?sseTabId=proof-tab")
                .header(axum::http::header::COOKIE, cookie_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("synthetic_secure_feed"));
    assert!(body.contains("Case alpha"));
    assert!(body.contains("Recent access audit"));
    assert!(body.contains("authorized_record_read"));
    assert!(body.contains("allowed"));
    assert!(body.contains("Boundary state"));
    assert!(body.contains("retry_after_unauthorized"));
    assert!(body.contains("http://127.0.0.1:4002/"));
    assert!(body.contains("Key custody"));
    assert!(body.contains("active_data_key"));
    assert!(body.contains("legacy_data_key: 1"));
    assert!(body.contains("authenticated (redacted)"));
    assert_redacted_support_trace(&body, &["proof-tab", &session_id, &user_id]);
}

#[tokio::test]
async fn guest_request_meta_redacts_internal_request_context() {
    let app = test_app(Vec::new());
    let response = app
        .oneshot(
            Request::get("/partials/request-meta?sseTabId=request-meta-tab")
                .header("x-request-id", "req-meta-guest")
                .header("x-real-ip", "203.0.113.5")
                .header(axum::http::header::USER_AGENT, "ExampleBrowser/1.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("req-meta-guest"));
    assert!(body.contains("present (redacted)"));
    assert!(body.contains("captured (redacted)"));
    assert!(!body.contains("203.0.113.5"));
    assert!(!body.contains("ExampleBrowser/1.0"));
    assert_redacted_support_trace(&body, &["request-meta-tab"]);
}

#[tokio::test]
async fn signed_in_request_meta_redacts_authenticated_identifiers() {
    let app = test_app(Vec::new());
    let cookie_header = login_cookie(app.clone()).await;
    let session_id = raw_session_id(&cookie_header).to_string();
    let user_id = USER_ID.to_string();
    let response = app
        .oneshot(
            Request::get("/partials/request-meta?sseTabId=request-meta-tab")
                .header(axum::http::header::COOKIE, cookie_header)
                .header("x-request-id", "req-meta-user")
                .header("x-real-ip", "198.51.100.7")
                .header(axum::http::header::USER_AGENT, "SignedInBrowser/2.0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("req-meta-user"));
    assert!(body.contains("authenticated (redacted)"));
    assert!(body.contains("present (redacted)"));
    assert!(body.contains("captured (redacted)"));
    assert!(!body.contains("198.51.100.7"));
    assert!(!body.contains("SignedInBrowser/2.0"));
    assert_redacted_support_trace(&body, &["request-meta-tab", &session_id, &user_id]);
}
