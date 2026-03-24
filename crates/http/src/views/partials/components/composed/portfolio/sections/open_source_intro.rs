use maud::Render;

use crate::views::partials::components::portfolio::content::CrateSectionContent;

pub struct OpenSourceHeroAside<'a> {
    pub crate_section: &'a CrateSectionContent,
}

impl Render for OpenSourceHeroAside<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-open-source-hero-aside" {
                div class="ui-open-source-hero-intro" {
                    p class="ui-portfolio-hero-aside-kicker" { "Library proof" }
                    h2 { "Three crates. One invariants-first through-line." }
                    p class="ui-portfolio-card-summary" {
                        "Typestate, nested-enum ergonomics, and namespace discipline packaged as reusable Rust APIs."
                    }
                }
                ul class="ui-open-source-hero-list" {
                    @for card in self.crate_section.cards.iter().take(3) {
                        li class="ui-open-source-hero-item" {
                            strong { (&card.name) }
                            div class="ui-open-source-hero-item-tags" {
                                @for tag in card.tags.iter().take(2) {
                                    span class="ui-open-source-hero-item-tag" { (tag) }
                                }
                            }
                        }
                    }
                }
                p class="ui-open-source-hero-footnote" {
                    strong { "What to inspect" }
                    " Read the API, then check the code and docs against the same published surface."
                }
            }
        }
    }
}

pub struct OpenSourceMobileIntro<'a> {
    pub crate_section: &'a CrateSectionContent,
}

impl Render for OpenSourceMobileIntro<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-open-source-mobile-intro" {
                p class="ui-open-source-mobile-intro-eyebrow" { "Crate walkthrough" }
                p class="ui-portfolio-summary" {
                    (&self.crate_section.subtitle)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::open_source_index_content;

    #[test]
    fn renders_open_source_hero_support_components() {
        let content = open_source_index_content();

        let aside_markup = OpenSourceHeroAside {
            crate_section: &content.crate_section,
        }
        .render()
        .into_string();
        let mobile_markup = OpenSourceMobileIntro {
            crate_section: &content.crate_section,
        }
        .render()
        .into_string();

        assert!(aside_markup.contains("ui-open-source-hero-aside"));
        assert!(aside_markup.contains("Three crates. One invariants-first through-line."));
        assert!(aside_markup.contains("ui-open-source-hero-item-tag"));
        assert!(mobile_markup.contains("ui-open-source-mobile-intro"));
        assert!(mobile_markup.contains(content.crate_section.subtitle.to_string().as_str()));
    }
}
