use bon::Builder;
use maud::Render;

use crate::views::{page, partials};

use super::portfolio_shell;

#[derive(Builder, Default)]
pub struct Work {
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for Work {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::work_index_content();
        let archive_cases = partials::components::portfolio::content::supporting_archive_cases();

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::WorkFlow {
                    content,
                    archive_cases,
                })
            },
        }
        .render();
        portfolio_shell::render(
            &content.page_title.to_string(),
            body,
            crate::paths::Route::Work,
            self.user.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_current_proof_and_compact_archive() {
        let content = partials::components::portfolio::content::work_index_content();
        let markup = Work::default().render().into_string();
        let lead_title = content.hero.title.to_string();
        let current_title = content.current_proof_section.title.to_string();
        let supporting_subtitle = content.supporting_cases_section.subtitle.to_string();
        let archive_title = content.archive_details.title.to_string();

        assert!(markup.contains(lead_title.as_str()));
        assert!(markup.contains(current_title.as_str()));
        assert!(markup.contains(supporting_subtitle.as_str()));
        assert!(markup.contains(archive_title.as_str()));
        assert!(!markup.contains("data-code-block"));
        assert!(!markup.contains("data-portfolio-crate-switcher"));
        assert!(!markup.contains("images/work/chat-realtime"));
        assert!(!markup.contains("Automation at scale case preview"));
        assert!(markup.contains("ui-portfolio-work-flow"));
        assert!(markup.contains("ui-portfolio-hero"));
        assert!(
            markup.find(lead_title.as_str()).unwrap()
                < markup.find(current_title.as_str()).unwrap()
        );
        assert!(
            markup.find(current_title.as_str()).unwrap()
                < markup.find(supporting_subtitle.as_str()).unwrap()
        );
        assert!(
            markup.find(supporting_subtitle.as_str()).unwrap()
                < markup.find(archive_title.as_str()).unwrap()
        );
        assert!(markup.contains("Current proof first. Archive below."));
        assert!(markup.contains("Encrypted Sensitive Record Sync in Rust"));
        assert!(
            markup.find("Encrypted Sensitive Record Sync in Rust").unwrap()
                < markup.find("Automated Fundraiser Acknowledgment at Scale").unwrap()
        );
        assert!(markup.contains("Selected archive"));
        assert!(markup.contains("Archived work"));
        assert!(markup.contains("id=\"chat-realtime\""));
        assert!(markup.contains("id=\"command-sse\""));
        assert!(markup.contains("id=\"operational-visibility\""));
        assert!(!markup.contains(content.open_source_teaser.title.to_string().as_str()));
        assert!(!markup.contains("Explore open-source work"));
        assert!(!markup.contains("Current implementation proof plus supporting shipped systems."));
    }

    #[test]
    fn archive_entries_render_anchored_compact_summaries() {
        let markup = Work::default().render().into_string();

        for case in partials::components::portfolio::content::supporting_archive_cases() {
            let anchor_id = case
                .slug
                .archive_anchor_id()
                .expect("archived work case should expose an anchor id");

            assert!(markup.contains(&format!("id=\"{anchor_id}\"")));
            assert!(markup.contains(case.content.hero.title.to_string().as_str()));
            assert!(markup.contains(case.content.hero.summary.to_string().as_str()));
            assert!(!markup.contains(case.content.implementation.items[0].to_string().as_str()));
        }
    }
}
