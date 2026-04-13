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
mod tests;
