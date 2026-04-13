#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug, Builder)]
pub struct Composer {
    pub action: Text,
    pub input_label: Text,
    pub input_name: Text,
    pub input_id: Text,
    pub input_signal: Text,
    pub placeholder: Text,
    pub submit: partials::button::Button,
}

impl Render for Composer {
    fn render(&self) -> maud::Markup {
        let submit_action = self.submit_action();

        maud::html! {
            form method="post" action=(&self.action) data-chat-compose data-on:submit=(submit_action) {
                label for=(&self.input_id) {
                    span data-chat-compose-label { (&self.input_label) }
                }
                div data-chat-compose-row {
                    input
                        id=(&self.input_id)
                        type="text"
                        name=(&self.input_name)
                        placeholder=(&self.placeholder)
                        data-bind=(&self.input_signal)
                        required;
                    (self.submit.render())
                }
            }
        }
    }
}

impl Composer {
    fn submit_action(&self) -> String {
        let signal_name = self.input_signal.to_string();
        let include_pattern = format!(
            "/^(?:{}|sseTabId)$/",
            regex_literal_fragment(signal_name.as_str())
        );

        format!(
            "@post('{}', {{filterSignals: {{include: {include_pattern}}}}}); ${signal_name} = ''",
            self.action
        )
    }
}

fn regex_literal_fragment(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' | '.' | '+' | '*' | '?' | '^' | '$' | '(' | ')' | '[' | ']' | '{'
            | '}' | '|' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}
