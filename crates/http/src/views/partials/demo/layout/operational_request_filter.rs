use bon::Builder;
use maud::{PreEscaped, Render};

#[derive(Clone, Debug, Builder)]
pub struct OperationalRequestFilter {
    pub target_id: &'static str,
}

impl Render for OperationalRequestFilter {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section
                class="ui-op-filter"
                data-op-filter
                data-op-filter-target=(self.target_id)
                data-signals="{operations_filter_query: ''}"
                data-init="@post('/api/operations/filter')" {
                label class="ui-op-filter-label" for="operations-filter-query" {
                    "Filter out requests containing"
                }
                div class="ui-op-filter-row" {
                    input
                        id="operations-filter-query"
                        type="text"
                        placeholder="/events, /health, /partials/request-burst-probe"
                        autocomplete="off"
                        data-op-filter-query
                        data-bind="operations_filter_query"
                        data-on:input__debounce="@post('/api/operations/filter'); window.scrollOperationalTimelineTop()";
                    button
                        class="secondary"
                        type="button"
                        data-op-filter-clear
                        data-on:click="$operations_filter_query = ''; @post('/api/operations/filter'); window.scrollOperationalTimelineTop()" {
                        "Clear"
                    }
                }
                p class="ui-op-filter-meta" data-op-filter-status {
                    "Debounced command updates server-side request flow filtering."
                }
            }
            script {
                (PreEscaped(
                    r#"
(() => {
  if (typeof window.scrollOperationalTimelineTop === 'function') return;
  window.scrollOperationalTimelineTop = () => {
    const target = document.getElementById('network-log-target');
    if (!target) return;
    const flowList = target.querySelector('[data-log-flow-list]');
    if (flowList instanceof HTMLElement) {
      flowList.scrollTop = 0;
      flowList.scrollIntoView({ behavior: 'smooth', block: 'start' });
      return;
    }
    target.scrollIntoView({ behavior: 'smooth', block: 'start' });
  };
})();
"#,
                ))
            }
        }
    }
}
