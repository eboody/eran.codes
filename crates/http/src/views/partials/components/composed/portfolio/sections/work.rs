use maud::Render;

use super::{CardFooter, CardGrid, InsetCard, LeadCopy, SectionActions, SectionCopy, Surface};
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::portfolio::content::{
    ClosingContent, WorkCardContent, WorkIndexContent, WorkSectionContent,
};

#[derive(Clone, Copy, Debug, Default)]
pub enum WorkSectionVariant {
    #[default]
    Standard,
    CurrentProof,
}

pub struct WorkSection<'a> {
    pub content: &'a WorkSectionContent,
    pub variant: WorkSectionVariant,
}

impl Render for WorkSection<'_> {
    fn render(&self) -> maud::Markup {
        let extra_class = match self.variant {
            WorkSectionVariant::Standard => "ui-portfolio-work-section",
            WorkSectionVariant::CurrentProof => {
                "ui-portfolio-work-section ui-portfolio-work-section--current-proof"
            }
        };

        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                (WorkCards { cards: &self.content.cards })
                @if !self.content.actions.is_empty() {
                    (SectionActions {
                        actions: &self.content.actions,
                    })
                }
            }).extra_class(extra_class))
        }
    }
}

pub struct WorkIndexSection<'a> {
    pub content: &'a WorkIndexContent,
}

impl Render for WorkIndexSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-portfolio-work-index" {
                (Surface::section(maud::html! {
                    (LeadCopy {
                        eyebrow: &self.content.eyebrow,
                        title: &self.content.title,
                        summary: &self.content.summary,
                    })
                }).extra_class("ui-portfolio-lead-surface"))
                (WorkSection {
                    content: &self.content.current_proof_section,
                    variant: WorkSectionVariant::CurrentProof,
                })
                div data-work-supporting-proof {
                    (WorkSection {
                        content: &self.content.supporting_cases_section,
                        variant: WorkSectionVariant::Standard,
                    })
                }
            }
        }
    }
}

pub struct SupportingTeaserSection<'a> {
    pub content: &'a ClosingContent,
}

impl Render for SupportingTeaserSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                h2 { (&self.content.title) }
                p class="ui-portfolio-summary" { (&self.content.summary) }
                (SectionActions {
                    actions: &self.content.actions,
                })
            }).extra_class("ui-portfolio-supporting-teaser"))
        }
    }
}

struct WorkCards<'a> {
    cards: &'a [WorkCardContent],
}

impl Render for WorkCards<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (CardGrid::new(maud::html! {
                @for card in self.cards {
                    (WorkCard { content: card })
                }
            }))
        }
    }
}

struct WorkCard<'a> {
    content: &'a WorkCardContent,
}

impl Render for WorkCard<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (InsetCard::new(maud::html! {
                p class="ui-portfolio-card-kicker" { (&self.content.category) }
                h3 { (&self.content.title) }
                @if let Some(outcome) = &self.content.outcome {
                    p class="ui-portfolio-card-outcome" {
                        span class="ui-portfolio-card-outcome-label" { "Outcome" }
                        span class="ui-portfolio-card-outcome-text" { (outcome) }
                    }
                }
                p class="ui-portfolio-card-summary" { (&self.content.summary) }
                (CardFooter::new(maud::html! {
                    (partials::button::Button::builder()
                        .label(self.content.cta_label.clone())
                        .variant(partials::button::Variant::Secondary)
                        .role(partials::button::Role::link(Text::from(self.content.slug.public_href())))
                        .build())
                }))
            }).extra_class("ui-portfolio-work-card"))
        }
    }
}
