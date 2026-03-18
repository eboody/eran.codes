use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug, Builder)]
pub struct RequestMeta {
    pub request_id: Option<Text>,
    pub session_id: Option<Text>,
    pub user_id: Option<Text>,
    pub client_ip: Option<Text>,
    pub user_agent: Option<Text>,
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for RequestMeta {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="request-meta-target" {
                (partials::StatusCard::builder()
                    .title(Text::from("Request metadata"))
                    .items(vec![
                        partials::StatusCardItem::optional("request_id", self.request_id.clone()),
                        partials::StatusCardItem::optional("session_id", self.session_id.clone()),
                        partials::StatusCardItem::optional("user_id", self.user_id.clone()),
                        partials::StatusCardItem::optional("client_ip", self.client_ip.clone()),
                        partials::StatusCardItem::optional("user_agent", self.user_agent.clone()),
                    ])
                    .build())
                (partials::RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
