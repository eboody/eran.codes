use crate::trace_log::store;
use crate::types::Text;
use crate::views::partials::components;
use crate::views::partials::demo::log;

mod backend;
mod chat;
mod request;
#[cfg(test)]
mod tests;

pub(super) fn build_flow_event(
    kind: log::vm::request_flow::kind::FlowEvent,
    entry: &store::TraceEntry,
) -> components::logs::composed::FlowEvent {
    let (summary, stage_label, pills) = match kind {
        log::vm::request_flow::kind::FlowEvent::RequestEnd => {
            request::request_end_event(entry)
        }
        log::vm::request_flow::kind::FlowEvent::RequestStart => {
            request::request_start_event(entry)
        }
        log::vm::request_flow::kind::FlowEvent::ChatIncoming => {
            chat::chat_incoming_event(entry)
        }
        log::vm::request_flow::kind::FlowEvent::ChatBroadcast => {
            chat::chat_broadcast_event(entry)
        }
        log::vm::request_flow::kind::FlowEvent::Sse => (
            Text::from(format!("SSE event: {}", entry.message)),
            Text::from("sse"),
            log::vm::request_flow::pills::field_pills(entry),
        ),
        log::vm::request_flow::kind::FlowEvent::Backend => {
            backend::backend_event(entry)
        }
    };

    components::logs::composed::FlowEvent {
        timestamp: Text::from(entry.timestamp.clone()),
        stage_label,
        summary,
        pills,
    }
}
