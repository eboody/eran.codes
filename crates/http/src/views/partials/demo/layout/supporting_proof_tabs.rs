mod styles;
#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components;
use crate::views::proper_theme::THEME;

use super::{GuestChatFallback, OperationsSurface, RequestBurstDemo};

#[derive(Clone, Debug, Builder)]
pub struct SupportingProofTabs {
    pub chat_demo: Option<partials::chat::DemoSection>,
}

impl Render for SupportingProofTabs {
    fn render(&self) -> maud::Markup {
        let tabs = SupportingProofTab::ALL
            .into_iter()
            .map(|tab| components::Tab {
                id: tab.dom_id(),
                controls: tab.panel_dom_id(),
                palette: &THEME.gray,
                is_selected: tab == SupportingProofTab::DEFAULT,
                icon: None,
                primary_text: Text::from(tab.primary_label()),
                secondary_text: tab.secondary_label().map(Text::from),
                interaction: components::TabInteraction::LocalTabs {
                    value: tab.value(),
                },
            })
            .collect();

        components::LocalTabRoot {
            surface: components::LocalTabRootSurface::standard(
                "supporting-proof-tabs",
                "ui-lab-supporting-tabs",
            ),
            active_tab_id: SupportingProofTab::DEFAULT.value(),
            content: maud::html! {
                (styles::render())
                div data-supporting-proof-intro {
                    p data-supporting-proof-kicker { "Supporting views" }
                    h2 { "Validate the main proof from other angles" }
                    p data-supporting-proof-summary {
                        "The secure-data panel is the main event. Use runtime logs, the load harness, or chat only when you want supporting evidence."
                    }
                }
                (components::tab_set::tab::Set {
                    aria_label: Text::from("Supporting views"),
                    style: components::tab_set::tab::Style::PillCluster,
                    tabs: components::tab_set::tab::List { children: tabs },
                })
                @for tab in SupportingProofTab::ALL {
                    (components::LocalTabPanel {
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
