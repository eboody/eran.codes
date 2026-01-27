use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use tower_http::services::ServeDir;

pub struct Routes {
    router: Router,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn with_base_routes(mut self) -> Self {
        self.router = self.router.merge(base_routes());
        self
    }

    pub fn with_page_routes(mut self) -> Self {
        let pages = maybe_live_reload(pages_routes());
        self.router = self.router.merge(pages);
        self
    }

    pub fn with_route_tracing(mut self) -> Self {
        self.router = self
            .router
            .route_layer(from_fn(crate::trace::record_route_middleware));
        self
    }

    pub fn finish(self) -> Router {
        self.router
    }
}

fn base_routes() -> Router {
    use crate::paths::Route;
    Router::new()
        .route(Route::PartialPing.as_str(), get(crate::handlers::ping_partial))
        .route(
            Route::PartialAuthStatus.as_str(),
            get(crate::handlers::auth_status_partial),
        )
        .route(
            Route::PartialSessionStatus.as_str(),
            get(crate::handlers::session_status_partial),
        )
        .route(
            Route::PartialRequestMeta.as_str(),
            get(crate::handlers::request_meta_partial),
        )
        .route(
            Route::PartialBoundaryCheck.as_str(),
            get(crate::handlers::boundary_check_partial),
        )
        .route(Route::PartialDbCheck.as_str(), get(crate::handlers::db_check_partial))
        .route(
            Route::PartialSurrealGuarded.as_str(),
            get(crate::handlers::surreal_message_guarded),
        )
        .route(
            Route::PartialSurrealCancel.as_str(),
            get(crate::handlers::surreal_message_cancel),
        )
        .route(Route::ErrorTest.as_str(), get(crate::handlers::error_test))
        .route(Route::Events.as_str(), get(crate::handlers::events))
        .route(Route::Health.as_str(), get(crate::handlers::health))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
}

fn pages_routes() -> Router {
    use crate::paths::Route;
    let protected = Router::new()
        .route(Route::Protected.as_str(), get(crate::handlers::protected))
        .route_layer(from_fn(crate::auth::require_auth_middleware));

    let chat = Router::new()
        .route(Route::Chat.as_str(), get(crate::handlers::chat_page))
        .route(Route::ChatMessages.as_str(), post(crate::handlers::post_chat_message))
        .route(
            Route::ChatMessagesDemo.as_str(),
            post(crate::handlers::post_demo_chat_message),
        )
        .route(
            Route::ChatModeration.as_str(),
            get(crate::handlers::moderation_page)
                .post(crate::handlers::moderate_message),
        )
        .route_layer(from_fn(crate::auth::require_auth_middleware));

    Router::new()
        .route(Route::Home.as_str(), get(crate::handlers::home))
        .route(
            Route::Login.as_str(),
            get(crate::handlers::login_form).post(crate::handlers::login),
        )
        .route(
            Route::Register.as_str(),
            get(crate::handlers::register_form).post(crate::handlers::register),
        )
        .route(Route::Logout.as_str(), axum::routing::post(crate::handlers::logout))
        .merge(protected)
        .merge(chat)
}

fn maybe_live_reload(pages: Router) -> Router {
    #[cfg(all(debug_assertions, feature = "live-reload"))]
    {
        pages.layer(tower_livereload::LiveReloadLayer::new())
    }

    #[cfg(not(all(debug_assertions, feature = "live-reload")))]
    {
        pages
    }
}
