use maud::Render;

use crate::views::partials::components::portfolio::content::{
    open_source_index_content, PortfolioHomeContent,
};

use super::{
    CrateSection, FlagshipCrateHeroAside, PortfolioHero, WorkSection, WorkSectionVariant,
};

pub struct HomeFlow<'a> {
    pub content: &'a PortfolioHomeContent,
}

impl Render for HomeFlow<'_> {
    fn render(&self) -> maud::Markup {
        let open_source = open_source_index_content();
        let hero_aside = open_source.crate_section.cards.first().map(|card| {
            maud::html! {
                (FlagshipCrateHeroAside { card })
            }
        });

        maud::html! {
            (PortfolioHero {
                content: &self.content.hero,
                aside: hero_aside,
            })
            (CrateSection {
                content: &open_source.crate_section,
                show_heading: false,
            })
            (WorkSection {
                content: &self.content.current_proof_section,
                variant: WorkSectionVariant::CurrentProof,
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
        assert!(markup.contains("ui-portfolio-crate-showcase"));
        assert!(markup.contains("statum"));
        assert!(!markup.contains("Most relevant experience"));
    }
}
