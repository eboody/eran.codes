use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug, Builder)]
pub struct SensitiveProof {
    pub snapshot: app::sensitive::Snapshot,
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for SensitiveProof {
    fn render(&self) -> maud::Markup {
        let policy_items = policy_items(&self.snapshot);
        let token_items = token_items(&self.snapshot);
        let sync_items = sync_items(&self.snapshot);
        let boundary_items = boundary_items(&self.snapshot);
        let key_custody_items = key_custody_items(&self.snapshot);
        let record_items = record_items(&self.snapshot);
        let authorized_items = authorized_items(&self.snapshot);
        let audit_items = audit_items(&self.snapshot);

        maud::html! {
            article id="sensitive-proof-target" {
                (partials::StatusCard::builder()
                    .title(Text::from("Access policy"))
                    .items(policy_items)
                    .build())
                (partials::StatusCard::builder()
                    .title(Text::from("Provider token"))
                    .items(token_items)
                    .build())
                (partials::StatusCard::builder()
                    .title(Text::from("Latest sync"))
                    .items(sync_items)
                    .build())
                (partials::StatusCard::builder()
                    .title(Text::from("Boundary state"))
                    .items(boundary_items)
                    .build())
                (partials::StatusCard::builder()
                    .title(Text::from("Key custody"))
                    .items(key_custody_items)
                    .build())
                (partials::StatusCard::builder()
                    .title(Text::from("Redacted records"))
                    .items(record_items)
                    .build())
                (partials::StatusCard::builder()
                    .title(Text::from("Authorized sample"))
                    .items(authorized_items)
                    .build())
                (partials::StatusCard::builder()
                    .title(Text::from("Recent access audit"))
                    .items(audit_items)
                    .build())
                (partials::RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}

fn policy_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    vec![
        partials::StatusCardItem::text("viewer_tier", snapshot.viewer.tier.as_ref()),
        partials::StatusCardItem::text(
            "capabilities",
            format_capabilities(&snapshot.viewer.capabilities),
        ),
        partials::StatusCardItem::text(
            "authorized_record_policy",
            if snapshot.viewer.allows_authorized_record() {
                "persisted grant allows decrypt path"
            } else if snapshot.viewer.is_authenticated() {
                "signed-in viewer lacks authorized_record_read"
            } else {
                "sign in required before decrypt path"
            },
        ),
        partials::StatusCardItem::text(
            "token_status_policy",
            if snapshot.viewer.allows_token_status() {
                "operator grant allows token + sync visibility"
            } else {
                "token lifecycle remains hidden without operator grant"
            },
        ),
        partials::StatusCardItem::text(
            "audit_policy",
            if snapshot.viewer.allows_access_audit() {
                "operator grant allows recent access-event review"
            } else {
                "recent access audit remains hidden without operator grant"
            },
        ),
    ]
}

fn token_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    if !snapshot.viewer.allows_token_status() {
        return vec![partials::StatusCardItem::text(
            "access",
            "Token lifecycle state is restricted to sensitive operators.",
        )];
    }

    match &snapshot.token {
        Some(token) => vec![
            partials::StatusCardItem::text("provider", token.status.provider.as_ref()),
            partials::StatusCardItem::text("key_id", token.ciphertext.key_id.to_string()),
            partials::StatusCardItem::text(
                "expires_at",
                format_proof_time(token.status.expires_at),
            ),
            partials::StatusCardItem::text(
                "refreshed_at",
                format_proof_time(token.status.refreshed_at),
            ),
            partials::StatusCardItem::text("ciphertext_bytes", token.ciphertext.bytes.to_string()),
            partials::StatusCardItem::text("ciphertext_preview", token.ciphertext.preview.clone()),
        ],
        None => vec![partials::StatusCardItem::text(
            "status",
            "No provider token persisted yet.",
        )],
    }
}

fn sync_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    if !snapshot.viewer.allows_token_status() {
        return vec![partials::StatusCardItem::text(
            "access",
            "Latest sync state is restricted to sensitive operators.",
        )];
    }

    match &snapshot.latest_sync {
        Some(run) => vec![
            partials::StatusCardItem::text("provider", run.provider.as_ref()),
            partials::StatusCardItem::text("outcome", run.outcome.as_ref()),
            partials::StatusCardItem::text("records_seen", run.records_seen.to_string()),
            partials::StatusCardItem::text(
                "records_upserted",
                run.records_upserted.to_string(),
            ),
            partials::StatusCardItem::text("detail", run.detail.to_string()),
            partials::StatusCardItem::text("finished_at", format_proof_time(run.finished_at)),
        ],
        None => vec![partials::StatusCardItem::text(
            "status",
            "No sync run has completed yet.",
        )],
    }
}

fn record_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    if snapshot.records.is_empty() {
        return vec![partials::StatusCardItem::text(
            "records",
            "No redacted records persisted yet.",
        )];
    }

    snapshot
        .records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            partials::StatusCardItem::text(
                format!("record_{}", index + 1),
                format!(
                    "{} | last4 {} | {} bytes | {}",
                    record.label,
                    record.last4,
                    record.ciphertext.bytes,
                    record.ciphertext.preview
                ),
            )
        })
        .collect()
}

