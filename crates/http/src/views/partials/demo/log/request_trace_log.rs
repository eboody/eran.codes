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
            logs::primitives::PanelBody::Empty(Text::from(
                "No trace entries recorded yet.",
            ))
        } else {
            logs::primitives::PanelBody::Content(
                vm::build_grouped_feed(self.entries.iter()).render(),
            )
        };

        logs::primitives::Surface::builder()
            .layout(logs::primitives::SurfaceLayout::Stack)
            .children(vec![
                logs::primitives::Panel::builder()
                    .title(Text::from("Trace log"))
                    .body(body)
                    .build(),
            ])
            .build()
            .render()
    }
}
