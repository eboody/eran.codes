use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::{Body, to_bytes},
    http::{Request, header::SET_COOKIE},
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
            id: domain_user::Id::from(uuid::Uuid::nil()),
            username: domain_user::Username::try_new("demo_bot").unwrap(),
            email: email.clone(),
        }))
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

fn test_app() -> axum::Router {
    let user_repo = Arc::new(TestUserRepo);
    let hasher = Arc::new(TestHasher);
    let user_service = user::Service::new(user_repo, hasher);
    let auth_service = auth::Service::disabled();
    let sse_registry = app_http::sse::Registry::new();
    let cookie_key = Key::generate();
    let trace_log = app_http::trace_log::Store::builder()
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

struct LabSessionContext {
    cookie_header: String,
    sse_tab_id: String,
}

async fn load_lab_session_context(app: axum::Router) -> LabSessionContext {
    let response = app
        .oneshot(Request::get("/lab").body(Body::empty()).unwrap())
        .await
        .expect("lab page response");

    let cookie_header = response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| value.starts_with("session_id="))
        .and_then(|value| value.split(';').next())
        .map(str::to_string)
        .expect("session_id cookie");

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("lab body bytes");
    let body = String::from_utf8(body.to_vec()).expect("lab body utf-8");

    LabSessionContext {
        cookie_header,
        sse_tab_id: extract_between(&body, "sseTabId: '", "'").expect("lab sse tab id"),
    }
}

fn extract_between(body: &str, start: &str, end: &str) -> Option<String> {
    let start_index = body.find(start)? + start.len();
    let remainder = &body[start_index..];
    let end_index = remainder.find(end)?;
    Some(remainder[..end_index].to_string())
}

struct ChatRepo;

#[async_trait]
impl app::chat::Repository for ChatRepo {
    async fn create_room(&self, _room: &domain_chat::Room) -> app::chat::Result<()> {
        Ok(())
    }

    async fn find_room(
        &self,
        room_id: &domain_chat::room::Id,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(Some(domain_chat::Room {
            id: *room_id,
            name: domain_chat::room::Name::Lobby,
            created_by: domain_chat::UserId::from(uuid::Uuid::nil()),
        }))
    }

    async fn find_room_by_name(
        &self,
        name: &domain_chat::room::Name,
    ) -> app::chat::Result<Option<domain_chat::Room>> {
        Ok(Some(domain_chat::Room {
            id: domain_chat::room::Id::new_v4(),
            name: *name,
            created_by: domain_chat::UserId::from(uuid::Uuid::nil()),
        }))
    }

