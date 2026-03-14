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
            id: domain_user::UserId::from_uuid(uuid::Uuid::nil()),
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
        room_id: &domain_chat::RoomId,
        _limit: usize,
    ) -> app::chat::Result<Vec<domain_chat::Message>> {
        Ok(vec![
            domain_chat::Message::builder()
                .id(domain_chat::MessageId::from_uuid(uuid::Uuid::from_u128(
                    0xaaaa,
                )))
                .room_id(*room_id)
                .user_id(domain_chat::UserId::from_uuid(uuid::Uuid::nil()))
                .body(
                    domain_chat::MessageBody::try_new("hello from test")
                        .expect("valid body"),
                )
                .status(domain_chat::MessageStatus::Visible)
                .maybe_client_id(None)
                .created_at(std::time::SystemTime::UNIX_EPOCH)
                .build(),
        ])
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
    ) -> app::chat::Result<app::chat::PendingMutationResult> {
        Ok(app::chat::PendingMutationResult::Applied)
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

    async fn complete_if_pending(
        &self,
        _message_id: &domain_chat::MessageId,
        _reviewer_id: &domain_chat::UserId,
        _decision: app::chat::ModerationDecision,
        _reason: Option<app::chat::ModerationReason>,
    ) -> app::chat::Result<app::chat::PendingMutationResult> {
        Ok(app::chat::PendingMutationResult::Applied)
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
            PortfolioHomeContract::LabRoute => "href=\"/lab\"",
            PortfolioHomeContract::WorkCaseRoute => "href=\"/work/chat-realtime\"",
            PortfolioHomeContract::OpenSourceTitle => "Open-source crates",
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
async fn work_routes_render_successfully() {
    let app = test_app();
    let routes = [
        "/work",
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
