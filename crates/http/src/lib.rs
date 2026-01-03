mod error;
mod handlers;

use axum::{Router, routing::get};
pub use error::{Error, Result};

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
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(crate::handlers::health))
        .with_state(state)
}
