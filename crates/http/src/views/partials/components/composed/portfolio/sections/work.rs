use maud::Render;

use super::{LeadCopy, SectionCopy, Surface, render_actions};
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::portfolio::content::{
    ClosingContent, WorkCardContent, WorkIndexContent, WorkSectionContent,
};

pub struct WorkSection<'a> {
    pub content: &'a WorkSectionContent,
}

impl Render for WorkSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                (WorkCards { cards: &self.content.cards })
            }))
        }
    }
}

pub struct WorkIndexSection<'a> {
    pub content: &'a WorkIndexContent,
}

impl Render for WorkIndexSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (LeadCopy {
                    eyebrow: &self.content.eyebrow,
                    title: &self.content.title,
                    summary: &self.content.summary,
                })
            }).extra_class("ui-portfolio-lead-surface"))
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.cases_title,
                    subtitle: &self.content.cases_subtitle,
                })
                (WorkCards { cards: &self.content.cases })
            }).extra_class("ui-portfolio-case-section"))
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
                (render_actions(&self.content.actions))
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
            div class="ui-portfolio-card-grid" {
                @for card in self.cards {
                    (WorkCard { content: card })
                }
            }
        }
    }
}

struct WorkCard<'a> {
    content: &'a WorkCardContent,
}

impl Render for WorkCard<'_> {
    fn render(&self) -> maud::Markup {
        let route = self.content.slug.route();

        maud::html! {
            article class="ui-portfolio-card ui-portfolio-work-card u-inset-card" {
                p class="ui-portfolio-card-kicker" { (&self.content.category) }
                h3 { (&self.content.title) }
                @if let Some(outcome) = &self.content.outcome {
                    p class="ui-portfolio-card-outcome" {
                        span class="ui-portfolio-card-outcome-label" { "Outcome" }
                        span class="ui-portfolio-card-outcome-text" { (outcome) }
                    }
                }
                p class="ui-portfolio-card-summary" { (&self.content.summary) }
                div class="ui-portfolio-work-card-footer" {
                    (partials::button::Button::builder()
                        .label(self.content.cta_label.clone())
                        .variant(partials::button::Variant::Secondary)
                        .role(partials::button::Role::link(Text::from(route.to_string())))
                        .build())
                }
            }
        }
    }
}
