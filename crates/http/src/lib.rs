mod error;
mod handlers;
mod auth;
pub mod request;
pub mod sse;
mod trace;
mod views;

use axum::middleware::from_fn;
use axum::http::Request;
use axum::{Router, routing::get};
pub use error::{Error, Result};
pub use sse::Registry as SseRegistry;
use tower::ServiceBuilder;
use tower_cookies::{CookieManagerLayer, Key};
use tower_http::{
    classify::ServerErrorsFailureClass,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::field;
use std::time::Duration;
use axum_login::AuthManagerLayerBuilder;
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[derive(Clone)]
pub struct State {
    pub user: app::user::Service,
    pub auth: app::auth::Service,
    pub sse: sse::Registry,
}

impl State {
    pub fn new(
        user: app::user::Service,
        auth: app::auth::Service,
        sse: sse::Registry,
    ) -> Self {
        Self { user, auth, sse }
    }
}

pub fn router(
    state: State,
    session_secret: Vec<u8>,
) -> Router {
    let session_store = MemoryStore::default();
    let session_key = Key::from(&session_secret);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!cfg!(debug_assertions))
        .with_private(session_key);
    let auth_layer = AuthManagerLayerBuilder::new(
        crate::auth::Backend::new(state.auth.clone()),
        session_layer,
    )
    .build();

    let request_layers = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let span = tracing::info_span!(
                        "http.request",
                        method = %request.method(),
                        uri = %request.uri(),
                        path = %request.uri().path(),
                        route = field::Empty,
                        request_id = field::Empty,
                        session_id = field::Empty,
                        user_id = field::Empty,
                        client_ip = field::Empty,
                        user_agent = field::Empty,
                        kind = field::Empty,
                        status = field::Empty,
                        latency_ms = field::Empty,
                    );

                    if let Some(context) = request.extensions().get::<crate::request::Context>() {
                        if let Some(request_id) = context.request_id.as_deref() {
                            span.record("request_id", request_id);
                        }
                        if let Some(session_id) = context.session_id.as_deref() {
                            span.record("session_id", session_id);
                        }
                        if let Some(user_id) = context.user_id.as_deref() {
                            span.record("user_id", user_id);
                        }
                        if let Some(client_ip) = context.client_ip.as_deref() {
                            span.record("client_ip", client_ip);
                        }
                        if let Some(user_agent) = context.user_agent.as_deref() {
                            span.record("user_agent", user_agent);
                        }
                        span.record("kind", context.kind.as_str());
                    }

                    span
                })
                .on_request(|_request: &Request<_>, span: &tracing::Span| {
                    tracing::debug!(parent: span, "request started");
                })
                .on_response(|response: &axum::http::Response<_>, latency: Duration, span: &tracing::Span| {
                    span.record("status", response.status().as_u16());
                    span.record("latency_ms", latency.as_millis() as u64);
                    tracing::info!(parent: span, "request completed");
                })
                .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &tracing::Span| {
                    span.record("latency_ms", latency.as_millis() as u64);
                    tracing::error!(parent: span, error = %error, "request failed");
                }),
        )
        .layer(from_fn(crate::auth::set_user_context_middleware))
        .layer(from_fn(crate::request::set_context_middleware))
        .layer(SetRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
        ));

    let base = Router::new()
        .route("/partials/ping", get(crate::handlers::ping_partial))
        .route("/error-test", get(crate::handlers::error_test))
        .route("/events", get(crate::handlers::events))
        .route("/health", get(crate::handlers::health))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .route_layer(from_fn(crate::trace::record_route_middleware))
        .layer(CookieManagerLayer::new())
        .layer(request_layers)
        .layer(auth_layer)
        .with_state(state.clone());

    let pages = Router::new()
        .route("/", get(crate::handlers::home))
        .route("/login", get(crate::handlers::login_form).post(crate::handlers::login))
        .route("/logout", axum::routing::post(crate::handlers::logout))
        .route_layer(from_fn(crate::trace::record_route_middleware))
        .with_state(state);

    let router = {
        #[cfg(all(debug_assertions, feature = "live-reload"))]
        {
            base.merge(pages.layer(tower_livereload::LiveReloadLayer::new()))
        }

        #[cfg(not(all(debug_assertions, feature = "live-reload")))]
        {
            base.merge(pages)
        }
    };

    router
}
