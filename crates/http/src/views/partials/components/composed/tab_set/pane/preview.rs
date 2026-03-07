use maud::Render;

use crate::types::Text;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug)]
pub(crate) struct Preview {
    pub asset_ref: Option<Text>,
    pub badge_text: Option<Text>,
}

impl Preview {
    pub(crate) fn from_content(preview: &tab_set::content::Preview) -> Self {
        Self {
            asset_ref: preview.image.as_ref().map(|image| image.asset_ref.clone()),
            badge_text: preview.badge.as_ref().map(|badge| badge.text.clone()),
        }
    }
}

impl Render for Preview {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="tab-set__preview" {
                div class="tab-set__preview-frame ui-preview-frame" {
                    p class="tab-set__preview-label" { "Preview" }
                    @if let Some(asset_ref) = &self.asset_ref {
                        p class="tab-set__preview-asset" { (asset_ref) }
                    }
                    @if let Some(badge_text) = &self.badge_text {
                        p class="tab-set__badge" { (badge_text) }
                    }
                }
            }
        }
    }
}
