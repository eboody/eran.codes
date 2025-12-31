use axum::extract::State;

use crate::{respond::Html, views, State as AppState};

pub async fn home(State(_state): State<AppState>) -> Html {
    let page = views::pages::home::page(&[]);
    Html(page)
}

pub async fn health(State(_state): State<AppState>) -> &'static str {
    "ok"
}
