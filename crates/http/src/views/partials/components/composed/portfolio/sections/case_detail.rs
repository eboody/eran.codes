use maud::Render;

use crate::views::partials::components::portfolio::content::WorkCaseContent;

use super::{LeadCopy, Surface, render_actions};

pub struct WorkCaseDetail<'a> {
    pub content: &'a WorkCaseContent,
}

impl Render for WorkCaseDetail<'_> {
    fn render(&self) -> maud::Markup {
        let sections = [
            (&self.content.challenge.title, &self.content.challenge.items),
            (
                &self.content.implementation.title,
                &self.content.implementation.items,
            ),
            (&self.content.outcomes.title, &self.content.outcomes.items),
            (&self.content.stack.title, &self.content.stack.items),
        ];

        maud::html! {
            (Surface::section(maud::html! {
                (LeadCopy {
                    eyebrow: &self.content.eyebrow,
                    title: &self.content.title,
                    summary: &self.content.summary,
                })
                @if !self.content.actions.is_empty() {
                    (render_actions(&self.content.actions))
                }
            }).extra_class("ui-portfolio-lead-surface"))

            div class="ui-portfolio-case-grid" {
                @for (title, items) in sections {
                    (Surface::article(maud::html! {
                        h2 { (title) }
                        ul class="ui-portfolio-list" {
                            @for item in items {
                                li { (item) }
                            }
                        }
                    }).extra_class("ui-portfolio-case-section"))
                }
            }
        }
    }
}
