use bon::Builder;
use maud::Render;

use crate::views::partials::ChatWindow;

#[derive(Clone, Debug, Builder)]
pub struct ChatPanel {
    pub title: String,
    pub messages: Vec<crate::views::partials::ChatMessage>,
    pub action: String,
    pub input_label: String,
    pub placeholder: String,
    pub input_name: String,
    pub input_signal: String,
    pub button_label: String,
    pub button_class: Option<String>,
}

impl Render for ChatPanel {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="chat-stack" {
                (ChatWindow::builder()
                    .maybe_title(Some(self.title.clone()))
                    .messages(self.messages.clone())
                    .build()
                    .render())
                form method="post"
                    action=(self.action.as_str())
                    data-target=".chat-messages"
                    data-swap="append"
                    data-on:submit=(format!("@post('{}'); ${} = ''", self.action, self.input_signal))
                {
                    label {
                        (self.input_label.as_str())
                        input type="text"
                            name=(self.input_name.as_str())
                            placeholder=(self.placeholder.as_str())
                            data-bind=(self.input_signal.as_str())
                            required;
                    }
                    @if let Some(class) = &self.button_class {
                        button type="submit" class=(class) { (self.button_label.as_str()) }
                    } @else {
                        button type="submit" { (self.button_label.as_str()) }
                    }
                }
            }
        }
    }
}
