use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::{RequestTraceLog, StatusCard, StatusCardItem};

#[derive(Clone, Debug, Builder)]
pub struct RequestMeta {
    pub request_id: Option<Text>,
    pub session_id: Option<Text>,
    pub user_id: Option<Text>,
    pub client_ip: Option<Text>,
    pub user_agent: Option<Text>,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for RequestMeta {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="request-meta-target" {
                (StatusCard::builder()
                    .title(Text::from("Request metadata"))
                    .items(vec![
                        StatusCardItem::optional("request_id", self.request_id.clone()),
                        StatusCardItem::optional("session_id", self.session_id.clone()),
                        StatusCardItem::optional("user_id", self.user_id.clone()),
                        StatusCardItem::optional("client_ip", self.client_ip.clone()),
                        StatusCardItem::optional("user_agent", self.user_agent.clone()),
                    ])
                    .build())
                (RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
