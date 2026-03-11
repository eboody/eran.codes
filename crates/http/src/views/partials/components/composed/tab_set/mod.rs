use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{Tab, TabInteraction};
use crate::views::proper_theme::{Palette, THEME};

crate::views::scoped::inline_css!(
    r#"
me.tab-set-showcase {
  --_tab-set-list-gap: var(--space-2);
  --_tab-set-list-padding-block-end: calc(var(--space-3) + var(--interactive-bleed));
  --_tab-set-tab-gap: var(--control-gap);
  --_tab-set-tab-padding-block: var(--control-padding-block);
  --_tab-set-tab-padding-inline: var(--control-padding-inline);
  --_tab-set-tab-radius: var(--control-radius);
  --_tab-set-tab-border-width: var(--control-border-width);
  --_tab-set-tab-font-size: var(--control-font-size-compact);
  --_tab-set-tab-font-weight: var(--control-font-weight);
  --_tab-set-tab-line-height: var(--control-line-height);
  --_tab-set-tab-letter-spacing: var(--control-letter-spacing);
  --_tab-set-tab-white-space: var(--control-white-space);
  --_tab-set-tab-icon-size: var(--space-4);
  --_tab-set-panel-gap: clamp(1rem, 0.85rem + 0.8vw, 1.75rem);
  --_tab-set-preview-gap: var(--space-2);
  --_tab-set-feature-gap: var(--space-2);
  --_tab-set-feature-space: var(--space-4);

  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 38%, transparent),
      transparent 44%
    ),
    var(--surface-panel);
}

me .tab-set__tabs {
  display: flex;
  gap: var(--_tab-set-list-gap);
  position: relative;
  isolation: isolate;
  padding-block: var(--interactive-bleed) var(--_tab-set-list-padding-block-end);
  padding-inline: var(--interactive-bleed);
  margin: calc(var(--interactive-bleed) * -1);
  overflow-x: auto;
  border-bottom: var(--_tab-set-tab-border-width) solid var(--border-default);
  scrollbar-width: thin;
  scroll-snap-type: x proximity;
  overflow-y: visible;
}

me .tab-set__tab {
  appearance: none;
  display: inline-flex;
  align-items: center;
  gap: var(--_tab-set-tab-gap);
  margin: 0;
  padding-block: var(--_tab-set-tab-padding-block);
  padding-inline: var(--_tab-set-tab-padding-inline);
  border-radius: var(--_tab-set-tab-radius);
  border: var(--_tab-set-tab-border-width) solid var(--border-default);
  background: color-mix(in srgb, var(--surface-field) 72%, transparent);
  color: var(--tab-fg, var(--text-muted));
  cursor: pointer;
  font: inherit;
  font-size: var(--_tab-set-tab-font-size);
  font-weight: var(--_tab-set-tab-font-weight);
  line-height: var(--_tab-set-tab-line-height);
  letter-spacing: var(--_tab-set-tab-letter-spacing);
  white-space: var(--_tab-set-tab-white-space);
  position: relative;
  z-index: 0;
  scroll-snap-align: start;
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
}

me .tab-set__tab.is-selected {
  color: var(--tab-fg, var(--text-body));
  border-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 42%,
    var(--border-default)
  );
  background: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 10%,
    var(--surface-panel)
  );
  box-shadow:
    inset 0 1px 0 var(--surface-edge-soft),
    0 0 0 1px color-mix(in srgb, var(--tab-accent, var(--accent-signal)) 22%, transparent);
}

me .tab-set__tab:focus-visible {
  outline: none;
  border-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 54%,
    var(--border-default)
  );
  box-shadow:
    0 0 0 0.22rem color-mix(
      in srgb,
      var(--tab-accent, var(--accent-signal)) 18%,
      transparent
    ),
    inset 0 1px 0 var(--surface-edge-default);
}

me .tab-set__tab-icon {
  --control-icon-size: var(--_tab-set-tab-icon-size);

  display: var(--control-inline-display);
  align-items: var(--control-inline-align-items);
}

