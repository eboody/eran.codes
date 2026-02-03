use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::LogPanel;

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
                        li class="log-entry" {
                            span class="muted log-timestamp" { (entry.timestamp.clone()) }
                            span class=(format!("pill log-level {}", level_class(&entry.level))) { (entry.level.clone()) }
                            span class="pill log-target" { (entry.target.clone()) }
                            span class="log-message" { (entry.message.clone()) }
                            @if let Some(fields) = compact_fields(entry) {
                                (fields)
                            }
                        }
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

fn compact_fields(entry: &TraceEntry) -> Option<maud::Markup> {
    if entry.fields.is_empty() {
        return None;
    }
    let mut parts: Vec<maud::Markup> = Vec::new();
    let mut extras: Vec<String> = Vec::new();
    for (name, value) in entry.fields.iter() {
        match name.as_str() {
            "method" => {
                parts.push(maud::html! {
                    span class=(format!("pill method {}", method_class(value))) { (value) }
                });
            }
            "path" => {
                parts.push(maud::html! { span class="pill path" { (value) } });
            }
            "status" => {
                parts.push(maud::html! { span class=(format!("pill status {}", status_class(value))) { (value) } });
            }
            _ => extras.push(format!("{}={}", name, value)),
        }
    }
    if !extras.is_empty() {
        let extra = extras.into_iter().take(2).collect::<Vec<_>>().join(" · ");
        parts.push(maud::html! { span class="pill log-fields" { (extra) } });
    }
    if parts.is_empty() {
        None
    } else {
        Some(maud::html! {
            span class="log-fields" {
                @for part in parts {
                    (part)
                }
            }
        })
    }
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
