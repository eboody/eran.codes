use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;
use crate::views::partials::demo::log;

pub(super) fn request_end_event(
    entry: &TraceEntry,
) -> (Text, Text, Vec<components::Pill>) {
    let method = log::vm::request_flow::pills::method_or_unknown(entry);
    let path = log::vm::request_flow::pills::path_or_root(entry);
    let status = log::vm::request_flow::pills::status_or_dash(entry);

    let mut pills = vec![
        components::Pill::method(method.clone()),
        components::Pill::path(path.clone()),
        components::Pill::status(status.clone()),
    ];
    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[(LogFieldKey::LatencyMs, "latency_ms")],
    );

    (
        Text::from(format!("HTTP {method} {path} -> {status}")),
        Text::from("request"),
        pills,
    )
}

pub(super) fn request_start_event(
    entry: &TraceEntry,
) -> (Text, Text, Vec<components::Pill>) {
    let method = log::vm::request_flow::pills::method_or_unknown(entry);
    let path = log::vm::request_flow::pills::path_or_root(entry);

    (
        Text::from(format!("HTTP {method} {path} started")),
        Text::from("request"),
        vec![components::Pill::method(method), components::Pill::path(path)],
    )
}
