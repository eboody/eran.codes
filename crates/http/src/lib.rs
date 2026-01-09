mod error;
mod handlers;
pub mod request;
pub mod sse;
mod views;

use axum::{Router, routing::get};
pub use error::{Error, Result};
pub use sse::Registry as SseRegistry;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

#[derive(Clone)]
pub struct State {
    pub user: app::user::Service,
    pub sse: sse::Registry,
}

impl State {
    pub fn new(
        user: app::user::Service,
        sse: sse::Registry,
    ) -> Self {
        Self { user, sse }
    }
}

pub fn router(state: State) -> Router {
    let base = Router::new()
        .route("/partials/ping", get(crate::handlers::ping_partial))
        .route("/events", get(crate::handlers::events))
        .route("/health", get(crate::handlers::health))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .layer(CookieManagerLayer::new())
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
