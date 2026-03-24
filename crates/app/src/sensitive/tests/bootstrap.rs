use super::super::*;
use super::support::*;

#[tokio::test]
async fn bootstrap_operator_email_upserts_expected_grants() {
    let (service, repo) = service_with_state(RepoState::default());
    let service = service.with_bootstrap_grants(BootstrapGrants::new(
        Vec::new(),
        vec![user::Email::try_new("operator@example.com").expect("email")],
    ));

    let capabilities = service
        .reconcile_bootstrap_grants_for_user(
            test_user_id(),
            &user::Email::try_new("operator@example.com").expect("email"),
        )
        .await
        .expect("bootstrap grants");

    assert_eq!(
        capabilities,
        vec![
            sensitive::AccessCapability::AuthorizedRecordRead,
            sensitive::AccessCapability::TokenStatusRead,
            sensitive::AccessCapability::AccessAuditRead,
        ]
    );
    let state = repo.state.lock().expect("repo state");
    assert_eq!(state.access_grant_writes, 1);
    assert_eq!(state.grants.len(), 3);
}
