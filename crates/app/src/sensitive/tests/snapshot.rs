use super::super::*;
use super::support::*;

#[tokio::test]
async fn guest_snapshot_records_denied_access_without_loading_authorized_record() {
    let record_id = sensitive::Id::new_v4();
    let (service, repo) = service_with_state(RepoState {
        snapshot: StoredSnapshot::builder()
            .maybe_token(None)
            .maybe_latest_sync(None)
            .records(vec![record_proof(record_id)])
            .build(),
        authorized_record: Some(authorized_record(record_id)),
        ..RepoState::default()
    });

    let snapshot = service.snapshot(Viewer::Guest).await.expect("snapshot");

    assert!(snapshot.authorized_record.is_none());
    assert_eq!(snapshot.viewer.tier, ViewerTier::Guest);
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.authorized_loads, 0);
    assert_eq!(state.access_events.len(), 1);
    assert_eq!(
        state.access_events[0].outcome,
        sensitive::AccessOutcome::Denied
    );
    assert_eq!(state.access_events[0].user_id, None);
}

#[tokio::test]
async fn authenticated_snapshot_without_grant_records_denied_access() {
    let record_id = sensitive::Id::new_v4();
    let (service, repo) = service_with_state(RepoState {
        snapshot: StoredSnapshot::builder()
            .maybe_token(None)
            .maybe_latest_sync(None)
            .records(vec![record_proof(record_id)])
            .build(),
        authorized_record: Some(authorized_record(record_id)),
        ..RepoState::default()
    });

    let snapshot = service
        .snapshot(authenticated_viewer("reader@example.com"))
        .await
        .expect("snapshot");

    assert!(snapshot.authorized_record.is_none());
    assert_eq!(snapshot.viewer.tier, ViewerTier::Authenticated);
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.authorized_loads, 0);
    assert_eq!(state.access_events.len(), 1);
    assert_eq!(
        state.access_events[0].outcome,
        sensitive::AccessOutcome::Denied
    );
    assert_eq!(state.access_events[0].user_id, Some(test_user_id()));
}

#[tokio::test]
async fn authorized_record_grant_loads_authorized_record_and_records_allowed_event() {
    let record_id = sensitive::Id::new_v4();
    let authorized = authorized_record(record_id);
    let (service, repo) = service_with_state(RepoState {
        snapshot: StoredSnapshot::builder()
            .maybe_token(None)
            .maybe_latest_sync(None)
            .records(vec![record_proof(record_id)])
            .build(),
        authorized_record: Some(authorized.clone()),
        grants: vec![access_grant(
            sensitive::AccessCapability::AuthorizedRecordRead,
        )],
        ..RepoState::default()
    });

    let snapshot = service
        .snapshot(authenticated_viewer("reader@example.com"))
        .await
        .expect("snapshot");

    assert_eq!(snapshot.viewer.tier, ViewerTier::SensitiveReader);
    assert_eq!(snapshot.authorized_record, Some(authorized));
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.authorized_loads, 1);
    assert_eq!(state.access_events.len(), 1);
    assert_eq!(
        state.access_events[0].outcome,
        sensitive::AccessOutcome::Allowed
    );
}

#[tokio::test]
async fn authorized_grant_records_denied_when_record_disappears_after_snapshot() {
    let record_id = sensitive::Id::new_v4();
    let (service, repo) = service_with_state(RepoState {
        snapshot: StoredSnapshot::builder()
            .maybe_token(None)
            .maybe_latest_sync(None)
            .records(vec![record_proof(record_id)])
            .build(),
        authorized_record: None,
        grants: vec![access_grant(
            sensitive::AccessCapability::AuthorizedRecordRead,
        )],
        ..RepoState::default()
    });

    let snapshot = service
        .snapshot(authenticated_viewer("reader@example.com"))
        .await
        .expect("snapshot");

    assert!(snapshot.authorized_record.is_none());
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.authorized_loads, 1);
    assert_eq!(state.access_events.len(), 1);
    assert_eq!(
        state.access_events[0].outcome,
        sensitive::AccessOutcome::Denied
    );
}

#[tokio::test]
async fn operator_snapshot_exposes_token_sync_and_audit() {
    let record_id = sensitive::Id::new_v4();
    let (service, repo) = service_with_state(RepoState {
        snapshot: StoredSnapshot::builder()
            .token(token_proof())
            .latest_sync(latest_sync())
            .records(vec![record_proof(record_id)])
            .build(),
        authorized_record: Some(authorized_record(record_id)),
        grants: vec![
            access_grant(sensitive::AccessCapability::AuthorizedRecordRead),
            access_grant(sensitive::AccessCapability::TokenStatusRead),
            access_grant(sensitive::AccessCapability::AccessAuditRead),
        ],
        access_events: vec![
            sensitive::AccessEvent::builder()
                .user_id(test_user_id())
                .capability(sensitive::AccessCapability::AuthorizedRecordRead)
                .record_id(record_id)
                .outcome(sensitive::AccessOutcome::Denied)
                .detail(super::super::snapshot::access_detail_text(
                    "viewer lacks authorized_record_read grant",
                ))
                .occurred_at(std::time::UNIX_EPOCH)
                .build(),
        ],
        ..RepoState::default()
    });

    let snapshot = service
        .snapshot(authenticated_viewer("operator@example.com"))
        .await
        .expect("snapshot");

    assert_eq!(snapshot.viewer.tier, ViewerTier::SensitiveOperator);
    assert!(snapshot.token.is_some());
    assert!(snapshot.latest_sync.is_some());
    assert!(snapshot.key_custody.is_some());
    assert_eq!(snapshot.access_events.len(), 2);
    assert_eq!(
        snapshot.access_events[0].outcome,
        sensitive::AccessOutcome::Allowed
    );
    assert_eq!(
        snapshot.access_events[1].outcome,
        sensitive::AccessOutcome::Denied
    );
    assert_eq!(repo.state.lock().expect("repo state").authorized_loads, 1);
}

#[tokio::test]
async fn reader_snapshot_hides_token_and_access_audit() {
    let record_id = sensitive::Id::new_v4();
    let (service, _repo) = service_with_state(RepoState {
        snapshot: StoredSnapshot::builder()
            .token(token_proof())
            .latest_sync(latest_sync())
            .records(vec![record_proof(record_id)])
            .build(),
        authorized_record: Some(authorized_record(record_id)),
        grants: vec![access_grant(
            sensitive::AccessCapability::AuthorizedRecordRead,
        )],
        ..RepoState::default()
    });

    let snapshot = service
        .snapshot(authenticated_viewer("reader@example.com"))
        .await
        .expect("snapshot");

    assert_eq!(snapshot.viewer.tier, ViewerTier::SensitiveReader);
    assert!(snapshot.token.is_none());
    assert!(snapshot.latest_sync.is_none());
    assert!(snapshot.access_events.is_empty());
}
