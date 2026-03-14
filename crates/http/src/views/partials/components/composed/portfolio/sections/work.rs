use maud::Render;

use super::{LeadCopy, SectionCopy, Surface, render_actions};
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::portfolio::content::{
    WorkCardContent, WorkIndexContent, WorkSectionContent,
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
                @if !self.content.actions.is_empty() {
                    div class="ui-portfolio-section-actions" {
                        (render_actions(&self.content.actions))
                    }
                }
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
                    title: &self.content.title,
                    subtitle: &self.content.summary,
                })
                (WorkCards {
                    cards: &self.content.cases,
                })
                @if !self.content.actions.is_empty() {
                    div class="ui-portfolio-section-actions" {
                        (render_actions(&self.content.actions))
                    }
                }
            }))
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
            article class="ui-portfolio-card" {
                p class="ui-portfolio-card-kicker" { (&self.content.category) }
                h3 { (&self.content.title) }
                p class="ui-portfolio-card-summary" { (&self.content.summary) }
                @if let Some(outcome) = &self.content.outcome {
                    p class="ui-portfolio-card-outcome" { "Outcome: " (outcome) }
                }
                @if let Some(preview) = &self.content.preview {
                    p class="ui-portfolio-card-preview" {
                        span class="ui-portfolio-preview-key" { (&preview.asset_ref) }
                        span class="ui-portfolio-preview-alt" { (&preview.alt) }
                    }
                }
                @if !self.content.stack_tags.is_empty() {
                    ul class="ui-portfolio-badges" {
                        @for tag in &self.content.stack_tags {
                            li { (tag) }
                        }
                    }
                }
                ul class="ui-portfolio-list" {
                    @for item in &self.content.highlights {
                        li { (item) }
                    }
                }
                (partials::button::Button::builder()
                    .label(self.content.cta_label.clone())
                    .variant(partials::button::Variant::Secondary)
                    .role(partials::button::Role::link(Text::from(route.to_string())))
                    .build())
            }
        }
    }
}
