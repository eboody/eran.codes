use maud::Render;

use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::portfolio::content::WorkCardContent;

pub struct CurrentProofHeroAside<'a> {
    pub card: &'a WorkCardContent,
}

impl Render for CurrentProofHeroAside<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p class="ui-portfolio-hero-aside-kicker" { (&self.card.category) }
            h2 { (&self.card.title) }
            @if let Some(outcome) = &self.card.outcome {
                p class="ui-portfolio-hero-aside-outcome" {
                    span class="ui-portfolio-hero-aside-outcome-label" { "Outcome" }
                    span class="ui-portfolio-hero-aside-outcome-text" { (outcome) }
                }
            }
            p class="ui-portfolio-card-summary" { (&self.card.summary) }
            (partials::button::Button::builder()
                .label(self.card.cta_label.clone())
                .variant(partials::button::Variant::Secondary)
                .role(partials::button::Role::link(Text::from(self.card.slug.public_href())))
                .build())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::portfolio_home_content;

    #[test]
    fn renders_current_proof_aside_from_shared_card_content() {
        let content = portfolio_home_content();
        let card = content
            .current_proof_section
            .cards
            .first()
            .expect("home current proof should expose a hero card");
        let markup = CurrentProofHeroAside { card }.render().into_string();

        assert!(markup.contains(card.title.to_string().as_str()));
        assert!(markup.contains(card.summary.to_string().as_str()));
        assert!(markup.contains(card.cta_label.to_string().as_str()));
    }
}
