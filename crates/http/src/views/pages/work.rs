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

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::WorkFlow { content })
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
    fn renders_minimal_archive_surface() {
        let content = partials::components::portfolio::content::work_index_content();
        let markup = Work::default().render().into_string();
        let lead_title = content.hero.title.to_string();
        let supporting_subtitle = content.supporting_cases_section.subtitle.to_string();

        assert!(markup.contains(lead_title.as_str()));
        assert!(markup.contains(supporting_subtitle.as_str()));
        assert!(!markup.contains("data-code-block"));
        assert!(!markup.contains("data-portfolio-crate-switcher"));
        assert!(!markup.contains("images/work/chat-realtime"));
        assert!(!markup.contains("Automation at scale case preview"));
        assert!(markup.contains("ui-portfolio-work-flow"));
        assert!(markup.contains("ui-portfolio-hero"));
        assert!(markup.contains("Compact archive"));
        assert!(!markup.contains("Current flagship proof"));
        assert!(!markup.contains("Encrypted Sensitive Record Sync in Rust"));
        assert!(markup.contains("Automated Fundraiser Acknowledgment at Scale"));
        assert!(markup.contains("/work#chat-realtime"));
        assert!(markup.contains("/work#command-sse"));
        assert!(markup.contains("/work#operational-visibility"));
    }
}
