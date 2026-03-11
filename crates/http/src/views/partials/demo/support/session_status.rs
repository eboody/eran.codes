use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::{RequestTraceLog, StatusCard, StatusCardItem};

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
                        StatusCardItem::optional("session_id", self.session_id.clone()),
                        StatusCardItem::optional("expiry", self.expiry.clone()),
                    ])
                    .build())
                (RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
