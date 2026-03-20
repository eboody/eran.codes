use maud::Render;

use super::showcase::CrateShowcase;
use crate::types::Text;
use crate::views::partials::components::portfolio::content::CrateCardContent;
use crate::views::partials::components::{Tab, TabInteraction};
use crate::views::proper_theme::THEME;

crate::views::scoped::inline_css!(
    r#"
me {
  --_crate-switcher-panel-enter-offset: calc(var(--space-2) * 0.5);

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

me [data-portfolio-crate-panel][data-local-tab-entering] {
  animation: crate-switcher-panel-enter var(--motion-standard) var(--ease-3);
  transform-origin: top center;
}

@keyframes crate-switcher-panel-enter {
  from {
    opacity: 0;
    transform: translateY(var(--_crate-switcher-panel-enter-offset));
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
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

@media (prefers-reduced-motion: reduce) {
  me [data-portfolio-crate-panel][data-local-tab-entering] {
    animation: none;
  }
}
"#
);

pub(super) struct CrateShowcaseSwitcher<'a> {
    pub cards: &'a [CrateCardContent],
}

impl Render for CrateShowcaseSwitcher<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section
                data-portfolio-crate-switcher
                data-local-tabs-root
                data-local-tabs-active=(tab_value(0)) {
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
                            interaction: TabInteraction::LocalTabs {
                                value: tab_value(index),
                            },
                        })
                    }
                }
                @for (index, card) in self.cards.iter().enumerate() {
                    (CrateShowcasePanel {
                        card,
                        tab_index: index,
                    })
                }
            }
        }
    }
}

struct CrateShowcasePanel<'a> {
    card: &'a CrateCardContent,
    tab_index: usize,
}

impl Render for CrateShowcasePanel<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section
                id=(panel_dom_id(self.tab_index))
                class="ui-portfolio-crate-panel"
                role="tabpanel"
                aria-labelledby=(tab_dom_id(self.tab_index))
                data-portfolio-crate-panel
                tabindex=(if self.tab_index == 0 { 0 } else { -1 })
                hidden[self.tab_index != 0] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switcher_uses_local_tab_root_and_initial_selection() {
        let markup = CrateShowcaseSwitcher {
            cards: &[CrateCardContent {
                name: Text::from("statum"),
                summary: Text::from("summary"),
                highlights: vec![Text::from("highlight")],
                gallery: None,
                repository_url: Text::from("https://example.com"),
                repository_label: Text::from("Repository"),
                docs_url: None,
                docs_label: None,
                tags: vec![],
            }],
        }
        .render()
        .into_string();

        assert!(markup.contains("data-local-tabs-root"));
        assert!(markup.contains("data-local-tabs-active=\"crate_0\""));
        assert!(markup.contains("data-local-tab-value=\"crate_0\""));
        assert!(!markup.contains("data-signals="));
        assert!(!markup.contains("data-show="));
    }
}
