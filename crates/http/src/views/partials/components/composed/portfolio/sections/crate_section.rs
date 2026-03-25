mod card;
mod links;
mod showcase;
mod switcher;

use maud::Render;

use self::card::Crate as Card;
use self::showcase::Crate as Showcase;
use self::switcher::CrateShowcase as ShowcaseSwitcher;
use super::{SectionCopy, Surface};
use crate::views::partials::components::portfolio::content::{CrateCardContent, CrateSectionContent};

pub struct CrateSection<'a> {
    pub content: &'a CrateSectionContent,
    pub show_heading: bool,
}

impl Render for CrateSection<'_> {
    fn render(&self) -> maud::Markup {
        let uses_showcases = crate_section_uses_showcases(&self.content.cards);
        let uses_switcher = uses_showcases && self.content.cards.len() > 1;
        let surface = if uses_showcases {
            Surface::section(maud::html! {
                @if self.show_heading {
                    (SectionCopy {
                        title: &self.content.title,
                        subtitle: &self.content.subtitle,
                    })
                }
                @if uses_switcher {
                    (ShowcaseSwitcher {
                        cards: &self.content.cards,
                    })
                } @else {
                    div class="ui-portfolio-showcase-stack" {
                        @for card in &self.content.cards {
                            (Showcase { card })
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
