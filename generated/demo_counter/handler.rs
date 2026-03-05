use async_stream::stream;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Router,
};
use datastar::axum::ReadSignals;
use std::{
    convert::Infallible,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::broadcast;

use crate::generated::demo_counter::events::{
    CounterSignalPatch, CounterSseEvent, CounterStreamInput, SyncCounterInput,
};

// BEGIN MDS GENERATED:handler
#[derive(Clone)]
pub struct CounterHandlerState {
    count: Arc<AtomicI64>,
    tx: broadcast::Sender<CounterSseEvent>,
}

impl Default for CounterHandlerState {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(32);
        Self {
            count: Arc::new(AtomicI64::new(0)),
            tx,
        }
    }
}

pub fn counter_routes(state: CounterHandlerState) -> Router {
    Router::new()
        .route("/api/counter/sync", post(sync_counter))
        .route("/api/counter/events", get(counter_events))
        .with_state(state)
}

// ci: datastar-command sync_counter
pub async fn sync_counter(
    State(state): State<CounterHandlerState>,
    ReadSignals(input): ReadSignals<SyncCounterInput>,
) -> StatusCode {
    let delta = input.delta;
    let mut current = state.count.load(Ordering::Relaxed);
    let next_count = loop {
        let next = (current + delta).max(0);
        match state.count.compare_exchange(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break next,
            Err(observed) => current = observed,
        }
    };
    let _ = state.tx.send(CounterSseEvent { count: next_count });
    StatusCode::NO_CONTENT
}

pub async fn counter_events(
    State(state): State<CounterHandlerState>,
    Query(input): Query<CounterStreamInput>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let _component_id = input.component_id;
    let mut rx = state.tx.subscribe();
    let initial_count = state.count.load(Ordering::Relaxed);

    let event_stream = stream! {
        let initial_patch = CounterSignalPatch {
            server_count: initial_count,
            server_connected: true,
        };
        yield Ok(Event::default().event("datastar-patch-signals").data(format!(
            "signals {{server_connected: {}, server_count: {}}}",
            initial_patch.server_connected,
            initial_patch.server_count
        )));

        loop {
            match rx.recv().await {
                Ok(update) => {
                    let patch = CounterSignalPatch {
                        server_count: update.count,
                        server_connected: true,
                    };
                    yield Ok(Event::default().event("datastar-patch-signals").data(format!(
                        "signals {{server_connected: {}, server_count: {}}}",
                        patch.server_connected,
                        patch.server_count
                    )));
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}
// END MDS GENERATED:handler
