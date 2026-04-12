use maud::Render;

use super::{CardFooter, CardGrid, InsetCard, SectionActions, SectionCopy, Surface};
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::portfolio::content::{
    WorkCardContent, WorkSectionContent,
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

        let section_copy = maud::html! {
            (SectionCopy {
                title: &self.content.title,
                subtitle: &self.content.subtitle,
            })
        };
        let section_actions = (!self.content.actions.is_empty()).then(|| {
            maud::html! {
                (SectionActions {
                    actions: &self.content.actions,
                })
            }
        });
        let cards = maud::html! {
            (Cards { cards: &self.content.cards })
        };

        maud::html! {
            (Surface::section(maud::html! {
                @match self.variant {
                    SectionVariant::Standard => {
                        (section_copy)
                        (cards)
                        @if let Some(section_actions) = section_actions {
                            (section_actions)
                        }
                    }
                    SectionVariant::CurrentProof => {
                        div class="ui-portfolio-work-section-rail" {
                            (section_copy)
                            @if let Some(section_actions) = section_actions {
                                (section_actions)
                            }
                        }
                        (cards)
                    }
                }
            }).extra_class(extra_class))
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
