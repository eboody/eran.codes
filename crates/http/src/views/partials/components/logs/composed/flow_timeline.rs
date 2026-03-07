use bon::Builder;
use maud::Render;

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

        maud::html! {
            div class="ui-log-flow-shell" data-log-flow-shell {
                nav class="ui-log-flow-list" aria-label="Recent request flows" {
                    @for (index, flow) in self.flows.iter().enumerate() {
                        a class=(item_class(index == 0))
                            data-log-flow-item
                            data-flow-id=(&flow.id)
                            href=(format!("#{}", flow.detail_id)) {
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
                        section
                            id=(&flow.detail_id)
                            class=(detail_class(index == 0))
                            data-flow-id=(&flow.id)
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

fn item_class(is_default: bool) -> &'static str {
    if is_default {
        "ui-log-flow-item is-default"
    } else {
        "ui-log-flow-item"
    }
}

fn detail_class(is_default: bool) -> &'static str {
    if is_default {
        "ui-log-flow-detail is-default"
    } else {
        "ui-log-flow-detail"
    }
}

fn status_pill(value: &Text) -> maud::Markup {
    Pill::status(value.clone()).render()
}
