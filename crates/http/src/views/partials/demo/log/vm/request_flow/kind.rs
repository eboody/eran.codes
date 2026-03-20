use crate::trace_log::{
    store,
    log::{message, target},
};
use crate::types::LogFieldKey;

#[derive(Clone, Copy, Debug)]
pub(super) enum FlowEvent {
    RequestStart,
    RequestEnd,
    ChatIncoming,
    ChatBroadcast,
    Sse,
    Backend,
}

pub(super) fn flow_event(entry: &store::TraceEntry) -> Option<FlowEvent> {
    let (target_kind, message_kind) = entry.kinds();

    match (target_kind, message_kind) {
        (
            target::Kind::Known(target::Known::DemoRequest),
            message::Kind::Known(message::Known::RequestEnd),
        ) => Some(FlowEvent::RequestEnd),
        (
            target::Kind::Known(target::Known::DemoRequestDiagnostic),
            message::Kind::Known(message::Known::RequestStart),
        ) => Some(FlowEvent::RequestStart),
        (
            target::Kind::Known(target::Known::DemoChat),
            message::Kind::Known(message::Known::ChatMessageIncoming),
        ) => Some(FlowEvent::ChatIncoming),
        (
            target::Kind::Known(target::Known::DemoSse),
            message::Kind::Known(message::Known::ChatMessageBroadcast),
        ) => Some(FlowEvent::ChatBroadcast),
        (target::Kind::Known(target::Known::DemoSse), _) => Some(FlowEvent::Sse),
        (target::Kind::Known(target::Known::DemoRequest), _)
        | (target::Kind::Known(target::Known::DemoRequestDiagnostic), _)
        | (target::Kind::Known(target::Known::DemoChat), _) => {
            Some(FlowEvent::Backend)
        }
        (target::Kind::Other(_), _)
            if entry.field_text(LogFieldKey::RequestId).is_some() =>
        {
            Some(FlowEvent::Backend)
        }
        _ => None,
    }
}
