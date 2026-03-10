use bon::Builder;
use maud::Render;

use crate::types::Text;

use super::SectionHeader;

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
        maud::html! {
            section
                id="request-burst-demo"
                class="ui-surface-card ui-lab-burst"
                data-request-burst-root
                data-endpoint=(&self.endpoint)
                data-concurrency=(self.concurrency)
            {
                (SectionHeader::builder()
                    .title(Text::from("High-Volume Request Burst"))
                    .subtitle(Text::from(
                        "Use the slider to send a large burst of requests from this browser and watch live request logs and SSE updates in real time.",
                    ))
                    .build())
                div class="ui-burst-controls" {
                    label class="ui-burst-slider" {
                        span { "Request count" }
                        input
                            type="range"
                            data-burst-count
                            min=(self.min_requests)
                            max=(self.max_requests)
                            step=(self.request_step)
                            value=(self.default_requests);
                    }
                    p class="ui-burst-selected" {
                        "Burst size: "
                        strong data-burst-count-label { (self.default_requests) }
                        " requests"
                    }
                    div class="ui-burst-actions" {
                        button type="button" data-burst-run { "Send burst" }
                        p data-muted {
                            "Concurrency: "
                            strong { (self.concurrency) }
                            " workers"
                        }
                    }
                    p class="ui-burst-result" data-burst-result {
                        "Ready. Choose a burst size and run the load."
                    }
                }
                script src="/static/request-burst.js" {}
            }
        }
    }
}
