use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use statum::{machine, state, transition};
use tower_http::services::ServeDir;

#[state]
pub enum Flow {
    Incoming,
    BaseRoutesAdded,
    PageRoutesAdded,
    RouteTracingAdded,
}

#[machine]
pub struct Routes<Flow> {
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
            get(crate::handlers::demo::partials::ping_partial),
        )
        .route(
            Route::PartialAuthStatus.as_str(),
            get(crate::handlers::demo::partials::auth_status_partial),
        )
        .route(
            Route::PartialSessionStatus.as_str(),
            get(crate::handlers::demo::partials::session_status_partial),
        )
        .route(
            Route::PartialRequestMeta.as_str(),
            get(crate::handlers::demo::partials::request_meta_partial),
        )
        .route(
            Route::PartialBoundaryCheck.as_str(),
            get(crate::handlers::demo::partials::boundary_check_partial),
        )
        .route(
            Route::PartialDbCheck.as_str(),
            get(crate::handlers::demo::partials::db_check_partial),
        )
        .route(
            Route::PartialRequestBurstProbe.as_str(),
            get(crate::handlers::demo::partials::request_burst_probe),
        )
        .route(
            Route::PartialSensitiveProof.as_str(),
            get(crate::handlers::demo::partials::sensitive_proof_partial),
        )
        .route(
            Route::ErrorTest.as_str(),
            get(crate::handlers::pages::error_test),
        )
        .route(Route::Events.as_str(), get(crate::handlers::sse::events))
        .route(Route::Health.as_str(), get(crate::handlers::pages::health))
        .route(
            "/api/counter/sync",
            post(crate::handlers::pages::counter_sync),
        )
        .route(
            "/api/operations/filter",
            post(crate::handlers::pages::operations_filter_update),
        )
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
}

fn pages_routes() -> Router {
    use crate::paths::Route;
    let protected = Router::new()
        .route(
            Route::Protected.as_str(),
            get(crate::handlers::auth::protected),
        )
        .route_layer(from_fn(crate::auth::require_auth_middleware));

    let chat_protected = Router::new()
        .route(
            Route::ChatMessages.as_str(),
            post(crate::handlers::demo::chat::post_chat_message),
        )
        .route(
            Route::ChatModeration.as_str(),
            get(crate::handlers::demo::chat::moderation_page)
                .post(crate::handlers::demo::chat::moderate_message),
        )
        .route_layer(from_fn(crate::auth::require_auth_middleware));

    Router::new()
        .route(Route::Home.as_str(), get(crate::handlers::pages::home))
        .route(Route::Lab.as_str(), get(crate::handlers::pages::lab))
        .route(Route::Work.as_str(), get(crate::handlers::pages::work))
        .route(
            Route::OpenSource.as_str(),
            get(crate::handlers::pages::open_source),
        )
        .route(
            Route::ResumeText.as_str(),
            get(crate::handlers::pages::resume_text),
        )
        .route(
            Route::WorkChatRealtime.as_str(),
            get(crate::handlers::pages::work_chat_realtime),
        )
        .route(
            Route::WorkCommandSse.as_str(),
            get(crate::handlers::pages::work_command_sse),
        )
        .route(
            Route::WorkOperationalVisibility.as_str(),
            get(crate::handlers::pages::work_operational_visibility),
        )
        .route(
            Route::WorkSensitiveSync.as_str(),
            get(crate::handlers::pages::work_sensitive_sync),
        )
        .route(
            Route::Login.as_str(),
            get(crate::handlers::auth::login_form).post(crate::handlers::auth::login),
        )
        .route(
            Route::Register.as_str(),
            get(crate::handlers::auth::register_form).post(crate::handlers::auth::register),
        )
        .route(
            Route::Logout.as_str(),
            axum::routing::post(crate::handlers::auth::logout),
        )
        .route(
            Route::ChatMessagesDemo.as_str(),
            post(crate::handlers::demo::chat::post_demo_chat_message),
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
