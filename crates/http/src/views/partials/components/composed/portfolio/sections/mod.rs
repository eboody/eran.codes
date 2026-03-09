use maud::Render;

use crate::types::Text;
use crate::views::partials::{CtaItem, CtaLink, CtaRow, CtaTone};

use super::content::{CmsActionLink, CtaKind, ProofKind};

mod case_detail;
mod closing;
mod crate_section;
mod hero;
mod proof_strip;
mod work;

pub use case_detail::WorkCaseDetail;
pub use closing::ClosingSection;
pub use crate_section::CrateSection;
pub use hero::PortfolioHero;
pub use proof_strip::ProofStrip;
pub use work::{WorkIndexSection, WorkSection};

#[derive(Clone, Copy, Debug)]
pub(super) struct SectionCopy<'a> {
    pub title: &'a Text,
    pub subtitle: &'a Text,
}

impl Render for SectionCopy<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-portfolio-section-copy" {
                h2 { (&self.title) }
                p { (&self.subtitle) }
            }
        }
    }
}

pub(super) fn render_actions(actions: &[CmsActionLink]) -> CtaRow {
    CtaRow::builder()
        .items(
            actions
                .iter()
                .map(|action| {
                    CtaItem::Link(
                        CtaLink::builder()
                            .label(action.label.clone())
                            .href(action.href.clone())
                            .tone(match action.tone {
                                CtaKind::Primary => CtaTone::Primary,
                                CtaKind::Secondary => CtaTone::Secondary,
                            })
                            .external(action.kind.is_external())
                            .build(),
                    )
                })
                .collect(),
        )
        .build()
}

pub(super) fn proof_kind_attr(kind: ProofKind) -> &'static str {
    match kind {
        ProofKind::Outcome => "outcome",
        ProofKind::Architecture => "architecture",
        ProofKind::Reliability => "reliability",
    }
}
