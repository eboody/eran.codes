use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::button;

#[derive(Clone, Debug, Builder)]
pub struct Composer {
    pub action: Text,
    pub input_label: Text,
    pub input_name: Text,
    pub input_id: Text,
    pub input_signal: Text,
    pub placeholder: Text,
    pub submit: button::Button,
}

impl Render for Composer {
    fn render(&self) -> maud::Markup {
        let submit_action = format!("@post('{}'); ${} = ''", self.action, self.input_signal);

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
