use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Tab;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug)]
// ci: style-system-component
pub(crate) struct Item {
    pub tab_dom_id: Text,
    pub panel_dom_id: Text,
    pub is_selected: bool,
    pub preview: Option<tab_set::pane::Preview>,
    pub body: Option<tab_set::pane::Body>,
    pub action: Option<tab_set::pane::Action>,
}

impl Item {
    pub(crate) fn from_content(
        tab: &Tab,
        tab_content: &tab_set::content::Tab,
        is_selected: bool,
    ) -> Self {
        Self {
            tab_dom_id: tab.id.clone(),
            panel_dom_id: tab.controls.clone(),
            is_selected,
            preview: tab_content
                .preview
                .as_ref()
                .map(tab_set::pane::Preview::from_content),
            body: tab_content
                .body
                .as_ref()
                .map(tab_set::pane::Body::from_content),
            action: tab_content
                .action
                .as_ref()
                .map(tab_set::pane::Action::from_content),
        }
    }
}

impl Render for Item {
    fn render(&self) -> maud::Markup {
        let tab_index = if self.is_selected { 0 } else { -1 };

        maud::html! {
            section
                id=(&self.panel_dom_id)
                class="tab-set__panel"
                role="tabpanel"
                aria-labelledby=(&self.tab_dom_id)
                tabindex=(tab_index)
                hidden[!self.is_selected] {
                @if let Some(preview) = &self.preview {
                    (preview)
                }
                div class="tab-set__copy" {
                    @if let Some(body) = &self.body {
                        (body)
                    }
                    @if let Some(action) = &self.action {
                        (action)
                    }
                }
            }
        }
    }
}
