use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::chat;

#[derive(Clone, Copy, Debug)]
pub enum Role {
    You,
    Demo,
}

#[derive(Clone, Copy, Debug)]
struct RoleSpec {
    key: &'static str,
    title: &'static str,
    input_label: &'static str,
    placeholder: &'static str,
    action: Route,
    input_signal: &'static str,
    button_label: &'static str,
    side: chat::message::Side,
    input_id: &'static str,
}

impl Role {
    fn spec(self) -> RoleSpec {
        match self {
            Role::You => RoleSpec {
                key: "you",
                title: "You",
                input_label: "Message as you",
                placeholder: "Say something...",
                action: Route::ChatMessages,
                input_signal: "body",
                button_label: "Send",
                side: chat::message::Side::Right,
                input_id: "chat-input-you",
            },
            Role::Demo => RoleSpec {
                key: "demo",
                title: "Demo user",
                input_label: "Message as demo user",
                placeholder: "Send as demo user...",
                action: Route::ChatMessagesDemo,
                input_signal: "botBody",
                button_label: "Send as demo",
                side: chat::message::Side::Left,
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

#[derive(Clone, Debug, Builder)]
pub struct Panel {
    pub role: Role,
    pub messages: Vec<chat::Message>,
    #[builder(default)]
    pub interactivity: Mode,
}

impl Render for Panel {
    fn render(&self) -> maud::Markup {
        let role = self.role.spec();
        let can_compose = matches!(self.interactivity, Mode::Interactive)
            || matches!(
                (self.interactivity, self.role),
                (Mode::DemoOnly, Role::Demo)
            );
        let submit_action = format!("@post('{}'); ${} = ''", role.action, role.input_signal);
        maud::html! {
            div data-chat-panel data-chat-panel-role=(role.key) {
                ({
                    chat::Window::builder()
                        .title(Text::from(role.title))
                        .side(role.side)
                        .messages(self.messages.clone())
                        .build()
                })
                @if can_compose {
                    form
                        method="post"
                        action=(role.action)
                        data-chat-compose
                        data-on:submit=(submit_action)
                    {
                        label for=(role.input_id) {
                            span data-chat-compose-label { (role.input_label) }
                        }
                        div data-chat-compose-row {
                            input
                                id=(role.input_id)
                                type="text"
                                name="body"
                                placeholder=(role.placeholder)
                                data-bind=(role.input_signal)
                                required;
                            button type="submit" data-chat-send=(role.key) {
                                (role.button_label)
                            }
                        }
                    }
                } @else {
                    div data-muted data-chat-readonly {
                        "Read-only as you. "
                        a href=(Route::Login) { "Sign in" }
                        " to post with your account."
                    }
                }
            }
        }
    }
}
