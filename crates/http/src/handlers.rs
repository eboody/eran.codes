use async_stream::stream;
use axum::{
    extract::State,
    response::{IntoResponse, Sse},
};
use core::{convert::Infallible, time::Duration};
use datastar::prelude::PatchElements;
use maud::Render;

pub async fn health(State(_state): State<crate::State>) -> &'static str {
    "ok"
}

pub async fn home(State(_state): State<crate::State>) -> axum::response::Html<String> {
    crate::views::render(crate::views::pages::home::HomePage)
}

pub async fn ping_partial(State(_state): State<crate::State>) -> impl IntoResponse {
    Sse::new(stream! {
        loop {
            let elements = crate::views::partials::ping::PingPartial
                .render()
                .into_string();
            let patch = PatchElements::new(elements);
            let event = patch.write_as_axum_sse_event();

            yield Ok::<_, Infallible>(event);

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    })
}
