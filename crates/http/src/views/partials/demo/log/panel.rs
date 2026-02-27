use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::EmptyState;

#[derive(Clone, Debug, Builder)]
pub struct Panel {
    pub title: Text,
    pub body: maud::Markup,
    pub empty_message: Option<Text>,
}

impl Render for Panel {
    fn render(&self) -> maud::Markup {
        let body = if let Some(message) = &self.empty_message {
            maud::html! { (EmptyState::builder().message(message.clone()).build()) }
        } else {
            self.body.clone()
        };
        maud::html! {
            article data-demo-result data-log-panel {
                header data-log-heading {
                    h3 { (&self.title) }
                }
                div data-log-scroll {
                    (body)
                }
            }
        }
    }
}
