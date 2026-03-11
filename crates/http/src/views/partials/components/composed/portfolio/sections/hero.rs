use maud::Render;

use super::{LeadCopy, Surface, render_actions};
use crate::views::partials::components::portfolio::content::PortfolioHeroContent;

pub struct PortfolioHero<'a> {
    pub content: &'a PortfolioHeroContent,
}

impl Render for PortfolioHero<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::header(maud::html! {
                (LeadCopy {
                    eyebrow: &self.content.eyebrow,
                    title: &self.content.title,
                    summary: &self.content.summary,
                })
                ul class="ui-portfolio-badges" {
                    @for badge in &self.content.badges {
                        li { (badge) }
                    }
                }
                @if !self.content.actions.is_empty() {
                    (render_actions(&self.content.actions))
                }
            }).extra_class("ui-portfolio-lead-surface ui-portfolio-hero"))
        }
    }
}
