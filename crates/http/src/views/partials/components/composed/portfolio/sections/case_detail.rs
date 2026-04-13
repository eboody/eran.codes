mod section_card;
#[cfg(test)]
mod tests;

use maud::Render;

use crate::types::Text;
use crate::views::partials::components::portfolio::content::{
    WorkCaseContent, WorkCaseDetailLayout,
};
use crate::views::partials::components::portfolio::content::model::CaseListSection;

use super::{CardGrid, InsetCard, PortfolioHero};
use section_card::{CaseSectionCard, ListKind};

const ARCHIVE_GRID_ORDER: [CaseSectionKind; 4] = [
    CaseSectionKind::Challenge,
    CaseSectionKind::Implementation,
    CaseSectionKind::Outcomes,
    CaseSectionKind::Stack,
];
const CURRENT_PROOF_MAIN_ORDER: [CaseSectionKind; 2] =
    [CaseSectionKind::Outcomes, CaseSectionKind::Implementation];
const CURRENT_PROOF_RAIL_ORDER: [CaseSectionKind; 2] =
    [CaseSectionKind::Challenge, CaseSectionKind::Stack];

pub struct Work<'a> {
    pub content: &'a WorkCaseContent,
}

impl Render for Work<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-portfolio-hero-flow ui-portfolio-work-case-flow" {
                (PortfolioHero {
                    content: &self.content.hero,
                    aside: None,
                })
                (case_sections(self.content))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CaseSectionKind {
    Challenge,
    Implementation,
    Outcomes,
    Stack,
}

#[derive(Clone, Copy, Debug)]
struct CaseSectionRef<'a> {
    kind: CaseSectionKind,
    content: &'a CaseListSection,
}

impl CaseSectionRef<'_> {
    fn title(&self) -> &Text {
        &self.content.title
    }

    fn items(&self) -> &[Text] {
        &self.content.items
    }
}

fn case_sections(content: &WorkCaseContent) -> maud::Markup {
    match content.detail_layout {
        WorkCaseDetailLayout::CurrentProof => current_proof_sections(content),
        WorkCaseDetailLayout::ArchiveGrid => archive_grid_sections(content),
    }
}

fn section_ref<'a>(content: &'a WorkCaseContent, kind: CaseSectionKind) -> CaseSectionRef<'a> {
    let section = match kind {
        CaseSectionKind::Challenge => &content.challenge,
        CaseSectionKind::Implementation => &content.implementation,
        CaseSectionKind::Outcomes => &content.outcomes,
        CaseSectionKind::Stack => &content.stack,
    };

    CaseSectionRef { kind, content: section }
}

fn section_refs<'a, const N: usize>(
    content: &'a WorkCaseContent,
    kinds: [CaseSectionKind; N],
) -> [CaseSectionRef<'a>; N] {
    kinds.map(|kind| section_ref(content, kind))
}

fn archive_grid_sections(content: &WorkCaseContent) -> maud::Markup {
    let sections = section_refs(content, ARCHIVE_GRID_ORDER);

    maud::html! {
        (CardGrid::new(maud::html! {
            @for section in sections {
                (CaseSectionCard::new(section)
                    .extra_class("ui-portfolio-case-section"))
            }
        }).extra_class("ui-portfolio-case-grid"))
    }
}

fn current_proof_sections(content: &WorkCaseContent) -> maud::Markup {
    let main_sections = section_refs(content, CURRENT_PROOF_MAIN_ORDER);
    let rail_sections = section_refs(content, CURRENT_PROOF_RAIL_ORDER);

    maud::html! {
        section class="ui-portfolio-current-proof-detail" {
            div class="ui-portfolio-current-proof-main" {
                @for section in main_sections {
                    (render_current_proof_main_card(section))
                }
            }
            aside class="ui-portfolio-current-proof-rail" {
                @for section in rail_sections {
                    (render_current_proof_rail_card(section))
                }
            }
        }
    }
}

fn render_current_proof_main_card(section: CaseSectionRef<'_>) -> CaseSectionCard<'_> {
    let extra_class = match section.kind {
        CaseSectionKind::Outcomes => "ui-portfolio-case-section ui-portfolio-case-section--lead",
        CaseSectionKind::Implementation => "ui-portfolio-case-section",
        CaseSectionKind::Challenge | CaseSectionKind::Stack => unreachable!(
            "current proof main sections should only render outcomes and implementation"
        ),
    };

    CaseSectionCard::new(section).extra_class(extra_class)
}

fn render_current_proof_rail_card(section: CaseSectionRef<'_>) -> CaseSectionCard<'_> {
    let card = CaseSectionCard::new(section)
        .extra_class("ui-portfolio-case-section ui-portfolio-current-proof-rail-card");

    match section.kind {
        CaseSectionKind::Challenge => card.title_override("Boundary and scope"),
        CaseSectionKind::Stack => card.list_kind(ListKind::Badges).extra_class(
            "ui-portfolio-case-section ui-portfolio-current-proof-rail-card",
        ),
        CaseSectionKind::Implementation | CaseSectionKind::Outcomes => unreachable!(
            "current proof rail sections should only render challenge and stack"
        ),
    }
}
