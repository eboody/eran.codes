use bon::Builder;
use maud::Render;

use crate::views::{page, partials};

#[derive(Clone, Debug, Builder)]
pub struct WorkCase {
    pub slug: partials::components::portfolio::content::WorkCaseSlug,
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
        let page_content = page::Frame::builder().content(body).build().render();

        page::Layout::builder()
            .title(&content.page_title.to_string())
            .content(page_content)
            .nav_mode(page::NavMode::Portfolio)
            .current_route(crate::paths::Route::Work)
            .build()
            .render()
    }
}
