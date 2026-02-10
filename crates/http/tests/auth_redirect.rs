use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use secrecy::{ExposeSecret, SecretString};
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

struct TestAuthProvider;

const USER_ID: uuid::Uuid = uuid::Uuid::from_u128(0xd358d153_19a1_4a4c_8c52_73ff1a1f44d3);

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
            && credentials.password.expose_secret()
                == demo.password().expose_secret()
        {
            return Ok(Some(test_user()));
        }
        Ok(None)
    }

    async fn get_user(
        &self,
        user_id: &domain_user::Id,
    ) -> auth::Result<Option<auth::AuthenticatedUser>> {
        if *user_id == domain_user::Id::from_uuid(USER_ID) {
            return Ok(Some(test_user()));
        }
        Ok(None)
    }
}

fn test_user() -> auth::AuthenticatedUser {
    let username =
        domain_user::Username::try_new("Demo").expect("username");
    let email = TestCredential::Demo.email();
    auth::AuthenticatedUser::builder()
        .id(domain_user::Id::from_uuid(USER_ID))
        .username(username)
        .email(email)
        .session_hash(auth::SessionHash::new("hash"))
        .build()
}

#[derive(Default)]
struct ChatRepo {
    room: Mutex<Option<domain_chat::Room>>,
}

#[async_trait]
impl app::chat::Repository for ChatRepo {
    async fn create_room(
        &self,
        _room: &domain_chat::Room,
    ) -> app::chat::Result<()> {
        let mut slot = self.room.lock().expect("room lock");
        *slot = Some(_room.clone());
        Ok(())
    }

    async fn find_room(
        &self,
        _room_id: &domain_chat::RoomId,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        let slot = self.room.lock().expect("room lock");
        Ok(slot
            .as_ref()
            .filter(|room| &room.id == _room_id)
            .cloned())
    }

    async fn find_room_by_name(
        &self,
        _name: &domain_chat::RoomName,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        let slot = self.room.lock().expect("room lock");
        Ok(slot
            .as_ref()
            .filter(|room| &room.name == _name)
            .cloned())
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

fn test_app() -> axum::Router {
    let user_repo = Arc::new(TestUserRepo);
    let hasher = Arc::new(TestHasher);
    let user_service = user::Service::new(user_repo, hasher);
    let auth_provider = Arc::new(TestAuthProvider);
    let auth_service = auth::Service::new(auth_provider);
    let sse_registry = app_http::SseRegistry::new();
    let trace_log = app_http::trace_log::TraceLogStore::builder()
        .with_sse(sse_registry.clone())
        .build();
    let cookie_key = Key::generate();
    let chat = app::chat::Service::builder()
        .with_repo(Arc::new(ChatRepo::default()))
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

#[tokio::test]
async fn unauthenticated_chat_redirects_to_login() {
    let app = test_app();
    let response = app
        .oneshot(Request::get("/demo/chat").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert!(response.status().is_redirection());
    let location = response
        .headers()
        .get(axum::http::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "/login?next=%2Fdemo%2Fchat");
}

#[tokio::test]
async fn login_redirects_to_next() {
    let app = test_app();
    let body = "email=demo%40example.com&password=password&next=%2Fdemo%2Fchat";
    let response = app
        .clone()
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

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response
        .headers()
        .get(axum::http::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "/demo/chat");
}

#[tokio::test]
async fn login_sets_session_cookie_and_allows_chat() {
    let app = test_app();
    let body = "email=demo%40example.com&password=password&next=%2Fdemo%2Fchat";
    let response = app
        .clone()
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

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let set_cookie = response
        .headers()
        .get_all(axum::http::header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| CookieName::SessionId.matches_cookie(value))
        .map(|value| value.to_string())
        .expect("eran.sid cookie");

    let cookie_header = set_cookie
        .split(';')
        .next()
        .expect("cookie")
        .to_string();
    let response = app
        .oneshot(
            Request::get("/demo/chat")
                .header(axum::http::header::COOKIE, cookie_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[derive(Clone, Copy, Debug)]
enum CookieName {
    SessionId,
}

impl CookieName {
    fn as_str(self) -> &'static str {
        match self {
            CookieName::SessionId => "eran.sid",
        }
    }

    fn matches_cookie(self, value: &str) -> bool {
        value.strip_prefix(self.as_str())
            .and_then(|value| value.strip_prefix('='))
            .is_some()
    }
}
