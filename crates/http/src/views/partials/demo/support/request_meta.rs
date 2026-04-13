use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

use super::Results;

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
        Results::builder()
            .target_id(Text::from("request-meta-target"))
            .summary(
                partials::StatusCard::builder()
                    .title(Text::from("Request metadata"))
                    .items(vec![
                        partials::StatusCardItem::optional("request_id", self.request_id.clone()),
                        partials::StatusCardItem::text(
                            "session_id",
                            super::present_redacted(&self.session_id),
                        ),
                        partials::StatusCardItem::text(
                            "user_id",
                            super::authenticated_redacted(&self.user_id),
                        ),
                        partials::StatusCardItem::text(
                            "client_ip",
                            super::captured_redacted(&self.client_ip),
                        ),
                        partials::StatusCardItem::text(
                            "user_agent",
                            super::captured_redacted(&self.user_agent),
                        ),
                    ])
                    .build()
                    .render(),
            )
            .trace(
                partials::RequestTraceLog::builder()
                    .entries(&self.trace)
                    .build()
                    .render(),
            )
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_internal_request_metadata_values() {
        let markup = RequestMeta::builder()
            .request_id(Text::from("req-123"))
            .session_id(Text::from("session-abc"))
            .user_id(Text::from("user-xyz"))
            .client_ip(Text::from("203.0.113.5"))
            .user_agent(Text::from("ExampleBrowser/1.0"))
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("req-123"));
        assert!(markup.contains("present (redacted)"));
        assert!(markup.contains("authenticated (redacted)"));
        assert!(markup.contains("captured (redacted)"));
        assert!(!markup.contains("session-abc"));
        assert!(!markup.contains("user-xyz"));
        assert!(!markup.contains("203.0.113.5"));
        assert!(!markup.contains("ExampleBrowser/1.0"));
    }
}
