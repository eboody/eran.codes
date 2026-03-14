use axum::{body::Body, extract::MatchedPath, http, middleware::Next, response};
use tracing::Span;

pub(crate) async fn record_route_middleware(
    req: http::Request<Body>,
    next: Next,
) -> response::Response {
    if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        Span::current().record("route", matched_path.as_str());
    }
    next.run(req).await
}
