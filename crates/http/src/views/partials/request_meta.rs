use maud::Render;

use crate::views::partials::TraceLog;

pub struct RequestMeta<'a> {
    pub request_id: Option<&'a str>,
    pub session_id: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub client_ip: Option<&'a str>,
    pub user_agent: Option<&'a str>,
    pub trace: Vec<crate::trace_log::Entry>,
}

impl Render for RequestMeta<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="request-meta-target" {
                div class="demo-result" {
                    p { strong { "Request metadata" } }
                    ul {
                        li { "request_id: " (self.request_id.unwrap_or("none")) }
                        li { "session_id: " (self.session_id.unwrap_or("none")) }
                        li { "user_id: " (self.user_id.unwrap_or("none")) }
                        li { "client_ip: " (self.client_ip.unwrap_or("none")) }
                        li { "user_agent: " (self.user_agent.unwrap_or("none")) }
                    }
                }
                (TraceLog { entries: &self.trace }.render())
            }
        }
    }
}
