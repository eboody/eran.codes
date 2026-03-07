use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::Text;
use crate::views::partials::components::logs;

use super::vm;

#[derive(Clone, Debug, Builder)]
pub struct ChatFlowPanel<'a> {
    pub entries: &'a [&'a TraceEntry],
}

impl Render for ChatFlowPanel<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            logs::primitives::EmptyState::builder()
                .message(Text::from(
                    "No chat messages yet. Send a message to see request/response flow.",
                ))
                .build()
                .render()
        } else {
            logs::primitives::Table::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Direction"),
                    Text::from("Sender"),
                    Text::from("Receiver"),
                    Text::from("User"),
                    Text::from("Body"),
                ])
                .rows(vm::chat_flow_rows(self.entries))
                .variant(logs::primitives::TableVariant::ChatFlow)
                .build()
                .render()
        };

        logs::primitives::Panel::builder()
            .title(Text::from("Chat message flow"))
            .body(body)
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText, TimestampText,
    };

    fn entry(timestamp: &str, fields: Vec<(&str, &str)>) -> TraceEntry {
        TraceEntry::builder()
            .timestamp(TimestampText::new(timestamp))
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new("demo.chat"))
            .message(LogMessageText::new("chat.message.incoming"))
            .fields(
                fields
                    .into_iter()
                    .map(|(name, value)| (LogFieldName::new(name), LogFieldValue::new(value)))
                    .collect(),
            )
            .build()
    }

    #[test]
    fn renders_empty_state_when_no_entries() {
        let markup = ChatFlowPanel::builder()
            .entries(&[])
            .build()
            .render()
            .into_string();

        assert!(markup.contains("No chat messages yet."));
    }

    #[test]
    fn renders_table_when_entries_exist() {
        let entries = vec![entry(
            "12:00:01",
            vec![
                ("direction", "incoming"),
                ("sender", "you"),
                ("receiver", "server"),
                ("user_id", "abc-def"),
                ("body", "hello"),
            ],
        )];
        let refs = entries.iter().collect::<Vec<_>>();
        let markup = ChatFlowPanel::builder()
            .entries(&refs)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("ui-log-table"));
        assert!(markup.contains("hello"));
        assert!(markup.contains("Chat message flow"));
    }
}
