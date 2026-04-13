mod styles;
#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

const OPERATIONAL_TIMELINE_SCROLL_ASSET_URL: &str =
    "/static/operational-timeline-scroll.js?v=20260328-runtime-ownership";

#[derive(Clone, Debug, Builder)]
pub struct OperationalRequestFilter {
    pub target_id: &'static str,
}

impl Render for OperationalRequestFilter {
    fn render(&self) -> maud::Markup {
        let request_action = request_action();
        let request_and_scroll =
            format!("{request_action}; window.scrollOperationalTimelineTop()");
        let clear_action = format!("$operations_filter_query = ''; {request_and_scroll}");

        maud::html! {
            section
                class="u-inset-card"
                data-op-filter
                data-op-filter-target=(self.target_id)
                data-signals="{operations_filter_query: ''}"
                data-init=(request_action) {
                (styles::render())
                label data-op-filter-label for="operations-filter-query" {
                    "Filter out requests containing"
                }
                div data-op-filter-row {
                    input
                        id="operations-filter-query"
                        type="text"
                        placeholder="/events, /health, /partials/request-burst-probe"
                        autocomplete="off"
                        data-op-filter-query
                        data-bind="operations_filter_query"
                        data-on:input__debounce=(request_and_scroll);
                    (partials::button::Button::builder()
                        .label(Text::from("Clear"))
                        .variant(partials::button::Variant::Secondary)
                        .data_attrs(vec![
                            partials::button::DataAttr::flag("data-op-filter-clear"),
                            partials::button::DataAttr::value("data-on:click", clear_action),
                        ])
                        .build())
                }
                p data-op-filter-meta data-op-filter-status {
                    "Debounced command updates server-side request flow filtering."
                }
            }
            script src=(OPERATIONAL_TIMELINE_SCROLL_ASSET_URL) {}
        }
    }
}

fn request_action() -> &'static str {
    "@post('/api/operations/filter', {filterSignals: {include: /^(operations_filter_query|sseTabId)$/}})"
}
