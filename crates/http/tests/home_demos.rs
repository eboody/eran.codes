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
use domain::user as domain_user;
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
        _password_hash: &str,
    ) -> user::Result<()> {
        Ok(())
    }
}

struct TestHasher;

impl auth::PasswordHasher for TestHasher {
    fn hash(&self, _password: &str) -> auth::Result<String> {
        Ok("hash".to_string())
    }

    fn verify(
        &self,
        _password: &str,
        _password_hash: &str,
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
    let trace_log = app_http::trace_log::Store::builder()
        .with_sse(sse_registry.clone())
        .build();
    let state = app_http::State::builder()
        .with_user(user_service)
        .with_auth(auth_service)
        .with_sse(sse_registry)
        .with_cookie_key(cookie_key)
        .with_trace_log(trace_log)
        .build();
    let session_store = MemoryStore::default();
    app_http::router(state, session_store)
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

    assert!(body.contains("Demo 1: Auth flow walkthrough"));
    assert!(body.contains("Demo 2: Persistent session resilience"));
    assert!(body.contains("Demo 3: Architecture boundary map"));
    assert!(body.contains("Demo 4: Error handling showcase"));
    assert!(body.contains("Demo 5: Tracing and observability"));
    assert!(body.contains("Demo 6: SSE and Datastar patches"));
    assert!(body.contains("Check auth status"));
    assert!(body.contains("Show session details"));
    assert!(body.contains("Check demo@example.com"));
    assert!(body.contains("Live backend log"));
    assert!(body.contains("Start demo"));
    assert!(body.contains("Sign in"));
    assert!(body.contains("/register"));
    assert!(body.contains("/login"));
}
