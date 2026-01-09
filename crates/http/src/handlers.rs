use async_stream::stream;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Sse},
};
use core::convert::Infallible;
use maud::Render;
use tokio::sync::broadcast::error::RecvError;
use tower_cookies::Cookies;

use crate::views::{self, pages};

pub async fn health(State(_state): State<crate::State>) -> &'static str {
    "ok"
}

pub async fn home(
    State(_state): State<crate::State>,
) -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Home))
}

const PING_EVENT: &str = "ping-patch";

pub async fn events(
    State(state): State<crate::State>,
    Extension(cookies): Extension<Cookies>,
) -> impl IntoResponse {
    // TODO: Support per-tab SSE streams by mixing a tab id into the session key.
    let session = crate::sse::Handle::from_cookies(&cookies);
    let mut receiver = state.sse.subscribe(&session);

    let stream = stream! {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let mut sse_event = axum::response::sse::Event::default().data(event.data);
                    if let Some(name) = event.name {
                        sse_event = sse_event.event(name);
                    }
                    yield Ok::<_, Infallible>(sse_event);
                }
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => break,
            }
        }
    };

    Sse::new(stream)
}

pub async fn ping_partial(
    State(state): State<crate::State>,
    Extension(cookies): Extension<Cookies>,
) -> impl IntoResponse {
    let elements = views::partials::Ping.render();
    let session = crate::sse::Handle::from_cookies(&cookies);
    let event = crate::sse::Event::named(PING_EVENT, elements);
    // TODO: Log or surface send failures once we have observability hooks.
    let _ = state.sse.send(&session, event);

    StatusCode::NO_CONTENT
}
