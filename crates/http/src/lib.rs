mod error;
mod extract;
mod respond;
mod router;
mod routes;
mod views;
pub use error::{Error, Result};
use axum::Router;

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
    router::router(state)
}
