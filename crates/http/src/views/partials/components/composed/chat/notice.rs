use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct Notice {
    pub lead: Text,
    pub link_href: Text,
    pub link_label: Text,
    #[builder(setters(name = with_tail))]
    pub tail: Option<Text>,
}

impl Render for Notice {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="u-muted" data-chat-readonly {
                (&self.lead) " "
                a href=(&self.link_href) { (&self.link_label) }
                @if let Some(tail) = &self.tail {
                    " " (&tail)
                }
            }
        }
    }
}
