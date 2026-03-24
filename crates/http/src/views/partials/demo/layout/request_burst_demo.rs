use bon::Builder;
use maud::{PreEscaped, Render};

use crate::types::Text;
use crate::views::partials;

use super::{SurfaceSection, SurfaceSectionAttr};

const REQUEST_BURST_SCRIPT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/request-burst.js"
));

crate::views::scoped::inline_css!(
    r#"
me [data-burst-controls] {
  display: grid;
  gap: var(--space-4);
}

me [data-burst-slider] {
  display: grid;
  gap: var(--space-2);
  font-size: var(--control-font-size);
  font-weight: 600;
}

me [data-burst-slider] > span {
  font-size: var(--text-size-label-xs);
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-burst-slider] input[type='range'] {
  width: 100%;
  margin: 0;
  accent-color: var(--ui-accent-primary);
}

me [data-burst-selected] {
  margin: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: var(--interactive-bleed);
  font-family: var(--font-display);
  font-size: var(--text-size-title-sm);
  color: var(--text-body);
}

me [data-burst-selected] strong {
  color: var(--text-strong);
}

me [data-burst-actions] {
  align-items: center;
  gap: var(--space-2) var(--space-3);
}

me [data-burst-actions-note] {
  margin: 0;
  font-size: var(--text-size-meta-lg);
}

me [data-burst-result] {
  margin: 0;
  border: 1px solid color-mix(in srgb, var(--accent-signal) 18%, var(--border-default));
  border-radius: var(--ui-radius-sm);
  padding: var(--space-3) var(--space-4);
  background: color-mix(in srgb, var(--accent-signal-soft) 38%, var(--surface-field));
  font-family: var(--ui-font-mono);
  font-size: var(--text-size-meta-sm);
  line-height: var(--text-line-body-loose);
  color: var(--text-body);
  overflow: visible;
}

me [data-burst-status] {
  margin: 0 0 var(--space-3);
}

me [data-burst-metrics] {
  margin: 0;
  display: grid;
  gap: var(--space-3) var(--space-4);
  grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
}

me [data-burst-metrics] > div {
  min-width: 0;
  display: grid;
  gap: var(--space-1);
}

me [data-burst-metrics] dt {
  font-size: var(--text-size-label-xs);
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-burst-metrics] dd {
  margin: 0;
  color: var(--text-strong);
}

me [data-burst-endpoint],
me [data-burst-previous],
me [data-burst-delta] {
  white-space: normal;
  overflow-wrap: anywhere;
}

@media (prefers-color-scheme: dark) {
  me [data-burst-result] {
    background:
      linear-gradient(
        180deg,
        color-mix(in srgb, var(--accent-signal) 10%, var(--surface-wash-top-soft)),
        transparent 30%
      ),
      color-mix(in srgb, var(--accent-signal) 14%, var(--surface-field));
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }
}

@media (min-width: 48rem) {
  me [data-burst-controls] {
    grid-template-columns: minmax(0, 1.35fr) minmax(14rem, 0.95fr);
    align-items: end;
    column-gap: var(--space-card);
  }

  me [data-burst-slider],
  me [data-burst-result] {
    grid-column: 1 / -1;
  }

  me [data-burst-actions] {
    justify-content: flex-end;
    align-self: end;
  }
}

@media (max-width: 48rem) {
  me [data-burst-controls] {
    gap: var(--space-3);
  }

  me [data-burst-selected] {
    font-size: var(--text-size-title-xs);
  }

  me [data-burst-actions] {
    align-items: stretch;
  }

  me [data-burst-actions] [data-button] {
    width: 100%;
  }

  me [data-burst-result] {
    padding: var(--space-3);
  }

  me [data-burst-status] {
    margin-bottom: var(--space-2);
  }

  me [data-burst-metrics] {
    gap: var(--space-2) var(--space-3);
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
"#
);

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
            .title(Text::from("High-Volume Request Burst"))
            .subtitle(Text::from(
                "Use the slider to send a large burst of requests from this browser and watch live request logs and SSE updates in real time.",
            ))
            .attrs(vec![
                SurfaceSectionAttr::flag("data-request-burst-root"),
                SurfaceSectionAttr::value("data-endpoint", self.endpoint.clone()),
                SurfaceSectionAttr::value("data-concurrency", self.concurrency.to_string()),
            ])
            .content(maud::html! {
                (css())
                div data-burst-controls {
                    label data-burst-slider {
                        span { "Request count" }
                        input
                            type="range"
                            data-burst-count
                            min=(self.min_requests)
                            max=(self.max_requests)
                            step=(self.request_step)
                            value=(self.default_requests);
                    }
                    p data-burst-selected {
                        "Burst size: "
                        strong data-burst-count-label { (self.default_requests) }
                        " requests"
                    }
                    div class="ui-button-row" data-burst-actions {
                        (partials::button::Button::builder()
                            .label(Text::from("Send burst"))
                            .data_attrs(vec![partials::button::DataAttr::flag("data-burst-run")])
                            .build())
                        p class="u-muted" data-burst-actions-note {
                            "Browser-observed latency and throughput"
                        }
                    }
                    div data-burst-result {
                        p data-burst-status {
                            "Ready. Choose a burst size and run the load."
                        }
                        dl data-burst-metrics {
                            div {
                                dt { "Endpoint" }
                                dd data-burst-endpoint { (&self.endpoint) }
                            }
                            div {
                                dt { "Workers" }
                                dd data-burst-workers { (self.concurrency) }
                            }
                            div {
                                dt { "Throughput" }
                                dd data-burst-rate { "—" }
                            }
                            div {
                                dt { "Duration" }
                                dd data-burst-duration { "—" }
                            }
                            div {
                                dt { "OK" }
                                dd data-burst-ok { "—" }
                            }
                            div {
                                dt { "Failed" }
                                dd data-burst-failed { "—" }
                            }
                            div {
                                dt { "Latency p50" }
                                dd data-burst-p50 { "—" }
                            }
                            div {
                                dt { "Latency p95" }
                                dd data-burst-p95 { "—" }
                            }
                            div {
                                dt { "Latency p99" }
                                dd data-burst-p99 { "—" }
                            }
                            div {
                                dt { "Baseline" }
                                dd data-burst-previous { "Run once to set a baseline." }
                            }
                            div {
                                dt { "Delta" }
                                dd data-burst-delta { "—" }
                            }
                        }
                    }
                }
                script { (PreEscaped(REQUEST_BURST_SCRIPT)) }
            })
            .build()
            .render()
    }
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

        assert!(markup.contains("Latency p95"));
        assert!(markup.contains("data-burst-endpoint"));
        assert!(markup.contains("data-burst-delta"));
        assert!(markup.contains("/partials/request-burst-probe"));
    }
}
