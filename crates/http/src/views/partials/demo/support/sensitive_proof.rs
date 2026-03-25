mod cards;
mod format;

use bon::Builder;
use maud::{Markup, Render};

use crate::views::partials;

use super::{CardGrid, Results};

#[derive(Clone, Debug, Builder)]
pub struct SensitiveProof {
    pub snapshot: app::sensitive::Snapshot,
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for SensitiveProof {
    fn render(&self) -> maud::Markup {
        let cards = cards::proof_cards(&self.snapshot)
            .into_iter()
            .map(render_card)
            .collect();

        Results::builder()
            .target_id(crate::types::Text::from("sensitive-proof-target"))
            .summary(CardGrid::builder().cards(cards).build().render())
            .trace(
                partials::RequestTraceLog::builder()
                    .entries(&self.trace)
                    .build()
                    .render(),
            )
            .build()
            .render()
    }
}

fn render_card(card: cards::ProofCard) -> Markup {
    partials::StatusCard::builder()
        .title(card.title)
        .items(card.items)
        .build()
        .render()
}

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use super::*;

    #[test]
    fn renders_human_readable_timestamps() {
        let snapshot = app::sensitive::Snapshot::builder()
            .viewer(
                app::sensitive::ViewerState::builder()
                    .tier(app::sensitive::ViewerTier::SensitiveOperator)
                    .capabilities(vec![
                        domain::sensitive::AccessCapability::AuthorizedRecordRead,
                        domain::sensitive::AccessCapability::TokenStatusRead,
                        domain::sensitive::AccessCapability::AccessAuditRead,
                    ])
                    .build(),
            )
            .token(
                app::sensitive::TokenProof::builder()
                    .status(
                        domain::sensitive::TokenStatus::builder()
                            .provider(domain::sensitive::Provider::SyntheticSecureFeed)
                            .expires_at(UNIX_EPOCH)
                            .refreshed_at(UNIX_EPOCH)
                            .build(),
                    )
                    .ciphertext(
                        app::sensitive::CiphertextEvidence::builder()
                            .key_id(
                                domain::sensitive::KeyId::try_new("legacy_data_key")
                                    .expect("key id"),
                            )
                            .preview("token-preview".to_string())
                            .bytes(47)
                            .build(),
                    )
                    .build(),
            )
            .latest_sync(
                domain::sensitive::SyncRun::builder()
                    .provider(domain::sensitive::Provider::SyntheticSecureFeed)
                    .outcome(domain::sensitive::SyncOutcome::Success)
                    .records_seen(3)
                    .records_upserted(3)
                    .detail(
                        domain::sensitive::DetailText::try_new(
                            "3 synthetic records processed for runtime proof",
                        )
                        .expect("detail"),
                    )
                    .started_at(UNIX_EPOCH)
                    .finished_at(UNIX_EPOCH)
                    .build(),
            )
            .integration_state(
                domain::sensitive::IntegrationState::builder()
                    .provider(domain::sensitive::Provider::SyntheticSecureFeed)
                    .mode(domain::sensitive::ProviderMode::LocalStub)
                    .endpoint(
                        domain::sensitive::DetailText::try_new(
                            "http://127.0.0.1:4002/",
                        )
                        .expect("detail"),
                    )
                    .auth_mode(domain::sensitive::ProviderAuthMode::StubIssuedToken)
                    .cursor(
                        domain::sensitive::SyncCursor::try_new("cursor-gamma")
                            .expect("cursor"),
                    )
                    .last_fetch_outcome(domain::sensitive::FetchOutcome::Success)
                    .last_auth_outcome(domain::sensitive::FetchOutcome::Success)
                    .token_strategy(
                        domain::sensitive::TokenStrategy::RetryAfterUnauthorized,
                    )
                    .last_error_category(
                        domain::sensitive::RemoteErrorCategory::Unauthorized,
                    )
                    .last_remote_status_code(401)
                    .retry_backoff_secs(45)
                    .last_successful_mode(domain::sensitive::ProviderMode::LocalStub)
                    .last_successful_fetch_at(UNIX_EPOCH)
                    .last_attempted_fetch_at(UNIX_EPOCH)
                    .failure_count(1)
                    .build(),
            )
            .key_custody(
                domain::sensitive::KeyCustodyState::builder()
                    .active_key_id(
                        domain::sensitive::KeyId::try_new("active_data_key")
                            .expect("key id"),
                    )
                    .configured_keys(vec![
                        domain::sensitive::ConfiguredKey::builder()
                            .key_id(
                                domain::sensitive::KeyId::try_new("active_data_key")
                                    .expect("key id"),
                            )
                            .status(domain::sensitive::CipherKeyStatus::Active)
                            .build(),
                        domain::sensitive::ConfiguredKey::builder()
                            .key_id(
                                domain::sensitive::KeyId::try_new("legacy_data_key")
                                    .expect("key id"),
                            )
                            .status(
                                domain::sensitive::CipherKeyStatus::ReadOnlyLegacy,
                            )
                            .build(),
                    ])
                    .token_counts(vec![domain::sensitive::KeyedCiphertextCount::builder()
                        .key_id(
                            domain::sensitive::KeyId::try_new("legacy_data_key")
                                .expect("key id"),
                        )
                        .count(1)
                        .build()])
                    .record_counts(vec![domain::sensitive::KeyedCiphertextCount::builder()
                        .key_id(
                            domain::sensitive::KeyId::try_new("legacy_data_key")
                                .expect("key id"),
                        )
                        .count(2)
                        .build()])
                    .stale_token_count(1)
                    .stale_record_count(2)
                    .last_rotation_run(
                        domain::sensitive::KeyRotationRun::builder()
                            .active_key_id(
                                domain::sensitive::KeyId::try_new("active_data_key")
                                    .expect("key id"),
                            )
                            .outcome(domain::sensitive::RotationOutcome::Success)
                            .rows_scanned(3)
                            .rows_rewrapped(3)
                            .rows_already_current(0)
                            .rows_failed(0)
                            .detail(
                                domain::sensitive::DetailText::try_new(
                                    "stale ciphertext rewrapped to the active key",
                                )
                                .expect("detail"),
                            )
                            .started_at(UNIX_EPOCH)
                            .finished_at(UNIX_EPOCH)
                            .build(),
                    )
                    .build(),
            )
            .records(Vec::new())
            .maybe_authorized_record(None)
            .access_events(Vec::new())
            .build();

        let markup = SensitiveProof::builder()
            .snapshot(snapshot)
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("1970-01-01 00:00"));
        assert!(!markup.contains("SystemTime {"));
        assert!(markup.contains("retry_after_unauthorized"));
        assert!(markup.contains("http://127.0.0.1:4002/"));
        assert!(markup.contains("active_key_id"));
        assert!(markup.contains("legacy_data_key: 2"));
    }

    #[test]
    fn renders_denied_state_for_signed_in_viewer_without_grant() {
        let snapshot = app::sensitive::Snapshot::builder()
            .viewer(
                app::sensitive::ViewerState::builder()
                    .tier(app::sensitive::ViewerTier::Authenticated)
                    .capabilities(Vec::new())
                    .build(),
            )
            .maybe_token(None)
            .maybe_latest_sync(None)
            .maybe_integration_state(None)
            .records(Vec::new())
            .maybe_authorized_record(None)
            .access_events(Vec::new())
            .build();

        let markup = SensitiveProof::builder()
            .snapshot(snapshot)
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("signed-in viewer lacks authorized_record_read"));
        assert!(markup.contains("Signed-in viewer is denied until an authorized_record_read grant exists."));
    }

    #[test]
    fn renders_operator_audit_evidence() {
        let snapshot = app::sensitive::Snapshot::builder()
            .viewer(
                app::sensitive::ViewerState::builder()
                    .tier(app::sensitive::ViewerTier::SensitiveOperator)
                    .capabilities(vec![
                        domain::sensitive::AccessCapability::AuthorizedRecordRead,
                        domain::sensitive::AccessCapability::TokenStatusRead,
                        domain::sensitive::AccessCapability::AccessAuditRead,
                    ])
                    .build(),
            )
            .maybe_token(None)
            .maybe_latest_sync(None)
            .maybe_integration_state(None)
            .records(Vec::new())
            .maybe_authorized_record(None)
            .access_events(vec![domain::sensitive::AccessEvent::builder()
                .maybe_user_id(None)
                .capability(domain::sensitive::AccessCapability::AuthorizedRecordRead)
                .maybe_record_id(None)
                .outcome(domain::sensitive::AccessOutcome::Denied)
                .detail(
                    domain::sensitive::DetailText::try_new(
                        "sign in required before authorized record read",
                    )
                    .expect("detail"),
                )
                .occurred_at(UNIX_EPOCH)
                .build()])
            .build();

        let markup = SensitiveProof::builder()
            .snapshot(snapshot)
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("Recent access audit"));
        assert!(markup.contains("guest"));
        assert!(markup.contains("authorized_record_read"));
        assert!(markup.contains("denied"));
    }

    #[test]
    fn renders_sandbox_boundary_state_for_operator() {
        let snapshot = app::sensitive::Snapshot::builder()
            .viewer(
                app::sensitive::ViewerState::builder()
                    .tier(app::sensitive::ViewerTier::SensitiveOperator)
                    .capabilities(vec![
                        domain::sensitive::AccessCapability::AuthorizedRecordRead,
                        domain::sensitive::AccessCapability::TokenStatusRead,
                        domain::sensitive::AccessCapability::AccessAuditRead,
                    ])
                    .build(),
            )
            .maybe_token(None)
            .maybe_latest_sync(None)
            .integration_state(
                domain::sensitive::IntegrationState::builder()
                    .provider(domain::sensitive::Provider::SyntheticSecureFeed)
                    .mode(domain::sensitive::ProviderMode::SandboxHttp)
                    .endpoint(
                        domain::sensitive::DetailText::try_new(
                            "https://sandbox.example.test/",
                        )
                        .expect("detail"),
                    )
                    .auth_mode(domain::sensitive::ProviderAuthMode::ClientCredentials)
                    .cursor(
                        domain::sensitive::SyncCursor::try_new("cursor-sandbox")
                            .expect("cursor"),
                    )
                    .last_fetch_outcome(domain::sensitive::FetchOutcome::Failed)
                    .last_auth_outcome(domain::sensitive::FetchOutcome::Failed)
                    .token_strategy(
                        domain::sensitive::TokenStrategy::RefreshedToken,
                    )
                    .last_error_category(domain::sensitive::RemoteErrorCategory::Forbidden)
                    .last_remote_status_code(403)
                    .retry_backoff_secs(60)
                    .last_successful_mode(domain::sensitive::ProviderMode::SandboxHttp)
                    .last_successful_fetch_at(UNIX_EPOCH)
                    .last_attempted_fetch_at(UNIX_EPOCH)
                    .failure_count(2)
                    .build(),
            )
            .records(Vec::new())
            .maybe_authorized_record(None)
            .access_events(Vec::new())
            .build();

        let markup = SensitiveProof::builder()
            .snapshot(snapshot)
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("sandbox_http"));
        assert!(markup.contains("client_credentials"));
        assert!(markup.contains("https://sandbox.example.test/"));
        assert!(markup.contains("403"));
        assert!(markup.contains("60"));
        assert!(markup.contains("refreshed_token"));
    }

    #[test]
    fn renders_cards_in_a_responsive_grid() {
        let markup = SensitiveProof::builder()
            .snapshot(
                app::sensitive::Snapshot::builder()
                    .viewer(
                        app::sensitive::ViewerState::builder()
                            .tier(app::sensitive::ViewerTier::Guest)
                            .capabilities(Vec::new())
                            .build(),
                    )
                    .records(Vec::new())
                    .access_events(Vec::new())
                    .build(),
            )
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-support-card-grid"));
        assert!(markup.contains("data-support-results"));
    }
}
