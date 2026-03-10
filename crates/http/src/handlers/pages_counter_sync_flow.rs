use std::sync::atomic::{AtomicI64, Ordering};

use axum::http::StatusCode;
use statum::{machine, state, transition};
use tower_cookies::{Cookies, Key};

use crate::types::{SessionId, SseTabId};

#[derive(Clone)]
pub struct SessionData {
    session: crate::sse::Handle,
}

#[derive(Clone)]
pub struct CounterUpdatedData {
    session: crate::sse::Handle,
    next_count: i64,
}

#[state]
pub enum CounterSyncState {
    Incoming,
    SessionBound(SessionData),
    StreamVerified(SessionData),
    CounterUpdated(CounterUpdatedData),
    PatchDispatched,
    SessionUnavailable,
    DispatchFailed,
}

#[machine]
pub(super) struct CounterSyncFlow<CounterSyncState> {
    delta: i64,
    sse_tab_id: Option<SseTabId>,
}

impl CounterSyncFlow<Incoming> {
    pub(super) fn new(delta: i64, sse_tab_id: Option<SseTabId>) -> Self {
        CounterSyncFlow::<Incoming>::builder()
            .delta(delta)
            .maybe_sse_tab_id(sse_tab_id)
            .build()
    }

    pub(super) fn bind_session(
        self,
        cookies: &Cookies,
        key: &Key,
    ) -> CounterSyncFlow<SessionBound> {
        let tab_id = self.sse_tab_id.clone();
        let session = crate::sse::Handle::from_cookies_with_tab(cookies, key, tab_id);
        self.mark_session_bound(session)
    }
}

#[transition]
impl CounterSyncFlow<Incoming> {
    fn mark_session_bound(
        self,
        session: crate::sse::Handle,
    ) -> CounterSyncFlow<SessionBound> {
        self.transition_with(SessionData { session })
    }
}

impl CounterSyncFlow<SessionBound> {
    pub(super) fn session_id(&self) -> SessionId {
        self.state_data.session.id()
    }

    pub(super) fn verify_streams(self, has_streams: bool) -> StreamCheckOutcome {
        if has_streams {
            StreamCheckOutcome::Ready(self.mark_stream_verified())
        } else {
            StreamCheckOutcome::Unavailable(self.mark_session_unavailable())
        }
    }
}

#[transition]
impl CounterSyncFlow<SessionBound> {
    fn mark_stream_verified(self) -> CounterSyncFlow<StreamVerified> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }

    fn mark_session_unavailable(self) -> CounterSyncFlow<SessionUnavailable> {
        self.transition()
    }
}

impl CounterSyncFlow<StreamVerified> {
    pub(super) fn update_counter(
        self,
        counter: &AtomicI64,
    ) -> CounterSyncFlow<CounterUpdated> {
        let mut current = counter.load(Ordering::Relaxed);
        loop {
            let next = (current + self.delta).max(0);
            match counter.compare_exchange(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return self.mark_counter_updated(next),
                Err(observed) => current = observed,
            }
        }
    }
}

#[transition]
impl CounterSyncFlow<StreamVerified> {
    fn mark_counter_updated(self, next_count: i64) -> CounterSyncFlow<CounterUpdated> {
        let session = self.state_data.session.clone();
        self.transition_with(CounterUpdatedData {
            session,
            next_count,
        })
    }
}

impl CounterSyncFlow<CounterUpdated> {
    pub(super) fn dispatch_patch(self, sse: &crate::sse::Registry) -> DispatchOutcome {
        let session = &self.state_data.session;
        let next = self.state_data.next_count;
        let event = crate::sse::Event::patch_signals(serde_json::json!({
            "server_count": next,
            "server_connected": true,
        }));

        match sse.send(session, event) {
            Ok(_) => DispatchOutcome::Dispatched(self.mark_patch_dispatched()),
            Err(crate::sse::SendError::SessionMissing) => {
                DispatchOutcome::Unavailable(self.mark_session_unavailable())
            }
            Err(crate::sse::SendError::SendFailed) => {
                DispatchOutcome::Failed(self.mark_dispatch_failed())
            }
        }
    }
}

