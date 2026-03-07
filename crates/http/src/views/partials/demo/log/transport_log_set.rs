use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::Text;
use crate::views::partials::components::logs;

use super::vm;

#[derive(Builder)]
pub struct TransportLogSet<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for TransportLogSet<'_> {
    fn render(&self) -> maud::Markup {
        let request_flows = vm::request_flows(self.entries, 20);
        let flow_body = logs::composed::FlowTimeline::builder()
            .flows(request_flows)
            .build()
            .render();

        logs::primitives::Surface::builder()
            .target_id(Text::from("network-log-target"))
            .layout(logs::primitives::SurfaceLayout::Panels)
            .children(vec![
                logs::primitives::Panel::builder()
                    .title(Text::from("System flow timeline"))
                    .body(logs::primitives::PanelBody::Content(flow_body))
                    .build(),
            ])
            .auto_scroll(
                logs::primitives::AutoScroll::builder()
                    .root_id(Text::from("network-log-target"))
                    .selector(Text::from("[data-log-scroll]"))
                    .scope(logs::primitives::AutoScrollScope::Single)
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
    ) -> TraceEntry {
        TraceEntry::builder()
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

        assert!(markup.contains("id=\"network-log-target\""));
        assert_eq!(markup.matches("id=\"network-log-target\"").count(), 1);
        assert!(markup.contains("System flow timeline"));
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
        assert!(markup.contains("SSE broadcast"));
        assert!(!markup.contains("HTTP requests"));
        assert!(!markup.contains("SSE pushes"));
        assert!(!markup.contains("Chat message flow"));
    }
}
