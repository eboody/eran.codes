use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{ChatFlow, DataTable, EmptyState, FieldValue, LogPanel, Pill, TableVariant};
use crate::trace_log::{LogMessageKind, LogTargetKind};

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
                matches!(LogTargetKind::from_str(&entry.target), LogTargetKind::DemoRequest)
                    && matches!(LogMessageKind::from_str(&entry.message), LogMessageKind::RequestEnd)
            })
            .collect();
        let sse_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| matches!(LogTargetKind::from_str(&entry.target), LogTargetKind::DemoSse))
            .collect();
        let chat_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                let target_kind = LogTargetKind::from_str(&entry.target);
                let message_kind = LogMessageKind::from_str(&entry.message);
                matches!(
                    (target_kind, message_kind),
                    (LogTargetKind::DemoChat, LogMessageKind::ChatMessageIncoming)
                        | (LogTargetKind::DemoSse, LogMessageKind::ChatMessageBroadcast)
                )
            })
            .collect();

        let request_body = if request_rows.is_empty() {
            EmptyState::builder()
                .message("No requests yet. Trigger a demo action to populate this table.".to_string())
                .build()
                .render()
        } else {
            let rows = request_rows
                .iter()
                .rev()
                .take(20)
                .map(|entry| {
                    vec![
                        maud::html! { (entry.timestamp.clone()) },
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
                    "Time".to_string(),
                    "Status".to_string(),
                    "Method".to_string(),
                    "Path".to_string(),
                    "Source".to_string(),
                    "Latency".to_string(),
                ])
                .rows(rows)
                .variant(TableVariant::Default)
                .build()
                .render()
        };

        let sse_body = if sse_rows.is_empty() {
            EmptyState::builder()
                .message("No SSE pushes yet. Send a chat message to broadcast an update.".to_string())
                .build()
                .render()
        } else {
            let rows = sse_rows
                .iter()
                .rev()
                .take(20)
                .map(|entry| {
                    vec![
                        maud::html! { (entry.timestamp.clone()) },
                        maud::html! { (entry.message.clone()) },
                        maud::html! { (field_value_text(entry, "selector")) },
                        maud::html! { (field_value_text(entry, "mode")) },
                        maud::html! { (field_value_text(entry, "payload_bytes")) },
                    ]
                })
                .collect::<Vec<_>>();
            DataTable::builder()
                .headers(vec![
                    "Time".to_string(),
                    "Event".to_string(),
                    "Selector".to_string(),
                    "Mode".to_string(),
                    "Payload (bytes)".to_string(),
                ])
                .rows(rows)
                .variant(TableVariant::Default)
                .build()
                .render()
        };

        maud::html! {
            section id="network-log-target" class="network-log-panels" {
                (LogPanel::builder()
                    .title("HTTP requests".to_string())
                    .body(request_body)
                    .build()
                    .render())
                (LogPanel::builder()
                    .title("SSE pushes".to_string())
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

fn field_value(entry: &TraceEntry, name: &str) -> FieldValue {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| FieldValue::from_str(value))
        .unwrap_or(FieldValue::Missing)
}

fn field_value_text(entry: &TraceEntry, name: &str) -> String {
    field_value(entry, name)
        .into_option()
        .unwrap_or_else(|| "-".to_string())
}

fn method_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, "method").into_option() {
        Some(method) => Pill::method(method),
        None => Pill::fields("-".to_string()),
    }
}

fn path_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, "path").into_option() {
        Some(path) => Pill::path(path),
        None => Pill::fields("-".to_string()),
    }
}

fn status_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, "status").into_option() {
        Some(status) => Pill::status(status),
        None => Pill::fields("-".to_string()),
    }
}

fn latency_pill(entry: &TraceEntry) -> Option<Pill> {
    field_value(entry, "latency_ms")
        .into_option()
        .map(|value| Pill::fields(format!("latency_ms={value}")))
}

fn source_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, "sender").into_option() {
        Some(sender) => Pill::fields(format!("source={sender}")),
        None => Pill::fields("source=unknown".to_string()),
    }
}
