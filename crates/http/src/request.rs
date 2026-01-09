use axum::{
    body::Body,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Page,
    Datastar,
}

tokio::task_local! {
    static REQUEST_KIND: Kind;
}

pub fn current_kind() -> Kind {
    REQUEST_KIND.try_with(|kind| *kind).unwrap_or(Kind::Page)
}

pub async fn set_kind_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let kind = kind_from_headers(req.headers());
    REQUEST_KIND.scope(kind, async move { next.run(req).await }).await
}

fn kind_from_headers(headers: &HeaderMap) -> Kind {
    if headers.contains_key("datastar-request") {
        Kind::Datastar
    } else {
        Kind::Page
    }
}
