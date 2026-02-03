use bon::Builder;
use maud::Render;

use crate::views::partials::{StatusCard, TraceLog};

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
                (StatusCard::builder()
                    .title("Session details".to_string())
                    .items(vec![
                        ("session_id".to_string(), self.session_id.unwrap_or("none").to_string()),
                        ("expiry".to_string(), self.expiry.unwrap_or("none").to_string()),
                    ])
                    .build()
                    .render())
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
