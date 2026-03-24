use bon::Builder;
use maud::Render;

use crate::views::partials;

use super::{SurfaceSection, SurfaceSectionAttr};

#[derive(Clone, Debug, Builder)]
pub struct GuestChatFallback {}

impl Render for GuestChatFallback {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::lab_page_content();
        let action = content.guest_chat.actions.first().map(|link| {
            partials::button::Button::builder()
                .label(link.label.clone())
                .variant(partials::button::Variant::Secondary)
                .role(partials::button::Role::link(link.href.clone()))
                .build()
        });

        SurfaceSection::builder()
            .id(crate::types::Text::from(
                partials::chat::DemoSection::ANCHOR_ID,
            ))
            .title(content.guest_chat.title.clone())
            .subtitle(content.guest_chat.summary.clone())
            .maybe_action(action)
            .attrs(vec![SurfaceSectionAttr::value(
                "data-chat-surface-variant",
                "lab",
            )])
            .content(maud::html! {})
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_guest_chat_surface_with_anchor_and_action() {
        let markup = GuestChatFallback::builder().build().render().into_string();

        assert!(markup.contains(partials::chat::DemoSection::ANCHOR_ID));
        assert!(markup.contains("data-chat-surface-variant=\"lab\""));
        assert!(markup.contains("Sign in"));
    }
}
