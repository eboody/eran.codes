use bon::Builder;
use maud::Render;

use crate::views::{page, partials};

use super::portfolio_shell;

#[derive(Clone, Debug, Builder)]
pub struct WorkCase {
    pub slug: partials::components::portfolio::content::WorkCaseSlug,
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for WorkCase {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::work_case_content(self.slug);

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::WorkCaseDetail {
                    content,
                })
            },
        }
        .render();
        portfolio_shell::render(
            &content.page_title.to_string(),
            body,
            self.slug.route(),
            self.user.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_sync_route_uses_current_proof_detail_layout() {
        let markup = WorkCase::builder()
            .slug(partials::components::portfolio::content::WorkCaseSlug::SensitiveSync)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("ui-portfolio-work-case-flow"));
        assert!(markup.contains("ui-portfolio-hero"));
        assert!(markup.contains("ui-portfolio-current-proof-detail"));
        assert!(markup.contains("ui-portfolio-current-proof-stack"));
        assert!(markup.contains("Boundary and scope"));
        assert!(!markup.contains("class=\"ui-portfolio-card-grid ui-portfolio-case-grid\""));
    }
}
