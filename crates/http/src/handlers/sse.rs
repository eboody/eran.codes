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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SurrealSignals {
    surreal_message: Option<String>,
    original_surreal_message: Option<String>,
    _surreal_status: Option<String>,
}

fn surreal_payload(
    message: &str,
    status: &str,
) -> crate::sse::Event {
    crate::sse::Event::patch_signals(serde_json::json!({
        "surrealMessage": message,
        "surrealStatus": status,
    }))
}

fn surreal_send(
    state: &crate::State,
    session: &crate::sse::Handle,
    message: &str,
    status: &str,
) -> bool {
    match state.sse.send(session, surreal_payload(message, status)) {
        Ok(()) => true,
        Err(err) => {
            tracing::debug!(?err, "sse session missing for surreal update");
            false
        }
    }
}

fn surreal_original(signals: SurrealSignals) -> String {
    signals
        .original_surreal_message
        .or(signals.surreal_message)
        .unwrap_or_else(|| "Ready.".to_string())
}

pub async fn surreal_message_guarded(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl axum::response::IntoResponse {
    let session =
        crate::sse::Handle::from_cookies(&cookies, &state.cookie_key);
    let session_id = session.id().to_string();
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
        .entry(session_id)
        .or_insert_with(|| std::sync::Arc::new(tokio::sync::Mutex::new(())))
        .clone();

    tokio::spawn(async move {
        let guard = match lock.try_lock() {
            Ok(guard) => {
                if !surreal_send(
                    &state,
                    &session,
                    &format!("Guarded says hi! #{sequence}"),
                    &format!("guarded running #{sequence}"),
                ) {
                    return;
                }
                guard
            }
            Err(_) => {
                if !surreal_send(
                    &state,
                    &session,
                    &format!("Guarded queued #{sequence}"),
                    &format!("guarded queued #{sequence}"),
                ) {
                    return;
                }
                let guard = lock.lock().await;
                if !surreal_send(
                    &state,
                    &session,
                    &format!("Guarded says hi! #{sequence}"),
                    &format!("guarded running #{sequence}"),
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
            &original,
            &format!("guarded done #{sequence}"),
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
    let session_id = session.id().to_string();
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
        .insert(session_id, token.clone())
    {
        previous.cancel();
    }

    tokio::spawn(async move {
        if !surreal_send(
            &state,
            &session,
            &format!("Cancelled says hi! #{sequence}"),
            &format!("cancel running #{sequence}"),
        ) {
            return;
        }

        tokio::select! {
            _ = sleep(Duration::from_secs(1)) => {
                surreal_send(
                    &state,
                    &session,
                    &original,
                    &format!("cancel done #{sequence}"),
                );
            }
            _ = token.cancelled() => {
                surreal_send(
                    &state,
                    &session,
                    &format!("Cancelled #{sequence}"),
                    &format!("cancelled #{sequence}"),
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
    let session_id = session.id().to_string();
    let (mut receiver, guard) = state.sse.subscribe(&session);
    let trace_guard = TraceLogGuard::new(state.trace_log.clone(), session_id.clone());

    tracing::info!(session_id = %session_id, "sse connected");

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
    store: crate::trace_log::Store,
    session_id: String,
}

impl TraceLogGuard {
    fn new(
        store: crate::trace_log::Store,
        session_id: impl Into<String>,
    ) -> Self {
        Self {
            store,
            session_id: session_id.into(),
        }
    }
}

impl Drop for TraceLogGuard {
    fn drop(&mut self) {
        self.store.clear_session(&self.session_id);
    }
}
