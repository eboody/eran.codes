use axum::{body::Body, extract::Extension, http, middleware::Next, response::Response};

use crate::sse::{self, SESSION_COOKIE};
use crate::types::{ClientIp, RequestId, SessionId, SseTabId, UserAgent, UserIdText};
use std::cell::RefCell;
use tower_cookies::{Cookies, Key};
use tracing::Span;

#[derive(Clone, Copy, Debug, strum_macros::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Kind {
    Page,
    Datastar,
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
    req: http::Request<Body>,
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
    pub request_id: Option<RequestId>,
    pub session_id: Option<SessionId>,
    pub sse_tab_id: Option<SseTabId>,
    pub user_id: Option<UserIdText>,
    pub client_ip: Option<ClientIp>,
    pub user_agent: Option<UserAgent>,
    pub kind: Kind,
}

pub fn set_user_id(user_id: impl Into<UserIdText>) {
    let user_id = user_id.into();
    if let Ok(()) = REQUEST_CONTEXT.try_with(|context| {
        context.borrow_mut().user_id = Some(user_id.clone());
    }) {
        Span::current().record("user_id", user_id.to_string().as_str());
    }
}

pub fn set_sse_tab_id(sse_tab_id: impl Into<SseTabId>) {
    let sse_tab_id = sse_tab_id.into();
    if let Ok(()) = REQUEST_CONTEXT.try_with(|context| {
        context.borrow_mut().sse_tab_id = Some(sse_tab_id.clone());
    }) {
        Span::current().record("sse_tab_id", sse_tab_id.to_string().as_str());
    }
}

fn context_from_request(req: &http::Request<Body>, key: &Key) -> Context {
    let cookies = req.extensions().get::<Cookies>();
    let session_id = cookies.map(|cookies| ensure_session_id(cookies, key));
    let sse_tab_id = sse_tab_id_from_uri(req.uri());

    crate::request_context_flow::IncomingFlow::new(
        req.headers().clone(),
        session_id,
        sse_tab_id,
    )
    .resolve_headers()
    .build_context()
    .into_context()
}

fn ensure_session_id(cookies: &Cookies, key: &Key) -> SessionId {
    if let Some(session_id) = session_id_from_cookies(cookies, key) {
        return session_id;
    }
    sse::Handle::from_cookies(cookies, key).id()
}

pub(crate) fn kind_from_headers(headers: &http::HeaderMap) -> Kind {
    if headers.contains_key("datastar-request") {
        Kind::Datastar
    } else {
        Kind::Page
    }
}

fn session_id_from_cookies(cookies: &Cookies, key: &Key) -> Option<SessionId> {
    cookies
        .signed(key)
        .get(SESSION_COOKIE)
        .map(|cookie| SessionId::new(cookie.value()))
}

pub(crate) fn id_from_headers(headers: &http::HeaderMap) -> Option<RequestId> {
    header_value(
        headers,
        http::header::HeaderName::from_static("x-request-id"),
    )
    .map(RequestId::new)
}

pub(crate) fn user_agent_from_headers(headers: &http::HeaderMap) -> Option<UserAgent> {
    header_value(headers, http::header::USER_AGENT).map(UserAgent::new)
}

pub(crate) fn client_ip_from_headers(headers: &http::HeaderMap) -> Option<ClientIp> {
    let forwarded = header_value(
        headers,
        http::header::HeaderName::from_static("x-forwarded-for"),
    )
    .and_then(|value| value.split(',').next().map(str::trim))
    .map(ClientIp::new);

    forwarded.or_else(|| {
        header_value(headers, http::header::HeaderName::from_static("x-real-ip"))
            .map(ClientIp::new)
    })
}

fn sse_tab_id_from_uri(uri: &http::Uri) -> Option<SseTabId> {
    uri.query().and_then(|query| {
        query.split('&').find_map(|segment| {
            let (name, value) = segment.split_once('=')?;
            if !matches!(name, "sseTabId" | "sse_tab_id") {
                return None;
            }
            let decoded = urlencoding::decode(value).ok()?;
            let value = decoded.trim();
            if value.is_empty() {
                return None;
            }
            Some(SseTabId::new(value.to_string()))
        })
    })
}

