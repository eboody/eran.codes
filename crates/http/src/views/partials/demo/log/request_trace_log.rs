use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::Text;
use crate::views::partials::components;

use super::vm;

#[derive(Builder)]
pub struct RequestTraceLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for RequestTraceLog<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            components::logs::primitives::PanelBody::Empty(Text::from(
                "No trace entries recorded yet.",
            ))
        } else {
            components::logs::primitives::PanelBody::Content(
                vm::build_grouped_feed(self.entries.iter()).render(),
            )
        };

        components::logs::primitives::Surface::builder()
            .layout(components::logs::primitives::SurfaceLayout::Stack)
            .children(vec![
                components::logs::primitives::Panel::builder()
                    .title(Text::from("Trace log"))
                    .body(body)
                    .build(),
            ])
            .build()
            .render()
    }
}
