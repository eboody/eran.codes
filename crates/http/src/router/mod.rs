mod layers;
mod routes;

use axum::Router;
use tower_sessions::SessionStore;

use crate::State;

pub use routes::Routes;

pub fn router<Store>(state: State, session_store: Store) -> Router
where
    Store: SessionStore + Clone + Send + Sync + 'static,
{
    let router = Routes::new()
        .with_base_routes()
        .with_page_routes()
        .with_route_tracing()
        .finish();
    layers::apply_request_layers(state, session_store, router)
}
