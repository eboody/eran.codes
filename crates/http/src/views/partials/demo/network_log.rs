use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::LogPanel;

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
            maud::html! { p class="muted" { "No requests yet. Trigger a demo action to populate this table." } }
        } else {
            maud::html! {
                table {
                    thead {
                        tr {
                            th { "Time" }
                            th { "Source" }
                            th { "Method" }
                            th { "Path" }
                            th { "User" }
                            th { "Session" }
                            th { "Status" }
                            th { "Latency (ms)" }
                            th { "At" }
                        }
                    }
                    tbody {
                        @for entry in request_rows.iter().rev().take(20) {
                            tr {
                                td { (entry.timestamp.clone()) }
                                td { (source_badge(entry)) }
                                    td { (method_pill(entry)) }
                                    td { (path_pill(entry)) }
                                td { (field_value(entry, "user_id")) }
                                td { (field_value(entry, "session_id")) }
                                    td { (status_pill(entry)) }
                                td { (field_value(entry, "latency_ms")) }
                                td { (field_value(entry, "sent_at")) }
                            }
                        }
                    }
                }
            }
        };

        let sse_body = if sse_rows.is_empty() {
            maud::html! { p class="muted" { "No SSE pushes yet. Send a chat message to broadcast an update." } }
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
            maud::html! { p class="muted" { "No chat messages yet. Send a message to see request/response flow." } }
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

fn source_badge(entry: &TraceEntry) -> maud::Markup {
    let sender = field_value(entry, "sender");
    match ChatRequestSource::from_sender(&sender) {
        Some(source) => source.badge_with_mismatch(entry),
        None => maud::html! {},
    }
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

fn method_pill(entry: &TraceEntry) -> maud::Markup {
    let method = field_value(entry, "method");
    if method == "-" {
        return maud::html! { span class="muted" { "-" } };
    }
    maud::html! { span class=(format!("pill method {}", method_class(&method))) { (method) } }
}

fn path_pill(entry: &TraceEntry) -> maud::Markup {
    let path = field_value(entry, "path");
    if path == "-" {
        return maud::html! { span class="muted" { "-" } };
    }
    maud::html! { span class="pill path" { (path) } }
}

fn status_pill(entry: &TraceEntry) -> maud::Markup {
    let status = field_value(entry, "status");
    if status == "-" {
        return maud::html! { span class="muted" { "-" } };
    }
    maud::html! { span class=(format!("pill status {}", status_class(&status))) { (status) } }
}

fn method_class(method: &str) -> &'static str {
    match method {
        "GET" => "method-get",
        "POST" => "method-post",
        "PUT" => "method-put",
        "PATCH" => "method-patch",
        "DELETE" => "method-delete",
        _ => "method-other",
    }
}

fn status_class(status: &str) -> &'static str {
    if let Some(code) = status.parse::<u16>().ok() {
        if code >= 500 {
            return "status-5xx";
        }
        if code >= 400 {
            return "status-4xx";
        }
        if code >= 300 {
            return "status-3xx";
        }
        if code >= 200 {
            return "status-2xx";
        }
    }
    "status-unknown"
}

#[derive(Clone, Copy)]
enum ChatRequestSource {
    You,
    Demo,
}

impl ChatRequestSource {
    fn from_sender(sender: &str) -> Option<Self> {
        match sender {
            "you" => Some(Self::You),
            "demo" => Some(Self::Demo),
            _ => None,
        }
    }

    fn badge(self) -> maud::Markup {
        match self {
            ChatRequestSource::You => maud::html! { span class="badge" { "You" } },
            ChatRequestSource::Demo => {
                maud::html! { span class="badge secondary" { "Demo" } }
            }
        }
    }

    fn badge_with_mismatch(self, entry: &TraceEntry) -> maud::Markup {
        let badge = self.badge();
        let user_id = field_value(entry, "user_id");
        let expected = match self {
            ChatRequestSource::You => "you",
            ChatRequestSource::Demo => "demo",
        };
        let sender = field_value(entry, "sender");
        let warn = sender != expected;
        if warn {
            return maud::html! {
                (badge)
                " "
                span class="badge warning" { "Mismatch" }
            };
        }
        if user_id == "-" {
            return badge;
        }
        if warn {
            return maud::html! {
                (badge)
                " "
                span class="badge warning" { "Mismatch" }
            };
        }
        badge
    }
}
