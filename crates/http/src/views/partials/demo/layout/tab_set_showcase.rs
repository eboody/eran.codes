use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug, Builder)]
pub struct TabSetShowcase {}

impl Render for TabSetShowcase {
    fn render(&self) -> maud::Markup {
        let content = load_content();

        maud::html! {
            (tab_set::Component::from_content(
                tab_set::ContentProps::builder()
                    .id("tab-set-showcase")
                    .class("u-surface-card tab-set-showcase")
                    .aria_label(Text::from("Solutions"))
                    .content(&content)
                    .build(),
            ))
        }
    }
}

fn load_content() -> tab_set::content::TabSetContent {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/views/partials/demo/layout/content/tab_set_showcase.json"
    ));
    serde_json::from_str(raw).expect("tab_set_showcase fixture must be valid JSON")
}
