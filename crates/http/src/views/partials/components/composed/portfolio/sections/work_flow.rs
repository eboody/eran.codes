use maud::Render;

use crate::views::partials::components::portfolio::content::WorkIndexContent;

use super::{PortfolioHero, WorkSection, WorkSectionVariant};

pub struct WorkFlow<'a> {
    pub content: &'a WorkIndexContent,
}

impl Render for WorkFlow<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-portfolio-hero-flow ui-portfolio-work-flow" {
                (PortfolioHero {
                    content: &self.content.hero,
                    aside: None,
                })
                (WorkSection {
                    content: &self.content.supporting_cases_section,
                    variant: WorkSectionVariant::Standard,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::work_index_content;

    #[test]
    fn renders_work_flow_sections_in_order() {
        let content = work_index_content();
        let markup = WorkFlow {
            content,
        }
        .render()
        .into_string();

        assert!(markup.contains("ui-portfolio-work-flow"));
        assert!(markup.contains(content.supporting_cases_section.title.to_string().as_str()));
        assert!(!markup.contains("Current flagship proof"));
    }
}
