use bon::Builder;
use maud::Render;

use crate::views::partials::TraceLog;

#[derive(Builder)]
pub struct BoundaryCheck<'a> {
    pub label: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub result: &'a str,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for BoundaryCheck<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="boundary-target" {
                div class="demo-result" {
                    p { strong { (self.label) } }
                    ul {
                        li { "username: " (self.username) }
                        li { "email: " (self.email) }
                        li { "result: " (self.result) }
                    }
                }
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
