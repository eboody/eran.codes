use crate::trace_log::{
    log::{message, target},
    store,
};
use crate::types::{LogFieldKey, Text};

pub(crate) fn summary(entry: &store::TraceEntry) -> Option<Text> {
    if !matches!(
        entry.target_kind(),
        target::Kind::Known(target::Known::DemoDb)
    ) {
        return None;
    }

    let label = match entry.message_kind() {
        message::Kind::Known(message::Known::DbQuery) => "DB query",
        message::Kind::Known(message::Known::DbQueryComplete) => "DB query complete",
        _ => return None,
    };

    let statement = entry
        .field_text(LogFieldKey::DbStatement)
        .map(|value| normalize_whitespace(&value.to_string()))
        .unwrap_or_default();

    if statement.is_empty() {
        return Some(Text::from(label));
    }

    Some(Text::from(format!("{label}: {statement}")))
}

pub(crate) fn is_summary_text(summary: &Text) -> bool {
    let summary = summary.to_string();
    matches!(summary.as_str(), "DB query" | "DB query complete")
        || summary.starts_with("DB query:")
        || summary.starts_with("DB query complete:")
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
