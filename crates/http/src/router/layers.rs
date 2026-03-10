use std::sync::Arc;

use axum::Extension;
use axum::Router;
use axum::extract::MatchedPath;
use axum::middleware::from_fn;
use axum_login::AuthManagerLayerBuilder;
use statum::{machine, state, transition};
use time::Duration as SessionDuration;
use tower_cookies::CookieManagerLayer;
use tower_cookies::cookie::SameSite;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer, SessionStore};
use tracing::field;

use crate::State;

type AuthLayerApplier = Arc<dyn Fn(Router, &State) -> Router + Send + Sync>;

pub fn apply_request_layers<Store>(
    state: State,
    session_store: Store,
    router: Router,
) -> Router
where
    Store: SessionStore + Clone + Send + Sync + 'static,
{
    let auth_layer_applier: AuthLayerApplier =
        Arc::new(move |router: Router, state: &State| {
            let session_key = state.cookie_key.clone();
            let session_layer = SessionManagerLayer::new(session_store.clone())
                .with_name("eran.sid")
                .with_secure(!cfg!(debug_assertions))
                .with_same_site(SameSite::Lax)
                .with_expiry(Expiry::OnInactivity(SessionDuration::days(7)))
                .with_private(session_key);

            let auth_layer = AuthManagerLayerBuilder::new(
                crate::auth::Backend::new(state.auth.clone()),
                session_layer,
            )
            .build();

            router.layer(auth_layer)
        });

    RequestLayerPipeline::<CoreReady>::builder()
        .router(router)
        .state(state)
        .auth_layer_applier(auth_layer_applier)
        .build()
        .add_trace()
        .add_audit()
        .add_user_context()
        .add_request_context()
        .add_request_id_propagation()
        .add_cookie_manager()
        .add_request_id_assignment()
        .add_auth()
        .add_state_extension()
        .finish()
}

#[state]
pub enum RequestLayerFlow {
    CoreReady,
    TraceAdded,
    AuditAdded,
    UserContextAdded,
    RequestContextAdded,
    RequestIdPropagationAdded,
    CookieManagerAdded,
    RequestIdAssignmentAdded,
    AuthAdded,
    StateExtensionAdded,
}

#[machine]
pub struct RequestLayerPipeline<RequestLayerFlow> {
    router: Router,
    state: State,
    auth_layer_applier: AuthLayerApplier,
}

#[transition]
impl RequestLayerPipeline<CoreReady> {
    pub fn add_trace(mut self) -> RequestLayerPipeline<TraceAdded> {
        self.router = self.router.layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<axum::body::Body>| {
                    let span = tracing::info_span!(
                        "http.request",
                        method = %request.method(),
                        uri = %request.uri(),
                        path = %request.uri().path(),
                        route = field::Empty,
                        request_id = field::Empty,
                        session_id = field::Empty,
                        user_id = field::Empty,
                        client_ip = field::Empty,
                        user_agent = field::Empty,
                        kind = field::Empty,
                        status = field::Empty,
                        latency_ms = field::Empty,
                    );

                    if let Some(context) =
                        request.extensions().get::<crate::request::Context>()
                    {
                        if let Some(request_id) = context.request_id.as_ref() {
                            span.record("request_id", request_id.to_string().as_str());
                        }
                        if let Some(session_id) = context.session_id.as_ref() {
                            span.record("session_id", session_id.to_string().as_str());
                        }
                        if let Some(user_id) = context.user_id.as_ref() {
                            span.record("user_id", user_id.to_string().as_str());
                        }
                        if let Some(client_ip) = context.client_ip.as_ref() {
                            span.record("client_ip", client_ip.to_string().as_str());
                        }
                        if let Some(user_agent) = context.user_agent.as_ref() {
                            span.record("user_agent", user_agent.to_string().as_str());
                        }
                        span.record("kind", context.kind.as_str());
                    }
                    let route = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|value| value.as_str())
                        .unwrap_or_else(|| request.uri().path());
                    span.record("route", route);

                    span
                })
                .on_request(
                    |_request: &axum::http::Request<axum::body::Body>,
                     span: &tracing::Span| {
                        tracing::debug!(parent: span, "request started");
                    },
                )
                .on_response(
                    |response: &axum::http::Response<axum::body::Body>,
                     latency: std::time::Duration,
                     span: &tracing::Span| {
                        span.record("status", response.status().as_u16());
                        span.record("latency_ms", latency.as_millis() as u64);
                        tracing::info!(parent: span, "request completed");
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass,
                     latency: std::time::Duration,
                     span: &tracing::Span| {
                        span.record("latency_ms", latency.as_millis() as u64);
                        tracing::error!(parent: span, error = %error, "request failed");
                    },
                ),
        );
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<TraceAdded> {
    pub fn add_audit(mut self) -> RequestLayerPipeline<AuditAdded> {
        self.router = self
            .router
            .layer(from_fn(crate::trace_log::audit_middleware));
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<AuditAdded> {
    pub fn add_user_context(mut self) -> RequestLayerPipeline<UserContextAdded> {
        self.router = self
            .router
            .layer(from_fn(crate::auth::set_user_context_middleware));
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<UserContextAdded> {
    pub fn add_request_context(mut self) -> RequestLayerPipeline<RequestContextAdded> {
        self.router = self
            .router
            .layer(from_fn(crate::request::set_context_middleware));
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<RequestContextAdded> {
    pub fn add_request_id_propagation(
        mut self,
    ) -> RequestLayerPipeline<RequestIdPropagationAdded> {
        self.router = self.router.layer(PropagateRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
        ));
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<RequestIdPropagationAdded> {
    pub fn add_cookie_manager(mut self) -> RequestLayerPipeline<CookieManagerAdded> {
        self.router = self.router.layer(CookieManagerLayer::new());
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<CookieManagerAdded> {
    pub fn add_request_id_assignment(
        mut self,
    ) -> RequestLayerPipeline<RequestIdAssignmentAdded> {
        self.router = self.router.layer(SetRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ));
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<RequestIdAssignmentAdded> {
    pub fn add_auth(mut self) -> RequestLayerPipeline<AuthAdded> {
        self.router = (self.auth_layer_applier)(self.router, &self.state);
        self.transition()
    }
}

#[transition]
impl RequestLayerPipeline<AuthAdded> {
    pub fn add_state_extension(mut self) -> RequestLayerPipeline<StateExtensionAdded> {
        self.router = self.router.layer(Extension(self.state.clone()));
        self.transition()
    }
}

impl RequestLayerPipeline<StateExtensionAdded> {
    pub fn finish(self) -> Router {
        self.router
    }
}
