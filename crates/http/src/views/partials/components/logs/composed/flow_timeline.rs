use bon::Builder;
use maud::Render;
use serde_json::json;

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
                nav class="ui-log-flow-list" aria-label="Recent request flows" {
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
                                div class="log-fields" {
                                    (Pill::fields(format!("request_id={}", flow.display_id)).render())
                                }
                            }

                            ol class="ui-log-entries" data-log-flow-events {
                                @for event in &flow.events {
                                    li class="ui-log-flow-event" data-log-flow-event {
                                        div class="ui-log-flow-event-head" {
                                            span data-log-timestamp { (&event.timestamp) }
                                            (Pill::fields(format!("stage={}", event.stage_label)).render())
                                        }
                                        p class="ui-log-flow-event-summary" { (&event.summary) }
                                        @if !event.pills.is_empty() {
                                            div class="log-fields" {
                                                @for pill in &event.pills {
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
}
