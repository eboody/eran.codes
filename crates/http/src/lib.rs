mod error;
mod handlers;
mod views;

use axum::{Router, routing::get};
pub use error::{Error, Result};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct State {
    pub user: app::user::Service,
}

impl State {
    pub fn new(user: app::user::Service) -> Self {
        Self { user }
    }
}

pub fn router(state: State) -> Router {
    let base = Router::new()
        .route("/partials/ping", get(crate::handlers::ping_partial))
        .route("/health", get(crate::handlers::health))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
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
