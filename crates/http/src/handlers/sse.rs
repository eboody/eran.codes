use async_stream::stream;
use axum::{
    extract::Extension,
    response::Sse,
};
use core::convert::Infallible;
use datastar::axum::ReadSignals;
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;
use tokio::time::{Duration, sleep};
use tower_cookies::Cookies;

use crate::types::{SessionId, Text};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SurrealSignals {
    surreal_message: Option<Text>,
    original_surreal_message: Option<Text>,
    _surreal_status: Option<Text>,
}

fn surreal_payload(
    message: &Text,
    status: &Text,
) -> crate::sse::Event {
    crate::sse::Event::patch_signals(serde_json::json!({
        "surrealMessage": message.to_string(),
        "surrealStatus": status.to_string(),
    }))
}

fn surreal_send(
    state: &crate::State,
    session: &crate::sse::Handle,
    message: Text,
    status: Text,
) -> bool {
    match state.sse.send(session, surreal_payload(&message, &status)) {
        Ok(()) => true,
        Err(err) => {
            tracing::debug!(?err, "sse session missing for surreal update");
            false
        }
    }
}

fn surreal_original(signals: SurrealSignals) -> Text {
    signals
        .original_surreal_message
        .or(signals.surreal_message)
        .unwrap_or_else(|| Text::from("Ready."))
}

pub async fn surreal_message_guarded(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl axum::response::IntoResponse {
    let session =
        crate::sse::Handle::from_cookies(&cookies, &state.cookie_key);
    let session_id = session.id();
    let sequence = state
        .demo
        .surreal
        .seq
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;
    let original = surreal_original(signals);

    let lock = state
        .demo
        .surreal
        .guard
        .entry(session_id.clone())
        .or_insert_with(|| std::sync::Arc::new(tokio::sync::Mutex::new(())))
        .clone();

    tokio::spawn(async move {
        let guard = match lock.try_lock() {
            Ok(guard) => {
                if !surreal_send(
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
                if !surreal_send(
                    &state,
                    &session,
                    Text::from(format!("Guarded queued #{sequence}")),
                    Text::from(format!("guarded queued #{sequence}")),
                ) {
                    return;
                }
                let guard = lock.lock().await;
                if !surreal_send(
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
        surreal_send(
            &state,
            &session,
            original,
            Text::from(format!("guarded done #{sequence}")),
        );
    });

    axum::http::StatusCode::ACCEPTED
}

pub async fn surreal_message_cancel(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl axum::response::IntoResponse {
    let session =
        crate::sse::Handle::from_cookies(&cookies, &state.cookie_key);
    let session_id = session.id();
    let sequence = state
        .demo
        .surreal
        .seq
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;
    let original = surreal_original(signals);

    let token = tokio_util::sync::CancellationToken::new();
    if let Some(previous) = state
        .demo
        .surreal
        .cancel
        .insert(session_id.clone(), token.clone())
    {
        previous.cancel();
    }

    tokio::spawn(async move {
        if !surreal_send(
            &state,
            &session,
            Text::from(format!("Cancelled says hi! #{sequence}")),
            Text::from(format!("cancel running #{sequence}")),
        ) {
            return;
        }

        tokio::select! {
            _ = sleep(Duration::from_secs(1)) => {
                surreal_send(
                    &state,
                    &session,
                    original,
                    Text::from(format!("cancel done #{sequence}")),
                );
            }
            _ = token.cancelled() => {
                surreal_send(
                    &state,
                    &session,
                    Text::from(format!("Cancelled #{sequence}")),
                    Text::from(format!("cancelled #{sequence}")),
                );
            }
        }
    });

    axum::http::StatusCode::ACCEPTED
}

pub async fn events(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
) -> impl axum::response::IntoResponse {
    // TODO: Support per-tab SSE streams by mixing a tab id into the session key.
    let session =
        crate::sse::Handle::from_cookies(&cookies, &state.cookie_key);
    let session_id = session.id();
    let (mut receiver, guard) = state.sse.subscribe(&session);
    let trace_guard = TraceLogGuard::new(state.trace_log.clone(), session_id.clone());

    tracing::info!(session_id = %session_id, "sse connected");
    let _ = state
        .sse
        .send(&session, crate::sse::Event::patch_signals(serde_json::json!({
            "sseConnected": true
        })));

    let stream = stream! {
        let _guard = guard;
        let _trace_guard = trace_guard;
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let sse_event = event.as_datastar_event().write_as_axum_sse_event();
                    yield Ok::<_, Infallible>(sse_event);
                }
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => {
                    tracing::info!(session_id = %session_id, "sse disconnected");
                    break;
                }
            }
        }
    };

    Sse::new(stream)
}

struct TraceLogGuard {
    store: crate::trace_log::TraceLogStore,
    session_id: SessionId,
}

impl TraceLogGuard {
    fn new(
        store: crate::trace_log::TraceLogStore,
        session_id: SessionId,
    ) -> Self {
        Self {
            store,
            session_id,
        }
    }
}

impl Drop for TraceLogGuard {
    fn drop(&mut self) {
        self.store.clear_session(&self.session_id);
    }
}
