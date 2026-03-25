use maud::Render;

use crate::views::partials::components::portfolio::content::PortfolioHomeContent;

use super::{
    CurrentProofHeroAside, ExperienceSection, PortfolioHero, WorkSection, WorkSectionVariant,
};

pub struct HomeFlow<'a> {
    pub content: &'a PortfolioHomeContent,
}

impl Render for HomeFlow<'_> {
    fn render(&self) -> maud::Markup {
        let hero_aside = self.content.current_proof_section.cards.first().map(|card| {
            maud::html! {
                (CurrentProofHeroAside { card })
            }
        });

        maud::html! {
            (PortfolioHero {
                content: &self.content.hero,
                aside: hero_aside,
            })
            (WorkSection {
                content: &self.content.current_proof_section,
                variant: WorkSectionVariant::CurrentProof,
            })
            (ExperienceSection {
                content: &self.content.experience_section,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::portfolio_home_content;

    #[test]
    fn renders_home_flow_with_current_proof_hero_aside() {
        let content = portfolio_home_content();
        let markup = HomeFlow { content }.render().into_string();

        assert!(markup.contains(content.current_proof_section.title.to_string().as_str()));
        assert!(markup.contains("ui-portfolio-hero-aside"));
        assert!(markup.contains(content.experience_section.title.to_string().as_str()));
        assert!(!markup.contains(content.project_section.title.to_string().as_str()));
        assert!(!markup.contains(content.skill_section.title.to_string().as_str()));
        assert!(!markup.contains(content.open_source_teaser.title.to_string().as_str()));
    }
}
