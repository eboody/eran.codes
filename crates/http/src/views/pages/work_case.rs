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
                (partials::components::portfolio::WorkCaseDetail { content })
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
