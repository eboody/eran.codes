use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;

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

        maud::html! {
            article id="network-log-target" class="demo-result" {
                p { strong { "Live network log" } }
                @if request_rows.is_empty() {
                    p class="muted" { "No requests yet. Trigger a demo action to populate this table." }
                } @else {
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
                                    td { (field_value(entry, "method")) }
                                    td { (field_value(entry, "path")) }
                                    td { (field_value(entry, "user_id")) }
                                    td { (field_value(entry, "session_id")) }
                                    td { (field_value(entry, "status")) }
                                    td { (field_value(entry, "latency_ms")) }
                                    td { (field_value(entry, "sent_at")) }
                                }
                            }
                        }
                    }
                }
                @if sse_rows.is_empty() {
                    p class="muted" { "No SSE pushes yet. Send a chat message to broadcast an update." }
                } @else {
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
                @if chat_rows.is_empty() {
                    p class="muted" { "No chat messages yet. Send a message to see request/response flow." }
                } @else {
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
