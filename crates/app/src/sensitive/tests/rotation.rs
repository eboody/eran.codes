use super::super::*;
use super::support::*;

#[tokio::test]
async fn key_rotation_pass_rewraps_stale_ciphertext_and_records_run() {
    let (service, repo) = service_with_state(RepoState::default());

    let run = service
        .run_key_rotation_pass(10)
        .await
        .expect("rotation pass should succeed");

    assert_eq!(run.outcome, sensitive::RotationOutcome::Success);
    assert_eq!(run.rows_rewrapped, 2);
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.recorded_rotation_runs.len(), 1);
    assert_eq!(state.key_custody.stale_token_count, 0);
    assert_eq!(state.key_custody.stale_record_count, 0);
    assert_eq!(state.key_custody.active_key_id, active_key_id());
}
