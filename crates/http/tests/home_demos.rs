use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use tower::ServiceExt;
use tower_cookies::Key;
use tower_sessions::MemoryStore;

use app::{auth, user};
use domain::{chat as domain_chat, user as domain_user};
use http as app_http;

struct TestUserRepo;

#[async_trait]
impl user::Repository for TestUserRepo {
    async fn find_by_email(
        &self,
        email: &domain_user::Email,
    ) -> user::Result<Option<domain_user::User>> {
        Ok(Some(domain_user::User {
            id: domain_user::Id::from_uuid(uuid::Uuid::nil()),
            username: domain_user::Username::try_new("demo_bot").unwrap(),
            email: email.clone(),
        }))
    }

    async fn create_with_credentials(
        &self,
        _user: &domain_user::User,
        _password_hash: &auth::PasswordHash,
    ) -> user::Result<()> {
        Ok(())
    }
}

struct TestHasher;

impl auth::PasswordHasher for TestHasher {
    fn hash(&self, _password: &str) -> auth::Result<auth::PasswordHash> {
        Ok(auth::PasswordHash::new("hash"))
    }

    fn verify(
        &self,
        _password: &str,
        _password_hash: &auth::PasswordHash,
    ) -> auth::Result<bool> {
        Ok(true)
    }
}

fn test_app() -> axum::Router {
    let user_repo = Arc::new(TestUserRepo);
    let hasher = Arc::new(TestHasher);
    let user_service = user::Service::new(user_repo, hasher);
    let auth_service = auth::Service::disabled();
    let sse_registry = app_http::SseRegistry::new();
    let cookie_key = Key::generate();
    let trace_log = app_http::trace_log::TraceLogStore::builder()
        .with_sse(sse_registry.clone())
        .build();
    let chat = app::chat::Service::builder()
        .with_repo(Arc::new(ChatRepo))
        .with_moderation_queue(Arc::new(ModerationQueue))
        .with_rate_limiter(Arc::new(RateLimiter))
        .with_audit_log(Arc::new(AuditLog))
        .with_clock(Arc::new(Clock))
        .with_id_generator(Arc::new(Ids))
        .build();
    let state = app_http::State::builder()
        .with_user(user_service)
        .with_auth(auth_service)
        .with_chat(chat)
        .with_sse(sse_registry)
        .with_cookie_key(cookie_key)
        .with_trace_log(trace_log)
        .build();
    let session_store = MemoryStore::default();
    app_http::router(state, session_store)
}

struct ChatRepo;

#[async_trait]
impl app::chat::Repository for ChatRepo {
    async fn create_room(&self, _room: &domain_chat::Room) -> app::chat::Result<()> {
        Ok(())
    }

    async fn find_room(
        &self,
        room_id: &domain_chat::RoomId,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(Some(domain_chat::Room {
            id: *room_id,
            name: domain_chat::RoomName::Lobby,
            created_by: domain_chat::UserId::from_uuid(uuid::Uuid::nil()),
        }))
    }

    async fn find_room_by_name(
        &self,
        name: &domain_chat::RoomName,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(Some(domain_chat::Room {
            id: domain_chat::RoomId::new_v4(),
            name: *name,
            created_by: domain_chat::UserId::from_uuid(uuid::Uuid::nil()),
        }))
    }

    async fn list_messages(
        &self,
        _room_id: &domain_chat::RoomId,
        _limit: usize,
    ) -> app::chat::Result<Vec<domain_chat::Message>> {
        Ok(Vec::new())
    }

    async fn find_message(
        &self,
        _message_id: &domain_chat::MessageId,
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
        _room_id: &domain_chat::RoomId,
        _user_id: &domain_chat::UserId,
        _role: app::chat::RoomRole,
    ) -> app::chat::Result<()> {
        Ok(())
    }

    async fn is_member(
        &self,
        _room_id: &domain_chat::RoomId,
        _user_id: &domain_chat::UserId,
    ) -> app::chat::Result<bool> {
        Ok(true)
    }

    async fn update_message_status(
        &self,
        _message_id: &domain_chat::MessageId,
        _status: domain_chat::MessageStatus,
    ) -> app::chat::Result<()> {
        Ok(())
    }
}

struct ModerationQueue;

#[async_trait]
impl app::chat::ModerationQueue for ModerationQueue {
    async fn enqueue(
        &self,
        _message_id: &domain_chat::MessageId,
        _reason: &app::chat::ModerationReason,
    ) -> app::chat::Result<()> {
        Ok(())
    }

