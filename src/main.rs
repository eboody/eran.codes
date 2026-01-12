mod config;
mod error;

use std::sync::Arc;

use app::user;
use error::{Error, Result};
use infra::user::Repository as UserRepo;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let cfg = config::Config::load()?;

    let addr = format!("{}:{}", &cfg.http.host, cfg.http.port);

    let infra = infra::Infra::init(&cfg.infra).await.map_err(Error::Infra)?;

    let user_repo = Arc::new(UserRepo::new(infra.db.clone()));
    let auth_hasher = Arc::new(infra::auth::Argon2Hasher::new());
    let user_service = user::Service::new(user_repo, auth_hasher.clone());

    let auth_repo = Arc::new(infra::auth::AuthRepository::new(infra.db.clone()));
    let auth_provider = app::auth::ProviderImpl::new(auth_repo, auth_hasher);
    let auth_service = app::auth::Service::new(Arc::new(auth_provider));

    let sse_registry = http::SseRegistry::new();
    let http_state = http::State::new(user_service, auth_service, sse_registry);

    let app = http::router(http_state, cfg.http.session_secret.clone());

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,http=debug".into());
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    let subscriber = tracing_subscriber::registry().with(env_filter);

    if log_format == "json" {
        subscriber
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_current_span(true)
                    .with_span_list(true),
            )
            .init();
    } else {
        subscriber
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    }
}
