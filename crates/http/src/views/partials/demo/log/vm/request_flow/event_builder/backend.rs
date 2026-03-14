use crate::trace_log::{LogTargetKind, LogTargetKnown, TraceEntry};
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::Pill;
use crate::views::partials::demo::log;

pub(super) fn backend_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let summary = db_backend_summary(entry)
        .unwrap_or_else(|| Text::from(format!("{}: {}", entry.target, entry.message)));

    (
        summary,
        Text::from("backend"),
        log::vm::request_flow::pills::field_pills(entry),
    )
}

fn db_backend_summary(entry: &TraceEntry) -> Option<Text> {
    if !matches!(
        LogTargetKind::parse(&entry.target.to_string()),
        LogTargetKind::Known(LogTargetKnown::DemoDb)
    ) {
        return None;
    }

    let label = match unquote(entry.message.to_string().as_str()) {
        "db query" => "DB query",
        "db query complete" => "DB query complete",
        _ => return None,
    };

    let statement = log::vm::field_text(entry, LogFieldKey::DbStatement)
        .map(|value| normalize_whitespace(&value.to_string()))
        .unwrap_or_default();

    if statement.is_empty() {
        return Some(Text::from(label));
    }

    Some(Text::from(format!("{label}: {statement}")))
}

fn unquote(value: &str) -> &str {
    let trimmed = value.trim();
    trimmed
        .strip_prefix('"')
        .and_then(|candidate| candidate.strip_suffix('"'))
        .unwrap_or(trimmed)
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
