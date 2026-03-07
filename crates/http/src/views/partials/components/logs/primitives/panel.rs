use bon::Builder;
use maud::Render;

use crate::types::Text;

use crate::views::partials::components::logs;

#[derive(Clone, Debug)]
pub enum PanelBody {
    Content(maud::Markup),
    Empty(Text),
}

#[derive(Clone, Debug, Builder)]
pub struct Panel {
    pub title: Text,
    pub body: PanelBody,
}

impl Render for Panel {
    fn render(&self) -> maud::Markup {
        let body = match &self.body {
            PanelBody::Content(markup) => markup.clone(),
            PanelBody::Empty(message) => {
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
