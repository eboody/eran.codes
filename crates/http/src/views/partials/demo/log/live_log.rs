use std::str::FromStr;

use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, LogFieldName, Text};
use crate::views::partials::components::Pill;
use crate::views::partials::demo::log;

#[derive(Builder)]
pub struct LiveLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for LiveLog<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            log::EmptyState::builder()
                .message(Text::from(
                    "No events yet. Trigger a demo action to start streaming.",
                ))
                .build()
                .render()
        } else {
            let grouped = group_by_request(self.entries.iter().rev().take(40));
            maud::html! {
                div data-log-groups {
                    @for group in grouped {
                        div data-log-group {
                            div data-log-group-header {
                                @if let Some(request_id) = &group.request_id {
                                    (Pill::fields(format!("request_id={}", short_request_id(request_id))).render())
                                } @else {
                                    (Pill::fields("request_id=unknown").render())
                                }
                                span data-muted { (format!("{} events", group.entries.len())) }
                            }
                            ul data-live-log-entries {
                                @for entry in group.entries {
                                    (log::Row::builder()
                                        .timestamp(Text::from(entry.timestamp.clone()))
                                        .message(Text::from(entry.message.clone()))
                                        .pills(build_pills(entry))
                                        .build())
                                }
                            }
                        }
                    }
                }
            }
        };

        maud::html! {
            section id="live-log-target" data-live-log {
                (log::Styles.render())
                (log::Panel::builder()
                    .title(Text::from("Live backend log"))
                    .body(body)
                    .build())
                script {
                    (maud::PreEscaped(r#"
(() => {
  const root = document.getElementById('live-log-target');
  if (!root) return;
  const scroller = root.querySelector('[data-log-scroll]');
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

struct LogGroup<'a> {
    request_id: Option<Text>,
    entries: Vec<&'a TraceEntry>,
}

fn group_by_request<'a, I>(entries: I) -> Vec<LogGroup<'a>>
where
    I: IntoIterator<Item = &'a TraceEntry>,
{
    let mut groups: Vec<LogGroup<'a>> = Vec::new();
    let mut order: Vec<Option<Text>> = Vec::new();
    let mut map: std::collections::HashMap<Option<Text>, Vec<&'a TraceEntry>> =
        std::collections::HashMap::new();
    for entry in entries {
        let request_id = field_value(entry, &LogFieldName::from(LogFieldKey::RequestId));
        if !map.contains_key(&request_id) {
            order.push(request_id.clone());
        }
        map.entry(request_id).or_default().push(entry);
    }
    for key in order {
        if let Some(entries) = map.remove(&key) {
            groups.push(LogGroup {
                request_id: key,
                entries,
            });
        }
    }
    groups
}

fn short_request_id(value: &Text) -> String {
    let value = value.to_string();
    value
        .split('-')
        .next()
        .unwrap_or(value.as_str())
        .to_string()
}

fn build_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();
    pills.push(Pill::level(entry.level.clone()));
    if let Some(status) = field_value(entry, &LogFieldName::from(LogFieldKey::Status)) {
        pills.push(Pill::status(status.clone()));
    }
    if let Some(method) = field_value(entry, &LogFieldName::from(LogFieldKey::Method)) {
        pills.push(Pill::method(method.clone()));
    }
    if let Some(path) = field_value(entry, &LogFieldName::from(LogFieldKey::Path)) {
        pills.push(Pill::path(path));
    }
    pills.push(Pill::target(entry.target.clone()));
    pills.extend(compact_fields(entry));
    pills
}

fn compact_fields(entry: &TraceEntry) -> Vec<Pill> {
    if entry.fields.is_empty() {
        return Vec::new();
    }
    entry
        .fields
        .iter()
        .filter_map(|(name, value)| {
            let field_kind = LogFieldKey::from_str(&name.to_string()).ok();
            if matches!(
                field_kind,
                Some(LogFieldKey::Method | LogFieldKey::Path | LogFieldKey::Status)
            ) {
                return None;
            }
            let extra = CompactField {
                name: name.clone(),
                value: Text::from(value.to_string()),
            };
            Some(Pill::fields(extra.render()))
        })
        .take(2)
        .collect()
}

struct CompactField {
    name: LogFieldName,
    value: Text,
}

impl CompactField {
    fn render(&self) -> Text {
        Text::from(format!("{}={}", self.name, self.value))
    }
}

fn field_value(entry: &TraceEntry, name: &LogFieldName) -> Option<Text> {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| log::FieldValue::from_log_value(Some(value)))
        .and_then(|value| value.into_option())
}
