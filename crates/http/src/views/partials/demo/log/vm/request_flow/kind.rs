use crate::trace_log::{
    LogMessageKind, LogMessageKnown, LogTargetKind, LogTargetKnown, TraceEntry,
};
use crate::types::LogFieldKey;
use crate::views::partials::demo::log;

#[derive(Clone, Copy, Debug)]
pub(super) enum FlowEventKind {
    RequestStart,
    RequestEnd,
    ChatIncoming,
    ChatBroadcast,
    Sse,
    Backend,
}

pub(super) fn flow_event_kind(entry: &TraceEntry) -> Option<FlowEventKind> {
    let target_kind = LogTargetKind::parse(&entry.target.to_string());
    let message_kind = LogMessageKind::parse(&entry.message.to_string());

    match (target_kind, message_kind) {
        (
            LogTargetKind::Known(LogTargetKnown::DemoRequest),
            LogMessageKind::Known(LogMessageKnown::RequestEnd),
        ) => Some(FlowEventKind::RequestEnd),
        (
            LogTargetKind::Known(LogTargetKnown::DemoRequestDiagnostic),
            LogMessageKind::Known(LogMessageKnown::RequestStart),
        ) => Some(FlowEventKind::RequestStart),
        (
            LogTargetKind::Known(LogTargetKnown::DemoChat),
            LogMessageKind::Known(LogMessageKnown::ChatMessageIncoming),
        ) => Some(FlowEventKind::ChatIncoming),
        (
            LogTargetKind::Known(LogTargetKnown::DemoSse),
            LogMessageKind::Known(LogMessageKnown::ChatMessageBroadcast),
        ) => Some(FlowEventKind::ChatBroadcast),
        (LogTargetKind::Known(LogTargetKnown::DemoSse), _) => Some(FlowEventKind::Sse),
        (LogTargetKind::Known(LogTargetKnown::DemoRequest), _)
        | (LogTargetKind::Known(LogTargetKnown::DemoRequestDiagnostic), _)
        | (LogTargetKind::Known(LogTargetKnown::DemoChat), _) => {
            Some(FlowEventKind::Backend)
        }
        (LogTargetKind::Other(_), _)
            if log::vm::field_text(entry, LogFieldKey::RequestId).is_some() =>
        {
            Some(FlowEventKind::Backend)
        }
        _ => None,
    }
}
