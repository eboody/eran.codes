use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::trace_log::{LogMessageKind, LogMessageKnown, LogTargetKind, LogTargetKnown};
use crate::types::{LogFieldKey, LogFieldName, Text};
use crate::views::partials::components::Pill;
use crate::views::partials::demo::log;

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
                    LogTargetKind::parse(&entry.target.to_string()),
                    LogTargetKind::Known(LogTargetKnown::DemoRequest)
                ) && matches!(
                    LogMessageKind::parse(&entry.message.to_string()),
                    LogMessageKind::Known(LogMessageKnown::RequestEnd)
                )
            })
            .collect();
        let sse_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                matches!(
                    LogTargetKind::parse(&entry.target.to_string()),
                    LogTargetKind::Known(LogTargetKnown::DemoSse)
                )
            })
            .collect();
        let chat_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                let target_kind = LogTargetKind::parse(&entry.target.to_string());
                let message_kind = LogMessageKind::parse(&entry.message.to_string());
                matches!(
                    (target_kind, message_kind),
                    (
                        LogTargetKind::Known(LogTargetKnown::DemoChat),
                        LogMessageKind::Known(LogMessageKnown::ChatMessageIncoming)
                    ) | (
                        LogTargetKind::Known(LogTargetKnown::DemoSse),
                        LogMessageKind::Known(LogMessageKnown::ChatMessageBroadcast)
                    )
                )
            })
            .collect();

        let request_body = if request_rows.is_empty() {
            log::EmptyState::builder()
                .message(Text::from(
                    "No requests yet. Trigger a demo action to populate this table.",
                ))
                .build()
                .render()
        } else {
            let rows = request_rows
                .iter()
                .rev()
                .take(20)
                .map(|entry| {
                    vec![
                        maud::html! { (&entry.timestamp) },
                        maud::html! { (status_pill(entry).render()) },
                        maud::html! { (method_pill(entry).render()) },
                        maud::html! { (path_pill(entry).render()) },
                        maud::html! { (source_pill(entry).render()) },
                        maud::html! { (latency_pill(entry).map(|pill| pill.render()).unwrap_or_default()) },
                    ]
                })
                .collect::<Vec<_>>();
            log::DataTable::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Status"),
                    Text::from("Method"),
                    Text::from("Path"),
                    Text::from("Source"),
                    Text::from("Latency"),
                ])
                .rows(rows)
                .variant(log::TableVariant::Default)
                .build()
                .render()
        };

        let sse_body = if sse_rows.is_empty() {
            log::EmptyState::builder()
                .message(Text::from(
                    "No SSE pushes yet. Send a chat message to broadcast an update.",
                ))
                .build()
                .render()
        } else {
            let rows = sse_rows
                .iter()
                .rev()
                .take(20)
                .map(|entry| {
                    vec![
                        maud::html! { (&entry.timestamp) },
                        maud::html! { (&entry.message) },
                        maud::html! { (field_value_text(entry, &LogFieldName::from(LogFieldKey::Selector)).unwrap_or_else(|| Text::from("-"))) },
                        maud::html! { (field_value_text(entry, &LogFieldName::from(LogFieldKey::Mode)).unwrap_or_else(|| Text::from("-"))) },
                        maud::html! { (field_value_text(entry, &LogFieldName::from(LogFieldKey::PayloadBytes)).unwrap_or_else(|| Text::from("-"))) },
                    ]
                })
                .collect::<Vec<_>>();
            log::DataTable::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Event"),
                    Text::from("Selector"),
                    Text::from("Mode"),
                    Text::from("Payload (bytes)"),
                ])
                .rows(rows)
                .variant(log::TableVariant::Default)
                .build()
                .render()
        };

        maud::html! {
            section id="network-log-target" data-log-panels {
                (log::Styles.render())
                (log::Panel::builder()
                    .title(Text::from("HTTP requests"))
                    .body(request_body)
                    .build())
                (log::Panel::builder()
                    .title(Text::from("SSE pushes"))
                    .body(sse_body)
                    .build())
                (log::ChatFlow::builder()
                    .entries(&chat_rows)
                    .build())
                script {
                    (maud::PreEscaped(r#"
(() => {
  const root = document.getElementById('network-log-target');
  if (!root) return;
  const panels = root.querySelectorAll('[data-log-scroll]');
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

fn field_value(entry: &TraceEntry, name: &LogFieldName) -> log::FieldValue {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| log::FieldValue::from_log_value(Some(value)))
        .unwrap_or(log::FieldValue::Missing)
}

fn field_value_text(entry: &TraceEntry, name: &LogFieldName) -> Option<Text> {
    field_value(entry, name).into_option()
}

fn method_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, &LogFieldName::from(LogFieldKey::Method)).into_option() {
        Some(method) => Pill::method(method),
        None => Pill::fields("-"),
    }
}

fn path_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, &LogFieldName::from(LogFieldKey::Path)).into_option() {
        Some(path) => Pill::path(path),
        None => Pill::fields("-"),
    }
}

fn status_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, &LogFieldName::from(LogFieldKey::Status)).into_option() {
        Some(status) => Pill::status(status),
        None => Pill::fields("-"),
    }
}

fn latency_pill(entry: &TraceEntry) -> Option<Pill> {
    field_value(entry, &LogFieldName::from(LogFieldKey::LatencyMs))
        .into_option()
        .map(|value: Text| Pill::fields(format!("latency_ms={}", value)))
}

fn source_pill(entry: &TraceEntry) -> Pill {
    let sender: Option<Text> =
        field_value(entry, &LogFieldName::from(LogFieldKey::Sender)).into_option();
    match sender {
        Some(sender) => Pill::fields(format!("source={}", sender)),
        None => Pill::fields("source=unknown"),
    }
}