me .tab-set__tab-line + .tab-set__tab-line {
  margin-left: 0.2rem;
}

me .tab-set__preview {
  min-width: 0;
}

me .tab-set__panel {
  display: grid;
  gap: var(--_tab-set-panel-gap);
  align-items: start;
  grid-template-columns: 1.05fr 1fr;
  min-width: 0;
  padding-block-start: var(--space-2);
}

me .tab-set__panel[hidden] {
  display: none;
}

me .tab-set__panel > * {
  min-width: 0;
}

me .tab-set__preview-frame {
  overflow: visible;
  border: var(--_tab-set-tab-border-width) solid var(--border-default);
  border-radius: calc(var(--radius-card) - 2px);
  min-height: 260px;
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 34%, transparent),
      transparent 42%
    ),
    color-mix(in srgb, var(--surface-field) 82%, var(--surface-panel));
  padding: var(--space-card);
  display: grid;
  gap: var(--_tab-set-preview-gap);
  align-content: start;
  box-shadow: inset 0 1px 0 var(--surface-edge-default);
}

me .tab-set__features {
  margin-block: var(--_tab-set-feature-space);
  padding-inline-start: var(--_tab-set-feature-space);
  display: grid;
  gap: var(--_tab-set-feature-gap);
}

me .tab-set__preview-label {
  margin: 0;
  font-size: 0.72rem;
  letter-spacing: 0.08rem;
  text-transform: uppercase;
  color: var(--text-muted);
}

me .tab-set__preview-asset {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
}

me .tab-set__badge {
  margin: 0.5rem 0 0;
  width: fit-content;
  border-radius: 999px;
  padding: 0.35rem 0.65rem;
  border: var(--_tab-set-tab-border-width) solid var(--border-subtle);
  font-size: 0.78rem;
  color: var(--text-muted);
  background: var(--ui-surface-soft);
}

me .tab-set__copy h2 {
  margin: 0;
  font-size: 2rem;
  line-height: 1.1;
  letter-spacing: -0.02rem;
  text-wrap: balance;
}

me .tab-set__subtitle {
  margin: 0.6rem 0 0;
  color: var(--text-muted);
  max-width: 52ch;
}

@media (prefers-color-scheme: dark) {
  me.tab-set-showcase {
    background:
      linear-gradient(
        180deg,
        var(--surface-wash-top-soft),
        transparent 34%
      ),
      color-mix(in srgb, var(--surface-panel) 92%, black 8%);
  }

  me .tab-set__preview-frame {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 28%),
      color-mix(in srgb, var(--surface-field) 92%, black 8%);
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }

  me .tab-set__tab {
    background: color-mix(in srgb, var(--surface-field) 90%, black 10%);
    border-color: color-mix(in srgb, var(--border-default) 90%, transparent);
  }

  me .tab-set__tab.is-selected {
    background: color-mix(in srgb, var(--accent-signal) 14%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--accent-signal) 38%, var(--border-default));
  }

  me .tab-set__badge {
    background: color-mix(in srgb, var(--accent-signal) 16%, var(--surface-field));
  }
}

@media (hover: hover) {
  me .tab-set__tab:hover {
    transform: translateY(-1px);
    z-index: 1;
  }
}

@media (max-width: 980px) {
  me .tab-set__panel {
    grid-template-columns: 1fr;
  }

  me .tab-set__copy h2 {
    font-size: 1.65rem;
  }
}

@media (max-width: 48rem) {
  me .tab-set__panel {
    gap: var(--space-4);
    padding-top: 0;
  }

  me .tab-set__preview-frame {
    min-height: 13.5rem;
  }
}
"#
);

pub(crate) mod content;
pub(crate) mod pane;
pub(crate) mod tab;

// ci: descriptive-module-import crate::views::partials::components::tab_set
#[derive(Clone, Debug, Builder)]
pub(crate) struct Component<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub signal_name: Text,
    pub active_tab_id: Text,
    pub tabs: tab::Set,
    pub panes: pane::List,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct ContentProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub aria_label: Text,
    pub content: &'a content::TabSetContent,
    pub active_tab_id: Option<Text>,
    pub signal_name: Option<Text>,
    pub palette: Option<&'static Palette>,
}

