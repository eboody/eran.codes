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
    Router::new()
        .route("/", get(crate::handlers::home))
        .route("/partials/ping", get(crate::handlers::ping_partial))
        .route("/health", get(crate::handlers::health))
        .nest_service("/static", ServeDir::new("crates/http/static"))
        .layer(tower_livereload::LiveReloadLayer::new())
        .with_state(state)
}
