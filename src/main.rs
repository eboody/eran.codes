mod config;
mod error;

use std::sync::Arc;

use app::user;
use error::{Error, Result};
use infra::user::Repository as UserRepo;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use tower_cookies::Key;
use tower_sessions::session_store::ExpiredDeletion;
use tower_sessions_sqlx_store::PostgresStore;

#[tokio::main]
async fn main() -> Result<()> {
    let sse_registry = http::SseRegistry::new();
    let trace_log = http::trace_log::TraceLogStore::builder()
        .with_sse(sse_registry.clone())
        .build();
    init_tracing(trace_log.clone());

    let cfg = config::Config::load()?;

    let addr = format!("{}:{}", &cfg.http.host, cfg.http.port);

    let infra = infra::Infra::init(&cfg.infra).await.map_err(Error::Infra)?;

    let user_repo = Arc::new(UserRepo::new(infra.db.clone()));
    let auth_hasher = Arc::new(infra::auth::Argon2Hasher::new());
    let user_service = user::Service::new(user_repo, auth_hasher.clone());

    let auth_repo = Arc::new(infra::auth::AuthRepository::new(infra.db.clone()));
    let auth_provider = app::auth::ProviderImpl::new(auth_repo, auth_hasher);
    let auth_service = app::auth::Service::new(Arc::new(auth_provider));

    let session_key = Key::from(&cfg.http.session_secret);
    let http_state = http::State::builder()
        .with_user(user_service)
        .with_auth(auth_service)
        .with_sse(sse_registry)
        .with_cookie_key(session_key.clone())
        .with_trace_log(trace_log)
        .build();

    let session_store = PostgresStore::new(infra.db.clone());
    let cleanup_store = session_store.clone();
    let cleanup_interval = std::time::Duration::from_secs(
        cfg.http.session_cleanup_interval_secs,
    );
    tokio::spawn(async move {
        if let Err(error) = cleanup_store
            .continuously_delete_expired(cleanup_interval)
            .await
        {
            tracing::warn!(?error, "session cleanup task failed");
        }
    });

    let app = http::router(http_state, session_store);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing(trace_log: http::trace_log::TraceLogStore) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,http=debug".into());
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    let subscriber = tracing_subscriber::registry().with(env_filter);
    let trace_layer = http::trace_log::TraceLogLayer::new(trace_log);

    if log_format == "json" {
        subscriber
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_current_span(true)
                    .with_span_list(true),
            )
            .with(trace_layer)
            .init();
    } else {
        subscriber
            .with(tracing_subscriber::fmt::layer().pretty())
            .with(trace_layer)
            .init();
    }
}
