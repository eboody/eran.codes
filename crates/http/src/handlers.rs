use axum::extract::State;

pub async fn health(State(_state): State<crate::State>) -> &'static str {
    "ok"
}

pub async fn home(State(_state): State<crate::State>) -> axum::response::Html<String> {
    crate::views::render(crate::views::pages::home::view())
}

pub async fn ping_partial(State(_state): State<crate::State>) -> axum::response::Html<String> {
    crate::views::render(crate::views::partials::ping::view())
}