fn header_value(headers: &http::HeaderMap, name: http::header::HeaderName) -> Option<&str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http;
    use tower_cookies::{Cookie, Cookies, Key};

    #[test]
    fn prefers_forwarded_ip_over_real_ip() {
        let mut headers = http::HeaderMap::new();
        headers.insert(
            http::header::HeaderName::from_static("x-forwarded-for"),
            http::HeaderValue::from_static("203.0.113.5, 10.0.0.1"),
        );
        headers.insert(
            http::header::HeaderName::from_static("x-real-ip"),
            http::HeaderValue::from_static("198.51.100.7"),
        );

        let client_ip = client_ip_from_headers(&headers);

        assert_eq!(
            client_ip.map(|value| value.to_string()).as_deref(),
            Some("203.0.113.5")
        );
    }

    #[test]
    fn detects_datastar_request() {
        let mut headers = http::HeaderMap::new();
        headers.insert("datastar-request", http::HeaderValue::from_static("1"));

        let kind = kind_from_headers(&headers);

        assert!(matches!(kind, Kind::Datastar));
    }

    #[test]
    fn reads_signed_session_cookie() {
        let key = Key::generate();
        let cookies = Cookies::default();
        cookies
            .signed(&key)
            .add(Cookie::new(SESSION_COOKIE, "signed123"));

        let session_id = session_id_from_cookies(&cookies, &key);

        assert_eq!(
            session_id.map(|value| value.to_string()).as_deref(),
            Some("signed123")
        );
    }

    #[test]
    fn context_prefers_signed_session_cookie() {
        let key = Key::generate();
        let cookies = Cookies::default();
        cookies
            .signed(&key)
            .add(Cookie::new(SESSION_COOKIE, "signed123"));

        let mut req = http::Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();
        req.extensions_mut().insert(cookies);

        let context = context_from_request(&req, &key);

        assert_eq!(
            context.session_id.map(|value| value.to_string()).as_deref(),
            Some("signed123")
        );
    }

    #[test]
    fn context_creates_session_cookie_when_missing() {
        let key = Key::generate();
        let cookies = Cookies::default();
        let mut req = http::Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();
        req.extensions_mut().insert(cookies.clone());

        let context = context_from_request(&req, &key);
        let from_cookie = session_id_from_cookies(&cookies, &key);

        assert!(context.session_id.is_some());
        assert_eq!(
            context.session_id.map(|value| value.to_string()),
            from_cookie.map(|value| value.to_string())
        );
    }

    #[test]
    fn ignores_unsigned_session_cookie() {
        let key = Key::generate();
        let req = http::Request::builder()
            .uri("/")
            .header(
                http::header::COOKIE,
                http::HeaderValue::from_static("session_id=unsigned"),
            )
            .body(Body::empty())
            .unwrap();

        let context = context_from_request(&req, &key);

        assert_eq!(context.session_id.map(|value| value.to_string()), None);
    }

    #[test]
    fn context_reads_sse_tab_id_from_query() {
        let key = Key::generate();
        let req = http::Request::builder()
            .uri("/partials/sensitive-proof?sseTabId=tab-from-query")
            .body(Body::empty())
            .unwrap();

        let context = context_from_request(&req, &key);

        assert_eq!(
            context.sse_tab_id.map(|value| value.to_string()).as_deref(),
            Some("tab-from-query")
        );
    }

    #[test]
    fn context_ignores_empty_sse_tab_id_query() {
        let key = Key::generate();
        let req = http::Request::builder()
            .uri("/partials/sensitive-proof?sseTabId=%20%20")
            .body(Body::empty())
            .unwrap();

        let context = context_from_request(&req, &key);

        assert_eq!(context.sse_tab_id.map(|value| value.to_string()), None);
    }

    #[tokio::test]
    async fn updates_user_id_in_context() {
        let context = Context {
            request_id: None,
            session_id: None,
            sse_tab_id: None,
            user_id: None,
            client_ip: None,
            user_agent: None,
            kind: Kind::Page,
        };

        REQUEST_CONTEXT
            .scope(RefCell::new(context), async move {
                set_user_id("user-123");
                let updated = current_context().expect("context");
                assert_eq!(
                    updated.user_id.map(|value| value.to_string()),
                    Some("user-123".to_string())
                );
            })
            .await;
    }
}
