use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::{page, partials};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-section);
  margin-top: clamp(1.1rem, 0.9rem + 0.9vw, 1.8rem);
  padding-bottom: calc(var(--space-section) + var(--space-6));
}

me > :where(header, section) {
  margin-top: 0;
  scroll-margin-top: var(--nav-scroll-offset);
}
"#
);

#[derive(Builder)]
pub struct Chat {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<partials::components::chat::Message>,
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for Chat {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="u-container" data-chat-page {
                (css())
                ({
                    partials::components::chat::Hero::builder()
                        .room_name(self.room_name.clone())
                        .room_id(self.room_id.clone())
                        .build()
                })

                (partials::components::chat::Surface::builder()
                    .room_id(self.room_id.clone())
                    .messages(self.messages.clone())
                    .mode(partials::components::chat::Mode::Interactive)
                    .build())
            }
        };

        page::Layout::builder()
            .title("Chat room")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