fn boundary_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    if !snapshot.viewer.allows_token_status() {
        return vec![partials::StatusCardItem::text(
            "access",
            "Provider boundary state is restricted to sensitive operators.",
        )];
    }

    match &snapshot.integration_state {
        Some(state) => vec![
            partials::StatusCardItem::text("provider", state.provider.as_ref()),
            partials::StatusCardItem::text("mode", state.mode.as_ref()),
            partials::StatusCardItem::text("endpoint", state.endpoint.to_string()),
            partials::StatusCardItem::text(
                "auth_mode",
                state
                    .auth_mode
                    .map(|mode| mode.as_ref().to_string())
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "cursor",
                state
                    .cursor
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "last_fetch_outcome",
                state.last_fetch_outcome.as_ref(),
            ),
            partials::StatusCardItem::text(
                "last_auth_outcome",
                state
                    .last_auth_outcome
                    .map(|outcome| outcome.as_ref().to_string())
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "token_strategy",
                state.token_strategy.as_ref(),
            ),
            partials::StatusCardItem::text(
                "last_error_category",
                state
                    .last_error_category
                    .map(|category| category.as_ref().to_string())
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "last_successful_fetch_at",
                state
                    .last_successful_fetch_at
                    .map(format_proof_time)
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "last_remote_status_code",
                state
                    .last_remote_status_code
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "retry_backoff_secs",
                state
                    .retry_backoff_secs
                    .map(|secs| secs.to_string())
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "last_successful_mode",
                state
                    .last_successful_mode
                    .map(|mode| mode.as_ref().to_string())
                    .unwrap_or_else(|| "none".to_string()),
            ),
            partials::StatusCardItem::text(
                "last_attempted_fetch_at",
                format_proof_time(state.last_attempted_fetch_at),
            ),
            partials::StatusCardItem::text(
                "failure_count",
                state.failure_count.to_string(),
            ),
        ],
        None => vec![partials::StatusCardItem::text(
            "status",
            "No provider boundary state has been recorded yet.",
        )],
    }
}

