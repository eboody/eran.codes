use axum::Extension;
use axum::Router;
use axum::middleware::from_fn;
use axum_login::AuthManagerLayerBuilder;
use bon::Builder;
use time::Duration as SessionDuration;
use tower_cookies::CookieManagerLayer;
use tower_cookies::cookie::SameSite;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer, SessionStore};
use tracing::field;

use crate::State;

pub fn apply_request_layers<Store>(
    state: State,
    session_store: Store,
    router: Router,
) -> Router
where
    Store: SessionStore + Clone + Send + Sync + 'static,
{
    RequestLayers::builder()
        .with_router(router)
        .with_state(state)
        .with_session_store(session_store)
        .build()
        .with_trace_layer()
        .with_audit_layer()
        .with_user_context_layer()
        .with_request_context_layer()
        .with_request_id_propagation()
        .with_cookie_manager()
        .with_request_id_assignment()
        .with_auth_layer()
        .with_state_extension()
        .finish()
}

#[derive(Builder)]
pub struct RequestLayers<Store> {
    #[builder(setters(name = with_router))]
    router: Router,
    #[builder(setters(name = with_state))]
    state: State,
    #[builder(setters(name = with_session_store))]
    session_store: Store,
}

impl<Store> RequestLayers<Store>
where
    Store: SessionStore + Clone + Send + Sync + 'static,
{
    fn with_state_extension(mut self) -> Self {
        self.router = self.router.layer(Extension(self.state.clone()));
        self
    }

    fn with_trace_layer(mut self) -> Self {
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
                            span.record("request_id", &request_id.to_string().as_str());
                        }
                        if let Some(session_id) = context.session_id.as_ref() {
                            span.record("session_id", &session_id.to_string().as_str());
                        }
                        if let Some(user_id) = context.user_id.as_ref() {
                            span.record("user_id", &user_id.to_string().as_str());
                        }
                        if let Some(client_ip) = context.client_ip.as_ref() {
                            span.record("client_ip", &client_ip.to_string().as_str());
                        }
                        if let Some(user_agent) = context.user_agent.as_ref() {
                            span.record("user_agent", &user_agent.to_string().as_str());
                        }
                        span.record("kind", context.kind.as_str());
                    }

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
        self
    }

    fn with_audit_layer(mut self) -> Self {
        self.router = self
            .router
            .layer(from_fn(crate::trace_log::audit_middleware));
        self
    }

    fn with_user_context_layer(mut self) -> Self {
        self.router = self
            .router
            .layer(from_fn(crate::auth::set_user_context_middleware));
        self
    }

    fn with_request_context_layer(mut self) -> Self {
        self.router = self
            .router
            .layer(from_fn(crate::request::set_context_middleware));
        self
    }

    fn with_request_id_propagation(mut self) -> Self {
        self.router = self.router.layer(PropagateRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
        ));
        self
    }

    fn with_cookie_manager(mut self) -> Self {
        self.router = self.router.layer(CookieManagerLayer::new());
        self
    }

    fn with_request_id_assignment(mut self) -> Self {
        self.router = self.router.layer(SetRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ));
        self
    }

    fn with_auth_layer(mut self) -> Self {
        let session_key = self.state.cookie_key.clone();
        let session_layer = SessionManagerLayer::new(self.session_store.clone())
            .with_name("eran.sid")
            .with_secure(!cfg!(debug_assertions))
            .with_same_site(SameSite::Lax)
            .with_expiry(Expiry::OnInactivity(SessionDuration::days(7)))
            .with_private(session_key);

        let auth_layer = AuthManagerLayerBuilder::new(
            crate::auth::Backend::new(self.state.auth.clone()),
            session_layer,
        )
        .build();

        self.router = self.router.layer(auth_layer);
        self
    }

    fn finish(self) -> Router {
        self.router
    }
}
