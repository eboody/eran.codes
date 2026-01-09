mod error;
mod handlers;
pub mod request;
pub mod sse;
mod views;

use axum::{Router, routing::get};
use axum::middleware::from_fn;
use tower::ServiceBuilder;
pub use error::{Error, Result};
pub use sse::Registry as SseRegistry;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::TraceLayer,
};

#[derive(Clone)]
pub struct State {
    pub user: app::user::Service,
    pub sse: sse::Registry,
}

impl State {
    pub fn new(user: app::user::Service, sse: sse::Registry) -> Self {
        Self { user, sse }
    }
}

pub fn router(state: State) -> Router {
    let request_layers = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
        ))
        .layer(TraceLayer::new_for_http());

    let base = Router::new()
        .route("/partials/ping", get(crate::handlers::ping_partial))
        .route("/error-test", get(crate::handlers::error_test))
        .route("/events", get(crate::handlers::events))
        .route("/health", get(crate::handlers::health))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .layer(CookieManagerLayer::new())
        .layer(from_fn(crate::request::set_kind_middleware))
        .layer(request_layers)
        .with_state(state.clone());

    let pages = Router::new()
        .route("/", get(crate::handlers::home))
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