    async fn list_pending(
        &self,
        _limit: usize,
    ) -> app::chat::Result<Vec<app::chat::ModerationItem>> {
        Ok(Vec::new())
    }

    async fn complete(
        &self,
        _message_id: &domain_chat::MessageId,
        _reviewer_id: &domain_chat::UserId,
        _decision: app::chat::ModerationDecision,
        _reason: Option<app::chat::ModerationReason>,
    ) -> app::chat::Result<()> {
        Ok(())
    }
}

struct RateLimiter;

#[async_trait]
impl app::chat::RateLimiter for RateLimiter {
    async fn check(
        &self,
        _room_id: &domain_chat::RoomId,
        _user_id: &domain_chat::UserId,
    ) -> app::chat::Result<()> {
        Ok(())
    }
}

struct AuditLog;

#[async_trait]
impl app::chat::AuditLog for AuditLog {
    async fn record(&self, _entry: app::chat::AuditEntry) -> app::chat::Result<()> {
        Ok(())
    }
}

struct Clock;

impl app::chat::Clock for Clock {
    fn now(&self) -> std::time::SystemTime {
        std::time::SystemTime::UNIX_EPOCH
    }
}

struct Ids;

impl app::chat::IdGenerator for Ids {
    fn new_room_id(&self) -> domain_chat::RoomId {
        domain_chat::RoomId::new_v4()
    }

    fn new_message_id(&self) -> domain_chat::MessageId {
        domain_chat::MessageId::new_v4()
    }
}

#[tokio::test]
async fn home_page_includes_demo_sections() {
    let app = test_app();
    let response = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert_eq!(status, axum::http::StatusCode::OK, "home body:\n{body}");

    for copy in HomeCopy::all() {
        assert!(body.contains(copy.as_str()));
    }
}

#[derive(Clone, Copy, Debug)]
enum HomeCopy {
    ThinkingSystems,
    AuditSection,
    ProfessionalismSection,
    FeatureGallery,
    ChatRoom,
    ChatAnchor,
    ReadOnlyPreview,
    LiveBackendLog,
    TablistRole,
    TabRole,
    TabpanelRole,
    ResumeLink,
    GithubLink,
    LinkedInLink,
    ContactLink,
    SignIn,
    RegisterPath,
    LoginPath,
}

impl HomeCopy {
    fn all() -> &'static [HomeCopy] {
        &[
            HomeCopy::ThinkingSystems,
            HomeCopy::AuditSection,
            HomeCopy::ProfessionalismSection,
            HomeCopy::FeatureGallery,
            HomeCopy::ChatRoom,
            HomeCopy::ChatAnchor,
            HomeCopy::ReadOnlyPreview,
            HomeCopy::LiveBackendLog,
            HomeCopy::TablistRole,
            HomeCopy::TabRole,
            HomeCopy::TabpanelRole,
            HomeCopy::ResumeLink,
            HomeCopy::GithubLink,
            HomeCopy::LinkedInLink,
            HomeCopy::ContactLink,
            HomeCopy::SignIn,
            HomeCopy::RegisterPath,
            HomeCopy::LoginPath,
        ]
    }

    fn as_str(self) -> &'static str {
        match self {
            HomeCopy::ThinkingSystems => "How I Think About Systems",
            HomeCopy::AuditSection => "Architecture Audit (What This Site Demonstrates)",
            HomeCopy::ProfessionalismSection => {
                "Professionalism In Practice (Detailed Breakdown)"
            }
            HomeCopy::FeatureGallery => {
                "Feature Gallery: Real-Time Delivery, Grounded in Systems"
            }
            HomeCopy::ChatRoom => "Live chat room",
            HomeCopy::ChatAnchor => "id=\"chat-demo\"",
            HomeCopy::ReadOnlyPreview => "Read-only preview.",
            HomeCopy::LiveBackendLog => "Live backend log (SSE)",
            HomeCopy::TablistRole => "role=\"tablist\"",
            HomeCopy::TabRole => "role=\"tab\"",
            HomeCopy::TabpanelRole => "role=\"tabpanel\"",
            HomeCopy::ResumeLink => "/static/resume.txt",
            HomeCopy::GithubLink => "https://github.com/eboody/eran.codes",
            HomeCopy::LinkedInLink => {
                "https://www.linkedin.com/search/results/all/?keywords=Eran%20Boodnero"
            }
            HomeCopy::ContactLink => "mailto:eboodnero@gmail.com",
            HomeCopy::SignIn => "Sign in",
            HomeCopy::RegisterPath => "/register",
            HomeCopy::LoginPath => "/login",
        }
    }
}
