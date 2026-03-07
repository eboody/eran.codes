use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::{Pill, logs};
use crate::views::partials::demo::log;

pub(super) fn build_flow_event(
    kind: log::vm::request_flow::kind::FlowEventKind,
    entry: &TraceEntry,
) -> logs::composed::FlowEvent {
    let (summary, stage_label, pills) = match kind {
        log::vm::request_flow::kind::FlowEventKind::RequestEnd => {
            request_end_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::RequestStart => {
            request_start_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::ChatIncoming => {
            chat_incoming_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::ChatBroadcast => {
            chat_broadcast_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::Sse => (
            Text::from(format!("SSE event: {}", entry.message)),
            Text::from("sse"),
            log::vm::request_flow::pills::field_pills(entry),
        ),
        log::vm::request_flow::kind::FlowEventKind::Backend => (
            Text::from(format!("{}: {}", entry.target, entry.message)),
            Text::from("backend"),
            log::vm::request_flow::pills::field_pills(entry),
        ),
    };

    logs::composed::FlowEvent {
        timestamp: Text::from(entry.timestamp.clone()),
        stage_label,
        summary,
        pills,
    }
}

fn request_end_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
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

fn request_start_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let method = log::vm::request_flow::pills::method_or_unknown(entry);
    let path = log::vm::request_flow::pills::path_or_root(entry);

    (
        Text::from(format!("HTTP {method} {path} started")),
        Text::from("request"),
        vec![Pill::method(method), Pill::path(path)],
    )
}

fn chat_incoming_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let sender = log::vm::field_text(entry, LogFieldKey::Sender)
        .unwrap_or_else(|| Text::from("unknown"));
    let mut pills = vec![Pill::fields(format!("sender={sender}"))];

    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::Receiver, "receiver"),
            (LogFieldKey::UserId, "user_id"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
        ],
    );

    (
        Text::from(format!("Backend accepted chat message from {sender}")),
        Text::from("backend"),
        pills,
    )
}

fn chat_broadcast_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let selector = log::vm::field_text(entry, LogFieldKey::Selector)
        .unwrap_or_else(|| Text::from("[unknown-selector]"));
    let mut pills = vec![Pill::fields(format!("selector={selector}"))];

    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::Mode, "mode"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
            (LogFieldKey::Sender, "sender"),
            (LogFieldKey::Receiver, "receiver"),
        ],
    );

    (
        Text::from(format!("SSE broadcast to {selector}")),
        Text::from("sse"),
        pills,
    )
}
