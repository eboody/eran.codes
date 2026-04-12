use maud::Render;

use crate::views::partials::components::portfolio::content::CrateCardContent;

pub struct FlagshipCrateHeroAside<'a> {
    pub card: &'a CrateCardContent,
}

impl Render for FlagshipCrateHeroAside<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p class="ui-portfolio-hero-aside-kicker" { "Flagship crate" }
            h2 { (&self.card.name) }
            p class="ui-portfolio-card-summary" { (&self.card.summary) }
            @if !self.card.tags.is_empty() {
                ul class="ui-portfolio-badges" {
                    @for tag in &self.card.tags {
                        li { (tag) }
                    }
                }
            }
            nav class="ui-portfolio-inline-links" aria-label=(format!("{} links", self.card.name)) {
                a
                    class="ui-portfolio-inline-link"
                    href=(self.card.repository_url)
                    target="_blank"
                    rel="noopener noreferrer" {
                    (&self.card.repository_label)
                }
                @if let (Some(docs_url), Some(docs_label)) = (&self.card.docs_url, &self.card.docs_label) {
                    a
                        class="ui-portfolio-inline-link"
                        href=(docs_url)
                        target="_blank"
                        rel="noopener noreferrer" {
                        (docs_label)
                    }
                }
            }
        }
    }
}
