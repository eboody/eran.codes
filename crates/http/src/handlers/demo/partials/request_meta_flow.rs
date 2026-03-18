use axum::http;
use maud::Render;
use statum::{machine, state, transition};

use crate::types::Text;

#[state]
pub enum RequestMetaPartialState {
    Incoming,
    SnapshotPrepared,
}

#[machine]
pub(super) struct RequestMetaPartialFlow<RequestMetaPartialState> {
    request_id: Option<Text>,
    session_id: Option<Text>,
    user_id: Option<Text>,
    client_ip: Option<Text>,
    user_agent: Option<Text>,
    trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl RequestMetaPartialFlow<Incoming> {
    pub(super) fn new() -> Self {
        RequestMetaPartialFlow::<Incoming>::builder()
            .request_id(None)
            .session_id(None)
            .user_id(None)
            .client_ip(None)
            .user_agent(None)
            .trace(Vec::new())
            .build()
    }
}

#[transition]
impl RequestMetaPartialFlow<Incoming> {
    pub(super) fn prepare_snapshot(
        mut self,
        state: &crate::State,
    ) -> RequestMetaPartialFlow<SnapshotPrepared> {
        let context = crate::request::current_context();
        self.request_id = context
            .as_ref()
            .and_then(|value| value.request_id.clone())
            .map(|value| Text::from(value.to_string()));
        self.session_id = context
            .as_ref()
            .and_then(|value| value.session_id.clone())
            .map(|value| Text::from(value.to_string()));
        self.user_id = context
            .as_ref()
            .and_then(|value| value.user_id.clone())
            .map(|value| Text::from(value.to_string()));
        self.client_ip = context
            .as_ref()
            .and_then(|value| value.client_ip.clone())
            .map(|value| Text::from(value.to_string()));
        self.user_agent = context
            .as_ref()
            .and_then(|value| value.user_agent.clone())
            .map(|value| Text::from(value.to_string()));
        self.trace = super::trace_snapshot(state);
        self.transition()
    }
}

impl RequestMetaPartialFlow<SnapshotPrepared> {
    pub(super) fn into_response(self) -> (http::StatusCode, axum::response::Html<String>) {
        let partial = crate::views::partials::RequestMeta::builder()
            .maybe_request_id(self.request_id)
            .maybe_session_id(self.session_id)
            .maybe_user_id(self.user_id)
            .maybe_client_ip(self.client_ip)
            .maybe_user_agent(self.user_agent)
            .trace(self.trace)
            .build();
        (
            http::StatusCode::OK,
            axum::response::Html(partial.render().into_string()),
        )
    }
}

pub(super) type IncomingFlow = RequestMetaPartialFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepared_flow_renders_request_meta_target() {
        let prepared = RequestMetaPartialFlow::<SnapshotPrepared>::builder()
            .request_id(Some(Text::from("request-1")))
            .session_id(Some(Text::from("session-1")))
            .user_id(Some(Text::from("user-1")))
            .client_ip(Some(Text::from("127.0.0.1")))
            .user_agent(Some(Text::from("test-agent")))
            .trace(Vec::new())
            .build();

        let response = prepared.into_response();
        assert_eq!(response.0, http::StatusCode::OK);
        assert!(response.1.0.contains("request-meta-target"));
    }
}
