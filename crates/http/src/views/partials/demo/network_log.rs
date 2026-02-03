use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{BadgeKind, EmptyState, LogPanel, LogRow, Pill};

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
            maud::html! {
                table {
                    thead {
                        tr {
                            th { "Time" }
                            th { "Event" }
                            th { "Selector" }
                            th { "Mode" }
                            th { "Payload (bytes)" }
                        }
                    }
                    tbody {
                        @for entry in sse_rows.iter().rev().take(20) {
                            tr {
                                td { (entry.timestamp.clone()) }
                                td { (entry.message.clone()) }
                                td { (field_value(entry, "selector")) }
                                td { (field_value(entry, "mode")) }
                                td { (field_value(entry, "payload_bytes")) }
                            }
                        }
                    }
                }
            }
        };

        let chat_body = if chat_rows.is_empty() {
            EmptyState::builder()
                .message("No chat messages yet. Send a message to see request/response flow.".to_string())
                .build()
                .render()
        } else {
            maud::html! {
                table {
                    thead {
                        tr {
                            th { "Time" }
                            th { "Direction" }
                            th { "Sender" }
                            th { "Receiver" }
                            th { "User" }
                            th { "Body" }
                        }
                    }
                    tbody {
                        @for entry in chat_rows.iter().rev().take(20) {
                            tr {
                                td { (entry.timestamp.clone()) }
                                td { (field_value(entry, "direction")) }
                                td { (sender_label(entry)) }
                                td { (field_value(entry, "receiver")) }
                                td { (user_label(entry)) }
                                td { (field_value(entry, "body")) }
                            }
                        }
                    }
                }
            }
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
                (LogPanel::builder()
                    .title("Chat message flow".to_string())
                    .body(chat_body)
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

fn user_label(entry: &TraceEntry) -> maud::Markup {
    let user_id = field_value(entry, "user_id");
    if user_id == "-" {
        return maud::html! { span class="muted" { "unknown" } };
    }
    let short_id = user_id
        .split('-')
        .next()
        .unwrap_or(user_id.as_str());
    let sender = field_value(entry, "sender");
    if sender == "you" {
        maud::html! { span class="badge you" { "You (" (short_id) ")" } }
    } else if sender == "demo" {
        maud::html! { span class="badge demo" { "Demo (" (short_id) ")" } }
    } else {
        maud::html! { span class="badge secondary" { "User (" (short_id) ")" } }
    }
}

fn sender_label(entry: &TraceEntry) -> maud::Markup {
    let sender = field_value(entry, "sender");
    let label = match sender.as_str() {
        "you" => "You",
        "demo" => "Demo",
        "-" => "Unknown",
        _ => "User",
    };
    maud::html! {
        span class=(sender_badge_class(&sender)) { (label) }
    }
}

fn sender_badge_class(sender: &str) -> &'static str {
    match sender {
        "you" => "badge you",
        "demo" => "badge demo",
        "-" => "badge secondary",
        _ => "badge secondary",
    }
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
    let kind = match sender.as_str() {
        "you" => BadgeKind::You,
        "demo" => BadgeKind::Demo,
        _ => BadgeKind::Secondary,
    };
    Pill::badge(sender, kind)
}
