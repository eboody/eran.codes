#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;
use serde_json::json;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials;

use crate::views::partials::components::chat;

const CONNECTED_SIGNAL: &str = "$sseConnected";

#[derive(Clone, Debug)]
struct RoleSpec {
    title: &'static str,
    input_label: &'static str,
    placeholder: &'static str,
    action: Route,
    input_signal: &'static str,
    button_label: &'static str,
    button_variant: partials::button::Variant,
    side: chat::Side,
    input_id: &'static str,
}

#[derive(Clone, Copy, Debug)]
enum Role {
    You,
    Demo,
}

impl Role {
    fn spec(self) -> RoleSpec {
        match self {
            Self::You => RoleSpec {
                title: "You",
                input_label: "Message as you",
                placeholder: "Say something...",
                action: Route::ChatMessages,
                input_signal: "chatDraftBody",
                button_label: "Send",
                button_variant: partials::button::Variant::Primary,
                side: chat::Side::Right,
                input_id: "chat-input-you",
            },
            Self::Demo => RoleSpec {
                title: "Demo user",
                input_label: "Message as demo user",
                placeholder: "Send as demo user...",
                action: Route::ChatMessagesDemo,
                input_signal: "chatDemoDraftBody",
                button_label: "Send as demo",
                button_variant: partials::button::Variant::Secondary,
                side: chat::Side::Left,
                input_id: "chat-input-demo",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Mode {
    #[default]
    Interactive,
    DemoOnly,
}

impl From<bool> for Mode {
    fn from(value: bool) -> Self {
        if value { Self::Interactive } else { Self::DemoOnly }
    }
}

#[derive(Clone, Copy, Debug, Default, strum_macros::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Variant {
    #[default]
    Standalone,
    Lab,
}

#[derive(Clone, Debug, Builder)]
pub struct Surface {
    pub room_id: Text,
    pub messages: Vec<chat::Message>,
    #[builder(default)]
    pub mode: Mode,
    #[builder(default)]
    pub variant: Variant,
}

impl Surface {
    fn panels(&self) -> Vec<chat::Panel> {
        [Role::You, Role::Demo]
            .into_iter()
            .map(|role| self.panel(role))
            .collect()
    }

    fn panel(&self, role: Role) -> chat::Panel {
        let spec = role.spec();
        let window = chat::Window::builder()
            .with_title(Text::from(spec.title))
            .with_connected_signal(Text::from(CONNECTED_SIGNAL))
            .side(spec.side)
            .messages(self.messages.clone())
            .build();

        if self.can_compose(role) {
            chat::Panel::composer(
                window,
                chat::Composer::builder()
                    .action(Text::from(spec.action.as_str()))
                    .input_label(Text::from(spec.input_label))
                    .input_name(Text::from("body"))
                    .input_id(Text::from(spec.input_id))
                    .input_signal(Text::from(spec.input_signal))
                    .placeholder(Text::from(spec.placeholder))
                    .submit(
                        partials::button::Button::builder()
                            .label(Text::from(spec.button_label))
                            .variant(spec.button_variant)
                            .role(partials::button::Role::submit())
                            .build(),
                    )
                    .build(),
            )
        } else {
            chat::Panel::notice(
                window,
                chat::Notice::builder()
                    .lead(Text::from("Read-only as you."))
                    .link_href(Text::from(Route::Login.as_str()))
                    .link_label(Text::from("Sign in"))
                    .with_tail(Text::from("to post with your account."))
                    .build(),
            )
        }
    }

    fn can_compose(&self, role: Role) -> bool {
        matches!(self.mode, Mode::Interactive)
            || matches!((self.mode, role), (Mode::DemoOnly, Role::Demo))
    }

    fn signals(&self) -> String {
        json!({
            "chatDraftBody": "",
            "chatDemoDraftBody": ""
        })
        .to_string()
    }
}

impl Render for Surface {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div
                class="u-surface-card"
                data-chat-surface
                data-chat-room-id=(self.room_id)
                data-chat-surface-variant=(self.variant.as_ref())
                data-signals=(self.signals())
            {
                (chat::css())
                (chat::Set::builder()
                    .panels(self.panels())
                    .with_connection(
                        chat::Connection::builder()
                            .connected_signal(Text::from(CONNECTED_SIGNAL))
                            .build(),
                    )
                    .build())
                script src="/static/chat-demo.js" {}
            }
        }
    }
}
