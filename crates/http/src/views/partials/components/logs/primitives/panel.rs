use bon::Builder;
use maud::Render;

use crate::types::Text;

use crate::views::partials::components::logs;

#[derive(Clone, Debug)]
pub enum Body {
    Content(maud::Markup),
    Empty(Text),
}

#[derive(Clone, Debug, Builder)]
pub struct Panel {
    pub title: Text,
    pub body: Body,
}

impl Render for Panel {
    fn render(&self) -> maud::Markup {
        let body = match &self.body {
            Body::Content(markup) => markup.clone(),
            Body::Empty(message) => {
                maud::html! { (logs::primitives::EmptyState::builder().message(message.clone()).build()) }
            }
        };

        maud::html! {
            article class="ui-log-panel" data-log-panel {
                header data-log-heading {
                    h3 { (&self.title) }
                }
                div class="ui-log-scroll" data-log-scroll {
                    (body)
                }
            }
        }
    }
}
