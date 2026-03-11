use bon::Builder;
use maud::Render;

use crate::views::page::{Layout, NavMode};
use crate::views::partials::components::portfolio::{self, content::WorkCaseSlug};

#[derive(Clone, Debug, Builder)]
pub struct WorkCase {
    pub slug: WorkCaseSlug,
}

impl Render for WorkCase {
    fn render(&self) -> maud::Markup {
        let content = portfolio::content::work_case_content(self.slug);

        let body = portfolio::Page {
            content: maud::html! {
                (portfolio::WorkCaseDetail { content })
            },
        }
        .render();

        Layout::builder()
            .title(&content.page_title.to_string())
            .content(body)
            .nav_mode(NavMode::Portfolio)
            .build()
            .render()
    }
}
