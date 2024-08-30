use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .fallback_service(ServeDir::new("web-folder"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    println!("Listening on 3003");
    axum::serve(listener, app).await.unwrap();
}
