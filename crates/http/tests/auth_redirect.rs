use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::{
    body::{Body, to_bytes},
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
    let username = domain_user::Username::try_new("Demo").expect("username");
    let email = TestCredential::Demo.email();
    auth::AuthenticatedUser::builder()
        .id(domain_user::Id::from(USER_ID))
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
    async fn create_room(&self, _room: &domain_chat::Room) -> app::chat::Result<()> {
        let mut slot = self.room.lock().expect("room lock");
        *slot = Some(_room.clone());
        Ok(())
    }

    async fn find_room(
        &self,
        _room_id: &domain_chat::room::Id,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        let slot = self.room.lock().expect("room lock");
        Ok(slot.as_ref().filter(|room| &room.id == _room_id).cloned())
    }

    async fn find_room_by_name(
        &self,
        _name: &domain_chat::room::Name,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        let slot = self.room.lock().expect("room lock");
        Ok(slot.as_ref().filter(|room| &room.name == _name).cloned())
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
    fn now(&self) -> std::time::SystemTime {
        std::time::SystemTime::UNIX_EPOCH
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

fn test_app() -> axum::Router {
    let user_repo = Arc::new(TestUserRepo);
    let hasher = Arc::new(TestHasher);
    let user_service = user::Service::new(user_repo, hasher);
    let auth_provider = Arc::new(TestAuthProvider);
    let auth_service = auth::Service::new(auth_provider);
    let sse_registry = app_http::sse::Registry::new();
    let trace_log = app_http::trace_log::Store::builder()
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
        .with_sensitive(app::sensitive::Service::disabled())
        .with_sse(sse_registry)
        .with_cookie_key(cookie_key)
        .with_trace_log(trace_log)
        .build();
    let session_store = MemoryStore::default();
    app_http::router(state, session_store)
}

async fn login_cookie(app: axum::Router) -> String {
    let body = "email=demo%40example.com&password=password&next=%2F%23chat-demo";
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
        .find(|value| CookieName::SessionId.matches_cookie(value))
        .and_then(|set_cookie| set_cookie.split(';').next())
        .map(str::to_string)
        .expect("eran.sid cookie")
}

#[tokio::test]
async fn unauthenticated_chat_moderation_redirects_to_login() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::get("/demo/chat/moderation")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_redirection());
    let location = response
        .headers()
        .get(axum::http::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(location, "/login?next=%2Fdemo%2Fchat%2Fmoderation");
}

#[tokio::test]
async fn login_redirects_to_next() {
    let app = test_app();
    let body = "email=demo%40example.com&password=password&next=%2F%23chat-demo";
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
    assert_eq!(location, "/#chat-demo");
}

#[tokio::test]
async fn login_drops_unsafe_next_and_redirects_to_protected() {
    let app = test_app();
    let body = "email=demo%40example.com&password=password&next=%2F%2Fevil.example";
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
    assert_eq!(location, "/protected");
}

#[tokio::test]
async fn login_form_drops_unsafe_next_from_guest_render() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::get("/login?next=%2F%2Fevil.example")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);

    assert!(
        !body.contains("name=\"next\""),
        "guest login form should not keep an unsafe next field\n{body}",
    );
    assert!(
        !body.contains("next=%2F%2Fevil.example"),
        "guest login page should not echo an unsafe next target\n{body}",
    );
    assert!(
        body.contains("href=\"/register\""),
        "guest login page should fall back to the plain register route\n{body}",
    );
}

#[tokio::test]
async fn authenticated_login_form_drops_unsafe_next_and_redirects_to_protected() {
    let app = test_app();
    let cookie_header = login_cookie(app.clone()).await;
    let response = app
        .oneshot(
            Request::get("/login?next=%2F%2Fevil.example")
                .header(axum::http::header::COOKIE, cookie_header)
                .body(Body::empty())
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
    assert_eq!(location, "/protected");
}

#[tokio::test]
async fn login_sets_session_cookie_and_allows_chat_moderation() {
    let app = test_app();
    let cookie_header = login_cookie(app.clone()).await;
    let response = app
        .oneshot(
            Request::get("/demo/chat/moderation")
                .header(axum::http::header::COOKIE, cookie_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn logout_clears_authenticated_session_and_reprotects_routes() {
    let app = test_app();
    let cookie_header = login_cookie(app.clone()).await;

    let logout = app
        .clone()
        .oneshot(
            Request::post("/logout")
                .header(axum::http::header::COOKIE, cookie_header.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(logout.status(), StatusCode::SEE_OTHER);
    let logout_location = logout
        .headers()
        .get(axum::http::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(logout_location, "/");

    let protected = app
        .oneshot(
            Request::get("/protected")
                .header(axum::http::header::COOKIE, cookie_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(protected.status().is_redirection());
    let protected_location = protected
        .headers()
        .get(axum::http::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(protected_location, "/login?next=%2Fprotected");
}

#[tokio::test]
async fn authenticated_portfolio_pages_render_signed_in_nav() {
    let app = test_app();
    let cookie_header = login_cookie(app.clone()).await;

    for route in ["/", "/work", "/open-source", "/work/sensitive-sync"] {
        let response = app
            .clone()
            .oneshot(
                Request::get(route)
                    .header(axum::http::header::COOKIE, cookie_header.clone())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "route {route} should render"
        );

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8_lossy(&body);

        assert!(
            body.contains("data-nav-auth-text") && body.contains("Signed in as"),
            "route {route} should render the signed-in nav state\n{body}",
        );
        assert!(
            body.contains("Sign out"),
            "route {route} should render the sign-out action\n{body}",
        );
        assert!(
            !body.contains(">Sign in<"),
            "route {route} should not render guest sign-in controls\n{body}",
        );
        assert!(
            !body.contains(">Create account<"),
            "route {route} should not render guest register controls\n{body}",
        );
    }
}

#[tokio::test]
async fn chat_message_without_bound_lab_tab_returns_precondition_failed() {
    let app = test_app();
    let cookie_header = login_cookie(app.clone()).await;
    let response = app
        .oneshot(
            Request::post("/demo/chat/messages")
                .header(axum::http::header::COOKIE, cookie_header)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"chatDraftBody":"hello"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::PRECONDITION_FAILED);
}

#[tokio::test]
async fn invalid_moderation_decision_returns_bad_request() {
    let app = test_app();
    let cookie_header = login_cookie(app.clone()).await;
    let response = app
        .oneshot(
            Request::post("/demo/chat/moderation")
                .header(axum::http::header::COOKIE, cookie_header)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(Body::from(format!(
                    "message_id={}&decision=invalid",
                    uuid::Uuid::new_v4()
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
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
        value
            .strip_prefix(self.as_str())
            .and_then(|value| value.strip_prefix('='))
            .is_some()
    }
}
