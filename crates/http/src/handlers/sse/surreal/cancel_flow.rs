use axum::http;
use statum::{machine, state, transition};
use tokio::time::{Duration, sleep};
use tower_cookies::Cookies;

use crate::handlers::sse;
use crate::types::Text;

#[derive(Clone, Debug)]
pub struct TokenPreparedData {
    token: tokio_util::sync::CancellationToken,
}

#[state]
pub enum SurrealCancelState {
    Incoming,
    TokenPrepared(TokenPreparedData),
    Spawned,
}

#[machine]
pub(in crate::handlers::sse) struct SurrealCancelFlow<SurrealCancelState> {
    state: crate::State,
    session: crate::sse::Handle,
    stream_key: crate::sse::StreamKey,
    sequence: u64,
    original: Text,
}

impl SurrealCancelFlow<Incoming> {
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

        SurrealCancelFlow::<Incoming>::builder()
            .state(state)
            .session(session)
            .stream_key(stream_key)
            .sequence(sequence)
            .original(original)
            .build()
    }
}

#[transition]
impl SurrealCancelFlow<Incoming> {
    pub(in crate::handlers::sse) fn prepare_token(
        self,
    ) -> SurrealCancelFlow<TokenPrepared> {
        let token = tokio_util::sync::CancellationToken::new();
        if let Some(previous) = self
            .state
            .demo
            .surreal
            .cancel
            .insert(self.stream_key.clone(), token.clone())
        {
            previous.cancel();
        }
        self.transition_with(TokenPreparedData { token })
    }
}

#[transition]
impl SurrealCancelFlow<TokenPrepared> {
    pub(in crate::handlers::sse) fn spawn(self) -> SurrealCancelFlow<Spawned> {
        let state = self.state.clone();
        let session = self.session.clone();
        let sequence = self.sequence;
        let original = self.original.clone();
        let token = self.state_data.token.clone();

        tokio::spawn(async move {
            if !sse::surreal_send(
                &state,
                &session,
                Text::from(format!("Cancelled says hi! #{sequence}")),
                Text::from(format!("cancel running #{sequence}")),
            ) {
                return;
            }

            tokio::select! {
                _ = sleep(Duration::from_secs(1)) => {
                    sse::surreal_send(
                        &state,
                        &session,
                        original,
                        Text::from(format!("cancel done #{sequence}")),
                    );
                }
                _ = token.cancelled() => {
                    sse::surreal_send(
                        &state,
                        &session,
                        Text::from(format!("Cancelled #{sequence}")),
                        Text::from(format!("cancelled #{sequence}")),
                    );
                }
            }
        });

        self.transition()
    }
}

impl SurrealCancelFlow<Spawned> {
    pub(in crate::handlers::sse) fn status_code(&self) -> http::StatusCode {
        http::StatusCode::ACCEPTED
    }
}

pub(in crate::handlers::sse) type IncomingFlow = SurrealCancelFlow<Incoming>;
