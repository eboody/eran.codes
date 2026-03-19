use maud::Render;

use crate::views::partials::components::chat;

#[derive(Clone, Debug)]
enum Footer {
    Composer(chat::Composer),
    Notice(chat::Notice),
}

#[derive(Clone, Debug)]
pub struct Panel {
    window: chat::Window,
    footer: Footer,
}

impl Panel {
    pub fn composer(window: chat::Window, composer: chat::Composer) -> Self {
        Self {
            window,
            footer: Footer::Composer(composer),
        }
    }

    pub fn notice(window: chat::Window, notice: chat::Notice) -> Self {
        Self {
            window,
            footer: Footer::Notice(notice),
        }
    }
}

impl Render for Panel {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-panel {
                (self.window.render())
                @match &self.footer {
                    Footer::Composer(composer) => {
                        (composer.render())
                    }
                    Footer::Notice(notice) => {
                        (notice.render())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Text;
    use crate::views::partials;

    fn window() -> chat::Window {
        chat::Window::builder()
            .with_title(Text::from("You"))
            .with_connected_signal(Text::from("$sseConnected"))
            .side(chat::Side::Right)
            .messages(Vec::new())
            .build()
    }

    #[test]
    fn renders_composer_footer() {
        let panel = Panel::composer(
            window(),
            chat::Composer::builder()
                .action(Text::from("/chat/messages"))
                .input_label(Text::from("Message as you"))
                .input_name(Text::from("body"))
                .input_id(Text::from("chat-input-you"))
                .input_signal(Text::from("chatDraftBody"))
                .placeholder(Text::from("Say something..."))
                .submit(
                    partials::button::Button::builder()
                        .label(Text::from("Send"))
                        .variant(partials::button::Variant::Primary)
                        .role(partials::button::Role::submit())
                        .build(),
                )
                .build(),
        );

        let markup = panel.render().into_string();

        assert!(markup.contains("data-chat-compose"));
        assert!(markup.contains("chat-input-you"));
    }

    #[test]
    fn renders_read_only_notice_footer() {
        let panel = Panel::notice(
            window(),
            chat::Notice::builder()
                .lead(Text::from("Read-only as you."))
                .link_href(Text::from("/login"))
                .link_label(Text::from("Sign in"))
                .with_tail(Text::from("to post with your account."))
                .build(),
        );

        let markup = panel.render().into_string();

        assert!(markup.contains("data-chat-readonly"));
        assert!(markup.contains(">Sign in<"));
    }
}