fn key_custody_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    if !snapshot.viewer.allows_token_status() {
        return vec![partials::StatusCardItem::text(
            "access",
            "Key custody state is restricted to sensitive operators.",
        )];
    }

    match &snapshot.key_custody {
        Some(custody) => {
            let mut items = vec![
                partials::StatusCardItem::text(
                    "active_key_id",
                    custody.active_key_id.to_string(),
                ),
                partials::StatusCardItem::text(
                    "configured_keys",
                    custody
                        .configured_keys
                        .iter()
                        .map(|entry| format!("{} ({})", entry.key_id, entry.status))
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
                partials::StatusCardItem::text(
                    "token_counts",
                    format_key_counts(&custody.token_counts),
                ),
                partials::StatusCardItem::text(
                    "record_counts",
                    format_key_counts(&custody.record_counts),
                ),
                partials::StatusCardItem::text(
                    "stale_token_count",
                    custody.stale_token_count.to_string(),
                ),
                partials::StatusCardItem::text(
                    "stale_record_count",
                    custody.stale_record_count.to_string(),
                ),
            ];

            match &custody.last_rotation_run {
                Some(run) => {
                    items.push(partials::StatusCardItem::text(
                        "last_rotation_outcome",
                        run.outcome.as_ref(),
                    ));
                    items.push(partials::StatusCardItem::text(
                        "last_rotation_rows",
                        format!(
                            "scanned {} | rewrapped {} | already_current {} | failed {}",
                            run.rows_scanned,
                            run.rows_rewrapped,
                            run.rows_already_current,
                            run.rows_failed
                        ),
                    ));
                    items.push(partials::StatusCardItem::text(
                        "last_rotation_detail",
                        run.detail.to_string(),
                    ));
                    items.push(partials::StatusCardItem::text(
                        "last_rotation_finished_at",
                        format_proof_time(run.finished_at),
                    ));
                }
                None => items.push(partials::StatusCardItem::text(
                    "last_rotation",
                    "No key rotation pass has completed yet.",
                )),
            }

            items
        }
        None => vec![partials::StatusCardItem::text(
            "status",
            "No key custody state has been recorded yet.",
        )],
    }
}

fn authorized_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    match &snapshot.authorized_record {
        Some(record) => vec![
            partials::StatusCardItem::text("label", record.label.to_string()),
            partials::StatusCardItem::text("last4", record.last4.to_string()),
            partials::StatusCardItem::text(
                "subject_name",
                record.authorized.subject_name.to_string(),
            ),
            partials::StatusCardItem::text(
                "classification",
                record.authorized.classification.to_string(),
            ),
            partials::StatusCardItem::text("note", record.authorized.note.to_string()),
        ],
        None if snapshot.viewer.allows_authorized_record() => vec![
            partials::StatusCardItem::text(
                "status",
                "No authorized sample record is available yet.",
            ),
        ],
        None if snapshot.viewer.is_authenticated() => vec![partials::StatusCardItem::text(
            "access",
            "Signed-in viewer is denied until an authorized_record_read grant exists.",
        )],
        None => vec![partials::StatusCardItem::text(
            "access",
            "Sign in to request the authorized sample record path.",
        )],
    }
}

fn audit_items(snapshot: &app::sensitive::Snapshot) -> Vec<partials::StatusCardItem> {
    if !snapshot.viewer.allows_access_audit() {
        return vec![partials::StatusCardItem::text(
            "access",
            "Recent access-audit evidence is restricted to sensitive operators.",
        )];
    }

    if snapshot.access_events.is_empty() {
        return vec![partials::StatusCardItem::text(
            "status",
            "No access decisions have been recorded yet.",
        )];
    }

    snapshot
        .access_events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            partials::StatusCardItem::text(
                format!("event_{}", index + 1),
                format!(
                    "{} | {} | viewer {} | {} | {}",
                    event.outcome,
                    event.capability,
                    event_actor(event),
                    event
                        .record_id
                        .map(|record_id| record_id.as_ref().to_string())
                        .unwrap_or_else(|| "none".to_string()),
                    event.detail
                ),
            )
        })
        .collect()
}