#[transition]
impl CounterSyncFlow<CounterUpdated> {
    fn mark_patch_dispatched(self) -> CounterSyncFlow<PatchDispatched> {
        self.transition()
    }

    fn mark_session_unavailable(self) -> CounterSyncFlow<SessionUnavailable> {
        self.transition()
    }

    fn mark_dispatch_failed(self) -> CounterSyncFlow<DispatchFailed> {
        self.transition()
    }
}

impl CounterSyncFlow<PatchDispatched> {
    pub(super) fn status_code(&self) -> StatusCode {
        StatusCode::NO_CONTENT
    }
}

impl CounterSyncFlow<SessionUnavailable> {
    pub(super) fn status_code(&self) -> StatusCode {
        StatusCode::PRECONDITION_REQUIRED
    }
}

impl CounterSyncFlow<DispatchFailed> {
    pub(super) fn status_code(&self) -> StatusCode {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

pub(super) enum StreamCheckOutcome {
    Ready(CounterSyncFlow<StreamVerified>),
    Unavailable(CounterSyncFlow<SessionUnavailable>),
}

pub(super) enum DispatchOutcome {
    Dispatched(CounterSyncFlow<PatchDispatched>),
    Unavailable(CounterSyncFlow<SessionUnavailable>),
    Failed(CounterSyncFlow<DispatchFailed>),
}

impl StreamCheckOutcome {
    pub(super) fn dispatch(
        self,
        counter: &AtomicI64,
        sse: &crate::sse::Registry,
    ) -> DispatchOutcome {
        match self {
            Self::Ready(ready) => ready.update_counter(counter).dispatch_patch(sse),
            Self::Unavailable(unavailable) => DispatchOutcome::Unavailable(unavailable),
        }
    }
}

impl DispatchOutcome {
    pub(super) fn status_code(&self) -> StatusCode {
        match self {
            Self::Dispatched(dispatched) => dispatched.status_code(),
            Self::Unavailable(unavailable) => unavailable.status_code(),
            Self::Failed(failed) => failed.status_code(),
        }
    }
}

pub(super) type IncomingFlow = CounterSyncFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_streams_routes_to_unavailable_when_missing() {
        let cookies = Cookies::default();
        let key = Key::generate();
        let bound = IncomingFlow::new(1, None).bind_session(&cookies, &key);
        let outcome = bound.verify_streams(false);
        assert!(matches!(outcome, StreamCheckOutcome::Unavailable(_)));
    }

    #[test]
    fn update_counter_applies_floor_at_zero() {
        let cookies = Cookies::default();
        let key = Key::generate();
        let counter = AtomicI64::new(0);
        let bound = IncomingFlow::new(-5, None).bind_session(&cookies, &key);
        let ready = match bound.verify_streams(true) {
            StreamCheckOutcome::Ready(ready) => ready,
            StreamCheckOutcome::Unavailable(_) => panic!("expected ready state"),
        };
        let _updated = ready.update_counter(&counter);

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn dispatch_patch_succeeds_with_subscribed_stream() {
        let registry = crate::sse::Registry::new();
        let cookies = Cookies::default();
        let key = Key::generate();
        let handle = crate::sse::Handle::from_cookies_with_tab(&cookies, &key, None);
        let (_receiver, _guard) = registry.subscribe(&handle);
        let counter = AtomicI64::new(0);

        let bound = IncomingFlow::new(2, None).bind_session(&cookies, &key);
        let ready = match bound.verify_streams(true) {
            StreamCheckOutcome::Ready(ready) => ready,
            StreamCheckOutcome::Unavailable(_) => panic!("expected ready state"),
        };
        let updated = ready.update_counter(&counter);
        let outcome = updated.dispatch_patch(&registry);

        assert!(matches!(outcome, DispatchOutcome::Dispatched(_)));
    }
}
