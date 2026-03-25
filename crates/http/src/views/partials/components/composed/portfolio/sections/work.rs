use maud::Render;

use super::{CardFooter, CardGrid, InsetCard, LeadCopy, SectionActions, SectionCopy, Surface};
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::portfolio::content::{
    WorkCardContent, WorkIndexContent, WorkSectionContent,
};

#[derive(Clone, Copy, Debug, Default)]
pub enum SectionVariant {
    #[default]
    Standard,
    CurrentProof,
}

pub struct Section<'a> {
    pub content: &'a WorkSectionContent,
    pub variant: SectionVariant,
}

impl Render for Section<'_> {
    fn render(&self) -> maud::Markup {
        let extra_class = match self.variant {
            SectionVariant::Standard => "ui-portfolio-work-section",
            SectionVariant::CurrentProof => {
                "ui-portfolio-work-section ui-portfolio-work-section--current-proof"
            }
        };

        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                (Cards { cards: &self.content.cards })
                @if !self.content.actions.is_empty() {
                    (SectionActions {
                        actions: &self.content.actions,
                    })
                }
            }).extra_class(extra_class))
        }
    }
}

pub struct IndexSection<'a> {
    pub content: &'a WorkIndexContent,
}

impl Render for IndexSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-portfolio-work-index" {
                (Surface::section(maud::html! {
                    (LeadCopy {
                        eyebrow: &self.content.eyebrow,
                        title: &self.content.title,
                        summary: &self.content.summary,
                    })
                }).extra_class("ui-portfolio-lead-surface ui-portfolio-lead-surface--compact"))
                (Section {
                    content: &self.content.current_proof_section,
                    variant: SectionVariant::CurrentProof,
                })
                div data-work-supporting-proof {
                    (Section {
                        content: &self.content.supporting_cases_section,
                        variant: SectionVariant::Standard,
                    })
                }
            }
        }
    }
}

struct Cards<'a> {
    cards: &'a [WorkCardContent],
}

impl Render for Cards<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (CardGrid::new(maud::html! {
                @for card in self.cards {
                    (Card { content: card })
                }
            }))
        }
    }
}

struct Card<'a> {
    content: &'a WorkCardContent,
}

impl Render for Card<'_> {
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
