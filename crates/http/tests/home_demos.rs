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
        _email: &domain_user::Email,
    ) -> user::Result<Option<domain_user::User>> {
        Ok(None)
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
    async fn create_room(
        &self,
        _room: &domain_chat::Room,
    ) -> app::chat::Result<()> {
        Ok(())
    }

    async fn find_room(
        &self,
        _room_id: &domain_chat::RoomId,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(None)
    }

    async fn find_room_by_name(
        &self,
        _name: &domain_chat::RoomName,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(None)
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
        _role: &str,
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
        _reason: &str,
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
        _reason: Option<String>,
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
    async fn record(
        &self,
        _entry: app::chat::AuditEntry,
    ) -> app::chat::Result<()> {
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

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = String::from_utf8_lossy(&body);

    for copy in HomeCopy::all() {
        assert!(body.contains(copy.as_str()));
    }
}

#[derive(Clone, Copy, Debug)]
enum HomeCopy {
    Demo1,
    Demo2,
    Demo3,
    Demo4,
    Demo5,
    Demo6,
    CheckAuth,
    ShowSession,
    CheckDemoEmail,
    LiveBackendLog,
    StartDemo,
    SignIn,
    RegisterPath,
    LoginPath,
}

impl HomeCopy {
    fn all() -> &'static [HomeCopy] {
        &[
            HomeCopy::Demo1,
            HomeCopy::Demo2,
            HomeCopy::Demo3,
            HomeCopy::Demo4,
            HomeCopy::Demo5,
            HomeCopy::Demo6,
            HomeCopy::CheckAuth,
            HomeCopy::ShowSession,
            HomeCopy::CheckDemoEmail,
            HomeCopy::LiveBackendLog,
            HomeCopy::StartDemo,
            HomeCopy::SignIn,
            HomeCopy::RegisterPath,
            HomeCopy::LoginPath,
        ]
    }

    fn as_str(self) -> &'static str {
        match self {
            HomeCopy::Demo1 => "Demo 1: Auth flow walkthrough",
            HomeCopy::Demo2 => "Demo 2: Persistent session resilience",
            HomeCopy::Demo3 => "Demo 3: Architecture boundary map",
            HomeCopy::Demo4 => "Demo 4: Error handling showcase",
            HomeCopy::Demo5 => "Demo 5: Tracing and observability",
            HomeCopy::Demo6 => "Demo 6: SSE and Datastar patches",
            HomeCopy::CheckAuth => "Check auth status",
            HomeCopy::ShowSession => "Show session details",
            HomeCopy::CheckDemoEmail => "Check demo@example.com",
            HomeCopy::LiveBackendLog => "Live backend log",
            HomeCopy::StartDemo => "Start demo",
            HomeCopy::SignIn => "Sign in",
            HomeCopy::RegisterPath => "/register",
            HomeCopy::LoginPath => "/login",
        }
    }
}
