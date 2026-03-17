use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{Tab, TabInteraction};
use crate::views::proper_theme::{Palette, THEME};

crate::views::scoped::inline_css!(
    r#"
me.tab-set-showcase {
  --_tab-set-panel-gap: clamp(1rem, 0.85rem + 0.8vw, 1.75rem);
  --_tab-set-tab-padding: var(--control-padding-block) var(--control-padding-inline);
  --_tab-set-shell-padding: clamp(0.95rem, 0.82rem + 0.55vw, 1.3rem);
  --_tab-set-code-stack-gap: clamp(0.8rem, 0.72rem + 0.35vw, 1.05rem);
  --_tab-set-badge-padding: 0.35rem 0.65rem;

  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 38%, transparent),
      transparent 44%
    ),
    var(--surface-panel);
  position: relative;
  overflow: clip;
}

me .tab-set__tabs {
  display: flex;
  gap: var(--space-2);
  position: relative;
  isolation: isolate;
  padding-block: var(--interactive-bleed) calc(var(--space-3) + var(--interactive-bleed));
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
  gap: var(--control-gap);
  margin: 0;
  padding: var(--_tab-set-tab-padding);
  border-radius: var(--ui-radius-md-inset);
  border: var(--control-border-width) solid var(--border-default);
  background: color-mix(in srgb, var(--surface-field) 78%, var(--surface-panel));
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  color: var(--text-muted);
  cursor: pointer;
  font: inherit;
  font-size: var(--control-font-size-compact);
  font-weight: var(--control-font-weight);
  line-height: var(--control-line-height);
  letter-spacing: var(--control-letter-spacing);
  white-space: var(--control-white-space);
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
  color: var(--text-body);
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
  --control-icon-size: var(--space-4);

  display: var(--control-inline-display);
  align-items: var(--control-inline-align-items);
}

me .tab-set__tab-label {
  display: grid;
  gap: calc(var(--space-1) * 0.2);
  text-align: left;
}

me .tab-set__tab-line {
  font-weight: 600;
}

me .tab-set__tab-secondary {
  font-size: var(--text-size-label-sm);
  font-weight: 500;
  line-height: var(--text-line-flat);
  letter-spacing: var(--text-track-label);
  color: color-mix(in srgb, currentColor 74%, var(--text-muted));
}

me .tab-set__preview {
  min-width: 0;
}

me .tab-set__panel {
  display: grid;
  gap: var(--_tab-set-panel-gap);
  align-items: start;
  grid-template-columns: minmax(0, 1.08fr) minmax(0, 0.92fr);
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
  border: var(--control-border-width) solid
    color-mix(in srgb, var(--accent-signal-soft) 28%, var(--border-default));
  border-radius: var(--ui-radius-md-inset);
  min-height: 260px;
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 34%, transparent),
      transparent 42%
    ),
    color-mix(in srgb, var(--surface-field) 82%, var(--surface-panel));
  padding: var(--_tab-set-shell-padding);
  display: grid;
  gap: var(--space-2);
  align-content: start;
  box-shadow:
    inset 0 1px 0 var(--surface-edge-default),
    inset 0 0 0 1px color-mix(in srgb, black 2%, transparent);
}

