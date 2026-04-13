mod metrics;
mod styles;

use bon::Builder;
use maud::{Markup, Render};

use crate::types::Text;
use crate::views::partials::{self, components};

use super::{Attr, SurfaceSection};

const REQUEST_BURST_ASSET_URL: &str =
    "/static/request-burst.js?v=20260328-runtime-ownership";

#[derive(Clone, Debug, Builder)]
pub struct RequestBurstDemo {
    pub endpoint: Text,
    #[builder(default = 100)]
    pub min_requests: usize,
    #[builder(default = 5000)]
    pub max_requests: usize,
    #[builder(default = 100)]
    pub request_step: usize,
    #[builder(default = 1000)]
    pub default_requests: usize,
    #[builder(default = 24)]
    pub concurrency: usize,
}

impl Render for RequestBurstDemo {
    fn render(&self) -> maud::Markup {
        SurfaceSection::builder()
            .id(Text::from("request-burst-demo"))
            .title(Text::from("Load harness"))
            .subtitle(Text::from(
                "Send a controlled burst from this browser and compare throughput, failures, and latency without leaving the page.",
            ))
            .attrs(vec![
                Attr::flag("data-request-burst-root"),
                Attr::value("data-endpoint", self.endpoint.clone()),
                Attr::value("data-concurrency", self.concurrency.to_string()),
            ])
            .content(maud::html! {
                (styles::render())
                div data-burst-controls {
                    (render_slider(self))
                    (render_selected_count(self.default_requests))
                    (render_actions())
                    (render_result_card(&self.endpoint, self.concurrency))
                }
                script src=(REQUEST_BURST_ASSET_URL) {}
            })
            .build()
            .render()
    }
}

fn render_slider(component: &RequestBurstDemo) -> Markup {
    maud::html! {
        label data-burst-slider {
            span { "Request count" }
            input
                type="range"
                data-burst-count
                min=(component.min_requests)
                max=(component.max_requests)
                step=(component.request_step)
                value=(component.default_requests);
        }
    }
}

fn render_selected_count(default_requests: usize) -> Markup {
    maud::html! {
        p data-burst-selected {
            "Burst size: "
            strong data-burst-count-label { (default_requests) }
            " requests"
        }
    }
}

fn render_actions() -> Markup {
    maud::html! {
        div
            class="ui-button-row"
            data-burst-actions
            data-button-row-frame="contained" {
            (partials::button::Button::builder()
                .label(Text::from("Send burst"))
                .data_attrs(vec![partials::button::DataAttr::flag("data-burst-run")])
                .build())
            p class="u-muted" data-burst-actions-note {
                "Browser-observed latency and throughput"
            }
        }
    }
}

fn render_result_card(endpoint: &Text, concurrency: usize) -> Markup {
    components::ResultCard::builder()
        .extra_class("ui-request-burst-result")
        .content(maud::html! {
            p class="ui-request-burst-status" data-burst-status {
                "Ready. Choose a burst size and run the load."
            }
            (components::KeyValueList::builder()
                .layout(components::KeyValueListLayout::MetricsGrid)
                .items(metrics::burst_metrics(endpoint.clone(), concurrency))
                .build())
        })
        .build()
        .render()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_metrics_panel() {
        let markup = RequestBurstDemo::builder()
            .endpoint(Text::from("/partials/request-burst-probe"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("Load harness"));
        assert!(markup.contains("Latency p95"));
        assert!(markup.contains("data-burst-endpoint"));
        assert!(markup.contains("data-burst-delta"));
        assert!(markup.contains("/partials/request-burst-probe"));
        assert!(markup.contains("u-demo-result-card"));
        assert!(markup.contains("data-key-value-layout=\"metrics-grid\""));
        assert!(markup.contains("data-button-row-frame=\"contained\""));
        assert!(markup.contains("/static/request-burst.js"));
    }
}
