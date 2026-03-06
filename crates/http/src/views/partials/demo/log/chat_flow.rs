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
            logs::EmptyState::builder()
                .message(Text::from(
                    "No chat messages yet. Send a message to see request/response flow.",
                ))
                .build()
                .render()
        } else {
            logs::Table::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Direction"),
                    Text::from("Sender"),
                    Text::from("Receiver"),
                    Text::from("User"),
                    Text::from("Body"),
                ])
                .rows(vm::chat_flow_rows(self.entries))
                .variant(logs::TableVariant::ChatFlow)
                .build()
                .render()
        };

        logs::Panel::builder()
            .title(Text::from("Chat message flow"))
            .body(body)
            .build()
            .render()
    }
}
