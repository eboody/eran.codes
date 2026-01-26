use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use axum_login::login_required;
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
    Router::new()
        .route("/partials/ping", get(crate::handlers::ping_partial))
        .route(
            "/partials/auth-status",
            get(crate::handlers::auth_status_partial),
        )
        .route(
            "/partials/session-status",
            get(crate::handlers::session_status_partial),
        )
        .route(
            "/partials/request-meta",
            get(crate::handlers::request_meta_partial),
        )
        .route(
            "/partials/boundary-check",
            get(crate::handlers::boundary_check_partial),
        )
        .route("/partials/db-check", get(crate::handlers::db_check_partial))
        .route(
            "/partials/surreal-message-guarded",
            get(crate::handlers::surreal_message_guarded),
        )
        .route(
            "/partials/surreal-message-cancel",
            get(crate::handlers::surreal_message_cancel),
        )
        .route("/error-test", get(crate::handlers::error_test))
        .route("/events", get(crate::handlers::events))
        .route("/health", get(crate::handlers::health))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
}

fn pages_routes() -> Router {
    let protected = Router::new()
        .route("/protected", get(crate::handlers::protected))
        .route_layer(login_required!(crate::auth::Backend, login_url = "/login"));

    let chat = Router::new()
        .route("/demo/chat", get(crate::handlers::chat_page))
        .route("/demo/chat/messages", post(crate::handlers::post_chat_message))
        .route(
            "/demo/chat/moderation",
            get(crate::handlers::moderation_page)
                .post(crate::handlers::moderate_message),
        )
        .route_layer(login_required!(crate::auth::Backend, login_url = "/login"));

    Router::new()
        .route("/", get(crate::handlers::home))
        .route(
            "/login",
            get(crate::handlers::login_form).post(crate::handlers::login),
        )
        .route(
            "/register",
            get(crate::handlers::register_form).post(crate::handlers::register),
        )
        .route("/logout", axum::routing::post(crate::handlers::logout))
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
