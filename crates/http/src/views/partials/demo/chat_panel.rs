use bon::Builder;
use maud::Render;

use crate::views::partials::ChatWindow;
use crate::paths::Route;

#[derive(Clone, Copy, Debug)]
pub enum ChatPanelRole {
    You,
    Demo,
}

impl ChatPanelRole {
    fn title(&self) -> &'static str {
        match self {
            ChatPanelRole::You => "You",
            ChatPanelRole::Demo => "Demo user",
        }
    }

    fn input_label(&self) -> &'static str {
        match self {
            ChatPanelRole::You => "Message as you",
            ChatPanelRole::Demo => "Message as demo user",
        }
    }

    fn placeholder(&self) -> &'static str {
        match self {
            ChatPanelRole::You => "Say something...",
            ChatPanelRole::Demo => "Send as demo user...",
        }
    }

    fn action(&self) -> &'static str {
        match self {
            ChatPanelRole::You => Route::ChatMessages.as_str(),
            ChatPanelRole::Demo => Route::ChatMessagesDemo.as_str(),
        }
    }

    fn input_signal(&self) -> &'static str {
        match self {
            ChatPanelRole::You => "body",
            ChatPanelRole::Demo => "botBody",
        }
    }

    fn button_label(&self) -> &'static str {
        match self {
            ChatPanelRole::You => "Send",
            ChatPanelRole::Demo => "Send as demo",
        }
    }

    fn button_class(&self) -> Option<&'static str> {
        match self {
            ChatPanelRole::You => None,
            ChatPanelRole::Demo => Some("secondary"),
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct ChatPanel {
    pub role: ChatPanelRole,
    pub messages: Vec<crate::views::partials::ChatMessage>,
}

impl Render for ChatPanel {
    fn render(&self) -> maud::Markup {
        let action = self.role.action();
        let input_signal = self.role.input_signal();
        maud::html! {
            div class="chat-stack" {
                (ChatWindow::builder()
                    .maybe_title(Some(self.role.title().to_string()))
                    .messages(self.messages.clone())
                    .build()
                    .render())
                form method="post"
                    action=(action)
                    data-target=".chat-messages"
                    data-swap="append"
                    data-on:submit=(format!("@post('{}'); ${} = ''", action, input_signal))
                {
                    label {
                        (self.role.input_label())
                        input type="text"
                            name="body"
                            placeholder=(self.role.placeholder())
                            data-bind=(input_signal)
                            required;
                    }
                    @if let Some(class) = self.role.button_class() {
                        button type="submit" class=(class) { (self.role.button_label()) }
                    } @else {
                        button type="submit" { (self.role.button_label()) }
                    }
                }
            }
        }
    }
}
