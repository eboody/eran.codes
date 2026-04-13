mod access;
mod operator;
mod records;

use crate::views::partials;

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
        access::policy(snapshot),
        operator::token(snapshot),
        operator::sync(snapshot),
        operator::boundary(snapshot),
        operator::key_custody(snapshot),
        records::redacted_records(snapshot),
        access::authorized(snapshot),
        access::audit(snapshot),
    ]
}
