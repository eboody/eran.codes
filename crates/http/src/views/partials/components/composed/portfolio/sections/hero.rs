use maud::Render;

use super::{LeadCopy, Surface, render_actions};
use crate::views::partials::components::portfolio::content::PortfolioHeroContent;

// ci: markup-slot-exempt portfolio hero aside accepts page-specific support markup.
pub struct Portfolio<'a> {
    pub content: &'a PortfolioHeroContent,
    pub aside: Option<maud::Markup>,
}

impl Render for Portfolio<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::header(maud::html! {
                div class="ui-portfolio-hero-grid" {
                    div class="ui-portfolio-hero-main" {
                        (LeadCopy {
                            eyebrow: &self.content.eyebrow,
                            title: &self.content.title,
                            summary: &self.content.summary,
                        })
                        @if !self.content.badges.is_empty() {
                            ul class="ui-portfolio-badges" {
                                @for badge in &self.content.badges {
                                    li { (badge) }
                                }
                            }
                        }
                        @if !self.content.actions.is_empty() {
                            (render_actions(&self.content.actions))
                        }
                    }

                    @if let Some(aside) = &self.aside {
                        aside class="ui-portfolio-hero-aside u-inset-card" {
                            (aside)
                        }
                    }
                }
            }).extra_class("ui-portfolio-lead-surface ui-portfolio-hero"))
        }
    }
}
