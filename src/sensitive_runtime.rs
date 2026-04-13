use std::sync::Arc;

use app::{sensitive, user};
use snafu::ResultExt;
use url::Url;

use crate::{config, error, sensitive_provider_stub};

pub(super) async fn maybe_spawn_stub(config: &config::Sensitive) -> error::Result<()> {
    if config.provider_mode != config::SensitiveProviderRuntimeMode::Stub {
        return Ok(());
    }

    let provider_stub_addr = config.provider_stub_addr();
    let provider_stub_listener = tokio::net::TcpListener::bind(&provider_stub_addr)
        .await
        .context(error::BindSensitiveProviderListenerSnafu {
        addr: provider_stub_addr.clone(),
    })?;
    let provider_stub = sensitive_provider_stub::router(config.provider_stub_failure_mode);
    tokio::spawn(async move {
        if let Err(error) = axum::serve(provider_stub_listener, provider_stub).await {
            tracing::warn!(?error, "sensitive provider stub exited");
        }
    });
    tracing::info!(
        "sensitive provider stub listening on http://{}",
        provider_stub_addr
    );

    Ok(())
}

pub(super) fn provider(
    http: reqwest::Client,
    config: &config::Sensitive,
) -> error::Result<Arc<dyn sensitive::ProviderClient>> {
    let provider: Arc<dyn sensitive::ProviderClient> = match config.provider_mode {
        config::SensitiveProviderRuntimeMode::Stub => Arc::new(
            infra::sensitive_boundary::HttpProvider::new_stub(http, stub_base_url(config)?),
        ),
        config::SensitiveProviderRuntimeMode::SandboxHttp => {
            Arc::new(infra::sensitive_boundary::HttpProvider::new_sandbox(
                http,
                infra::sensitive_boundary::SandboxHttpConfig {
                    base_url: config
                        .sandbox
                        .base_url
                        .clone()
                        .map(infra::sensitive_boundary::SandboxBaseUrl::parse),
                    client_id: config
                        .sandbox
                        .client_id
                        .clone()
                        .map(infra::sensitive_boundary::SandboxClientId::new),
                    client_secret: config.sandbox.client_secret.clone(),
                    timeout: std::time::Duration::from_secs(config.sandbox.timeout_secs),
                    retry_backoff: std::time::Duration::from_secs(
                        config.sandbox.retry_backoff_secs,
                    ),
                },
            ))
        }
    };

    Ok(provider)
}

pub(super) async fn prime(
    service: &sensitive::Service,
    user_service: &user::Service,
    rotation_batch_size: usize,
) {
    reconcile_bootstrap_grants(service, user_service).await;

    if let Err(error) = service.refresh_provider_token().await {
        tracing::warn!(?error, "initial sensitive token refresh failed");
    }
    if let Err(error) = service.run_sync().await {
        tracing::warn!(?error, "initial sensitive sync failed");
    }
    if let Err(error) = service.run_key_rotation_pass(rotation_batch_size).await {
        tracing::warn!(?error, "initial sensitive key rotation pass failed");
    }
}

pub(super) fn spawn_background_tasks(
    service: sensitive::Service,
    config: &config::Sensitive,
) {
    let token_refresh_interval =
        std::time::Duration::from_secs(config.token_refresh_interval_secs);
    spawn_repeating_task("sensitive token refresh", token_refresh_interval, {
        let service = service.clone();
        move || {
            let service = service.clone();
            async move { service.refresh_provider_token().await.map(|_| ()) }
        }
    });

    let sync_interval = std::time::Duration::from_secs(config.sync_interval_secs);
    spawn_repeating_task("sensitive sync", sync_interval, {
        let service = service.clone();
        move || {
            let service = service.clone();
            async move { service.run_sync().await.map(|_| ()) }
        }
    });

    let rotation_interval = std::time::Duration::from_secs(config.rotation_interval_secs);
    let rotation_batch_size = config.rotation_batch_size;
    spawn_repeating_task("sensitive key rotation", rotation_interval, {
        let service = service.clone();
        move || {
            let service = service.clone();
            async move {
                service
                    .run_key_rotation_pass(rotation_batch_size)
                    .await
                    .map(|_| ())
            }
        }
    });
}

async fn reconcile_bootstrap_grants(
    service: &sensitive::Service,
    user_service: &user::Service,
) {
    for email in service.bootstrap_grants().configured_emails() {
        match user_service.find_by_email(email.clone()).await {
            Ok(Some(user)) => {
                if let Err(error) = service
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
}

fn stub_base_url(config: &config::Sensitive) -> error::Result<Url> {
    let value = format!("http://{}/", config.provider_stub_addr());
    Url::parse(&value).context(error::ParseSensitiveProviderBaseUrlSnafu { value })
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
