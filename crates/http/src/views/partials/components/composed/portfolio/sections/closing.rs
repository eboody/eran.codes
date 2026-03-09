use maud::Render;

use crate::types::Text;
use crate::views::partials::components::portfolio::content::CmsActionLink;

use super::render_actions;

pub struct ClosingSection<'a> {
    pub title: &'a Text,
    pub summary: &'a Text,
    pub actions: &'a [CmsActionLink],
}

impl Render for ClosingSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-surface-card ui-portfolio-closing" {
                h2 { (&self.title) }
                p class="ui-portfolio-summary" { (&self.summary) }
                (render_actions(self.actions))
            }
        }
    }
}
