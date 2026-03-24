use crate::views::partials;

use super::format::{
    event_actor,
    format_capabilities,
    format_key_counts,
    format_proof_time,
};

#[derive(Debug)]
pub(super) struct ProofCard {
    pub title: crate::types::Text,
    pub items: Vec<partials::StatusCardItem>,
}

impl ProofCard {
    fn new(title: &'static str, items: Vec<partials::StatusCardItem>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }
}

pub(super) fn proof_cards(snapshot: &app::sensitive::Snapshot) -> Vec<ProofCard> {
    vec![
        ProofCard::new("Access policy", policy_items(snapshot)),
        ProofCard::new("Provider token", token_items(snapshot)),
        ProofCard::new("Latest sync", sync_items(snapshot)),
        ProofCard::new("Boundary state", boundary_items(snapshot)),
        ProofCard::new("Key custody", key_custody_items(snapshot)),
        ProofCard::new("Redacted records", record_items(snapshot)),
        ProofCard::new("Authorized sample", authorized_items(snapshot)),
        ProofCard::new("Recent access audit", audit_items(snapshot)),
    ]
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
