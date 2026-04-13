use maud::Render;

use super::*;
use crate::types::Text;
use crate::views::partials::components::portfolio::content::{CmsActionLink, CtaKind};

#[test]
fn render_actions_uses_contained_button_row_frame() {
    let markup = render_actions(&[CmsActionLink {
        label: Text::from("Inspect"),
        href: Text::from("/work"),
        kind: Default::default(),
        tone: CtaKind::Primary,
    }])
    .render()
    .into_string();

    assert!(markup.contains("data-button-row-frame=\"contained\""));
    assert!(markup.contains("data-button-row-narrow=\"stack\""));
}
