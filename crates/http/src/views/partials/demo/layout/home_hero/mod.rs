mod actions;
mod styles;

use bon::Builder;
use maud::{Markup, Render};

use crate::views::{page, partials};

#[derive(Clone, Debug, Builder)]
pub struct HomeHero {
    pub user: Option<page::UserNav>,
}

impl Render for HomeHero {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::lab_page_content();

        maud::html! {
            header id="home-hero" data-home-hero {
                (styles::render())
                div data-home-hero-copy {
                    p data-home-hero-kicker { (&content.hero.eyebrow) }
                    h1 data-home-hero-title { (&content.hero.title) }
                    p data-home-hero-summary { (&content.hero.summary) }
                    (render_badges(&content.hero.badges))
                    (actions::hero_actions(&content.hero.actions))
                }
                aside data-home-hero-card {
                    h3 data-home-hero-card-title { (&content.session_card.title) }
                    (render_session_card(
                        &content.session_card.guest_status,
                        &content.session_card.guest_summary,
                        &content.session_card.guest_actions,
                        &content.session_card.signed_in_action_label,
                        self.user.as_ref(),
                    ))
                }
            }
        }
    }
}

fn render_badges(badges: &[crate::types::Text]) -> Markup {
    maud::html! {
        div data-home-hero-tags {
            @for badge in badges {
                (partials::components::Pill::builder().text(badge.clone()).build())
            }
        }
    }
}

fn render_session_card(
    guest_status: &crate::types::Text,
    guest_summary: &crate::types::Text,
    guest_actions: &[partials::components::portfolio::content::CmsActionLink],
    signed_in_action_label: &crate::types::Text,
    user: Option<&page::UserNav>,
) -> Markup {
    match user {
        Some(user) => maud::html! {
            p data-home-hero-card-status { "Signed in as " strong { (&user.username) } "." }
            p class="u-muted" data-home-hero-card-detail { (&user.email) }
            (actions::signed_in_session_action(signed_in_action_label))
        },
        None => maud::html! {
            p data-home-hero-card-status { (guest_status) }
            p class="u-muted" data-home-hero-card-detail { (guest_summary) }
            (actions::guest_session_actions(guest_actions))
        },
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::*;

    #[test]
    fn renders_explicit_hero_hooks() {
        let markup = HomeHero::builder().build().render().into_string();

        assert!(markup.contains("data-home-hero-title"));
        assert!(markup.contains("data-home-hero-summary"));
        assert!(markup.contains("data-home-hero-primary-actions"));
        assert!(markup.contains("data-home-hero-card-actions"));
        assert!(markup.contains("data-home-hero-card-secondary-link"));
        assert!(markup.contains("data-home-hero-card-secondary-copy"));
    }
}
