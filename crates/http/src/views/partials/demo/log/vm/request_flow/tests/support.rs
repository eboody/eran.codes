use crate::trace_log::store;
use crate::types::{
    LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
    TimestampText,
};

pub(super) fn entry(
    timestamp: &str,
    target: &str,
    message: &str,
    fields: Vec<(&str, &str)>,
) -> store::TraceEntry {
    store::TraceEntry::builder()
        .timestamp(TimestampText::new(timestamp))
        .level(LogLevelText::new("INFO"))
        .target(LogTargetText::new(target))
        .message(LogMessageText::new(message))
        .fields(
            fields
                .into_iter()
                .map(|(name, value)| {
                    (LogFieldName::new(name), LogFieldValue::new(value))
                })
                .collect(),
        )
        .build()
}

pub(super) fn request_end(
    timestamp: &str,
    request_id: &str,
    method: &str,
    path: &str,
    extra_fields: Vec<(&str, &str)>,
) -> store::TraceEntry {
    let mut fields = vec![
        ("request_id", request_id),
        ("method", method),
        ("path", path),
    ];
    fields.extend(extra_fields);
    entry(timestamp, "demo.request", "request.end", fields)
}

pub(super) fn request_start(
    timestamp: &str,
    request_id: &str,
    method: &str,
    path: &str,
    extra_fields: Vec<(&str, &str)>,
) -> store::TraceEntry {
    let mut fields = vec![
        ("request_id", request_id),
        ("method", method),
        ("path", path),
    ];
    fields.extend(extra_fields);
    entry(timestamp, "demo.request.diagnostic", "request.start", fields)
}
