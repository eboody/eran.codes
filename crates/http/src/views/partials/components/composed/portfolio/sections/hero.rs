use maud::Render;

use super::render_actions;
use crate::views::partials::components::portfolio::content::PortfolioHeroContent;

pub struct PortfolioHero<'a> {
    pub content: &'a PortfolioHeroContent,
}

impl Render for PortfolioHero<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="ui-portfolio-hero ui-surface-card" {
                p class="ui-portfolio-eyebrow" { (&self.content.eyebrow) }
                h1 { (&self.content.title) }
                p class="ui-portfolio-summary" { (&self.content.summary) }
                ul class="ui-portfolio-badges" {
                    @for badge in &self.content.badges {
                        li { (badge) }
                    }
                }
                (render_actions(&self.content.actions))
            }
        }
    }
}
