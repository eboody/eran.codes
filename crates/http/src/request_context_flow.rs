use axum::http;
use statum::{machine, state, transition};

use crate::{
    request,
    types::{ClientIp, RequestId, SessionId, SseTabId, UserAgent},
};

#[derive(Clone, Debug)]
pub struct BuiltData {
    context: crate::request::Context,
}

#[state]
pub enum RequestContextState {
    Incoming,
    HeadersResolved,
    Built(BuiltData),
}

#[machine]
pub(crate) struct RequestContextFlow<RequestContextState> {
    headers: http::HeaderMap,
    session_id: Option<SessionId>,
    sse_tab_id: Option<SseTabId>,
    request_id: Option<RequestId>,
    client_ip: Option<ClientIp>,
    user_agent: Option<UserAgent>,
    kind: request::Kind,
}

impl RequestContextFlow<Incoming> {
    pub(crate) fn new(
        headers: http::HeaderMap,
        session_id: Option<SessionId>,
        sse_tab_id: Option<SseTabId>,
    ) -> Self {
        RequestContextFlow::<Incoming>::builder()
            .headers(headers)
            .session_id(session_id)
            .sse_tab_id(sse_tab_id)
            .request_id(None)
            .client_ip(None)
            .user_agent(None)
            .kind(crate::request::Kind::Page)
            .build()
    }
}

#[transition]
impl RequestContextFlow<Incoming> {
    pub(crate) fn resolve_headers(mut self) -> RequestContextFlow<HeadersResolved> {
        self.request_id = crate::request::id_from_headers(&self.headers);
        self.client_ip = crate::request::client_ip_from_headers(&self.headers);
        self.user_agent = crate::request::user_agent_from_headers(&self.headers);
        self.kind = crate::request::kind_from_headers(&self.headers);
        self.transition()
    }
}

#[transition]
impl RequestContextFlow<HeadersResolved> {
    pub(crate) fn build_context(self) -> RequestContextFlow<Built> {
        let request_id = self.request_id.clone();
        let session_id = self.session_id.clone();
        let sse_tab_id = self.sse_tab_id.clone();
        let client_ip = self.client_ip.clone();
        let user_agent = self.user_agent.clone();
        let kind = self.kind;
        let context = crate::request::Context {
            request_id,
            session_id,
            sse_tab_id,
            user_id: None,
            client_ip,
            user_agent,
            kind,
        };
        self.transition_with(BuiltData { context })
    }
}

impl RequestContextFlow<Built> {
    pub(crate) fn into_context(self) -> crate::request::Context {
        self.state_data.context
    }
}

pub(crate) type IncomingFlow = RequestContextFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http;

    #[test]
    fn resolve_headers_detects_datastar_kind() {
        let mut headers = http::HeaderMap::new();
        headers.insert("datastar-request", http::HeaderValue::from_static("1"));

        let built = IncomingFlow::new(headers, None, None)
            .resolve_headers()
            .build_context()
            .into_context();

        assert!(matches!(built.kind, crate::request::Kind::Datastar));
    }

    #[test]
    fn build_context_preserves_prefilled_sse_tab_id() {
        let built = IncomingFlow::new(
            http::HeaderMap::new(),
            None,
            Some(crate::types::SseTabId::new("tab-prefilled")),
        )
        .resolve_headers()
        .build_context()
        .into_context();

        assert_eq!(
            built.sse_tab_id.map(|value| value.to_string()).as_deref(),
            Some("tab-prefilled")
        );
    }
}
