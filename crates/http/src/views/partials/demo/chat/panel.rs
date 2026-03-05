use bon::Builder;
use maud::Render;
use maud_extensions::inline_css;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::chat;

#[derive(Clone, Copy, Debug)]
pub enum Role {
    You,
    Demo,
}

impl Role {
    fn key(&self) -> &'static str {
        match self {
            Role::You => "you",
            Role::Demo => "demo",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Role::You => "You",
            Role::Demo => "Demo user",
        }
    }

    fn input_label(&self) -> &'static str {
        match self {
            Role::You => "Message as you",
            Role::Demo => "Message as demo user",
        }
    }

    fn placeholder(&self) -> &'static str {
        match self {
            Role::You => "Say something...",
            Role::Demo => "Send as demo user...",
        }
    }

    fn action(&self) -> Route {
        match self {
            Role::You => Route::ChatMessages,
            Role::Demo => Route::ChatMessagesDemo,
        }
    }

    fn input_signal(&self) -> &'static str {
        match self {
            Role::You => "body",
            Role::Demo => "botBody",
        }
    }

    fn button_label(&self) -> &'static str {
        match self {
            Role::You => "Send",
            Role::Demo => "Send as demo",
        }
    }

    fn side(&self) -> chat::message::Side {
        match self {
            Role::You => chat::message::Side::Right,
            Role::Demo => chat::message::Side::Left,
        }
    }

    fn input_id(&self) -> &'static str {
        match self {
            Role::You => "chat-input-you",
            Role::Demo => "chat-input-demo",
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
        let action = self.role.action();
        let input_signal = self.role.input_signal();
        let input_id = self.role.input_id();
        let can_compose = matches!(self.interactivity, Mode::Interactive)
            || matches!(
                (self.interactivity, self.role),
                (Mode::DemoOnly, Role::Demo)
            );
        let submit_action = format!("@post('{}'); ${} = ''", action, input_signal);
        maud::html! {
            div data-chat-panel data-chat-panel-role=(self.role.key()) {
                (css())
                ({
                    chat::Window::builder()
                        .title(Text::from(self.role.title()))
                        .side(self.role.side())
                        .messages(self.messages.clone())
                        .build()
                })
                @if can_compose {
                    form
                        method="post"
                        action=(action)
                        data-chat-compose
                        data-on:submit=(submit_action)
                    {
                        label for=(input_id) {
                            span data-chat-compose-label { (self.role.input_label()) }
                        }
                        div data-chat-compose-row {
                            input
                                id=(input_id)
                                type="text"
                                name="body"
                                placeholder=(self.role.placeholder())
                                data-bind=(input_signal)
                                required;
                            button type="submit" data-chat-send=(self.role.key()) {
                                (self.role.button_label())
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

inline_css! {
    me {
      display: grid;
      grid-template-rows: minmax(0, 1fr) auto;
      gap: var(--chat-space-2);
      min-height: 0;
    }
    me > [data-chat-compose] {
      display: grid;
      gap: var(--chat-space-1);
      border: 1px solid var(--chat-compose-border);
      border-radius: var(--chat-radius-compose);
      background: var(--chat-compose-surface);
      padding: var(--chat-space-2);
    }
    me > [data-chat-compose] > label {
      margin: 0;
    }
    me > [data-chat-compose] > label > [data-chat-compose-label] {
      display: block;
      margin: 0;
      font-size: var(--chat-font-label);
      font-weight: 700;
      letter-spacing: 0.05rem;
      text-transform: uppercase;
      color: color-mix(in srgb, var(--ui-text-muted) 92%, var(--ui-text) 8%);
    }
    me > [data-chat-compose] > [data-chat-compose-row] {
      display: grid;
      grid-template-columns: minmax(0, 1fr) auto;
      gap: var(--chat-space-1);
    }
    me > [data-chat-compose] > [data-chat-compose-row] > input[type="text"] {
      margin: 0;
      min-width: 0;
    }
    me > [data-chat-compose] > [data-chat-compose-row] > button {
      margin: 0;
      min-width: var(--chat-send-min-width);
    }
    me > [data-chat-compose] > [data-chat-compose-row] > [data-chat-send="demo"] {
      background: color-mix(in srgb, var(--ui-surface-card) 82%, transparent);
      border-color: var(--ui-border-soft);
      color: var(--ui-text);
    }
    me > [data-chat-readonly] {
      border: 1px dashed var(--ui-border-muted);
      border-radius: var(--chat-radius-compose);
      padding: var(--chat-space-2) var(--chat-readonly-pad-inline);
      font-size: var(--chat-font-body-sm);
    }
    @media (max-width: 520px) {
      me > [data-chat-compose] > [data-chat-compose-row] {
        grid-template-columns: 1fr;
      }
      me > [data-chat-compose] > [data-chat-compose-row] > button {
        width: 100%;
        min-width: 0;
      }
    }
}
