use bon::Builder;
use maud::Render;

use crate::views::partials::TraceLog;

#[derive(Builder)]
pub struct DbCheck<'a> {
    pub email: &'a str,
    pub exists: bool,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for DbCheck<'_> {
    fn render(&self) -> maud::Markup {
        let status = if self.exists { "found" } else { "not found" };
        maud::html! {
            article id="db-check-target" {
                div class="demo-result" {
                    p { strong { "DB lookup" } }
                    ul {
                        li { "email: " (self.email) }
                        li { "result: " (status) }
                    }
                }
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
