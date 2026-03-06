use bon::Builder;
use maud::Render;

use crate::views::partials::{StatusCard, RequestTraceLog};
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct SessionStatus {
    pub session_id: Option<Text>,
    pub expiry: Option<Text>,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for SessionStatus {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="session-status-target" {
                (StatusCard::builder()
                    .title(Text::from("Session details"))
                    .items(vec![
                        (
                            Text::from("session_id"),
                            self.session_id.clone().unwrap_or_else(|| Text::from("none")),
                        ),
                        (
                            Text::from("expiry"),
                            self.expiry.clone().unwrap_or_else(|| Text::from("none")),
                        ),
                    ])
                    .build())
                (RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
