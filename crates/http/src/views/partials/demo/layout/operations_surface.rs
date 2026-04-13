mod styles;
#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

use super::{Attr, SurfaceSection};

#[derive(Clone, Debug, Builder)]
pub struct OperationsSurface {}

impl Render for OperationsSurface {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::lab_page_content();

        SurfaceSection::builder()
            .id(Text::from("operations-surface"))
            .title(content.operations_surface.title.clone())
            .subtitle(content.operations_surface.subtitle.clone())
            .attrs(vec![Attr::flag("data-operations-surface")])
            .content(maud::html! {
                (styles::render())
                (partials::OperationalRequestFilter::builder()
                    .target_id("network-log-target")
                    .build())
                (partials::DemoResultPlaceholder::builder()
                    .target_id(Text::from("network-log-target"))
                    .message(content.operations_surface.empty_message.clone())
                    .build())
            })
            .build()
            .render()
    }
}
