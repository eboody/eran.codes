use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::{
    LocalTabPanel, LocalTabRoot, LocalTabRootSurface, Tab, TabInteraction, tab_set,
};
use crate::views::proper_theme::THEME;

use super::{GuestChatFallback, OperationsSurface, RequestBurstDemo};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-4);
}

me [data-supporting-proof-intro] {
  display: grid;
  gap: var(--space-2);
  max-width: 60ch;
}

me [data-supporting-proof-kicker] {
  margin: 0;
  font-size: var(--text-size-label-xs);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wider);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-supporting-proof-intro] h2 {
  margin: 0;
}

me [data-supporting-proof-summary] {
  margin: 0;
  color: color-mix(in srgb, var(--text-body) 88%, var(--text-muted) 12%);
}

me .tab-set__tabs {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
}

me .tab-set__tab {
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

me .tab-set__tab-line {
  font-weight: 600;
}

me .tab-set__tab.is-selected {
  color: var(--text-body);
  border-color: color-mix(in srgb, var(--accent-signal) 34%, var(--border-default));
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

me .tab-set__tab:focus-visible {
  outline: none;
  border-color: color-mix(in srgb, var(--accent-signal) 54%, var(--border-default));
  box-shadow:
    0 0 0 0.22rem color-mix(in srgb, var(--accent-signal) 18%, transparent),
    inset 0 1px 0 var(--surface-edge-default);
}

me [data-supporting-proof-panel] {
  display: grid;
}

me [data-supporting-proof-panel][data-local-tab-entering='1'] {
  animation: supporting-proof-panel-enter var(--motion-slow) var(--ease-out-3);
}

@keyframes supporting-proof-panel-enter {
  from {
    opacity: 0;
    transform: translateY(0.35rem);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (prefers-color-scheme: dark) {
  me .tab-set__tab {
    background: color-mix(in srgb, var(--surface-field) 92%, black 8%);
    border-color: color-mix(in srgb, var(--border-default) 90%, transparent);
  }

  me .tab-set__tab.is-selected {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 44%),
      color-mix(in srgb, var(--accent-signal) 14%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--accent-signal) 40%, var(--border-default));
  }
}

@media (hover: hover) {
  me .tab-set__tab:hover {
    transform: translateY(-1px);
  }
}

@media (max-width: 48rem) {
  me {
    gap: var(--space-3);
  }

  me .tab-set__tabs {
    gap: var(--space-1);
  }

  me .tab-set__tab {
    padding: var(--control-padding-block-compact) var(--control-padding-inline-compact);
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct SupportingProofTabs {
    pub chat_demo: Option<partials::chat::DemoSection>,
}

impl Render for SupportingProofTabs {
    fn render(&self) -> maud::Markup {
        let tabs = SupportingProofTab::ALL
            .into_iter()
            .map(|tab| Tab {
                id: tab.dom_id(),
                controls: tab.panel_dom_id(),
                palette: &THEME.gray,
                is_selected: tab == SupportingProofTab::DEFAULT,
                icon: None,
                primary_text: Text::from(tab.primary_label()),
                secondary_text: tab.secondary_label().map(Text::from),
                interaction: TabInteraction::LocalTabs {
                    value: tab.value(),
                },
            })
            .collect();

        LocalTabRoot {
            surface: LocalTabRootSurface::standard(
                "supporting-proof-tabs",
                "ui-lab-supporting-tabs",
            ),
            active_tab_id: SupportingProofTab::DEFAULT.value(),
            content: maud::html! {
                (css())
                div data-supporting-proof-intro {
                    p data-supporting-proof-kicker { "Supporting views" }
                    h2 { "Validate the main proof from other angles" }
                    p data-supporting-proof-summary {
                        "The secure-data panel is the main event. Use runtime logs, the load harness, or chat only when you want supporting evidence."
                    }
                }
                (tab_set::tab::Set {
                    aria_label: Text::from("Supporting views"),
                    tabs: tab_set::tab::List { children: tabs },
                })
                @for tab in SupportingProofTab::ALL {
                    (LocalTabPanel {
                        id: tab.panel_dom_id(),
                        labelled_by: tab.dom_id(),
                        class: "ui-lab-supporting-panel",
                        is_selected: tab == SupportingProofTab::DEFAULT,
                        content: tab.panel_markup(self.chat_demo.as_ref()),
                    })
                }
            },
        }
        .render()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum SupportingProofTab {
    #[default]
    RuntimeInspection,
    LoadHarness,
    LiveChat,
}

impl SupportingProofTab {
    const ALL: [Self; 3] = [Self::RuntimeInspection, Self::LoadHarness, Self::LiveChat];
    const DEFAULT: Self = Self::RuntimeInspection;

    fn panel_markup(self, chat_demo: Option<&partials::chat::DemoSection>) -> maud::Markup {
        match self {
            Self::RuntimeInspection => {
                maud::html! { (OperationsSurface::builder().build()) }
            }
            Self::LoadHarness => maud::html! {
                (RequestBurstDemo::builder()
                    .endpoint(Text::from(Route::PartialRequestBurstProbe.as_str()))
                    .build())
            },
            Self::LiveChat => match chat_demo {
                Some(chat_demo) => chat_demo.render(),
                None => GuestChatFallback::builder().build().render(),
            },
        }
    }

    const fn primary_label(self) -> &'static str {
        match self {
            Self::RuntimeInspection => "Runtime",
            Self::LoadHarness => "Load",
            Self::LiveChat => "Chat",
        }
    }

    const fn secondary_label(self) -> Option<&'static str> {
        match self {
            Self::RuntimeInspection => Some("Inspection"),
            Self::LoadHarness => Some("Harness"),
            Self::LiveChat => Some("Flow"),
        }
    }

    const fn key(self) -> &'static str {
        match self {
            Self::RuntimeInspection => "runtime_inspection",
            Self::LoadHarness => "load_harness",
            Self::LiveChat => "live_chat",
        }
    }

    fn value(self) -> Text {
        Text::from(self.key())
    }

    fn dom_id(self) -> Text {
        Text::from(format!("supporting-proof-tab-{}", self.key()))
    }

    fn panel_dom_id(self) -> Text {
        Text::from(format!("supporting-proof-panel-{}", self.key()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_local_tab_root_with_runtime_selected_by_default() {
        let markup = SupportingProofTabs::builder().build().render().into_string();

        assert!(markup.contains("id=\"supporting-proof-tabs\""));
        assert!(markup.contains("data-local-tabs-root"));
        assert!(markup.contains("data-local-tabs-active=\"runtime_inspection\""));
        assert!(markup.contains("Validate the main proof from other angles"));
        assert!(markup.contains("role=\"tablist\""));
        assert!(markup.contains("role=\"tabpanel\""));
    }
}
