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
                                span class="pill log-fields" { (fields) }
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

fn compact_fields(entry: &TraceEntry) -> Option<String> {
    if entry.fields.is_empty() {
        return None;
    }
    let mut parts = Vec::new();
    for (name, value) in entry.fields.iter().take(3) {
        parts.push(format!("{}={}", name, value));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
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
