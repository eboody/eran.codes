use axum::{
    body::Body, extract::MatchedPath, http::Request, middleware::Next, response::Response,
};
use tracing::Span;

pub(crate) async fn record_route_middleware(req: Request<Body>, next: Next) -> Response {
    if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        Span::current().record("route", matched_path.as_str());
    }
    next.run(req).await
}
