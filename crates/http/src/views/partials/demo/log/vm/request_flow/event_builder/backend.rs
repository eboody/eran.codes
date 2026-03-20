use crate::trace_log::{
    demo_db,
    store,
};
use crate::types::Text;
use crate::views::partials::components;
use crate::views::partials::demo::log;

pub(super) fn backend_event(
    entry: &store::TraceEntry,
) -> (Text, Text, Vec<components::Pill>) {
    let summary = demo_db::summary(entry)
        .unwrap_or_else(|| Text::from(format!("{}: {}", entry.target, entry.message)));

    (
        summary,
        Text::from("backend"),
        log::vm::request_flow::pills::field_pills(entry),
    )
}
