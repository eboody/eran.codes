use std::sync::Arc;

use axum::http;
use statum::{machine, state, transition};
use tokio::time::{Duration, sleep};
use tower_cookies::Cookies;

use crate::handlers::sse;
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
pub(in crate::handlers::sse) struct SurrealGuardedFlow<SurrealGuardedState> {
    state: crate::State,
    session: crate::sse::Handle,
    stream_key: crate::sse::StreamKey,
    sequence: u64,
    original: Text,
}

impl SurrealGuardedFlow<Incoming> {
    pub(in crate::handlers::sse) fn from_request(
        state: crate::State,
        cookies: Cookies,
        signals: sse::SurrealSignals,
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
        let original = sse::surreal_original(signals);

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
    pub(in crate::handlers::sse) fn prepare_lock(self) -> SurrealGuardedFlow<LockPrepared> {
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
    pub(in crate::handlers::sse) fn spawn(self) -> SurrealGuardedFlow<Spawned> {
        let state = self.state.clone();
        let session = self.session.clone();
        let sequence = self.sequence;
        let original = self.original.clone();
        let lock = self.state_data.lock.clone();

        tokio::spawn(async move {
            let guard = match lock.try_lock() {
                Ok(guard) => {
                    if !sse::surreal_send(
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
                    if !sse::surreal_send(
                        &state,
                        &session,
                        Text::from(format!("Guarded queued #{sequence}")),
                        Text::from(format!("guarded queued #{sequence}")),
                    ) {
                        return;
                    }
                    let guard = lock.lock().await;
                    if !sse::surreal_send(
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
            sse::surreal_send(
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
    pub(in crate::handlers::sse) fn status_code(&self) -> http::StatusCode {
        http::StatusCode::ACCEPTED
    }
}

pub(in crate::handlers::sse) type IncomingFlow = SurrealGuardedFlow<Incoming>;
