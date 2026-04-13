mod card;
mod links;
mod showcase;

use maud::Render;

use self::card::{Crate as Card, SupportingCrate};
use self::showcase::Crate as Showcase;
use super::{CardGrid, SectionCopy, Surface};
use crate::views::partials::components::portfolio::content::{
    CrateCardContent, CrateSectionContent,
};

#[derive(Clone, Copy, Debug)]
enum HeadingMode {
    Hidden,
    Visible,
}

#[derive(Clone, Debug)]
enum CardPresentation<'a> {
    Showcase {
        lead_card: &'a CrateCardContent,
        supporting_cards: Vec<&'a CrateCardContent>,
    },
    Grid(&'a [CrateCardContent]),
}

pub struct CrateSection<'a> {
    content: &'a CrateSectionContent,
    heading_mode: HeadingMode,
}

impl<'a> CrateSection<'a> {
    pub fn standalone(content: &'a CrateSectionContent) -> Self {
        Self {
            content,
            heading_mode: HeadingMode::Hidden,
        }
    }

    #[allow(dead_code)]
    pub fn with_heading(content: &'a CrateSectionContent) -> Self {
        Self {
            content,
            heading_mode: HeadingMode::Visible,
        }
    }

    fn render_heading(&self) -> Option<maud::Markup> {
        matches!(self.heading_mode, HeadingMode::Visible).then(|| {
            maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
            }
        })
    }

    fn presentation(&self) -> CardPresentation<'a> {
        if let Some((lead_index, lead_card)) = self
            .content
            .cards
            .iter()
            .enumerate()
            .find(|(_, card)| card.gallery.is_some())
        {
            let supporting_cards = self
                .content
                .cards
                .iter()
                .enumerate()
                .filter_map(|(index, card)| (index != lead_index).then_some(card))
                .collect();

            CardPresentation::Showcase {
                lead_card,
                supporting_cards,
            }
        } else {
            CardPresentation::Grid(&self.content.cards)
        }
    }

    fn surface_class(&self, presentation: &CardPresentation<'_>) -> Option<&'static str> {
        match presentation {
            CardPresentation::Showcase { .. } => Some(match self.heading_mode {
                HeadingMode::Hidden => {
                    "ui-portfolio-crate-section ui-portfolio-crate-section--standalone"
                }
                HeadingMode::Visible => "ui-portfolio-crate-section",
            }),
            CardPresentation::Grid(_) => None,
        }
    }
}

impl Render for CrateSection<'_> {
    fn render(&self) -> maud::Markup {
        let presentation = self.presentation();
        let surface_class = self.surface_class(&presentation);
        let surface = Surface::section(maud::html! {
            @if let Some(heading) = self.render_heading() {
                (heading)
            }
            @match presentation {
                CardPresentation::Showcase {
                    lead_card,
                    supporting_cards,
                } => {
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
                CardPresentation::Grid(cards) => {
                    div class="ui-portfolio-card-grid" {
                        @for card in cards {
                            (Card { card })
                        }
                    }
                }
            }
        });
        let surface = match surface_class {
            Some(extra_class) => surface.extra_class(extra_class),
            None => surface,
        };

        maud::html! {
            (surface)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Text;
    use crate::views::partials::components::portfolio::content::open_source_index_content;

    #[test]
    fn showcase_sections_render_lead_showcase_and_supporting_cards() {
        let content = open_source_index_content();
        let markup = CrateSection::standalone(&content.crate_section)
            .render()
            .into_string();

        assert!(markup.contains("ui-portfolio-crate-showcase"));
        assert!(markup.contains("Also published"));
        assert!(markup.contains("ui-open-source-supporting-card"));
        assert!(!markup.contains("data-portfolio-crate-switcher"));
    }

    #[test]
    fn grid_mode_renders_heading_without_showcase_surface_class() {
        let content = CrateSectionContent {
            title: Text::from("Open source"),
            subtitle: Text::from("Smaller utilities"),
            cards: vec![CrateCardContent {
                name: Text::from("nestum"),
                summary: Text::from("A rewrite helper"),
                highlights: vec![Text::from("Typed transforms")],
                gallery: None,
                repository_url: Text::from("https://example.com/repo"),
                repository_label: Text::from("Repository"),
                docs_url: None,
                docs_label: None,
                tags: vec![],
            }],
        };
        let markup = CrateSection::with_heading(&content).render().into_string();

        assert!(markup.contains("Open source"));
        assert!(markup.contains("ui-portfolio-card-grid"));
        assert!(!markup.contains("ui-portfolio-crate-section--standalone"));
        assert!(!markup.contains("ui-portfolio-crate-showcase"));
    }

    #[test]
    fn showcase_mode_uses_first_gallery_card_as_lead_even_if_not_first_in_list() {
        let source = open_source_index_content().crate_section.clone();
        let lead_card = source.cards[0].clone();
        let supporting_card = CrateCardContent {
            name: Text::from("supporting"),
            gallery: None,
            ..lead_card.clone()
        };
        let content = CrateSectionContent {
            title: source.title,
            subtitle: source.subtitle,
            cards: vec![supporting_card, lead_card.clone()],
        };
        let lead_summary = lead_card.summary.to_string();
        let supporting_name = content.cards[0].name.to_string();

        let markup = CrateSection::standalone(&content).render().into_string();

        assert!(markup.contains(supporting_name.as_str()));
        assert!(markup.contains(lead_summary.as_str()));
        assert!(
            markup.find(lead_summary.as_str()).unwrap() < markup.find(supporting_name.as_str()).unwrap()
        );
    }
}
