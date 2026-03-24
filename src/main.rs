mod config;
mod error;
mod sensitive_provider_stub;

use std::sync::Arc;

use app::user;
use snafu::ResultExt;
use tower_cookies::Key;
use tower_sessions::session_store::ExpiredDeletion;
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

    if cfg.sensitive.provider_mode == config::SensitiveProviderRuntimeMode::Stub {
        let provider_stub_addr = cfg.sensitive.provider_stub_addr();
        let provider_stub_listener = tokio::net::TcpListener::bind(&provider_stub_addr)
            .await
            .context(error::BindSensitiveProviderListenerSnafu {
                addr: provider_stub_addr.clone(),
            })?;
        let provider_stub =
            sensitive_provider_stub::router(cfg.sensitive.provider_stub_failure_mode);
        tokio::spawn(async move {
            if let Err(error) = axum::serve(provider_stub_listener, provider_stub).await {
                tracing::warn!(?error, "sensitive provider stub exited");
            }
        });
        tracing::info!(
            "sensitive provider stub listening on http://{}",
            provider_stub_addr
        );
    }

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
    let sensitive_provider = Arc::new(match cfg.sensitive.provider_mode {
        config::SensitiveProviderRuntimeMode::Stub => {
            infra::sensitive_boundary::HttpProvider::new_stub(
                infra.http.clone(),
                &cfg.sensitive
                    .provider_base_url()
                    .expect("stub provider base url should exist"),
            )
        }
        config::SensitiveProviderRuntimeMode::SandboxHttp => {
            infra::sensitive_boundary::HttpProvider::new_sandbox(
                infra.http.clone(),
                infra::sensitive_boundary::SandboxHttpConfig {
                    base_url: cfg.sensitive.sandbox.base_url.clone(),
                    client_id: cfg.sensitive.sandbox.client_id.clone(),
                    client_secret: cfg.sensitive.sandbox.client_secret.clone(),
                    timeout_secs: cfg.sensitive.sandbox.timeout_secs,
                    retry_backoff_secs: cfg.sensitive.sandbox.retry_backoff_secs,
                },
            )
        }
    });
    let sensitive_clock = Arc::new(infra::sensitive::SystemClock::new());
    let sensitive_bootstrap = app::sensitive::BootstrapGrants::new(
        cfg.sensitive.reader_emails.clone(),
        cfg.sensitive.operator_emails.clone(),
    );
    let sensitive_service =
        app::sensitive::Service::new(sensitive_repo, sensitive_provider, sensitive_clock)
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

    for email in sensitive_service.bootstrap_grants().configured_emails() {
        match user_service.find_by_email(email.clone()).await {
            Ok(Some(user)) => {
                if let Err(error) = sensitive_service
                    .reconcile_bootstrap_grants_for_user(user.id, &user.email)
                    .await
                {
                    tracing::warn!(
                        ?error,
                        email = %email,
                        "sensitive bootstrap grant reconciliation failed",
                    );
                }
            }
            Ok(None) => {
                tracing::info!(
                    email = %email,
                    "sensitive bootstrap grant skipped because user was not found",
                );
            }
            Err(error) => {
                tracing::warn!(
                    ?error,
                    email = %email,
                    "sensitive bootstrap grant lookup failed",
                );
            }
        }
    }

    if let Err(error) = sensitive_service.refresh_provider_token().await {
        tracing::warn!(?error, "initial sensitive token refresh failed");
    }
    if let Err(error) = sensitive_service.run_sync().await {
        tracing::warn!(?error, "initial sensitive sync failed");
    }
    if let Err(error) = sensitive_service
        .run_key_rotation_pass(cfg.sensitive.rotation_batch_size)
        .await
    {
        tracing::warn!(?error, "initial sensitive key rotation pass failed");
    }

    let sensitive_refresh_interval =
        std::time::Duration::from_secs(cfg.sensitive.token_refresh_interval_secs);
    spawn_repeating_task("sensitive token refresh", sensitive_refresh_interval, {
        let sensitive = sensitive_service.clone();
        move || {
            let sensitive = sensitive.clone();
            async move { sensitive.refresh_provider_token().await.map(|_| ()) }
        }
    });

    let sensitive_sync_interval =
        std::time::Duration::from_secs(cfg.sensitive.sync_interval_secs);
    spawn_repeating_task("sensitive sync", sensitive_sync_interval, {
        let sensitive = sensitive_service.clone();
        move || {
            let sensitive = sensitive.clone();
            async move { sensitive.run_sync().await.map(|_| ()) }
        }
    });

    let sensitive_rotation_interval =
        std::time::Duration::from_secs(cfg.sensitive.rotation_interval_secs);
    let sensitive_rotation_batch_size = cfg.sensitive.rotation_batch_size;
    spawn_repeating_task("sensitive key rotation", sensitive_rotation_interval, {
        let sensitive = sensitive_service.clone();
        move || {
            let sensitive = sensitive.clone();
            async move {
                sensitive
                    .run_key_rotation_pass(sensitive_rotation_batch_size)
                    .await
                    .map(|_| ())
            }
        }
    });

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

fn spawn_repeating_task<Factory, Fut, E>(
    task_name: &'static str,
    interval: std::time::Duration,
    make_future: Factory,
) where
    Factory: Fn() -> Fut + Send + Sync + 'static,
    Fut: core::future::Future<Output = Result<(), E>> + Send + 'static,
    E: std::fmt::Debug + Send + 'static,
{
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        ticker.tick().await;
        loop {
            ticker.tick().await;
            if let Err(error) = make_future().await {
                tracing::warn!(task = task_name, ?error, "background task failed");
            }
        }
    });
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
