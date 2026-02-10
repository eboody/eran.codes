use bon::Builder;
use maud::Render;

use crate::views::partials::EmptyState;

#[derive(Clone, Debug, Builder)]
pub struct LogPanel {
    pub title: String,
    pub body: maud::Markup,
    pub empty_message: Option<String>,
}

impl Render for LogPanel {
    fn render(&self) -> maud::Markup {
        let body = if let Some(message) = &self.empty_message {
            maud::html! { (EmptyState::builder().message(message.clone()).build().render()) }
        } else {
            self.body.clone()
        };
        maud::html! {
            article class="demo-result network-log-panel" {
                p { strong { (&self.title) } }
                div class="network-log-scroll" {
                    (body)
                }
            }
        }
    }
}
