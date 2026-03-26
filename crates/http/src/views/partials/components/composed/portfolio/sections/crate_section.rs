mod card;
mod links;
mod showcase;

use maud::Render;

use self::card::{Crate as Card, SupportingCrate};
use self::showcase::Crate as Showcase;
use super::{CardGrid, SectionCopy, Surface};
use crate::views::partials::components::portfolio::content::{CrateCardContent, CrateSectionContent};

pub struct CrateSection<'a> {
    pub content: &'a CrateSectionContent,
    pub show_heading: bool,
}

impl Render for CrateSection<'_> {
    fn render(&self) -> maud::Markup {
        let uses_showcases = crate_section_uses_showcases(&self.content.cards);
        let surface = if uses_showcases {
            Surface::section(maud::html! {
                @if self.show_heading {
                    (SectionCopy {
                        title: &self.content.title,
                        subtitle: &self.content.subtitle,
                    })
                }
                @if let Some((lead_card, supporting_cards)) = self.content.cards.split_first() {
                    div class="ui-portfolio-showcase-stack" {
                        (Showcase { card: lead_card })
                    }
                    @if !supporting_cards.is_empty() {
                        div class="ui-open-source-supporting-libraries" {
                            p class="ui-open-source-supporting-label" { "Also published" }
                            (CardGrid::new(maud::html! {
                                @for card in supporting_cards {
                                    (SupportingCrate { card })
                                }
                            })
                            .extra_class("ui-open-source-supporting-grid"))
                        }
                    }
                }
            })
            .extra_class(crate_section_surface_class(self.show_heading))
        } else {
            Surface::section(maud::html! {
                @if self.show_heading {
                    (SectionCopy {
                        title: &self.content.title,
                        subtitle: &self.content.subtitle,
                    })
                }
                div class="ui-portfolio-card-grid" {
                    @for card in &self.content.cards {
                        (Card { card })
                    }
                }
            })
        };

        maud::html! {
            (surface)
        }
    }
}

fn crate_section_uses_showcases(cards: &[CrateCardContent]) -> bool {
    cards.iter().any(|card| card.gallery.is_some())
}

fn crate_section_surface_class(show_heading: bool) -> &'static str {
    if show_heading {
        "ui-portfolio-crate-section"
    } else {
        "ui-portfolio-crate-section ui-portfolio-crate-section--standalone"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::open_source_index_content;

    #[test]
    fn showcase_sections_render_lead_showcase_and_supporting_cards() {
        let content = open_source_index_content();
        let markup = CrateSection {
            content: &content.crate_section,
            show_heading: false,
        }
        .render()
        .into_string();

        assert!(markup.contains("ui-portfolio-crate-showcase"));
        assert!(markup.contains("Also published"));
        assert!(markup.contains("ui-open-source-supporting-card"));
        assert!(!markup.contains("data-portfolio-crate-switcher"));
    }
}
