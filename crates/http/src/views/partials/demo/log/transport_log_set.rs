use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::Text;
use crate::views::partials::components::logs;

use super::chat_flow::ChatFlowPanel;
use super::vm;

#[derive(Builder)]
pub struct TransportLogSet<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for TransportLogSet<'_> {
    fn render(&self) -> maud::Markup {
        let request_rows = vm::request_rows(self.entries);
        let sse_rows = vm::sse_rows(self.entries);
        let chat_entries = vm::chat_entries(self.entries);

        let request_body = if request_rows.is_empty() {
            logs::EmptyState::builder()
                .message(Text::from(
                    "No requests yet. Trigger a demo action to populate this table.",
                ))
                .build()
                .render()
        } else {
            logs::Table::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Status"),
                    Text::from("Method"),
                    Text::from("Path"),
                    Text::from("Source"),
                    Text::from("Latency"),
                ])
                .rows(request_rows)
                .variant(logs::TableVariant::Default)
                .build()
                .render()
        };

        let sse_body = if sse_rows.is_empty() {
            logs::EmptyState::builder()
                .message(Text::from(
                    "No SSE pushes yet. Send a chat message to broadcast an update.",
                ))
                .build()
                .render()
        } else {
            logs::Table::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Event"),
                    Text::from("Selector"),
                    Text::from("Mode"),
                    Text::from("Payload (bytes)"),
                ])
                .rows(sse_rows)
                .variant(logs::TableVariant::Default)
                .build()
                .render()
        };

        logs::Surface::builder()
            .target_id(Text::from("network-log-target"))
            .layout(logs::SurfaceLayout::Panels)
            .children(vec![
                logs::Panel::builder()
                    .title(Text::from("HTTP requests"))
                    .body(request_body)
                    .build()
                    .render(),
                logs::Panel::builder()
                    .title(Text::from("SSE pushes"))
                    .body(sse_body)
                    .build()
                    .render(),
                ChatFlowPanel::builder().entries(&chat_entries).build().render(),
            ])
            .auto_scroll(
                logs::AutoScroll::builder()
                    .root_id(Text::from("network-log-target"))
                    .selector(Text::from("[data-log-scroll]"))
                    .scope(logs::AutoScrollScope::All)
                    .build(),
            )
            .build()
            .render()
    }
}
