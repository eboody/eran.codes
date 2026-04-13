use crate::views::partials;

use super::ProofCard;

pub(super) fn redacted_records(snapshot: &app::sensitive::Snapshot) -> ProofCard {
    let items = if snapshot.records.is_empty() {
        vec![partials::StatusCardItem::text(
            "records",
            "No redacted records persisted yet.",
        )]
    } else {
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
    };

    ProofCard::new("Redacted records", items)
}
