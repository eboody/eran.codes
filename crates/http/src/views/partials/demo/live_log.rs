use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{LogPanel, LogRow, Pill};

#[derive(Builder)]
pub struct LiveLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for LiveLog<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            maud::html! { p class="muted" { "No events yet. Trigger a demo action to start streaming." } }
        } else {
            maud::html! {
                ul class="live-log-entries" {
                    @for entry in self.entries.iter().rev().take(20) {
                        (LogRow::builder()
                            .timestamp(entry.timestamp.clone())
                            .message(entry.message.clone())
                            .pills(build_pills(entry))
                            .build()
                            .render())
                    }
                }
            }
        };

        maud::html! {
            section id="live-log-target" class="live-log-panels" {
                (LogPanel::builder()
                    .title("Live backend log".to_string())
                    .body(body)
                    .build()
                    .render())
                script {
                    (maud::PreEscaped(r#"
(() => {
  const root = document.getElementById('live-log-target');
  if (!root) return;
  const scroller = root.querySelector('.network-log-scroll');
  if (!scroller) return;
  const scroll = () => { scroller.scrollTop = scroller.scrollHeight; };
  requestAnimationFrame(scroll);
  const obs = new MutationObserver(scroll);
  obs.observe(scroller, { childList: true, subtree: true });
})();
                    "#))
                }
            }
        }
    }
}

fn build_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();
    pills.push(
        Pill::builder()
            .text(entry.level.clone())
            .extra_class(format!("log-level {}", level_class(&entry.level)))
            .build(),
    );
    if let Some(status) = field_value(entry, "status") {
        pills.push(
            Pill::builder()
                .text(status.clone())
                .extra_class(format!("status {}", status_class(&status)))
                .build(),
        );
    }
    if let Some(method) = field_value(entry, "method") {
        pills.push(
            Pill::builder()
                .text(method.clone())
                .extra_class(format!("method {}", method_class(&method)))
                .build(),
        );
    }
    if let Some(path) = field_value(entry, "path") {
        pills.push(
            Pill::builder()
                .text(path)
                .extra_class("path".to_string())
                .build(),
        );
    }
    pills.push(
        Pill::builder()
            .text(entry.target.clone())
            .extra_class("log-target".to_string())
            .build(),
    );
    pills.extend(compact_fields(entry));
    pills
}

fn compact_fields(entry: &TraceEntry) -> Vec<Pill> {
    if entry.fields.is_empty() {
        return Vec::new();
    }
    let mut parts: Vec<Pill> = Vec::new();
    let mut extras: Vec<String> = Vec::new();
    for (name, value) in entry.fields.iter() {
        match name.as_str() {
            "method" => {
                continue;
            }
            "path" => {
                continue;
            }
            "status" => {
                continue;
            }
            _ => extras.push(format!("{}={}", name, value)),
        }
    }
    if !extras.is_empty() {
        let extra = extras.into_iter().take(2).collect::<Vec<_>>().join(" · ");
        parts.push(
            Pill::builder()
                .text(extra)
                .extra_class("log-fields".to_string())
                .build(),
        );
    }
    parts
}

fn level_class(level: &str) -> &'static str {
    match level.to_ascii_lowercase().as_str() {
        "error" => "log-level-error",
        "warn" | "warning" => "log-level-warn",
        "debug" => "log-level-debug",
        "trace" => "log-level-trace",
        _ => "log-level-info",
    }
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

fn field_value(entry: &TraceEntry, name: &str) -> Option<String> {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| value.clone())
        .filter(|value| value != "-")
}
