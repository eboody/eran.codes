use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

use crate::State;

pub fn router(state: State) -> Router {
    Router::new()
        .route("/", get(crate::routes::home::home))
        .route("/health", get(crate::routes::home::health))
        .route("/users", post(crate::routes::users::create))
        .nest_service("/static", ServeDir::new("crates/http/static"))
        .with_state(state)
}
