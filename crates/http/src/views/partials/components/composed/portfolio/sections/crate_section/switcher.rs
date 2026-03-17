use maud::Render;

use super::showcase::CrateShowcase;
use crate::types::Text;
use crate::views::partials::components::portfolio::content::CrateCardContent;
use crate::views::partials::components::{Tab, TabInteraction};
use crate::views::proper_theme::THEME;

const ACTIVE_CRATE_SIGNAL: &str = "active_crate_id";

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-3);
}

me [data-portfolio-crate-switcher-nav] {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
}

me [data-portfolio-crate-switcher-nav] .tab-set__tab {
  width: fit-content;
  justify-content: center;
  padding:
    calc(var(--control-padding-block-compact) + var(--space-1) * 0.25)
    calc(var(--control-padding-inline-compact) + var(--space-1) * 0.5);
  border-radius: var(--radius-pill);
  border: var(--control-border-width) solid var(--border-default);
  background: color-mix(in srgb, var(--surface-field) 84%, var(--surface-panel));
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  color: var(--text-muted);
  font-size: var(--text-size-body-xs);
}

me [data-portfolio-crate-switcher-nav] .tab-set__tab-line {
  font-weight: 600;
}

me [data-portfolio-crate-switcher-nav] .tab-set__tab.is-selected {
  color: var(--text-body);
  border-color: color-mix(in srgb, var(--accent-signal) 36%, var(--border-default));
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 18%, transparent),
      transparent 70%
    ),
    color-mix(in srgb, var(--surface-panel) 92%, var(--accent-signal-soft));
  box-shadow:
    inset 0 1px 0 var(--surface-edge-default),
    0 0 0 1px color-mix(in srgb, var(--accent-signal) 10%, transparent);
}

me [data-portfolio-crate-switcher-nav] .tab-set__tab:focus-visible {
  outline: none;
  border-color: color-mix(in srgb, var(--accent-signal) 54%, var(--border-default));
  box-shadow:
    0 0 0 0.22rem color-mix(in srgb, var(--accent-signal) 18%, transparent),
    inset 0 1px 0 var(--surface-edge-default);
}

me [data-portfolio-crate-panel][hidden] {
  display: none;
}

me [data-portfolio-crate-panel] {
  align-content: start;
}

@media (prefers-color-scheme: dark) {
  me [data-portfolio-crate-switcher-nav] .tab-set__tab {
    background: color-mix(in srgb, var(--surface-field) 92%, black 8%);
    border-color: color-mix(in srgb, var(--border-default) 90%, transparent);
  }

  me [data-portfolio-crate-switcher-nav] .tab-set__tab.is-selected {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 44%),
      color-mix(in srgb, var(--accent-signal) 14%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--accent-signal) 40%, var(--border-default));
  }
}

@media (hover: hover) {
  me [data-portfolio-crate-switcher-nav] .tab-set__tab:hover {
    transform: translateY(-1px);
  }
}

@media (max-width: 48rem) {
  me {
    gap: var(--space-1);
  }

  me [data-portfolio-crate-switcher-nav] {
    gap: var(--space-1);
  }

  me [data-portfolio-crate-switcher-nav] .tab-set__tab {
    padding: var(--control-padding-block-compact) var(--control-padding-inline-compact);
  }
}
"#
);

pub(super) struct CrateShowcaseSwitcher<'a> {
    pub cards: &'a [CrateCardContent],
}

impl Render for CrateShowcaseSwitcher<'_> {
    fn render(&self) -> maud::Markup {
        let signal_name = Text::from(ACTIVE_CRATE_SIGNAL);
        let active_crate_id = tab_value(0);
        let signals = switcher_signals(&signal_name, &active_crate_id);

        maud::html! {
            section data-portfolio-crate-switcher data-signals=(signals) {
                (css())
                nav data-portfolio-crate-switcher-nav role="tablist" aria-label="Open source crate selection" {
                    @for (index, card) in self.cards.iter().enumerate() {
                        (Tab {
                            id: tab_dom_id(index),
                            controls: panel_dom_id(index),
                            palette: &THEME.gray,
                            is_selected: index == 0,
                            icon: None,
                            primary_text: card.name.clone(),
                            secondary_text: None,
                            interaction: TabInteraction::DatastarLocal {
                                signal: signal_name.clone(),
                                value: tab_value(index),
                            },
                        })
                    }
                }
                @for (index, card) in self.cards.iter().enumerate() {
                    (CrateShowcasePanel {
                        card,
                        signal_name: &signal_name,
                        tab_index: index,
                    })
                }
            }
        }
    }
}

struct CrateShowcasePanel<'a> {
    card: &'a CrateCardContent,
    signal_name: &'a Text,
    tab_index: usize,
}

impl Render for CrateShowcasePanel<'_> {
    fn render(&self) -> maud::Markup {
        let value = tab_value(self.tab_index);
        let show_expr = show_expr(self.signal_name, &value);
        let tabindex_expr = format!("{show_expr} ? '0' : '-1'");

        maud::html! {
            section
                id=(panel_dom_id(self.tab_index))
                class="ui-portfolio-crate-panel"
                role="tabpanel"
                aria-labelledby=(tab_dom_id(self.tab_index))
                data-portfolio-crate-panel
                data-show=(show_expr)
                data-attr:tabindex=(tabindex_expr) {
                (CrateShowcase { card: self.card })
            }
        }
    }
}

fn tab_value(index: usize) -> Text {
    Text::from(format!("crate_{index}"))
}

fn tab_dom_id(index: usize) -> Text {
    Text::from(format!("portfolio-crate-tab-{index}"))
}

fn panel_dom_id(index: usize) -> Text {
    Text::from(format!("portfolio-crate-panel-{index}"))
}

fn show_expr(signal_name: &Text, value: &Text) -> String {
    format!("${} == {}", signal_name, json_literal(value))
}

fn switcher_signals(signal_name: &Text, active_crate_id: &Text) -> String {
    let mut signals = serde_json::Map::new();
    signals.insert(
        signal_name.to_string(),
        serde_json::Value::String(active_crate_id.to_string()),
    );
    serde_json::Value::Object(signals).to_string()
}

fn json_literal(value: &Text) -> String {
    serde_json::to_string(&value.to_string()).unwrap_or_else(|_| "\"\"".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switcher_signals_use_json_encoding() {
        let signals = switcher_signals(&Text::from("active_crate_id"), &Text::from("crate'\"0"));
        assert_eq!(signals, "{\"active_crate_id\":\"crate'\\\"0\"}");
    }

    #[test]
    fn show_expression_uses_json_literal() {
        assert_eq!(
            show_expr(&Text::from("active_crate_id"), &Text::from("crate'\"1")),
            "$active_crate_id == \"crate'\\\"1\"",
        );
    }
}