fn format_capabilities(capabilities: &[domain::sensitive::AccessCapability]) -> String {
    if capabilities.is_empty() {
        return "none".to_string();
    }

    capabilities
        .iter()
        .map(|capability| capability.as_ref().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_key_counts(counts: &[domain::sensitive::KeyedCiphertextCount]) -> String {
    if counts.is_empty() {
        return "none".to_string();
    }

    counts
        .iter()
        .map(|count| format!("{}: {}", count.key_id, count.count))
        .collect::<Vec<_>>()
        .join(", ")
}

fn event_actor(event: &domain::sensitive::AccessEvent) -> String {
    event.user_id
        .map(|user_id| user_id.as_ref().to_string())
        .unwrap_or_else(|| "guest".to_string())
}

fn format_proof_time(value: std::time::SystemTime) -> String {
    let time = time::OffsetDateTime::from(value);
    let format = time::format_description::parse(
        "[year]-[month]-[day] [hour repr:24 padding:zero]:[minute padding:zero]",
    )
    .unwrap_or_else(|_| Vec::new());
    time.format(&format).unwrap_or_else(|_| "--:--".to_string())
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
            .maybe_token(Some(
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
            ))
            .maybe_latest_sync(Some(
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
            ))
            .maybe_integration_state(Some(
                domain::sensitive::IntegrationState::builder()
                    .provider(domain::sensitive::Provider::SyntheticSecureFeed)
                    .mode(domain::sensitive::ProviderMode::LocalStub)
                    .endpoint(
                        domain::sensitive::DetailText::try_new(
                            "http://127.0.0.1:4002/",
                        )
                        .expect("detail"),
                    )
                    .maybe_auth_mode(Some(
                        domain::sensitive::ProviderAuthMode::StubIssuedToken,
                    ))
                    .maybe_cursor(Some(
                        domain::sensitive::SyncCursor::try_new("cursor-gamma")
                            .expect("cursor"),
                    ))
                    .last_fetch_outcome(domain::sensitive::FetchOutcome::Success)
                    .maybe_last_auth_outcome(Some(
                        domain::sensitive::FetchOutcome::Success,
                    ))
                    .token_strategy(
                        domain::sensitive::TokenStrategy::RetryAfterUnauthorized,
                    )
                    .maybe_last_error_category(Some(
                        domain::sensitive::RemoteErrorCategory::Unauthorized,
                    ))
                    .maybe_last_remote_status_code(Some(401))
                    .maybe_retry_backoff_secs(Some(45))
                    .maybe_last_successful_mode(Some(
                        domain::sensitive::ProviderMode::LocalStub,
                    ))
                    .maybe_last_successful_fetch_at(Some(UNIX_EPOCH))
                    .last_attempted_fetch_at(UNIX_EPOCH)
                    .failure_count(1)
                    .build(),
            ))
            .maybe_key_custody(Some(
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
                    .maybe_last_rotation_run(Some(
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
                    ))
                    .build(),
            ))
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
            .maybe_integration_state(Some(
                domain::sensitive::IntegrationState::builder()
                    .provider(domain::sensitive::Provider::SyntheticSecureFeed)
                    .mode(domain::sensitive::ProviderMode::SandboxHttp)
                    .endpoint(
                        domain::sensitive::DetailText::try_new(
                            "https://sandbox.example.test/",
                        )
                        .expect("detail"),
                    )
                    .maybe_auth_mode(Some(
                        domain::sensitive::ProviderAuthMode::ClientCredentials,
                    ))
                    .maybe_cursor(Some(
                        domain::sensitive::SyncCursor::try_new("cursor-sandbox")
                            .expect("cursor"),
                    ))
                    .last_fetch_outcome(domain::sensitive::FetchOutcome::Failed)
                    .maybe_last_auth_outcome(Some(
                        domain::sensitive::FetchOutcome::Failed,
                    ))
                    .token_strategy(
                        domain::sensitive::TokenStrategy::RefreshedToken,
                    )
                    .maybe_last_error_category(Some(
                        domain::sensitive::RemoteErrorCategory::Forbidden,
                    ))
                    .maybe_last_remote_status_code(Some(403))
                    .maybe_retry_backoff_secs(Some(45))
                    .maybe_last_successful_mode(Some(
                        domain::sensitive::ProviderMode::LocalStub,
                    ))
                    .maybe_last_successful_fetch_at(None)
                    .last_attempted_fetch_at(UNIX_EPOCH)
                    .failure_count(2)
                    .build(),
            ))
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
        assert!(markup.contains("403"));
        assert!(markup.contains("retry_backoff_secs"));
        assert!(markup.contains("local_stub"));
    }
}
