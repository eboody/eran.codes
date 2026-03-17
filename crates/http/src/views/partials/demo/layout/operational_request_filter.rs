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
        maud::html! {
            section
                class="u-inset-card"
                data-op-filter
                data-op-filter-target=(self.target_id)
                data-signals="{operations_filter_query: ''}"
                data-init="@post('/api/operations/filter')" {
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
                        data-on:input__debounce="@post('/api/operations/filter'); window.scrollOperationalTimelineTop()";
                    (partials::button::Button::builder()
                        .label(Text::from("Clear"))
                        .variant(partials::button::Variant::Secondary)
                        .data_attrs(vec![
                            partials::button::DataAttr::flag("data-op-filter-clear"),
                            partials::button::DataAttr::value(
                                "data-on:click",
                                "$operations_filter_query = ''; @post('/api/operations/filter'); window.scrollOperationalTimelineTop()",
                            ),
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
