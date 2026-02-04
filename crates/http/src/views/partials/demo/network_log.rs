use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{ChatFlow, DataTable, EmptyState, LogPanel, LogRow, Pill, TableVariant};

#[derive(Builder)]
pub struct NetworkLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for NetworkLog<'_> {
    fn render(&self) -> maud::Markup {
        let request_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| entry.target == "demo.request" && entry.message == "request.end")
            .collect();
        let sse_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| entry.target == "demo.sse")
            .collect();
        let chat_rows: Vec<&TraceEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                entry.target == "demo.chat"
                    || (entry.target == "demo.sse"
                        && entry.message.contains("chat"))
            })
            .collect();

        let request_body = if request_rows.is_empty() {
            EmptyState::builder()
                .message("No requests yet. Trigger a demo action to populate this table.".to_string())
                .build()
                .render()
        } else {
            maud::html! {
                ul class="live-log-entries" {
                    @for entry in request_rows.iter().rev().take(20) {
                        (LogRow::builder()
                            .timestamp(entry.timestamp.clone())
                            .message(entry.message.clone())
                            .pills(request_pills(entry))
                            .build()
                            .render())
                    }
                }
            }
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
                        maud::html! { (field_value(entry, "selector")) },
                        maud::html! { (field_value(entry, "mode")) },
                        maud::html! { (field_value(entry, "payload_bytes")) },
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

fn field_value(entry: &TraceEntry, name: &str) -> String {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| value.clone())
        .unwrap_or_else(|| "-".to_string())
}

fn request_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();
    pills.push(Pill::level(entry.level.clone()));
    pills.push(status_pill(entry));
    pills.push(method_pill(entry));
    pills.push(path_pill(entry));
    pills.push(source_pill(entry));
    if let Some(latency) = latency_pill(entry) {
        pills.push(latency);
    }
    if let Some(user) = user_pill(entry) {
        pills.push(user);
    }
    pills
}

fn method_pill(entry: &TraceEntry) -> Pill {
    let method = field_value(entry, "method");
    if method == "-" {
        return Pill::fields("-".to_string());
    }
    Pill::method(method)
}

fn path_pill(entry: &TraceEntry) -> Pill {
    let path = field_value(entry, "path");
    if path == "-" {
        return Pill::fields("-".to_string());
    }
    Pill::path(path)
}

fn status_pill(entry: &TraceEntry) -> Pill {
    let status = field_value(entry, "status");
    if status == "-" {
        return Pill::fields("-".to_string());
    }
    Pill::status(status)
}

fn latency_pill(entry: &TraceEntry) -> Option<Pill> {
    let value = field_value(entry, "latency_ms");
    if value == "-" {
        return None;
    }
    Some(Pill::fields(format!("latency_ms={value}")))
}

fn user_pill(entry: &TraceEntry) -> Option<Pill> {
    let value = field_value(entry, "user_id");
    if value == "-" {
        return None;
    }
    let short = value.split('-').next().unwrap_or(value.as_str());
    Some(Pill::fields(format!("user={short}")))
}

fn source_pill(entry: &TraceEntry) -> Pill {
    let sender = field_value(entry, "sender");
    let label = if sender == "-" { "unknown" } else { &sender };
    Pill::fields(format!("source={label}"))
}
