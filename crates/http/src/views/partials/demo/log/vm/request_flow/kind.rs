use crate::trace_log::{
    LogMessageKind, LogMessageKnown, LogTargetKind, LogTargetKnown, TraceEntry,
};
use crate::types::LogFieldKey;
use crate::views::partials::demo::log;

#[derive(Clone, Copy, Debug)]
pub(super) enum FlowEvent {
    RequestStart,
    RequestEnd,
    ChatIncoming,
    ChatBroadcast,
    Sse,
    Backend,
}

pub(super) fn flow_event(entry: &TraceEntry) -> Option<FlowEvent> {
    let target_kind = LogTargetKind::parse(&entry.target.to_string());
    let message_kind = LogMessageKind::parse(&entry.message.to_string());

    match (target_kind, message_kind) {
        (
            LogTargetKind::Known(LogTargetKnown::DemoRequest),
            LogMessageKind::Known(LogMessageKnown::RequestEnd),
        ) => Some(FlowEvent::RequestEnd),
        (
            LogTargetKind::Known(LogTargetKnown::DemoRequestDiagnostic),
            LogMessageKind::Known(LogMessageKnown::RequestStart),
        ) => Some(FlowEvent::RequestStart),
        (
            LogTargetKind::Known(LogTargetKnown::DemoChat),
            LogMessageKind::Known(LogMessageKnown::ChatMessageIncoming),
        ) => Some(FlowEvent::ChatIncoming),
        (
            LogTargetKind::Known(LogTargetKnown::DemoSse),
            LogMessageKind::Known(LogMessageKnown::ChatMessageBroadcast),
        ) => Some(FlowEvent::ChatBroadcast),
        (LogTargetKind::Known(LogTargetKnown::DemoSse), _) => Some(FlowEvent::Sse),
        (LogTargetKind::Known(LogTargetKnown::DemoRequest), _)
        | (LogTargetKind::Known(LogTargetKnown::DemoRequestDiagnostic), _)
        | (LogTargetKind::Known(LogTargetKnown::DemoChat), _) => {
            Some(FlowEvent::Backend)
        }
        (LogTargetKind::Other(_), _)
            if log::vm::field_text(entry, LogFieldKey::RequestId).is_some() =>
        {
            Some(FlowEvent::Backend)
        }
        _ => None,
    }
}
