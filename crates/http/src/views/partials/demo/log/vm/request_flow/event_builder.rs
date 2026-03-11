use crate::trace_log::TraceEntry;
use crate::types::Text;
use crate::views::partials::components::logs;
use crate::views::partials::demo::log;

mod backend;
mod chat;
mod request;
#[cfg(test)]
mod tests;

pub(super) fn build_flow_event(
    kind: log::vm::request_flow::kind::FlowEventKind,
    entry: &TraceEntry,
) -> logs::composed::FlowEvent {
    let (summary, stage_label, pills) = match kind {
        log::vm::request_flow::kind::FlowEventKind::RequestEnd => {
            request::request_end_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::RequestStart => {
            request::request_start_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::ChatIncoming => {
            chat::chat_incoming_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::ChatBroadcast => {
            chat::chat_broadcast_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::Sse => (
            Text::from(format!("SSE event: {}", entry.message)),
            Text::from("sse"),
            log::vm::request_flow::pills::field_pills(entry),
        ),
        log::vm::request_flow::kind::FlowEventKind::Backend => {
            backend::backend_event(entry)
        }
    };

    logs::composed::FlowEvent {
        timestamp: Text::from(entry.timestamp.clone()),
        stage_label,
        summary,
        pills,
    }
}
