use bon::Builder;
use maud::Render;

use crate::views::partials::{StatusCard, TraceLog};
use crate::types::Text;

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
                        (
                            Text::from("request_id"),
                            self.request_id.clone().unwrap_or_else(|| Text::from("none")),
                        ),
                        (
                            Text::from("session_id"),
                            self.session_id.clone().unwrap_or_else(|| Text::from("none")),
                        ),
                        (
                            Text::from("user_id"),
                            self.user_id.clone().unwrap_or_else(|| Text::from("none")),
                        ),
                        (
                            Text::from("client_ip"),
                            self.client_ip.clone().unwrap_or_else(|| Text::from("none")),
                        ),
                        (
                            Text::from("user_agent"),
                            self.user_agent.clone().unwrap_or_else(|| Text::from("none")),
                        ),
                    ])
                    .build()
                    .render())
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
