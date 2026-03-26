use maud::Render;

use crate::views::partials::components::portfolio::content::{ArchivedWorkCaseContent, WorkIndexContent};

use super::{ArchiveCaseDetailsSection, PortfolioHero, WorkIndexSection};

pub struct WorkFlow<'a> {
    pub content: &'a WorkIndexContent,
    pub archive_cases: &'a [ArchivedWorkCaseContent],
}

impl Render for WorkFlow<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-portfolio-hero-flow ui-portfolio-work-flow" {
                (PortfolioHero {
                    content: &self.content.hero,
                    aside: None,
                })
                (WorkIndexSection {
                    content: self.content,
                })
                (ArchiveCaseDetailsSection {
                    intro: &self.content.archive_details,
                    cases: self.archive_cases,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::{
        supporting_archive_cases, work_index_content,
    };

    #[test]
    fn renders_work_flow_sections_in_order() {
        let content = work_index_content();
        let archive_cases = supporting_archive_cases();
        let markup = WorkFlow {
            content,
            archive_cases,
        }
        .render()
        .into_string();

        assert!(markup.contains("ui-portfolio-work-flow"));
        assert!(markup.contains(content.current_proof_section.title.to_string().as_str()));
        assert!(markup.contains(content.archive_details.title.to_string().as_str()));
        assert!(!markup.contains(content.open_source_teaser.title.to_string().as_str()));
    }
}
