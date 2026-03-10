use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use statum::{machine, state, transition};
use tower_http::services::ServeDir;

#[state]
pub enum RoutesFlow {
    Incoming,
    BaseRoutesAdded,
    PageRoutesAdded,
    RouteTracingAdded,
}

#[machine]
pub struct Routes<RoutesFlow> {
    router: Router,
}

impl Routes<Incoming> {
    pub fn new() -> Self {
        Routes::<Incoming>::builder().router(Router::new()).build()
    }
}

#[transition]
impl Routes<Incoming> {
    pub fn with_base_routes(mut self) -> Routes<BaseRoutesAdded> {
        self.router = self.router.merge(base_routes());
        self.transition()
    }
}

#[transition]
impl Routes<BaseRoutesAdded> {
    pub fn with_page_routes(mut self) -> Routes<PageRoutesAdded> {
        let pages = maybe_live_reload(pages_routes());
        self.router = self.router.merge(pages);
        self.transition()
    }
}

#[transition]
impl Routes<PageRoutesAdded> {
    pub fn with_route_tracing(mut self) -> Routes<RouteTracingAdded> {
        self.router = self
            .router
            .route_layer(from_fn(crate::trace::record_route_middleware));
        self.transition()
    }
}

impl Routes<RouteTracingAdded> {
    pub fn finish(self) -> Router {
        self.router
    }
}

fn base_routes() -> Router {
    use crate::paths::Route;
    Router::new()
        .route(
            Route::PartialPing.as_str(),
            get(crate::handlers::ping_partial),
        )
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
        .route(
            Route::PartialDbCheck.as_str(),
            get(crate::handlers::db_check_partial),
        )
        .route(
            Route::PartialRequestBurstProbe.as_str(),
            get(crate::handlers::request_burst_probe),
        )
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
        .route("/api/counter/sync", post(crate::handlers::counter_sync))
        .route(
            "/api/operations/filter",
            post(crate::handlers::operations_filter_update),
        )
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

    let chat_protected = Router::new()
        .route(
            Route::ChatMessages.as_str(),
            post(crate::handlers::post_chat_message),
        )
        .route(
            Route::ChatModeration.as_str(),
            get(crate::handlers::moderation_page).post(crate::handlers::moderate_message),
        )
        .route_layer(from_fn(crate::auth::require_auth_middleware));

    Router::new()
        .route(Route::Home.as_str(), get(crate::handlers::home))
        .route(Route::Lab.as_str(), get(crate::handlers::lab))
        .route(Route::Work.as_str(), get(crate::handlers::work))
        .route(
            Route::WorkChatRealtime.as_str(),
            get(crate::handlers::work_chat_realtime),
        )
        .route(
            Route::WorkCommandSse.as_str(),
            get(crate::handlers::work_command_sse),
        )
        .route(
            Route::WorkOperationalVisibility.as_str(),
            get(crate::handlers::work_operational_visibility),
        )
        .route(
            Route::Login.as_str(),
            get(crate::handlers::login_form).post(crate::handlers::login),
        )
        .route(
            Route::Register.as_str(),
            get(crate::handlers::register_form).post(crate::handlers::register),
        )
        .route(
            Route::Logout.as_str(),
            axum::routing::post(crate::handlers::logout),
        )
        .route(
            Route::ChatMessagesDemo.as_str(),
            post(crate::handlers::post_demo_chat_message),
        )
        .merge(protected)
        .merge(chat_protected)
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
