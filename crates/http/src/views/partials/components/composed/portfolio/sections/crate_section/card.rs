use maud::Render;

use super::super::InsetCard;
use super::links::CrateLinks;
use crate::views::partials::components::portfolio::content::CrateCardContent;

pub(super) struct CrateCard<'a> {
    pub card: &'a CrateCardContent,
}

impl Render for CrateCard<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (InsetCard::new(maud::html! {
                h3 { (&self.card.name) }
                p class="ui-portfolio-card-summary" { (&self.card.summary) }
                ul class="ui-portfolio-list" {
                    @for item in &self.card.highlights {
                        li { (item) }
                    }
                }
                @if !self.card.tags.is_empty() {
                    ul class="ui-portfolio-badges" {
                        @for tag in &self.card.tags {
                            li { (tag) }
                        }
                    }
                }
                (CrateLinks { card: self.card })
            }))
        }
    }
}
