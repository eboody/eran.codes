use bon::Builder;
use maud::Render;
use maud_extensions::css;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct EmptyState {
    pub message: Text,
}

impl Render for EmptyState {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p data-empty-state { (&self.message) }
            ({
                css! {
                    me [data-empty-state] {
                      margin: 0;
                      font-size: 0.86rem;
                      line-height: 1.45;
                      color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
                    }
                }
            })
        }
    }
}
