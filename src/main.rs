mod config;
mod error;
mod sensitive_provider_stub;
mod sensitive_runtime;

use std::sync::Arc;

use app::user;
use snafu::ResultExt;
use tower_cookies::Key;
use tower_sessions_compat::session_store::ExpiredDeletion;
use tower_sessions_sqlx_store::PostgresStore;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> error::Result<()> {
    let sse_registry = http::sse::Registry::new();
    let trace_log = http::trace_log::Store::builder()
        .with_sse(sse_registry.clone())
        .build();
    let diagnostic_log = http::trace_log::Store::builder()
        .with_sse(sse_registry.clone())
        .with_max_entries(100)
        .with_emit_sse(false)
        .build();
    init_tracing(trace_log.clone(), diagnostic_log.clone());

    let cfg = config::Config::load().context(error::LoadConfigSnafu)?;

    let addr = format!("{}:{}", &cfg.http.host, cfg.http.port);

    let infra = infra::Infra::init(&cfg.infra)
        .await
        .context(error::InitInfraSnafu)?;

    sensitive_runtime::maybe_spawn_stub(&cfg.sensitive).await?;

    let user_repo = Arc::new(infra::repo::user::Repository::new(infra.db.clone()));
    let auth_hasher = Arc::new(infra::auth::Argon2Hasher::new());
    let user_service = user::Service::new(user_repo, auth_hasher.clone());

    let auth_repo = Arc::new(infra::auth::Repository::new(infra.db.clone()));
    let auth_provider = app::auth::ProviderImpl::new(auth_repo, auth_hasher);
    let auth_service = app::auth::Service::new(Arc::new(auth_provider));

    let chat_repo = Arc::new(infra::chat::Repository::new(infra.db.clone()));
    let chat_moderation = Arc::new(infra::chat::ModerationQueue::new(infra.db.clone()));
    let chat_rate_limiter = Arc::new(infra::chat::RateLimiter::new(infra.db.clone()));
    let chat_audit = Arc::new(infra::chat::AuditLog::new(infra.db.clone()));
    let chat_clock = Arc::new(infra::chat::SystemClock::new());
    let chat_ids = Arc::new(infra::chat::UuidGenerator::new());
    let chat_service = app::chat::Service::builder()
        .with_repo(chat_repo)
        .with_moderation_queue(chat_moderation)
        .with_rate_limiter(chat_rate_limiter)
        .with_audit_log(chat_audit)
        .with_clock(chat_clock)
        .with_id_generator(chat_ids)
        .build();

    let sensitive_crypto = infra::crypto::Keyring::new(
        cfg.sensitive.data_encryption_keys.clone(),
        cfg.sensitive.active_data_key_id.clone(),
        &cfg.sensitive.disabled_data_key_ids,
    )
    .context(error::BuildSensitiveKeyringSnafu)?;
    let sensitive_repo = Arc::new(infra::sensitive::Repository::new(
        infra.db.clone(),
        sensitive_crypto,
    ));
    let sensitive_provider =
        sensitive_runtime::provider(infra.http.clone(), &cfg.sensitive)?;
    let sensitive_clock = Arc::new(infra::sensitive::SystemClock::new());
    let sensitive_bootstrap = app::sensitive::BootstrapGrants::new(
        cfg.sensitive.reader_emails.clone(),
        cfg.sensitive.operator_emails.clone(),
    );
    let sensitive_service = app::sensitive::Service::builder()
        .with_repo(sensitive_repo)
        .with_provider(sensitive_provider)
        .with_clock(sensitive_clock)
        .build()
        .with_bootstrap_grants(sensitive_bootstrap);

    let session_key = Key::from(&cfg.http.session_secret);
    let http_state = http::State::builder()
        .with_user(user_service.clone())
        .with_auth(auth_service)
        .with_chat(chat_service)
        .with_sensitive(sensitive_service.clone())
        .with_sse(sse_registry)
        .with_cookie_key(session_key.clone())
        .with_trace_log(trace_log)
        .build();

    let session_store = PostgresStore::new(infra.db.clone());
    let cleanup_store = session_store.clone();
    let cleanup_interval =
        std::time::Duration::from_secs(cfg.http.session_cleanup_interval_secs);
    tokio::spawn(async move {
        if let Err(error) = cleanup_store
            .continuously_delete_expired(cleanup_interval)
            .await
        {
            tracing::warn!(?error, "session cleanup task failed");
        }
    });

    sensitive_runtime::prime(
        &sensitive_service,
        &user_service,
        cfg.sensitive.rotation_batch_size,
    )
    .await;
    sensitive_runtime::spawn_background_tasks(sensitive_service.clone(), &cfg.sensitive);

    let app = http::router(http_state, session_store);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context(error::BindHttpListenerSnafu { addr: addr.clone() })?;

    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app)
        .await
        .context(error::ServeHttpSnafu)?;

    Ok(())
}

fn init_tracing(trace_log: http::trace_log::Store, diagnostic_log: http::trace_log::Store) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,http=debug".into());
    let log_format = LogFormat::from_env();

    let subscriber = tracing_subscriber::registry().with(env_filter);
    let trace_layer = http::trace_log::Layer::new(trace_log);
    let diagnostic_layer = http::trace_log::DiagnosticLayer::new(diagnostic_log);

    match log_format {
        LogFormat::Json => {
            subscriber
                .with(
                    tracing_subscriber::fmt::layer()
                        .json()
                        .with_current_span(true)
                        .with_span_list(true),
                )
                .with(trace_layer)
                .with(diagnostic_layer)
                .init();
        }
        LogFormat::Pretty => {
            subscriber
                .with(tracing_subscriber::fmt::layer().pretty())
                .with(trace_layer)
                .with(diagnostic_layer)
                .init();
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum LogFormat {
    Json,
    Pretty,
}

impl LogFormat {
    fn from_env() -> Self {
        LogFormatValue::from_env().into_format()
    }
}

#[derive(Clone, Copy, Debug)]
enum LogFormatValue {
    Json,
    Pretty,
    Unknown,
}

impl LogFormatValue {
    fn from_env() -> Self {
        std::env::var("LOG_FORMAT")
            .ok()
            .as_deref()
            .map(Self::from_str)
            .unwrap_or(Self::Pretty)
    }

    fn from_str(value: &str) -> Self {
        match value {
            "json" => Self::Json,
            "pretty" => Self::Pretty,
            _ => Self::Unknown,
        }
    }

    fn into_format(self) -> LogFormat {
        match self {
            Self::Json => LogFormat::Json,
            Self::Pretty | Self::Unknown => LogFormat::Pretty,
        }
    }
}
