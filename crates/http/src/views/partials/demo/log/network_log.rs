use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::{
    DataTable, EmptyState, FieldValue, LogPanel, Pill, TableVariant,
};
use crate::views::partials::ChatFlow;
use crate::trace_log::{LogMessageKind, LogMessageKnown, LogTargetKind, LogTargetKnown};

#[derive(Builder)]
pub struct NetworkLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for NetworkLog<'_> {
    fn render(&self) -> maud::Markup {
        let request_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                matches!(
                    LogTargetKind::from_str(&entry.target.to_string()),
                    LogTargetKind::Known(LogTargetKnown::DemoRequest)
                ) && matches!(
                    LogMessageKind::from_str(&entry.message.to_string()),
                    LogMessageKind::Known(LogMessageKnown::RequestEnd)
                )
            })
            .collect();
        let sse_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| matches!(
                LogTargetKind::from_str(&entry.target.to_string()),
                LogTargetKind::Known(LogTargetKnown::DemoSse)
            ))
            .collect();
        let chat_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                let target_kind = LogTargetKind::from_str(&entry.target.to_string());
                let message_kind = LogMessageKind::from_str(&entry.message.to_string());
                matches!(
                    (target_kind, message_kind),
                    (
                        LogTargetKind::Known(LogTargetKnown::DemoChat),
                        LogMessageKind::Known(LogMessageKnown::ChatMessageIncoming)
                    )
                        | (
                            LogTargetKind::Known(LogTargetKnown::DemoSse),
                            LogMessageKind::Known(LogMessageKnown::ChatMessageBroadcast)
                        )
                )
            })
            .collect();

        let request_body = if request_rows.is_empty() {
            EmptyState::builder()
                .message(Text::from("No requests yet. Trigger a demo action to populate this table."))
                .build()
                .render()
        } else {
            let rows = request_rows
                .iter()
                .rev()
                .take(20)
                .map(|entry| {
                    vec![
                        maud::html! { (entry.timestamp.to_string()) },
                        maud::html! { (status_pill(entry).render()) },
                        maud::html! { (method_pill(entry).render()) },
                        maud::html! { (path_pill(entry).render()) },
                        maud::html! { (source_pill(entry).render()) },
                        maud::html! { (latency_pill(entry).map(|pill| pill.render()).unwrap_or_default()) },
                    ]
                })
                .collect::<Vec<_>>();
            DataTable::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Status"),
                    Text::from("Method"),
                    Text::from("Path"),
                    Text::from("Source"),
                    Text::from("Latency"),
                ])
                .rows(rows)
                .variant(TableVariant::Default)
                .build()
                .render()
        };

        let sse_body = if sse_rows.is_empty() {
            EmptyState::builder()
                .message(Text::from("No SSE pushes yet. Send a chat message to broadcast an update."))
                .build()
                .render()
        } else {
            let rows = sse_rows
                .iter()
                .rev()
                .take(20)
                .map(|entry| {
                    vec![
                        maud::html! { (entry.timestamp.to_string()) },
                        maud::html! { (entry.message.to_string()) },
                        maud::html! { (field_value_text(entry, &crate::types::LogFieldName::from(LogFieldKey::Selector)).unwrap_or_else(|| Text::from("-")).to_string()) },
                        maud::html! { (field_value_text(entry, &crate::types::LogFieldName::from(LogFieldKey::Mode)).unwrap_or_else(|| Text::from("-")).to_string()) },
                        maud::html! { (field_value_text(entry, &crate::types::LogFieldName::from(LogFieldKey::PayloadBytes)).unwrap_or_else(|| Text::from("-")).to_string()) },
                    ]
                })
                .collect::<Vec<_>>();
            DataTable::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Event"),
                    Text::from("Selector"),
                    Text::from("Mode"),
                    Text::from("Payload (bytes)"),
                ])
                .rows(rows)
                .variant(TableVariant::Default)
                .build()
                .render()
        };

        maud::html! {
            section id="network-log-target" class="network-log-panels" {
                (LogPanel::builder()
                    .title(Text::from("HTTP requests"))
                    .body(request_body)
                    .build()
                    .render())
                (LogPanel::builder()
                    .title(Text::from("SSE pushes"))
                    .body(sse_body)
                    .build()
                    .render())
                (ChatFlow::builder()
                    .entries(&chat_rows)
                    .build()
                    .render())
                script {
                    (maud::PreEscaped(r#"
(() => {
  const root = document.getElementById('network-log-target');
  if (!root) return;
  const panels = root.querySelectorAll('.network-log-scroll');
  panels.forEach((panel) => {
    const scroll = () => { panel.scrollTop = panel.scrollHeight; };
    requestAnimationFrame(scroll);
    const obs = new MutationObserver(scroll);
    obs.observe(panel, { childList: true, subtree: true });
  });
})();
                    "#))
                }
            }
        }
    }
}

fn field_value(entry: &TraceEntry, name: &crate::types::LogFieldName) -> FieldValue {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| FieldValue::from_log_value(Some(value)))
        .unwrap_or(FieldValue::Missing)
}

fn field_value_text(entry: &TraceEntry, name: &crate::types::LogFieldName) -> Option<Text> {
    field_value(entry, name).into_option()
}

fn method_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, &crate::types::LogFieldName::from(LogFieldKey::Method)).into_option() {
        Some(method) => Pill::method(method),
        None => Pill::fields("-"),
    }
}

fn path_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, &crate::types::LogFieldName::from(LogFieldKey::Path)).into_option() {
        Some(path) => Pill::path(path),
        None => Pill::fields("-"),
    }
}

fn status_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, &crate::types::LogFieldName::from(LogFieldKey::Status)).into_option() {
        Some(status) => Pill::status(status),
        None => Pill::fields("-"),
    }
}

fn latency_pill(entry: &TraceEntry) -> Option<Pill> {
    field_value(entry, &crate::types::LogFieldName::from(LogFieldKey::LatencyMs))
        .into_option()
        .map(|value: Text| {
            Pill::fields(format!("latency_ms={}", value.to_string()))
        })
}

fn source_pill(entry: &TraceEntry) -> Pill {
    let sender: Option<Text> =
        field_value(entry, &crate::types::LogFieldName::from(LogFieldKey::Sender)).into_option();
    match sender {
        Some(sender) => Pill::fields(format!("source={}", sender.to_string())),
        None => Pill::fields("source=unknown"),
    }
}
