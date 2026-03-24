use maud::Render;

use crate::types::Text;
use crate::views::{page, partials};

pub struct Home;

impl Render for Home {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::portfolio_home_content();
        let hero_aside = content.current_proof_section.cards.first().map(|card| {
            maud::html! {
                p class="ui-portfolio-hero-aside-kicker" { (&card.category) }
                h2 { (&card.title) }
                @if let Some(outcome) = &card.outcome {
                    p class="ui-portfolio-hero-aside-outcome" {
                        span class="ui-portfolio-hero-aside-outcome-label" { "Outcome" }
                        span class="ui-portfolio-hero-aside-outcome-text" { (outcome) }
                    }
                }
                p class="ui-portfolio-card-summary" { (&card.summary) }
                (partials::button::Button::builder()
                    .label(card.cta_label.clone())
                    .variant(partials::button::Variant::Secondary)
                    .role(partials::button::Role::link(Text::from(card.slug.public_href())))
                    .build())
            }
        });

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::PortfolioHero {
                    content: &content.hero,
                    aside: hero_aside,
                })
                (partials::components::portfolio::ExperienceSection {
                    content: &content.experience_section,
                })
                (partials::components::portfolio::WorkSection {
                    content: &content.current_proof_section,
                })
                (partials::components::portfolio::WorkSection {
                    content: &content.project_section,
                })
                (partials::components::portfolio::ClosingSection {
                    title: &content.open_source_teaser.title,
                    summary: &content.open_source_teaser.summary,
                    actions: &content.open_source_teaser.actions,
                })
                (partials::components::portfolio::SkillGroupsSection {
                    content: &content.skill_section,
                })
                (partials::components::portfolio::ClosingSection {
                    title: &content.contact_section.title,
                    summary: &content.contact_section.summary,
                    actions: &content.contact_section.actions,
                })
            },
        }
        .render();
        let page_content = page::Frame::builder().content(body).build().render();

        page::Layout::builder()
            .title(&content.page_title.to_string())
            .content(page_content)
            .nav_mode(page::NavMode::Portfolio)
            .current_route(crate::paths::Route::Home)
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_resume_first_sections_in_order() {
        let content = partials::components::portfolio::content::portfolio_home_content();
        let markup = Home.render().into_string();
        let experience_title = content.experience_section.title.to_string();
        let project_title = content.project_section.title.to_string();
        let proof_title = content.current_proof_section.title.to_string();
        let skills_title = content.skill_section.title.to_string();

        assert!(
            markup.find(experience_title.as_str()).unwrap()
                < markup.find(proof_title.as_str()).unwrap()
        );
        assert!(
            markup.find(proof_title.as_str()).unwrap()
                < markup.find(project_title.as_str()).unwrap()
        );
        assert!(
            markup.find(proof_title.as_str()).unwrap() < markup.find(skills_title.as_str()).unwrap()
        );
        assert!(markup.contains("I build secure backend systems with explicit trust boundaries."));
        assert!(!markup.contains("I ship systems that remove operational bottlenecks and improve execution speed."));
        assert!(markup.contains("Most relevant experience"));
        assert!(markup.contains("Current secure-data proof"));
        assert!(markup.contains("href=\"/resume.txt\""));
        assert!(markup.contains("ui-portfolio-hero-aside"));
    }
}