me .tab-set__preview-frame[data-preview-kind="code"] {
  min-height: 0;
  gap: var(--space-3);
  border-color: color-mix(
    in srgb,
    var(--accent-signal-soft) 18%,
    var(--border-default)
  );
  background: color-mix(in srgb, var(--surface-panel) 92%, transparent);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me.ui-portfolio-crate-gallery--flat {
  background: transparent;
}

me.ui-portfolio-crate-gallery--flat .tab-set__tabs {
  gap: var(--space-2);
  padding-block-end: calc(var(--space-2) + var(--interactive-bleed));
  border-bottom-color: color-mix(in srgb, var(--border-default) 48%, transparent);
}

me.ui-portfolio-crate-gallery--flat .tab-set__panel {
  gap: clamp(1.35rem, 1.12rem + 0.85vw, 2rem);
  grid-template-columns: minmax(0, 1.28fr) minmax(22rem, 0.72fr);
}

me.ui-portfolio-crate-gallery--flat .tab-set__copy {
  gap: var(--space-3);
  align-content: start;
  max-inline-size: 27rem;
}

me.ui-portfolio-crate-gallery--flat .tab-set__copy h2 {
  font-size: var(--text-size-title-md);
  line-height: var(--text-line-heading);
}

me.ui-portfolio-crate-gallery--flat .tab-set__preview-frame[data-preview-kind="code"] {
  padding: clamp(1.2rem, 1.02rem + 0.7vw, 1.6rem);
  border: var(--control-border-width) solid
    color-mix(in srgb, var(--accent-signal-soft) 18%, var(--border-default));
  border-radius: var(--ui-radius-md-inset);
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 12%, transparent),
      transparent 36%
    ),
    color-mix(in srgb, var(--surface-panel) 88%, var(--surface-field));
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me.ui-portfolio-crate-gallery--flat .tab-set__preview-meta {
  gap: var(--space-2);
}

me.ui-portfolio-crate-gallery--flat .tab-set__tab {
  padding-block: var(--space-2);
  padding-inline: var(--space-2);
  border-width: 0 0 var(--control-border-width);
  border-style: solid;
  border-color: transparent transparent color-mix(in srgb, var(--border-default) 40%, transparent);
  border-radius: 0;
  background: transparent;
  box-shadow: none;
  font-size: var(--text-size-body-xs);
}

me.ui-portfolio-crate-gallery--flat .tab-set__tab.is-selected {
  border-bottom-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 52%,
    var(--border-default)
  );
  background: transparent;
  box-shadow: none;
}

me.ui-portfolio-crate-gallery--flat .tab-set__tab:focus-visible {
  border-bottom-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 58%,
    var(--border-default)
  );
  box-shadow: 0 0 0 0.18rem color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 16%,
    transparent
  );
}

me.ui-portfolio-crate-gallery--flat .tab-set__features {
  display: none;
}

me .tab-set__preview-meta {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-2);
}

me .tab-set__preview-code-stack {
  display: grid;
  gap: var(--_tab-set-code-stack-gap);
}

me .tab-set__preview-code {
  margin: 0;
}

me .tab-set__features {
  margin-block: var(--space-4) 0;
  padding-block-start: var(--space-3);
  padding-inline-start: var(--space-4);
  display: grid;
  gap: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--border-default) 80%, transparent);
}

me .tab-set__preview-label {
  margin: 0;
  font-size: var(--text-size-label-2xs);
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-muted);
}

me .tab-set__preview-asset {
  margin: 0;
  font-size: var(--control-font-size);
  font-weight: 600;
}

me .tab-set__badge {
  margin: 0;
  width: fit-content;
  border-radius: var(--radius-pill);
  padding: var(--_tab-set-badge-padding);
  border: var(--control-border-width) solid var(--border-subtle);
  font-size: var(--text-size-meta-xs);
  color: var(--text-muted);
  background: var(--ui-surface-soft);
}

me .tab-set__copy {
  display: grid;
  gap: var(--space-2);
}

me .tab-set__copy h2 {
  margin: 0;
  font-size: var(--text-size-title-md);
  line-height: var(--text-line-control);
  letter-spacing: var(--text-track-tight);
  text-wrap: balance;
}

me .tab-set__subtitle {
  margin: 0;
  color: var(--text-muted);
  max-width: 52ch;
}

