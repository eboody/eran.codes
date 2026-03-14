use maud::Render;

use crate::views::{page, partials};

pub struct Work;

impl Render for Work {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::work_index_content();

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::WorkIndexSection { content })
            },
        }
        .render();

        page::Layout::builder()
            .title(&content.page_title.to_string())
            .content(body)
            .nav_mode(page::NavMode::Portfolio)
            .build()
            .render()
    }
}