    async fn list_messages(
        &self,
        room_id: &domain_chat::room::Id,
        _limit: usize,
    ) -> app::chat::Result<Vec<domain_chat::Message>> {
        Ok(vec![
            domain_chat::Message::builder()
                .id(domain_chat::message::Id::from(uuid::Uuid::from_u128(
                    0xaaaa,
                )))
                .room_id(*room_id)
                .user_id(domain_chat::UserId::from(uuid::Uuid::nil()))
                .body(
                    domain_chat::message::Body::try_new("hello from test")
                        .expect("valid body"),
                )
                .status(domain_chat::message::Status::Visible)
                .maybe_client_id(None)
                .created_at(std::time::SystemTime::UNIX_EPOCH)
                .build(),
        ])
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

#[tokio::test]
async fn lab_page_includes_demo_sections() {
    let app = test_app();
    let response = app
        .oneshot(Request::get("/lab").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert_eq!(status, axum::http::StatusCode::OK, "home body:\n{body}");

    for copy in LabContract::all() {
        assert!(body.contains(copy.as_str()));
    }
}

#[derive(Clone, Copy, Debug)]
enum LabContract {
    OperationsSurface,
    NetworkLogTarget,
    RequestBurstEndpoint,
    ChatAnchor,
    ChatMessageBody,
    ChatRoomName,
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

impl LabContract {
    fn all() -> &'static [LabContract] {
        &[
            LabContract::OperationsSurface,
            LabContract::NetworkLogTarget,
            LabContract::RequestBurstEndpoint,
            LabContract::ChatAnchor,
            LabContract::ChatMessageBody,
            LabContract::ChatRoomName,
            LabContract::TablistRole,
            LabContract::TabRole,
            LabContract::TabpanelRole,
            LabContract::ResumeLink,
            LabContract::GithubLink,
            LabContract::LinkedInLink,
            LabContract::ContactLink,
            LabContract::SignIn,
            LabContract::RegisterPath,
            LabContract::LoginPath,
        ]
    }

    fn as_str(self) -> &'static str {
        match self {
            LabContract::OperationsSurface => "id=\"operations-surface\"",
            LabContract::NetworkLogTarget => "id=\"network-log-target\"",
            LabContract::RequestBurstEndpoint => "/partials/request-burst-probe",
            LabContract::ChatAnchor => "id=\"chat-demo\"",
            LabContract::ChatMessageBody => "hello from test",
            LabContract::ChatRoomName => "Room: Lobby",
            LabContract::TablistRole => "role=\"tablist\"",
            LabContract::TabRole => "role=\"tab\"",
            LabContract::TabpanelRole => "role=\"tabpanel\"",
            LabContract::ResumeLink => "/static/resume.txt",
            LabContract::GithubLink => "https://github.com/eboody/eran.codes",
            LabContract::LinkedInLink => {
                "https://www.linkedin.com/search/results/all/?keywords=Eran%20Boodnero"
            }
            LabContract::ContactLink => "mailto:eboodnero@gmail.com",
            LabContract::SignIn => "Sign in",
            LabContract::RegisterPath => "/register",
            LabContract::LoginPath => "/login",
        }
    }
}

#[tokio::test]
async fn home_page_includes_portfolio_sections() {
    let app = test_app();
    let response = app
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert_eq!(status, axum::http::StatusCode::OK, "home body:\n{body}");

    for copy in PortfolioHomeContract::all() {
        assert!(body.contains(copy.as_str()));
    }
}

#[derive(Clone, Copy, Debug)]
enum PortfolioHomeContract {
    PortfolioMain,
    HeroSection,
    WorkRoute,
    OpenSourceRoute,
    LabRoute,
    WorkCaseRoute,
    OpenSourceTitle,
    BrandMarkWrap,
    LightBrandLogo,
    DarkBrandLogo,
    SvgLightFavicon,
    SvgDarkFavicon,
    PngFavicon,
    AppleTouchIcon,
}

impl PortfolioHomeContract {
    fn all() -> &'static [PortfolioHomeContract] {
        &[
            PortfolioHomeContract::PortfolioMain,
            PortfolioHomeContract::HeroSection,
            PortfolioHomeContract::WorkRoute,
            PortfolioHomeContract::OpenSourceRoute,
            PortfolioHomeContract::LabRoute,
            PortfolioHomeContract::WorkCaseRoute,
            PortfolioHomeContract::OpenSourceTitle,
            PortfolioHomeContract::BrandMarkWrap,
            PortfolioHomeContract::LightBrandLogo,
            PortfolioHomeContract::DarkBrandLogo,
            PortfolioHomeContract::SvgLightFavicon,
            PortfolioHomeContract::SvgDarkFavicon,
            PortfolioHomeContract::PngFavicon,
            PortfolioHomeContract::AppleTouchIcon,
        ]
    }

    fn as_str(self) -> &'static str {
        match self {
            PortfolioHomeContract::PortfolioMain => "data-portfolio-page",
            PortfolioHomeContract::HeroSection => "ui-portfolio-hero",
            PortfolioHomeContract::WorkRoute => "href=\"/work\"",
            PortfolioHomeContract::OpenSourceRoute => "href=\"/open-source\"",
            PortfolioHomeContract::LabRoute => "href=\"/lab\"",
            PortfolioHomeContract::WorkCaseRoute => "href=\"/work/chat-realtime\"",
            PortfolioHomeContract::OpenSourceTitle => "Open-source proof of how I think",
            PortfolioHomeContract::BrandMarkWrap => "data-nav-brand-mark-wrap",
            PortfolioHomeContract::LightBrandLogo => "/static/eran.codes-light.svg",
            PortfolioHomeContract::DarkBrandLogo => "/static/eran.codes-dark.svg",
            PortfolioHomeContract::SvgLightFavicon => {
                "media=\"(prefers-color-scheme: light)\""
            }
            PortfolioHomeContract::SvgDarkFavicon => {
                "media=\"(prefers-color-scheme: dark)\""
            }
            PortfolioHomeContract::PngFavicon => "/static/eran.codes-favicon.png",
            PortfolioHomeContract::AppleTouchIcon => {
                "rel=\"apple-touch-icon\" sizes=\"1024x1024\" href=\"/static/eran.codes.png\""
            }
        }
    }
}

#[tokio::test]
async fn request_burst_probe_returns_no_content() {
    let app = test_app();
    let response = app
        .oneshot(
            Request::get("/partials/request-burst-probe")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::NO_CONTENT);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert!(body.is_empty());
}

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let app = test_app();
    let response = app
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&body[..], b"OK");
}

#[tokio::test]
async fn work_routes_render_successfully() {
    let app = test_app();
    let routes = [
        "/work",
        "/open-source",
        "/work/chat-realtime",
        "/work/command-sse",
        "/work/operational-visibility",
    ];

    for route in routes {
        let response = app
            .clone()
            .oneshot(Request::get(route).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            axum::http::StatusCode::OK,
            "route {route} should return 200",
        );
    }
}

#[tokio::test]
async fn demo_chat_message_accepts_bound_room_for_current_tab() {
    let app = test_app();
    let context = load_lab_session_context(app.clone()).await;
    let response = app
        .oneshot(
            Request::post("/demo/chat/messages/demo")
                .header(axum::http::header::COOKIE, context.cookie_header)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .header("datastar-request", "1")
                .body(Body::from(format!(
                    r#"{{"chatDemoDraftBody":"hello","sseTabId":"{}"}}"#,
                    context.sse_tab_id
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::ACCEPTED);
}

#[tokio::test]
async fn demo_chat_message_rejects_unbound_tab_for_current_tab() {
    let app = test_app();
    let context = load_lab_session_context(app.clone()).await;
    let response = app
        .oneshot(
            Request::post("/demo/chat/messages/demo")
                .header(axum::http::header::COOKIE, context.cookie_header)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .header("datastar-request", "1")
                .body(Body::from(format!(
                    r#"{{"chatDemoDraftBody":"hello","sseTabId":"{}"}}"#,
                    uuid::Uuid::new_v4()
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    assert!(body.contains("\"transportErrorStatus\":412"));
    assert!(body.contains("\"transportErrorKind\":\"precondition\""));
}
