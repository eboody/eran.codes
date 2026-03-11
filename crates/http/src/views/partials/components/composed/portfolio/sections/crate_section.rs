use maud::Render;

use super::{SectionCopy, Surface};
use crate::views::partials::button;
use crate::views::partials::components::portfolio::content::CrateSectionContent;

pub struct CrateSection<'a> {
    pub content: &'a CrateSectionContent,
}

impl Render for CrateSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
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
                                (button::Button::builder()
                                    .label(card.repository_label.clone())
                                    .variant(button::Variant::Secondary)
                                    .role(button::Role::external_link(card.repository_url.clone()))
                                    .build())
                                @if let (Some(docs_url), Some(docs_label)) = (&card.docs_url, &card.docs_label) {
                                    (button::Button::builder()
                                        .label(docs_label.clone())
                                        .variant(button::Variant::Secondary)
                                        .role(button::Role::external_link(docs_url.clone()))
                                        .build())
                                }
                            }
                        }
                    }
                }
            }))
        }
    }
}
