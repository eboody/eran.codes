use axum::http::StatusCode;
use statum::{machine, state, transition};
use tower_cookies::{Cookies, Key};

use crate::types::{SessionId, SseTabId, Text};

#[derive(Clone, Debug)]
pub struct SessionData {
    session_id: SessionId,
}

#[state]
pub enum OperationsFilterState {
    Incoming,
    SessionBound(SessionData),
    FilterApplied(SessionData),
    Refreshed,
}

#[machine]
pub(super) struct OperationsFilterFlow<OperationsFilterState> {
    sse_tab_id: Option<SseTabId>,
    filter_query: Option<Text>,
}

impl OperationsFilterFlow<Incoming> {
    pub(super) fn new(filter_query: Option<Text>, sse_tab_id: Option<SseTabId>) -> Self {
        OperationsFilterFlow::<Incoming>::builder()
            .maybe_sse_tab_id(sse_tab_id)
            .maybe_filter_query(filter_query)
            .build()
    }

    pub(super) fn bind_session(
        self,
        cookies: &Cookies,
        key: &Key,
    ) -> OperationsFilterFlow<SessionBound> {
        let tab_id = self.sse_tab_id.clone();
        let session = crate::sse::Handle::from_cookies_with_tab(cookies, key, tab_id);
        self.mark_session_bound(session.id())
    }
}

#[transition]
impl OperationsFilterFlow<Incoming> {
    fn mark_session_bound(
        self,
        session_id: SessionId,
    ) -> OperationsFilterFlow<SessionBound> {
        self.transition_with(SessionData { session_id })
    }
}

impl OperationsFilterFlow<SessionBound> {
    pub(super) fn apply_filter(
        self,
        trace_log: &crate::trace_log::TraceLogStore,
    ) -> OperationsFilterFlow<FilterApplied> {
        let query = self.filter_query.as_ref().map(|value| value.to_string());
        trace_log.set_stream_flow_filter(
            &self.state_data.session_id,
            self.sse_tab_id.as_ref(),
            query.as_deref(),
        );
        self.mark_filter_applied()
    }
}

#[transition]
impl OperationsFilterFlow<SessionBound> {
    fn mark_filter_applied(self) -> OperationsFilterFlow<FilterApplied> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl OperationsFilterFlow<FilterApplied> {
    pub(super) fn refresh_panels(
        self,
        trace_log: &crate::trace_log::TraceLogStore,
    ) -> OperationsFilterFlow<Refreshed> {
        trace_log.refresh_stream_log_panels(
            &self.state_data.session_id,
            self.sse_tab_id.as_ref(),
        );
        self.mark_refreshed()
    }
}

#[transition]
impl OperationsFilterFlow<FilterApplied> {
    fn mark_refreshed(self) -> OperationsFilterFlow<Refreshed> {
        self.transition()
    }
}

impl OperationsFilterFlow<Refreshed> {
    pub(super) fn status_code(&self) -> StatusCode {
        StatusCode::NO_CONTENT
    }
}

pub(super) type IncomingFlow = OperationsFilterFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refreshed_state_returns_no_content() {
        let cookies = Cookies::default();
        let key = Key::generate();
        let trace_log =
            crate::trace_log::TraceLogStore::new(crate::sse::Registry::new(), 32, false);
        let refreshed = IncomingFlow::new(None, None)
            .bind_session(&cookies, &key)
            .apply_filter(&trace_log)
            .refresh_panels(&trace_log);

        assert_eq!(refreshed.status_code(), StatusCode::NO_CONTENT);
    }
}