impl<'a> Component<'a> {
    pub(crate) fn from_content(props: ContentProps<'a>) -> Self {
        let signal_name = props
            .signal_name
            .unwrap_or_else(|| Text::from("active_tab_id"));
        let palette = props.palette.unwrap_or(&THEME.gray);
        let active_tab_id = props.active_tab_id.unwrap_or_else(|| {
            props
                .content
                .tabs
                .first()
                .map(|tab| tab.id.clone())
                .unwrap_or_else(|| Text::from("tab_0"))
        });
        let tabs = tabs_from_content(
            props.id,
            &signal_name,
            palette,
            &active_tab_id,
            props.content,
        );
        let panes = panes_from_content(&signal_name, props.content, &tabs);

        Self {
            id: props.id,
            class: props.class,
            signal_name,
            active_tab_id,
            tabs: tab::Set {
                aria_label: props.aria_label,
                tabs: tab::List { children: tabs },
            },
            panes: pane::List { children: panes },
        }
    }
}

impl Render for Component<'_> {
    fn render(&self) -> maud::Markup {
        let signals = component_signals(&self.signal_name, &self.active_tab_id);
        maud::html! {
            section
                id=(self.id)
                class=(self.class)
                data-signals=(signals) {
                (css())
                (self.tabs)
                (self.panes)
            }
        }
    }
}

fn tabs_from_content(
    root_id: &str,
    signal_name: &Text,
    palette: &'static Palette,
    active_tab_id: &Text,
    content: &content::TabSetContent,
) -> Vec<Tab> {
    content
        .tabs
        .iter()
        .enumerate()
        .map(|(index, tab)| Tab {
            id: Text::from(format!("{root_id}-tab-{index}")),
            controls: Text::from(format!("{root_id}-pane-{index}")),
            palette,
            is_selected: tab.id == *active_tab_id,
            icon: tab.icon.clone(),
            text: tab.label.text(),
            interaction: TabInteraction::DatastarLocal {
                signal: signal_name.clone(),
                value: tab.id.clone(),
            },
        })
        .collect()
}

fn panes_from_content(
    signal_name: &Text,
    content: &content::TabSetContent,
    tabs: &[Tab],
) -> Vec<pane::Item> {
    content
        .tabs
        .iter()
        .zip(tabs.iter())
        .map(|(tab_content, tab)| {
            pane::Item::from_content(signal_name, tab, tab_content.id.clone(), tab_content)
        })
        .collect()
}

fn component_signals(signal_name: &Text, active_tab_id: &Text) -> String {
    let mut signals = serde_json::Map::new();
    signals.insert(
        signal_name.to_string(),
        serde_json::Value::String(active_tab_id.to_string()),
    );
    serde_json::Value::Object(signals).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_data_signals_uses_dynamic_signal_name_and_json_encoding() {
        let signals =
            component_signals(&Text::from("selected_tab"), &Text::from("alpha'\"beta"));
        assert_eq!(signals, "{\"selected_tab\":\"alpha'\\\"beta\"}");
    }

    #[test]
    fn from_content_defaults_to_first_tab_and_default_signal_name() {
        let content = content::TabSetContent {
            tabs: vec![content::Tab {
                id: Text::from("policy"),
                label: content::Label {
                    primary: Text::from("Policy"),
                    secondary: None,
                },
                icon: None,
                preview: None,
                body: None,
                action: None,
            }],
        };

        let component = Component::from_content(
            ContentProps::builder()
                .id("tab-set")
                .class("tab-set-showcase")
                .aria_label(Text::from("Solutions"))
                .content(&content)
                .build(),
        );

        assert_eq!(component.signal_name, Text::from("active_tab_id"));
        assert_eq!(component.active_tab_id, Text::from("policy"));
        assert_eq!(component.tabs.tabs.children.len(), 1);
        assert_eq!(component.panes.children.len(), 1);
    }
}
