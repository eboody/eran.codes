use bon::Builder;
use maud::Render;

use crate::views::partials::{KeyValueList, TraceLog};

#[derive(Builder)]
pub struct RequestMeta<'a> {
    pub request_id: Option<&'a str>,
    pub session_id: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub client_ip: Option<&'a str>,
    pub user_agent: Option<&'a str>,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for RequestMeta<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="request-meta-target" {
                div class="demo-result" {
                    p { strong { "Request metadata" } }
                    (KeyValueList::builder()
                        .items(vec![
                            ("request_id".to_string(), self.request_id.unwrap_or("none").to_string()),
                            ("session_id".to_string(), self.session_id.unwrap_or("none").to_string()),
                            ("user_id".to_string(), self.user_id.unwrap_or("none").to_string()),
                            ("client_ip".to_string(), self.client_ip.unwrap_or("none").to_string()),
                            ("user_agent".to_string(), self.user_agent.unwrap_or("none").to_string()),
                        ])
                        .build()
                        .render())
                }
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
