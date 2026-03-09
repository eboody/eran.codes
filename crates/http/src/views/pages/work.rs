use maud::Render;

use crate::views::page::{Layout, NavMode};
use crate::views::partials::components::portfolio;

pub struct Work;

impl Render for Work {
    fn render(&self) -> maud::Markup {
        let content = portfolio::content::work_index_content();

        let body = maud::html! {
            main class="container ui-portfolio-main" {
                (portfolio::WorkIndexSection { content })
            }
        };

        Layout::builder()
            .title(&content.page_title.to_string())
            .content(body)
            .nav_mode(NavMode::Portfolio)
            .build()
            .render()
    }
}
