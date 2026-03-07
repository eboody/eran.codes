use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{tab_set, Tab, TabInteraction};
use crate::views::proper_theme::THEME;

#[derive(Clone, Debug, Builder)]
pub struct TabSetShowcase {}

impl Render for TabSetShowcase {
    fn render(&self) -> maud::Markup {
        let content = load_content();
        let active_tab_id = content
            .tabs
            .first()
            .map(|tab| tab.id.to_string())
            .unwrap_or_else(|| String::from("tab_0"));

        let tabs: Vec<Tab> = content
            .tabs
            .iter()
            .enumerate()
            .map(|(index, tab)| Tab {
                id: Text::from(format!("tab-set-tab-{index}")),
                controls: Text::from(format!("tab-set-pane-{index}")),
                palette: &THEME.gray,
                is_selected: index == 0,
                icon: tab.icon.clone(),
                text: tab.label.text(),
                interaction: TabInteraction::DatastarLocal {
                    signal: Text::from("active_tab_id"),
                    value: tab.id.clone(),
                },
            })
            .collect();

        let panes: Vec<tab_set::pane::Item> = content
            .tabs
            .iter()
            .zip(tabs.iter())
            .map(|(tab_content, tab)| {
                tab_set::pane::Item::from_content(tab, tab_content.id.clone(), tab_content)
            })
            .collect();

        let active_tab_id = active_tab_id.as_str();

        maud::html! {
            (tab_set::Component::builder()
                .id("tab-set-showcase")
                .class("tab-set ui-surface-card")
                .active_tab_id(active_tab_id)
                .tabs(tab_set::tab::Set {
                    aria_label: Text::from("Solutions"),
                    tabs: tab_set::tab::List {
                        children: tabs.as_slice(),
                    },
                })
                .panes(tab_set::pane::List {
                    children: panes.as_slice(),
                })
                .build())
        }
    }
}

fn load_content() -> tab_set::content::TabSetContent {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/cms/tab_set_showcase.json"
    ));
    serde_json::from_str(raw)
        .expect("tab_set_showcase fixture must be valid JSON")
}
