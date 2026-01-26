use bon::Builder;
use maud::Render;

use crate::views::partials::TraceLog;

#[derive(Builder)]
pub struct SessionStatus<'a> {
    pub session_id: Option<&'a str>,
    pub expiry: Option<&'a str>,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for SessionStatus<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="session-status-target" {
                div class="demo-result" {
                    p { strong { "Session details" } }
                    ul {
                        li { "session_id: " (self.session_id.unwrap_or("none")) }
                        li { "expiry: " (self.expiry.unwrap_or("none")) }
                    }
                }
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
