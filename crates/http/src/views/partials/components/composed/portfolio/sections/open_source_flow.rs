mod styles;

use maud::Render;

use crate::views::partials::components::portfolio::content::OpenSourceIndexContent;

use super::{CrateSection, OpenSourceHeroAside, OpenSourceMobileIntro, PortfolioHero};

pub struct OpenSourceFlow<'a> {
    pub content: &'a OpenSourceIndexContent,
}

impl Render for OpenSourceFlow<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-portfolio-hero-flow ui-portfolio-open-source-flow" {
                (styles::render())
                (PortfolioHero {
                    content: &self.content.hero,
                    aside: Some(maud::html! {
                        (OpenSourceHeroAside {
                            crate_section: &self.content.crate_section,
                        })
                    }),
                })
                (OpenSourceMobileIntro {
                    crate_section: &self.content.crate_section,
                })
                (CrateSection::standalone(&self.content.crate_section))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::open_source_index_content;

    #[test]
    fn renders_open_source_flow_from_shared_components() {
        let content = open_source_index_content();
        let markup = OpenSourceFlow { content }.render().into_string();

        assert!(markup.contains("ui-portfolio-open-source-flow"));
        assert!(markup.contains("ui-open-source-hero-aside"));
        assert!(markup.contains("ui-open-source-mobile-intro"));
        assert!(markup.contains("ui-open-source-supporting-grid"));
        assert!(!markup.contains("data-portfolio-crate-switcher"));
    }
}