@media (prefers-color-scheme: dark) {
  me.tab-set-showcase {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 26%),
      color-mix(in srgb, var(--surface-panel) 95%, black 5%);
  }

  me .tab-set__preview-frame {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 22%),
      color-mix(in srgb, var(--surface-field) 95%, black 5%);
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }

  me .tab-set__preview-frame[data-preview-kind="code"] {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 20%),
      color-mix(in srgb, var(--surface-panel) 96%, black 4%);
  }

  me .tab-set__tab {
    background: color-mix(in srgb, var(--surface-field) 90%, black 10%);
    border-color: color-mix(in srgb, var(--border-default) 90%, transparent);
  }

  me .tab-set__tab.is-selected {
    background: color-mix(in srgb, var(--accent-signal) 8%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--accent-signal) 24%, var(--border-default));
  }

  me .tab-set__badge {
    background: color-mix(in srgb, var(--surface-field) 94%, black 6%);
  }
}

@media (hover: hover) {
  me .tab-set__tab:hover {
    transform: translateY(-1px);
    z-index: 1;
  }

  me.ui-portfolio-crate-gallery--flat .tab-set__tab:hover {
    transform: none;
    color: var(--text-body);
  }
}

@media (max-width: 980px) {
  me .tab-set__panel {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 48rem) {
  me.tab-set-showcase {
    --_tab-set-panel-gap: var(--space-3);
    --_tab-set-tab-padding:
      calc(var(--control-padding-block) - 0.08rem)
      calc(var(--control-padding-inline) - 0.16rem);
    --_tab-set-shell-padding: var(--control-padding-inline-compact);
    --_tab-set-code-stack-gap: var(--space-2);
    --_tab-set-badge-padding: 0.28rem 0.55rem;
  }

  me .tab-set__tabs {
    gap: var(--space-1);
    padding-block: var(--interactive-bleed) calc(var(--space-2) + var(--interactive-bleed));
  }

  me .tab-set__tab {
    font-size: var(--text-size-meta-xs);
  }

  me .tab-set__panel {
    gap: var(--space-3);
    padding-top: 0;
  }

  me.ui-portfolio-crate-gallery--flat .tab-set__tabs {
    gap: var(--space-1);
    padding-block-end: calc(var(--space-1) + var(--interactive-bleed));
  }

  me.ui-portfolio-crate-gallery--flat .tab-set__tab {
    padding-block: var(--space-1);
    padding-inline: calc(var(--space-1) * 0.75);
  }

  me.ui-portfolio-crate-gallery--flat .tab-set__panel {
    gap: var(--space-2);
    grid-template-columns: 1fr;
  }

  me.ui-portfolio-crate-gallery--flat .tab-set__copy {
    order: -1;
    max-inline-size: none;
  }

  me.ui-portfolio-crate-gallery--flat .tab-set__copy h2 {
    font-size: var(--text-size-title-sm);
  }

  me .tab-set__preview-frame {
    min-height: 12rem;
    gap: var(--space-1);
  }

  me .tab-set__preview-frame[data-preview-kind="code"] {
    gap: var(--space-2);
  }

  me .tab-set__copy {
    gap: var(--space-1);
  }

  me .tab-set__copy h2 {
    font-size: var(--text-size-title-sm);
  }

  me .tab-set__subtitle {
    font-size: var(--text-size-body-lg);
  }

  me .tab-set__preview-meta {
    gap: var(--space-1);
  }

  me .tab-set__preview-code-stack > :not(:first-child) {
    display: none;
  }

  me .tab-set__features {
    margin-block: var(--space-3) 0;
    padding-block-start: var(--space-2);
    padding-inline-start: var(--space-3);
    gap: var(--space-1);
    display: none;
  }

  me .tab-set__badge {
    font-size: var(--text-size-label-sm);
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
            primary_text: tab.label.primary.clone(),
            secondary_text: tab.label.secondary.clone(),
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
                preview: Some(content::Preview {
                    code_examples: vec![content::CodeExample {
                        label: Some(Text::from("Example")),
                        code: Text::from("fn main() {}"),
                    }],
                    image: None,
                    badge: None,
                }),
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
