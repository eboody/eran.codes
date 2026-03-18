use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug, Builder)]
pub struct SessionStatus {
    pub session_id: Option<Text>,
    pub expiry: Option<Text>,
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for SessionStatus {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="session-status-target" {
                (partials::StatusCard::builder()
                    .title(Text::from("Session details"))
                    .items(vec![
                        partials::StatusCardItem::optional("session_id", self.session_id.clone()),
                        partials::StatusCardItem::optional("expiry", self.expiry.clone()),
                    ])
                    .build())
                (partials::RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
