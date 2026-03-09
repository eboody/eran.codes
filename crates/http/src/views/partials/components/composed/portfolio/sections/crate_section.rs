use maud::Render;

use super::SectionCopy;
use crate::views::partials::components::portfolio::content::CrateSectionContent;

pub struct CrateSection<'a> {
    pub content: &'a CrateSectionContent,
}

impl Render for CrateSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-surface-card" {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                div class="ui-portfolio-card-grid" {
                    @for card in &self.content.cards {
                        article class="ui-portfolio-card" {
                            h3 { (&card.name) }
                            p class="ui-portfolio-card-summary" { (&card.summary) }
                            @if !card.tags.is_empty() {
                                ul class="ui-portfolio-badges" {
                                    @for tag in &card.tags {
                                        li { (tag) }
                                    }
                                }
                            }
                            div class="ui-portfolio-card-links" {
                                a class="button secondary" href=(&card.repository_url) target="_blank" rel="noopener noreferrer" {
                                    (&card.repository_label)
                                }
                                @if let (Some(docs_url), Some(docs_label)) = (&card.docs_url, &card.docs_label) {
                                    a class="button secondary" href=(docs_url) target="_blank" rel="noopener noreferrer" {
                                        (docs_label)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
