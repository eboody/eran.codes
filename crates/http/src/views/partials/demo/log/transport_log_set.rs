use bon::Builder;
use maud::Render;

use crate::trace_log::{FlowFilterTerms, store};
use crate::types::Text;
use crate::views::partials::components;

use super::vm;

#[derive(Builder)]
pub struct TransportLogSet<'a> {
    pub entries: &'a [store::TraceEntry],
    pub active_tab_id: Option<crate::types::SseTabId>,
    #[builder(default)]
    pub excluded_terms: FlowFilterTerms,
}

impl Render for TransportLogSet<'_> {
    fn render(&self) -> maud::Markup {
        let mut request_flows =
            vm::request_flows(self.entries, 20, self.active_tab_id.as_ref());
        if !self.excluded_terms.is_empty() {
            request_flows.retain(|flow| {
                !components::logs::composed::flow_matches_any_search_term(
                    flow,
                    self.excluded_terms.as_slice(),
                )
            });
        }
        let flow_body = components::logs::composed::FlowTimeline::builder()
            .flows(request_flows)
            .build()
            .render();

        components::logs::primitives::Surface::builder()
            .target_id(Text::from("network-log-target"))
            .layout(components::logs::primitives::SurfaceLayout::Panels)
            .children(vec![
                components::logs::primitives::Panel::builder()
                    .title(Text::from("Session flow timeline"))
                    .body(components::logs::primitives::PanelBody::Content(flow_body))
                    .build(),
            ])
            .auto_scroll(
                components::logs::primitives::AutoScroll::builder()
                    .root_id(Text::from("network-log-target"))
                    .selector(Text::from("[data-log-scroll]"))
                    .scope(components::logs::primitives::AutoScrollScope::Single)
                    .build(),
            )
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
        TimestampText,
    };

    fn entry(
        timestamp: &str,
        target: &str,
        message: &str,
        fields: Vec<(&str, &str)>,
    ) -> store::TraceEntry {
        store::TraceEntry::builder()
            .timestamp(TimestampText::new(timestamp))
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new(target))
            .message(LogMessageText::new(message))
            .fields(
                fields
                    .into_iter()
                    .map(|(name, value)| {
                        (LogFieldName::new(name), LogFieldValue::new(value))
                    })
                    .collect(),
            )
            .build()
    }

    #[test]
    fn renders_empty_states_and_stable_patch_target() {
        let markup = TransportLogSet::builder()
            .entries(&[])
            .build()
            .render()
            .into_string();

        assert!(markup.contains(" id=\"network-log-target\""));
        assert_eq!(markup.matches(" id=\"network-log-target\"").count(), 1);
        assert!(markup.contains("Session flow timeline"));
        assert!(markup.contains("No request flows yet."));
    }

    #[test]
    fn renders_flow_timeline_for_chat_request_sequence() {
        let entries = vec![
            entry(
                "12:00:01",
                "demo.request",
                "request.end",
                vec![
                    ("request_id", "req-aaa-111"),
                    ("method", "POST"),
                    ("path", "/demo/chat/messages"),
                    ("status", "202"),
                ],
            ),
            entry(
                "12:00:02",
                "demo.chat",
                "chat.message.incoming",
                vec![
                    ("request_id", "req-aaa-111"),
                    ("sender", "you"),
                    ("receiver", "server"),
                ],
            ),
            entry(
                "12:00:03",
                "demo.sse",
                "chat message broadcast",
                vec![
                    ("request_id", "req-aaa-111"),
                    ("selector", "[data-chat-messages]"),
                    ("mode", "prepend"),
                ],
            ),
        ];

        let markup = TransportLogSet::builder()
            .entries(&entries)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("ui-log-flow-shell"));
        assert!(markup.contains("network-flow-req-aaa-111"));
        assert!(markup.contains("data-flow-search="));
        assert!(markup.contains("SSE broadcast"));
        assert!(!markup.contains("HTTP requests"));
        assert!(!markup.contains("SSE pushes"));
        assert!(!markup.contains("Chat message flow"));
    }

    #[test]
    fn excludes_matching_request_flows_from_render() {
        let entries = vec![
            entry(
                "12:00:01",
                "demo.request",
                "request.end",
                vec![
                    ("request_id", "req-hide-111"),
                    ("method", "GET"),
                    ("path", "/events"),
                    ("status", "200"),
                ],
            ),
            entry(
                "12:00:02",
                "demo.request",
                "request.end",
                vec![
                    ("request_id", "req-show-222"),
                    ("method", "POST"),
                    ("path", "/demo/chat/messages"),
                    ("status", "202"),
                ],
            ),
        ];

        let markup = TransportLogSet::builder()
            .entries(&entries)
            .excluded_terms(FlowFilterTerms::from("/events"))
            .build()
            .render()
            .into_string();

        assert!(!markup.contains("GET /events"));
        assert!(markup.contains("POST /demo/chat/messages"));
    }

    #[test]
    fn redacts_authenticated_user_ids_and_db_bind_values_from_flow_timeline() {
        let entries = vec![
            entry(
                "12:00:01",
                "demo.request",
                "request.end",
                vec![
                    ("request_id", "req-redact-111"),
                    ("method", "POST"),
                    ("path", "/demo/chat/messages"),
                    ("status", "202"),
                ],
            ),
            entry(
                "12:00:02",
                "app::chat::service",
                "moderation check passed",
                vec![("request_id", "req-redact-111"), ("user_id", "user-1")],
            ),
            entry(
                "12:00:03",
                "demo.db",
                "\"db query\"",
                vec![
                    ("request_id", "req-redact-111"),
                    (
                        "db_statement",
                        "SELECT id FROM chat_rooms WHERE id = $1 AND created_by = $2",
                    ),
                    ("db_bind_1", "room-secret"),
                    ("db_bind_2", "owner-secret"),
                ],
            ),
        ];

        let markup = TransportLogSet::builder()
            .entries(&entries)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("Session flow timeline"));
        assert!(markup.contains("user=authenticated (redacted)"));
        assert!(markup.contains("$1=(redacted)"));
        assert!(markup.contains("$2=(redacted)"));
        assert!(!markup.contains("user-1"));
        assert!(!markup.contains("room-secret"));
        assert!(!markup.contains("owner-secret"));
    }
}
