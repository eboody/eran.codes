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
                                th { "Status" }
                                th { "Latency (ms)" }
                            }
                        }
                        tbody {
                            @for entry in request_rows.iter().rev().take(20) {
                                tr {
                                    td { (entry.timestamp.clone()) }
                                    td { (source_badge(entry)) }
                                    td { (field_value(entry, "method")) }
                                    td { (field_value(entry, "path")) }
                                    td { (field_value(entry, "status")) }
                                    td { (field_value(entry, "latency_ms")) }
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
    let path = field_value(entry, "path");
    match ChatRequestSource::from_path(&path) {
        Some(source) => source.badge(),
        None => maud::html! {},
    }
}

#[derive(Clone, Copy)]
enum ChatRequestSource {
    You,
    Demo,
}

impl ChatRequestSource {
    fn from_path(path: &str) -> Option<Self> {
        match path {
            "/demo/chat/messages" => Some(Self::You),
            "/demo/chat/messages/demo" => Some(Self::Demo),
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
}
