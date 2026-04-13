mod styles;

use maud::{Markup, Render};

pub mod content;
mod sections;

pub use sections::{HomeFlow, OpenSourceFlow, WorkCaseDetail, WorkFlow};

pub struct Page {
    pub content: Markup,
}

impl Render for Page {
    fn render(&self) -> Markup {
        maud::html! {
            div data-portfolio-page data-page-section {
                (styles::render())
                (&self.content)
            }
        }
    }
}
