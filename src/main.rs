mod config;
mod error;

use std::sync::Arc;

use app::user;
use error::{Error, Result};
use infra::user::Repository as UserRepo;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,http=debug".into()),
        )
        .init();

    let cfg = config::Config::load()?;

    let addr = format!("{}:{}", &cfg.http.host, cfg.http.port);

    let infra = infra::Infra::init(&cfg.infra).await.map_err(Error::Infra)?;

    let user_repo = Arc::new(UserRepo::new(infra.db.clone()));
    let user_service = user::Service::new(user_repo);

    let sse_registry = http::SseRegistry::new();
    let http_state = http::State::new(user_service, sse_registry);

    let app = http::router(http_state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
