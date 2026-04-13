use crate::views::partials;

use super::ProofCard;
use super::super::format::{format_key_counts, format_proof_time};

pub(super) fn token(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    let items = if !snapshot.viewer.allows_token_status() {
        vec![partials::StatusCardItem::text(
            "access",
            "Token lifecycle state is restricted to sensitive operators.",
        )]
    } else {
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
                partials::StatusCardItem::text(
                    "ciphertext_bytes",
                    token.ciphertext.bytes.to_string(),
                ),
                partials::StatusCardItem::text(
                    "ciphertext_preview",
                    token.ciphertext.preview.clone(),
                ),
            ],
            None => vec![partials::StatusCardItem::text(
                "status",
                "No provider token persisted yet.",
            )],
        }
    };

    ProofCard::new("Provider token", items)
}

pub(super) fn sync(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    let items = if !snapshot.viewer.allows_token_status() {
        vec![partials::StatusCardItem::text(
            "access",
            "Latest sync state is restricted to sensitive operators.",
        )]
    } else {
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
                partials::StatusCardItem::text(
                    "finished_at",
                    format_proof_time(run.finished_at),
                ),
            ],
            None => vec![partials::StatusCardItem::text(
                "status",
                "No sync run has completed yet.",
            )],
        }
    };

    ProofCard::new("Latest sync", items)
}

pub(super) fn boundary(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    let items = if !snapshot.viewer.allows_token_status() {
        vec![partials::StatusCardItem::text(
            "access",
            "Provider boundary state is restricted to sensitive operators.",
        )]
    } else {
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
    };

    ProofCard::new("Boundary state", items)
}

pub(super) fn key_custody(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    let items = if !snapshot.viewer.allows_token_status() {
        vec![partials::StatusCardItem::text(
            "access",
            "Key custody state is restricted to sensitive operators.",
        )]
    } else {
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
    };

    ProofCard::new("Key custody", items)
}
