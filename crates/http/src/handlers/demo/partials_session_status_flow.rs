use axum::http::StatusCode;
use maud::Render;
use statum::{machine, state, transition};
use tower_sessions::Session;

use crate::types::Text;

#[state]
pub enum SessionStatusPartialState {
    Incoming,
    SnapshotPrepared,
}

#[machine]
pub(super) struct SessionStatusPartialFlow<SessionStatusPartialState> {
    session_id: Option<Text>,
    expiry: Option<Text>,
    trace: Vec<crate::trace_log::TraceEntry>,
}

impl SessionStatusPartialFlow<Incoming> {
    pub(super) fn new() -> Self {
        SessionStatusPartialFlow::<Incoming>::builder()
            .maybe_session_id(None)
            .maybe_expiry(None)
            .trace(Vec::new())
            .build()
    }
}

#[transition]
impl SessionStatusPartialFlow<Incoming> {
    pub(super) fn prepare_snapshot(
        mut self,
        session: &Session,
        state: &crate::State,
    ) -> SessionStatusPartialFlow<SnapshotPrepared> {
        self.session_id = session.id().map(|value| Text::from(value.to_string()));
        self.expiry = session
            .expiry()
            .map(|value| Text::from(format!("{value:?}")));
        self.trace = super::partials::trace_snapshot(state);
        self.transition()
    }
}

impl SessionStatusPartialFlow<SnapshotPrepared> {
    pub(super) fn into_response(self) -> (StatusCode, axum::response::Html<String>) {
        let partial = crate::views::partials::SessionStatus::builder()
            .maybe_session_id(self.session_id)
            .maybe_expiry(self.expiry)
            .trace(self.trace)
            .build();
        (
            StatusCode::OK,
            axum::response::Html(partial.render().into_string()),
        )
    }
}

pub(super) type IncomingFlow = SessionStatusPartialFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepared_flow_renders_session_status_target() {
        let prepared = SessionStatusPartialFlow::<SnapshotPrepared>::builder()
            .session_id(Text::from("session-1"))
            .expiry(Text::from("in one hour"))
            .trace(Vec::new())
            .build();

        let response = prepared.into_response();
        assert_eq!(response.0, StatusCode::OK);
        assert!(response.1.0.contains("session-status-target"));
    }
}
