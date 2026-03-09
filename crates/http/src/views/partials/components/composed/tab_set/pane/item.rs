use maud::Render;

use crate::types::Text;
use crate::views::partials::components::tab_set;
use crate::views::partials::components::Tab;

#[derive(Clone, Debug)]
// ci: style-system-component
pub(crate) struct Item {
    pub tab_dom_id: Text,
    pub panel_dom_id: Text,
    pub tab_value: Text,
    pub preview: Option<tab_set::pane::Preview>,
    pub body: Option<tab_set::pane::Body>,
    pub action: Option<tab_set::pane::Action>,
}

impl Item {
    pub(crate) fn from_content(
        tab: &Tab,
        tab_value: Text,
        tab_content: &tab_set::content::Tab,
    ) -> Self {
        Self {
            tab_dom_id: tab.id.clone(),
            panel_dom_id: tab.controls.clone(),
            tab_value,
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
        let show_expr = tab_set::pane::show_expr(&self.tab_value);
        let tabindex_expr = format!("{} ? '0' : '-1'", show_expr);

        maud::html! {
            section
                id=(&self.panel_dom_id)
                class="tab-set__panel ui-panel"
                role="tabpanel"
                aria-labelledby=(&self.tab_dom_id)
                data-show=(show_expr)
                data-attr:tabindex=(tabindex_expr) {
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
