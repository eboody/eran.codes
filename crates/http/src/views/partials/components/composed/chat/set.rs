#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::views::partials::components::chat;

#[derive(Clone, Debug, Builder)]
pub struct Set {
    pub panels: Vec<chat::Panel>,
    #[builder(setters(name = with_connection))]
    pub connection: Option<chat::Connection>,
}

impl Render for Set {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @if let Some(connection) = &self.connection {
                (connection.render())
            }
            div data-chat-columns {
                @for panel in &self.panels {
                    (panel.render())
                }
            }
        }
    }
}
