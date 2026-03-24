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
    let service = Service::new(
        repo.clone(),
        Arc::new(FailingProvider),
        Arc::new(FixedClock {
            now: UNIX_EPOCH + Duration::from_secs(100),
        }),
    );

    let error = service.run_sync().await.expect_err("sync should fail");

    assert!(matches!(
        error,
        Error::Provider {
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
    let service = Service::new(
        repo.clone(),
        Arc::new(RetryAfterUnauthorizedProvider {
            unauthorized_remaining: std::sync::Mutex::new(1),
            records: vec![example_record()],
        }),
        Arc::new(FixedClock {
            now: UNIX_EPOCH + Duration::from_secs(100),
        }),
    );

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
