use bon::Builder;
use maud::Render;
use serde_json::json;
use std::collections::BTreeMap;

use crate::types::Text;
use crate::views::partials::components::{Pill, logs};

#[derive(Clone, Debug, Builder)]
pub struct FlowTimeline {
    pub flows: Vec<Flow>,
}

impl Render for FlowTimeline {
    fn render(&self) -> maud::Markup {
        if self.flows.is_empty() {
            return logs::primitives::EmptyState::builder()
                .message(Text::from(
                    "No request flows yet. Trigger a demo action to trace request -> backend -> SSE.",
                ))
                .build()
                .render();
        }
        let initial_flow = self
            .flows
            .first()
            .map(|flow| flow.id.to_string())
            .unwrap_or_default();
        let local_signals = json!({ "active_flow_id": initial_flow }).to_string();

        maud::html! {
            div
                class="ui-log-flow-shell"
                data-log-flow-shell
                data-signals__ifmissing=(local_signals) {
                nav
                    class="ui-log-flow-list"
                    aria-label="Recent request flows"
                    data-log-flow-list {
                    @for (index, flow) in self.flows.iter().enumerate() {
                        @let selected_expr = selected_expr(&flow.id);
                        @let click_expr = click_expr(&flow.id);
                        @let flow_search = flow_search_text(flow);
                        button
                            type="button"
                            class=(item_class(index == 0))
                            data-class:is-selected=(selected_expr)
                            data-log-flow-item
                            data-flow-id=(&flow.id)
                            data-flow-search=(flow_search)
                            data-on:click=(click_expr) {
                            span class="ui-log-flow-item-id" { (&flow.display_id) }
                            span class="ui-log-flow-item-title" { (&flow.title) }
                            span class="ui-log-flow-item-meta" {
                                span class="ui-log-flow-item-time" { (&flow.latest_timestamp) }
                                @if let Some(status) = &flow.status {
                                    (status_pill(status))
                                }
                            }
                        }
                    }
                }

                div class="ui-log-flow-details" data-log-flow-details {
                    @for (index, flow) in self.flows.iter().enumerate() {
                        @let selected_expr = selected_expr(&flow.id);
                        @let flow_search = flow_search_text(flow);
                        section
                            id=(&flow.detail_id)
                            class=(detail_class(index == 0))
                            data-class:is-active=(selected_expr)
                            data-flow-id=(&flow.id)
                            data-flow-search=(flow_search)
                            data-log-flow-detail {
                            header class="ui-log-flow-detail-header" {
                                h4 class="ui-log-flow-detail-title" { (&flow.title) }
                                div class="ui-pill-cluster" {
                                    (Pill::fields(format!("request_id={}", flow.display_id)).render())
                                }
                            }

                            ol class="ui-log-entries" data-log-flow-events {
                                @for event in &flow.events {
                                    @let summary_markup = event_summary_markup(event);
                                    @let visible_pills = visible_event_pills(event);
                                    li class="ui-log-flow-event" data-log-flow-event {
                                        div class="ui-log-flow-event-head" {
                                            span data-log-timestamp { (&event.timestamp) }
                                            (Pill::fields(format!("stage={}", event.stage_label)).render())
                                        }
                                        p class=(event_summary_class(event)) {
                                            (summary_markup)
                                        }
                                        @if !visible_pills.is_empty() {
                                            div class="ui-pill-cluster" {
                                                @for pill in visible_pills {
                                                    (pill)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Flow {
    pub id: Text,
    pub detail_id: Text,
    pub display_id: Text,
    pub title: Text,
    pub latest_timestamp: Text,
    pub status: Option<Text>,
    pub events: Vec<FlowEvent>,
}

#[derive(Clone, Debug, Builder)]
pub struct FlowEvent {
    pub timestamp: Text,
    pub stage_label: Text,
    pub summary: Text,
    #[builder(default)]
    pub pills: Vec<Pill>,
}

fn status_pill(value: &Text) -> maud::Markup {
    Pill::status(value.clone()).render()
}

fn item_class(is_default: bool) -> &'static str {
    if is_default {
        "ui-log-flow-item is-selected"
    } else {
        "ui-log-flow-item"
    }
}

fn detail_class(is_default: bool) -> &'static str {
    if is_default {
        "ui-log-flow-detail is-active"
    } else {
        "ui-log-flow-detail"
    }
}

fn selected_expr(flow_id: &Text) -> String {
    format!("$active_flow_id == {}", json_literal(flow_id))
}

fn click_expr(flow_id: &Text) -> String {
    format!("$active_flow_id = {}", json_literal(flow_id))
}

fn json_literal(value: &Text) -> String {
    json!(value.to_string()).to_string()
}

pub fn flow_matches_any_search_term(flow: &Flow, terms: &[String]) -> bool {
    let search = flow_search_text(flow);
    terms.iter().any(|term| search.contains(term))
}

fn flow_search_text(flow: &Flow) -> String {
    let mut tokens: Vec<String> = vec![flow.display_id.to_string(), flow.title.to_string()];

    if let Some(status) = &flow.status {
        tokens.push(status.to_string());
    }

    for event in &flow.events {
        tokens.push(event.stage_label.to_string());
        tokens.push(event.summary.to_string());
        tokens.extend(event.pills.iter().map(|pill| pill.text.to_string()));
    }

    tokens.join(" ").to_lowercase()
}

fn event_summary_class(event: &FlowEvent) -> &'static str {
    if is_db_query_summary(&event.summary) {
        "ui-log-flow-event-summary ui-log-flow-event-summary-inline"
    } else {
        "ui-log-flow-event-summary"
    }
}

fn event_summary_markup(event: &FlowEvent) -> maud::Markup {
    if !is_db_query_summary(&event.summary) {
        return maud::html! { (&event.summary) };
    }

    let bind_pills = bind_pills_by_index(&event.pills);
    if bind_pills.is_empty() {
        return maud::html! { (&event.summary) };
    }

    let summary = event.summary.to_string();
    let parts = summary_parts_with_inline_bind_pills(summary.as_str(), &bind_pills);

    maud::html! {
        @for part in parts {
            @match part {
                SummaryPart::Text(text) => {
                    (text)
                }
                SummaryPart::Pill(pill) => {
                    (pill)
                }
            }
        }
    }
}

fn visible_event_pills(event: &FlowEvent) -> Vec<&Pill> {
    if !is_db_query_summary(&event.summary) {
        return event.pills.iter().collect();
    }
    event
        .pills
        .iter()
        .filter(|pill| bind_index(pill).is_none())
        .collect()
}

fn is_db_query_summary(summary: &Text) -> bool {
    let summary = summary.to_string();
    summary.starts_with("DB query:") || summary.starts_with("DB query complete:")
}

fn bind_pills_by_index(pills: &[Pill]) -> BTreeMap<usize, &Pill> {
    let mut binds = BTreeMap::new();
    for pill in pills {
        if let Some(index) = bind_index(pill) {
            binds.entry(index).or_insert(pill);
        }
    }
    binds
}

fn bind_index(pill: &Pill) -> Option<usize> {
    let text = pill.text.to_string();
    let rest = text.strip_prefix('$')?;
    let (index, _) = rest.split_once('=')?;
    index.parse::<usize>().ok()
}

enum SummaryPart<'a> {
    Text(String),
    Pill(&'a Pill),
}

fn summary_parts_with_inline_bind_pills<'a>(
    summary: &str,
    bind_pills: &'a BTreeMap<usize, &'a Pill>,
) -> Vec<SummaryPart<'a>> {
    let mut parts = Vec::new();
    let bytes = summary.as_bytes();
    let mut cursor = 0usize;
    let mut segment_start = 0usize;

    while cursor < bytes.len() {
        if bytes[cursor] == b'$' {
            let mut end = cursor + 1;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > cursor + 1
                && let Ok(index) = summary[cursor + 1..end].parse::<usize>()
                && let Some(pill) = bind_pills.get(&index)
            {
                if segment_start < cursor {
                    parts.push(SummaryPart::Text(
                        summary[segment_start..cursor].to_string(),
                    ));
                }
                parts.push(SummaryPart::Pill(*pill));
                segment_start = end;
                cursor = end;
                continue;
            }
        }
        cursor += 1;
    }

    if segment_start < summary.len() {
        parts.push(SummaryPart::Text(summary[segment_start..].to_string()));
    }

    if parts.is_empty() {
        parts.push(SummaryPart::Text(summary.to_string()));
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_search_text_includes_event_summary_and_pills() {
        let flow = Flow {
            id: Text::from("req-abc"),
            detail_id: Text::from("network-flow-req-abc"),
            display_id: Text::from("abc"),
            title: Text::from("Flow abc"),
            latest_timestamp: Text::from("12:00:00"),
            status: Some(Text::from("202")),
            events: vec![FlowEvent {
                timestamp: Text::from("12:00:00"),
                stage_label: Text::from("request"),
                summary: Text::from("HTTP GET /events started"),
                pills: vec![Pill::path("/events"), Pill::method("GET")],
            }],
        };

        let search = flow_search_text(&flow);

        assert!(search.contains("http get /events started"));
        assert!(search.contains("/events"));
        assert!(search.contains("get"));
        assert!(search.contains("202"));
    }

    #[test]
    fn db_summary_inlines_bind_pills_and_hides_them_from_extra_row() {
        let event = FlowEvent {
            timestamp: Text::from("12:00:00"),
            stage_label: Text::from("backend"),
            summary: Text::from(
                "DB query: SELECT id FROM chat_rooms WHERE id = $1 AND created_by = $2",
            ),
            pills: vec![
                Pill::fields("$1=room-1"),
                Pill::fields("$2=owner-1"),
                Pill::fields("sender=demo"),
            ],
        };

        let summary = event_summary_markup(&event).into_string();
        let visible_pills = visible_event_pills(&event);

        assert!(summary.contains("DB query: SELECT id FROM chat_rooms WHERE id = "));
        assert!(summary.contains("$1=room-1"));
        assert!(summary.contains("$2=owner-1"));
        assert_eq!(visible_pills.len(), 1);
        assert_eq!(visible_pills[0].text.to_string(), "sender=demo");
    }
}
