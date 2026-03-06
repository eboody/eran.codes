use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::Text;
use crate::views::partials::components::logs;

use super::vm;

#[derive(Builder)]
pub struct RequestTraceLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for RequestTraceLog<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            logs::EmptyState::builder()
                .message(Text::from("No trace entries recorded yet."))
                .build()
                .render()
        } else {
            vm::build_grouped_feed(self.entries.iter()).render()
        };

        logs::Surface::builder()
            .layout(logs::SurfaceLayout::Stack)
            .children(vec![
                logs::Panel::builder()
                    .title(Text::from("Trace log"))
                    .body(body)
                    .build()
                    .render(),
            ])
            .build()
            .render()
    }
}
