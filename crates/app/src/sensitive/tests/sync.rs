use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use super::super::*;
use super::support::*;

#[tokio::test]
async fn refresh_provider_token_writes_new_token() {
    let (service, repo) = service_with_state(RepoState::default());

    let token = service
        .refresh_provider_token()
        .await
        .expect("token refresh");

    assert_eq!(token.provider, PROVIDER);
    assert_eq!(repo.state.lock().expect("repo state").token_writes, 1);
}

#[tokio::test]
async fn refresh_provider_token_recovers_from_undecryptable_stored_token() {
    let (service, repo) = service_with_state(RepoState {
        token_load_failures_remaining: 1,
        ..RepoState::default()
    });

    let token = service
        .refresh_provider_token()
        .await
        .expect("token refresh should recover");

    assert_eq!(token.provider, PROVIDER);
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.token_writes, 1);
    assert_eq!(state.token_load_failures_remaining, 0);
}

#[tokio::test]
async fn run_sync_refreshes_missing_token_and_records_idempotent_runs() {
    let (service, repo) = service_with_state(RepoState::default());

    let first = service.run_sync().await.expect("first sync");
    let second = service.run_sync().await.expect("second sync");

    assert_eq!(first.records_seen, 1);
    assert_eq!(first.records_upserted, 1);
    assert_eq!(second.records_seen, 1);
    assert_eq!(second.records_upserted, 0);
    assert_eq!(
        repo.state.lock().expect("repo state").recorded_runs.len(),
        2
    );
}

#[tokio::test]
async fn failed_sync_records_failed_outcome() {
    let repo = Arc::new(TestRepository::new(RepoState::default()));
    let service = Service::builder()
        .with_repo(repo.clone())
        .with_provider(Arc::new(FailingProvider))
        .with_clock(Arc::new(FixedClock {
            now: UNIX_EPOCH + Duration::from_secs(100),
        }))
        .build();

    let error = service.run_sync().await.expect_err("sync should fail");

    assert!(matches!(
        error,
        failure::Error::Provider {
            operation: ProviderOperation::FetchRecords,
            ..
        }
    ));
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.recorded_runs.len(), 1);
    assert_eq!(
        state.recorded_runs[0].outcome,
        sensitive::SyncOutcome::Failed
    );
}

#[tokio::test]
async fn unauthorized_fetch_retries_with_refreshed_token_and_records_boundary_state() {
    let repo = Arc::new(TestRepository::new(RepoState::default()));
    let service = Service::builder()
        .with_repo(repo.clone())
        .with_provider(Arc::new(RetryAfterUnauthorizedProvider {
            unauthorized_remaining: std::sync::Mutex::new(1),
            records: vec![example_record()],
        }))
        .with_clock(Arc::new(FixedClock {
            now: UNIX_EPOCH + Duration::from_secs(100),
        }))
        .build();

    let run = service.run_sync().await.expect("sync should recover");

    assert_eq!(run.records_seen, 1);
    assert_eq!(run.records_upserted, 1);
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.token_writes, 2);
    assert_eq!(
        state
            .snapshot
            .integration_state
            .as_ref()
            .map(|value| value.token_strategy),
        Some(sensitive::TokenStrategy::RetryAfterUnauthorized)
    );
    assert_eq!(
        state
            .snapshot
            .integration_state
            .as_ref()
        .and_then(|value| value.cursor.as_ref().map(ToString::to_string)),
        Some("cursor-retried".to_string())
    );
}

#[tokio::test]
async fn configuration_failure_records_non_retryable_boundary_state() {
    let repo = Arc::new(TestRepository::new(RepoState {
        snapshot: StoredSnapshot::builder()
            .integration_state(
                sensitive::IntegrationState::builder()
                    .provider(PROVIDER)
                    .mode(sensitive::ProviderMode::LocalStub)
                    .endpoint(
                        sensitive::DetailText::try_new("http://127.0.0.1:4001")
                            .expect("detail"),
                    )
                    .auth_mode(sensitive::ProviderAuthMode::StubIssuedToken)
                    .cursor(
                        sensitive::SyncCursor::try_new("cursor-previous")
                            .expect("cursor"),
                    )
                    .last_fetch_outcome(sensitive::FetchOutcome::Success)
                    .token_strategy(sensitive::TokenStrategy::CachedToken)
                    .maybe_last_error_category(None)
                    .last_auth_outcome(sensitive::FetchOutcome::Success)
                    .maybe_last_remote_status_code(None)
                    .maybe_retry_backoff_secs(None)
                    .last_successful_mode(sensitive::ProviderMode::LocalStub)
                    .last_successful_fetch_at(UNIX_EPOCH)
                    .last_attempted_fetch_at(UNIX_EPOCH)
                    .failure_count(0)
                    .build(),
            )
            .maybe_token(None)
            .maybe_latest_sync(None)
            .records(Vec::new())
            .build(),
        ..RepoState::default()
    }));
    let service = Service::builder()
        .with_repo(repo.clone())
        .with_provider(Arc::new(ConfigurationFailingProvider))
        .with_clock(Arc::new(FixedClock {
            now: UNIX_EPOCH + Duration::from_secs(100),
        }))
        .build();

    let error = service.run_sync().await.expect_err("sync should fail closed");

    assert!(matches!(
        error,
        failure::Error::Provider {
            operation: ProviderOperation::RefreshToken,
            kind: ProviderFailureKind::Configuration,
            ..
        }
    ));
    let state = repo.state.lock().expect("repo state");
    let integration_state = state
        .snapshot
        .integration_state
        .as_ref()
        .expect("failed integration state should persist");
    assert_eq!(
        integration_state.mode,
        sensitive::ProviderMode::SandboxHttp
    );
    assert_eq!(
        integration_state.last_error_category,
        Some(sensitive::RemoteErrorCategory::Configuration)
    );
    assert_eq!(
        integration_state.last_auth_outcome,
        Some(sensitive::FetchOutcome::Failed)
    );
    assert_eq!(integration_state.retry_backoff_secs, None);
    assert_eq!(integration_state.failure_count, 1);
    assert_eq!(
        integration_state.last_successful_mode,
        Some(sensitive::ProviderMode::LocalStub)
    );
    assert_eq!(
        integration_state
            .cursor
            .as_ref()
            .map(ToString::to_string),
        Some("cursor-previous".to_string())
    );
    assert_eq!(state.recorded_runs.len(), 1);
    assert_eq!(
        state.recorded_runs[0].outcome,
        sensitive::SyncOutcome::Failed
    );
}
