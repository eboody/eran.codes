use axum::{
    body::Body,
    extract::Extension,
    http::{header, HeaderMap, Request},
    middleware::Next,
    response::Response,
};

use crate::sse::SESSION_COOKIE;
use std::cell::RefCell;
use tracing::Span;
use tower_cookies::{Cookies, Key};

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Page,
    Datastar,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Page => "page",
            Kind::Datastar => "datastar",
        }
    }
}

tokio::task_local! {
    static REQUEST_CONTEXT: RefCell<Context>;
}

pub fn current_kind() -> Kind {
    REQUEST_CONTEXT
        .try_with(|context| context.borrow().kind)
        .unwrap_or(Kind::Page)
}

pub fn current_context() -> Option<Context> {
    REQUEST_CONTEXT
        .try_with(|context| context.borrow().clone())
        .ok()
}

pub async fn set_context_middleware(
    Extension(state): Extension<crate::State>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let context = context_from_request(&req, &state.cookie_key);
    let mut req = req;
    req.extensions_mut().insert(context.clone());
    REQUEST_CONTEXT
        .scope(RefCell::new(context), async move { next.run(req).await })
        .await
}

#[derive(Clone, Debug)]
pub struct Context {
    pub request_id: Option<String>,
    pub session_id: Option<String>,
    pub user_id: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub kind: Kind,
}

pub fn set_user_id(user_id: impl Into<String>) {
    let user_id = user_id.into();
    if let Ok(()) = REQUEST_CONTEXT.try_with(|context| {
        context.borrow_mut().user_id = Some(user_id.clone());
    }) {
        Span::current().record("user_id", &user_id.as_str());
    }
}

fn context_from_request(
    req: &Request<Body>,
    key: &Key,
) -> Context {
    let headers = req.headers();
    let cookies = req.extensions().get::<Cookies>();
    Context {
        request_id: header_value(headers, header::HeaderName::from_static("x-request-id")),
        session_id: cookies.and_then(|cookies| session_id_from_cookies(cookies, key)),
        user_id: None,
        client_ip: client_ip_from_headers(headers),
        user_agent: header_value(headers, header::USER_AGENT),
        kind: kind_from_headers(headers),
    }
}

fn kind_from_headers(headers: &HeaderMap) -> Kind {
    if headers.contains_key("datastar-request") {
        Kind::Datastar
    } else {
        Kind::Page
    }
}

fn session_id_from_cookies(
    cookies: &Cookies,
    key: &Key,
) -> Option<String> {
    cookies
        .signed(key)
        .get(SESSION_COOKIE)
        .map(|cookie| cookie.value().to_string())
}

fn client_ip_from_headers(headers: &HeaderMap) -> Option<String> {
    let forwarded = header_value(
        headers,
        header::HeaderName::from_static("x-forwarded-for"),
    )
    .and_then(|value| value.split(',').next().map(|ip| ip.trim().to_string()));

    forwarded.or_else(|| header_value(headers, header::HeaderName::from_static("x-real-ip")))
}

fn header_value(
    headers: &HeaderMap,
    name: header::HeaderName,
) -> Option<String> {
    headers.get(name).and_then(|value| value.to_str().ok()).map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use tower_cookies::{Cookie, Cookies, Key};

    #[test]
    fn prefers_forwarded_ip_over_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-forwarded-for"),
            HeaderValue::from_static("203.0.113.5, 10.0.0.1"),
        );
        headers.insert(
            header::HeaderName::from_static("x-real-ip"),
            HeaderValue::from_static("198.51.100.7"),
        );

        let client_ip = client_ip_from_headers(&headers);

        assert_eq!(client_ip.as_deref(), Some("203.0.113.5"));
    }

    #[test]
    fn detects_datastar_request() {
        let mut headers = HeaderMap::new();
        headers.insert("datastar-request", HeaderValue::from_static("1"));

        let kind = kind_from_headers(&headers);

        assert!(matches!(kind, Kind::Datastar));
    }

    #[test]
    fn reads_signed_session_cookie() {
        let key = Key::generate();
        let cookies = Cookies::new();
        cookies
            .signed(&key)
            .add(Cookie::new(SESSION_COOKIE, "signed123"));

        let session_id = session_id_from_cookies(&cookies, &key);

        assert_eq!(session_id.as_deref(), Some("signed123"));
    }

    #[test]
    fn context_prefers_signed_session_cookie() {
        let key = Key::generate();
        let cookies = Cookies::new();
        cookies
            .signed(&key)
            .add(Cookie::new(SESSION_COOKIE, "signed123"));

        let mut req = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();
        req.extensions_mut().insert(cookies);

        let context = context_from_request(&req, &key);

        assert_eq!(context.session_id.as_deref(), Some("signed123"));
    }

    #[test]
    fn ignores_unsigned_session_cookie() {
        let key = Key::generate();
        let mut req = Request::builder()
            .uri("/")
            .header(
                header::COOKIE,
                HeaderValue::from_static("session_id=unsigned"),
            )
            .body(Body::empty())
            .unwrap();

        let context = context_from_request(&req, &key);

        assert_eq!(context.session_id.as_deref(), None);
    }

    #[tokio::test]
    async fn updates_user_id_in_context() {
        let context = Context {
            request_id: None,
            session_id: None,
            user_id: None,
            client_ip: None,
            user_agent: None,
            kind: Kind::Page,
        };

        REQUEST_CONTEXT
            .scope(RefCell::new(context), async move {
                set_user_id("user-123");
                let updated = current_context().expect("context");
                assert_eq!(updated.user_id.as_deref(), Some("user-123"));
            })
            .await;
    }
}
