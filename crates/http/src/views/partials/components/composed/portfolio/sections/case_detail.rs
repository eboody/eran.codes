use maud::Render;

use crate::views::partials::components::portfolio::content::WorkCaseContent;

use super::render_actions;

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
            section class="ui-surface-card ui-portfolio-case-hero" {
                p class="ui-portfolio-eyebrow" { (&self.content.eyebrow) }
                h1 { (&self.content.title) }
                p class="ui-portfolio-summary" { (&self.content.summary) }
                (render_actions(&self.content.actions))
            }

            div class="ui-portfolio-case-grid" {
                @for (title, items) in sections {
                    article class="ui-surface-card ui-portfolio-case-section" {
                        h2 { (title) }
                        ul class="ui-portfolio-list" {
                            @for item in items {
                                li { (item) }
                            }
                        }
                    }
                }
            }
        }
    }
}
