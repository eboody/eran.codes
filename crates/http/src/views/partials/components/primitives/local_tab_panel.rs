use maud::{Markup, Render};

use crate::types::Text;

pub(crate) struct LocalTabPanel<'a> {
    pub id: Text,
    pub labelled_by: Text,
    pub class: &'a str,
    pub is_selected: bool,
    pub content: Markup,
}

impl Render for LocalTabPanel<'_> {
    fn render(&self) -> Markup {
        let tab_index = if self.is_selected { 0 } else { -1 };
        let display_style = if self.is_selected { "" } else { "display: none;" };

        maud::html! {
            section
                id=(&self.id)
                class=(self.class)
                role="tabpanel"
                aria-labelledby=(&self.labelled_by)
                data-local-tab-panel
                style=(display_style)
                tabindex=(tab_index)
                hidden[!self.is_selected] {
                (&self.content)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_shared_local_tab_panel_contract() {
        let markup = LocalTabPanel {
            id: Text::from("panel-id"),
            labelled_by: Text::from("tab-id"),
            class: "example-panel",
            is_selected: false,
            content: maud::html! {
                p { "Example" }
            },
        }
        .render()
        .into_string();

        assert!(markup.contains("data-local-tab-panel"));
        assert!(markup.contains("role=\"tabpanel\""));
        assert!(markup.contains("aria-labelledby=\"tab-id\""));
        assert!(markup.contains("hidden"));
        assert!(markup.contains("style=\"display: none;\""));
    }
}
