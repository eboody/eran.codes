use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{Tab, TabPanel};

use super::{Component, Panel, PanelsComponent, Styles};

impl Render for Component {
    fn render(&self) -> maud::Markup {
        let item_count = self.tabs.len().min(self.panels.len());
        if item_count == 0 {
            return maud::html! {};
        }

        let tabs = TabPanel::normalize_selected_tabs(&self.tabs[..item_count]);
        let panels = &self.panels[..item_count];
        let theme_class = format!("showcase showcase--{}", self.theme.as_attr());

        maud::html! {
            section
                id=(&self.id)
                class=(theme_class)
            {
                div class="showcase-heading" {
                    header class="showcase-title" {
                        h2 class="showcase-title-text" { (&self.title) }
                        p class="showcase-title-subtitle is-muted" { (&self.subtitle) }
                    }
                }
                div class="showcase-root" {
                    div class="showcase-shell" {
                        (render_showcase_tabs(tabs.as_slice()))
                        (render_showcase_panels(tabs.as_slice(), panels))
                    }
                }
                (Styles.render())
            }
        }
    }
}

fn render_showcase_tabs(tabs: &[Tab]) -> maud::Markup {
    maud::html! {
        (TabPanel {
            tabs,
            aria_label: Text::from("Showcase tabs"),
        })
    }
}

fn render_showcase_panels(tabs: &[Tab], panels: &[Panel]) -> maud::Markup {
    maud::html! {
        (PanelsComponent::builder()
            .tabs(tabs)
            .panels(panels)
            .build())
    }
}
