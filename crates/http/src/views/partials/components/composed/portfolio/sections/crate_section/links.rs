use maud::Render;

use crate::views::partials::components::portfolio::content::CrateCardContent;

pub(super) struct Crate<'a> {
    pub card: &'a CrateCardContent,
}

impl Render for Crate<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
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
