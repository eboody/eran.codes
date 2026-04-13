use crate::views::partials;

use super::ProofCard;
use super::super::format::{event_actor, format_capabilities};

pub(super) fn policy(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    ProofCard::new(
        "Access policy",
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
        ],
    )
}

pub(super) fn authorized(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    let items = match &snapshot.authorized_record {
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
    };

    ProofCard::new("Authorized sample", items)
}

pub(super) fn audit(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    let items = if !snapshot.viewer.allows_access_audit() {
        vec![partials::StatusCardItem::text(
            "access",
            "Recent access-audit evidence is restricted to sensitive operators.",
        )]
    } else if snapshot.access_events.is_empty() {
        vec![partials::StatusCardItem::text(
            "status",
            "No access decisions have been recorded yet.",
        )]
    } else {
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
    };

    ProofCard::new("Recent access audit", items)
}
