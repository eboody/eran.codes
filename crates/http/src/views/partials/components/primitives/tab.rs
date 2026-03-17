use maud::Render;
use serde_json::json;

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
    DatastarLocal { signal: Text, value: Text },
}

impl Render for Tab {
    fn render(&self) -> maud::Markup {
        let tab_index = if self.is_selected { 0 } else { -1 };
        let icon = self.icon.clone();
        let style = format!("--tab-accent: {};", self.palette.main.as_ref());

        match &self.interaction {
            TabInteraction::DatastarLocal { signal, value } => {
                let selected_expr = format!("${} == {}", signal, json_literal(value));
                let selected_attr = format!("{} ? 'true' : 'false'", selected_expr);
                let tabindex_attr = format!("{} ? '0' : '-1'", selected_expr);
                let click_expr = format!("${} = {}", signal, json_literal(value));

                maud::html! {
                    button.tab-set__tab.is-selected[self.is_selected]
                        type="button"
                        role="tab"
                        id=(&self.id)
                        aria-controls=(&self.controls)
                        aria-selected=(self.is_selected)
                        tabindex=(tab_index)
                        data-tab-id=(value)
                        data-class:is-selected=(selected_expr)
                        data-attr:aria-selected=(selected_attr)
                        data-attr:tabindex=(tabindex_attr)
                        data-on:click=(click_expr)
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

fn json_literal(value: &Text) -> String {
    json!(value.to_string()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datastar_local_interaction_uses_json_literal_for_signal_value() {
        let value = Text::from("sso'and\"quotes");
        let selected_expr = format!("$active_tab_id == {}", json_literal(&value));
        let click_expr = format!("$active_tab_id = {}", json_literal(&value));

        assert_eq!(selected_expr, "$active_tab_id == \"sso'and\\\"quotes\"");
        assert_eq!(click_expr, "$active_tab_id = \"sso'and\\\"quotes\"");
    }
}
