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
            logs::primitives::PanelBody::Empty(Text::from(
                "No events yet. Trigger a demo action to start streaming.",
            ))
        } else {
            logs::primitives::PanelBody::Content(
                vm::build_grouped_feed(self.entries.iter().rev().take(40))
                    .render(),
            )
        };

        logs::primitives::Surface::builder()
            .target_id(Text::from("live-log-target"))
            .layout(logs::primitives::SurfaceLayout::Stack)
            .children(vec![
                logs::primitives::Panel::builder()
                    .title(Text::from("Live backend log"))
                    .body(body)
                    .build(),
            ])
            .auto_scroll(
                logs::primitives::AutoScroll::builder()
                    .root_id(Text::from("live-log-target"))
                    .selector(Text::from("[data-log-scroll]"))
                    .scope(logs::primitives::AutoScrollScope::Single)
                    .build(),
            )
            .build()
            .render()
    }
}
