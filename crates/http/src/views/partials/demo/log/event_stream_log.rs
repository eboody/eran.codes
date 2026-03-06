use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::Text;
use crate::views::partials::components::logs;

use super::vm;

#[derive(Builder)]
pub struct EventStreamLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for EventStreamLog<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            logs::EmptyState::builder()
                .message(Text::from(
                    "No events yet. Trigger a demo action to start streaming.",
                ))
                .build()
                .render()
        } else {
            vm::build_grouped_feed(self.entries.iter().rev().take(40)).render()
        };

        logs::Surface::builder()
            .target_id(Text::from("live-log-target"))
            .layout(logs::SurfaceLayout::Stack)
            .children(vec![
                logs::Panel::builder()
                    .title(Text::from("Live backend log"))
                    .body(body)
                    .build()
                    .render(),
            ])
            .auto_scroll(
                logs::AutoScroll::builder()
                    .root_id(Text::from("live-log-target"))
                    .selector(Text::from("[data-log-scroll]"))
                    .scope(logs::AutoScrollScope::Single)
                    .build(),
            )
            .build()
            .render()
    }
}
