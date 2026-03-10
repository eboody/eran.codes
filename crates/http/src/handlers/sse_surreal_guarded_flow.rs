use std::sync::Arc;

use axum::http::StatusCode;
use statum::{machine, state, transition};
use tokio::time::{Duration, sleep};
use tower_cookies::Cookies;

use crate::types::Text;

#[derive(Clone, Debug)]
pub struct LockPreparedData {
    lock: Arc<tokio::sync::Mutex<()>>,
}

#[state]
pub enum SurrealGuardedState {
    Incoming,
    LockPrepared(LockPreparedData),
    Spawned,
}

#[machine]
pub(super) struct SurrealGuardedFlow<SurrealGuardedState> {
    state: crate::State,
    session: crate::sse::Handle,
    stream_key: crate::sse::StreamKey,
    sequence: u64,
    original: Text,
}

impl SurrealGuardedFlow<Incoming> {
    pub(super) fn from_request(
        state: crate::State,
        cookies: Cookies,
        signals: super::sse::SurrealSignals,
    ) -> Self {
        if let Some(tab_id) = signals.sse_tab_id.clone() {
            crate::request::set_sse_tab_id(tab_id);
        }
        let session = crate::sse::Handle::from_cookies_with_tab(
            &cookies,
            &state.cookie_key,
            signals.sse_tab_id.clone(),
        );
        let stream_key = session.stream_key().clone();
        let sequence = state
            .demo
            .surreal
            .seq
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1;
        let original = super::sse::surreal_original(signals);

        SurrealGuardedFlow::<Incoming>::builder()
            .state(state)
            .session(session)
            .stream_key(stream_key)
            .sequence(sequence)
            .original(original)
            .build()
    }
}

#[transition]
impl SurrealGuardedFlow<Incoming> {
    pub(super) fn prepare_lock(self) -> SurrealGuardedFlow<LockPrepared> {
        let lock = self
            .state
            .demo
            .surreal
            .guard
            .entry(self.stream_key.clone())
            .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
            .clone();
        self.transition_with(LockPreparedData { lock })
    }
}

#[transition]
impl SurrealGuardedFlow<LockPrepared> {
    pub(super) fn spawn(self) -> SurrealGuardedFlow<Spawned> {
        let state = self.state.clone();
        let session = self.session.clone();
        let sequence = self.sequence;
        let original = self.original.clone();
        let lock = self.state_data.lock.clone();

        tokio::spawn(async move {
            let guard = match lock.try_lock() {
                Ok(guard) => {
                    if !super::sse::surreal_send(
                        &state,
                        &session,
                        Text::from(format!("Guarded says hi! #{sequence}")),
                        Text::from(format!("guarded running #{sequence}")),
                    ) {
                        return;
                    }
                    guard
                }
                Err(_) => {
                    if !super::sse::surreal_send(
                        &state,
                        &session,
                        Text::from(format!("Guarded queued #{sequence}")),
                        Text::from(format!("guarded queued #{sequence}")),
                    ) {
                        return;
                    }
                    let guard = lock.lock().await;
                    if !super::sse::surreal_send(
                        &state,
                        &session,
                        Text::from(format!("Guarded says hi! #{sequence}")),
                        Text::from(format!("guarded running #{sequence}")),
                    ) {
                        return;
                    }
                    guard
                }
            };

            sleep(Duration::from_secs(1)).await;
            drop(guard);
            super::sse::surreal_send(
                &state,
                &session,
                original,
                Text::from(format!("guarded done #{sequence}")),
            );
        });

        self.transition()
    }
}

impl SurrealGuardedFlow<Spawned> {
    pub(super) fn status_code(&self) -> StatusCode {
        StatusCode::ACCEPTED
    }
}

pub(super) type IncomingFlow = SurrealGuardedFlow<Incoming>;
