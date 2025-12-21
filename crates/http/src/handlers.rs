use axum::extract::State;

pub async fn health(State(_state): State<crate::State>) -> &'static str {
    "ok"
}
