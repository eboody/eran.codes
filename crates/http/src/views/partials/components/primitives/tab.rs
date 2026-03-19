use maud::Render;
use crate::types::Text;
use crate::views::proper_theme::Palette;

use super::Icon;

#[derive(Clone, Debug)]
pub(crate) struct Tab {
    pub id: Text,
    pub controls: Text,
    pub palette: &'static Palette,
    pub is_selected: bool,
    pub icon: Option<Icon>,
    pub primary_text: Text,
    pub secondary_text: Option<Text>,
    pub interaction: TabInteraction,
}

#[derive(Clone, Debug)]
pub(crate) enum TabInteraction {
    LocalTabs { value: Text },
}

impl Render for Tab {
    fn render(&self) -> maud::Markup {
        let tab_index = if self.is_selected { 0 } else { -1 };
        let icon = self.icon.clone();
        let style = format!("--tab-accent: {};", self.palette.main.as_ref());

        match &self.interaction {
            TabInteraction::LocalTabs { value } => {
                maud::html! {
                    button.tab-set__tab.is-selected[self.is_selected]
                        type="button"
                        role="tab"
                        id=(&self.id)
                        aria-controls=(&self.controls)
                        aria-selected=(self.is_selected)
                        tabindex=(tab_index)
                        data-tab-id=(value)
                        data-local-tab-value=(value)
                        style=(style) {
                        (render_content(&icon, &self.primary_text, self.secondary_text.as_ref()))
                    }
                }
            }
        }
    }
}

fn render_content(
    icon: &Option<Icon>,
    primary_text: &Text,
    secondary_text: Option<&Text>,
) -> maud::Markup {
    maud::html! {
        @if let Some(icon) = icon {
            span class="tab-set__tab-icon" { (icon) }
        }
        span class="tab-set__tab-label" {
            span class="tab-set__tab-line" { (primary_text) }
            @if let Some(secondary_text) = secondary_text {
                span class="tab-set__tab-secondary" { (secondary_text) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_tabs_interaction_renders_static_tab_value() {
        let markup = Tab {
            id: Text::from("example-tab"),
            controls: Text::from("example-panel"),
            palette: &crate::views::proper_theme::THEME.gray,
            is_selected: true,
            icon: None,
            primary_text: Text::from("Example"),
            secondary_text: None,
            interaction: TabInteraction::LocalTabs {
                value: Text::from("sso'and\"quotes"),
            },
        }
        .render()
        .into_string();

        assert!(markup.contains("data-tab-id=\"sso'and&quot;quotes\""));
        assert!(markup.contains("data-local-tab-value=\"sso'and&quot;quotes\""));
        assert!(!markup.contains("data-on:click"));
    }
}
