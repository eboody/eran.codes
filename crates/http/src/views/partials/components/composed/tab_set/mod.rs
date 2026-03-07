use bon::Builder;
use maud::Render;

pub(crate) mod content;
pub(crate) mod pane;
pub(crate) mod tab;

// ci: descriptive-module-import crate::views::partials::components::tab_set
#[derive(Clone, Debug, Builder)]
pub(crate) struct Component<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub active_tab_id: &'a str,
    pub tabs: tab::Set<'a>,
    pub panes: pane::List<'a>,
}

impl Render for Component<'_> {
    fn render(&self) -> maud::Markup {
        let signals = component_signals(self.active_tab_id);
        maud::html! {
            section
                id=(self.id)
                class=(self.class)
                data-signals=(signals) {
                (self.tabs)
                (self.panes)
            }
        }
    }
}

fn component_signals(active_tab_id: &str) -> String {
    serde_json::json!({ "active_tab_id": active_tab_id }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_data_signals_uses_json_encoding() {
        let signals = component_signals("alpha'\"beta");
        assert_eq!(signals, "{\"active_tab_id\":\"alpha'\\\"beta\"}");
    }
}
