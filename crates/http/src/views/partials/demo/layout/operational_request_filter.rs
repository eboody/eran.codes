use bon::Builder;
use maud::{PreEscaped, Render};

use crate::types::Text;
use crate::views::partials;

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-3);
  margin-top: var(--space-2);
  padding: var(--space-card);
  overflow: visible;
}

me [data-op-filter-label] {
  margin: 0;
  font-size: var(--text-size-label-xs);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-op-filter-row] {
  display: grid;
  gap: var(--space-2);
  grid-template-columns: minmax(0, 1fr) auto;
}

me [data-op-filter-row] > input[type='text'] {
  margin: 0;
  min-width: 0;
}

me [data-op-filter-row] > [data-button] {
  margin: 0;
}

me [data-op-filter-meta] {
  margin: 0;
  font-size: var(--text-size-meta-md);
  color: var(--text-muted);
}

@media (max-width: 48rem) {
  me [data-op-filter-row] {
    grid-template-columns: 1fr;
  }

  me [data-op-filter-row] > [data-button] {
    width: 100%;
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct OperationalRequestFilter {
    pub target_id: &'static str,
}

impl Render for OperationalRequestFilter {
    fn render(&self) -> maud::Markup {
        let request_action = request_action();
        let request_and_scroll =
            format!("{request_action}; window.scrollOperationalTimelineTop()");
        let clear_action =
            format!("$operations_filter_query = ''; {request_and_scroll}");

        maud::html! {
            section
                class="u-inset-card"
                data-op-filter
                data-op-filter-target=(self.target_id)
                data-signals="{operations_filter_query: ''}"
                data-init=(request_action) {
                (css())
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

fn request_action() -> &'static str {
    "@post('/api/operations/filter', {filterSignals: {include: /^(operations_filter_query|sseTabId)$/}})"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_only_filter_query_and_sse_tab_id() {
        let markup = OperationalRequestFilter::builder()
            .target_id("network-log-target")
            .build()
            .render()
            .into_string();

        let request_action = request_action();

        assert!(markup.contains(request_action));
        assert!(markup.contains("$operations_filter_query = '';"));
        assert_eq!(markup.matches(request_action).count(), 3);
    }
}
