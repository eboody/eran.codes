use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::Pill;
use crate::views::partials::demo::log;

pub(super) fn request_end_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let method = log::vm::request_flow::pills::method_or_unknown(entry);
    let path = log::vm::request_flow::pills::path_or_root(entry);
    let status = log::vm::request_flow::pills::status_or_dash(entry);

    let mut pills = vec![
        Pill::method(method.clone()),
        Pill::path(path.clone()),
        Pill::status(status.clone()),
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

pub(super) fn request_start_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let method = log::vm::request_flow::pills::method_or_unknown(entry);
    let path = log::vm::request_flow::pills::path_or_root(entry);

    (
        Text::from(format!("HTTP {method} {path} started")),
        Text::from("request"),
        vec![Pill::method(method), Pill::path(path)],
    )
}
