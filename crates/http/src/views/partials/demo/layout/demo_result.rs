use bon::Builder;
use maud::Render;
use maud_extensions::css;

use crate::views::partials::components::EmptyState;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct DemoResultPlaceholder {
    pub target_id: Text,
    pub message: Text,
}

impl Render for DemoResultPlaceholder {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id=(&self.target_id) data-demo-result data-muted {
                (EmptyState::builder()
                    .message(self.message.clone())
                    .build())
            }
            ({
                css! {
                    me [data-demo-result] {
                      margin-top: 0.8rem;
                      padding: 0.8rem 1rem;
                      border-radius: var(--ui-radius-sm);
                      border: 1px solid var(--pico-muted-border-color);
                      background: var(--pico-card-background-color);
                    }
                    me [data-demo-result][data-muted] {
                      color: color-mix(in srgb, var(--pico-muted-color) 94%, var(--pico-color) 6%);
                    }
                }
            })
        }
    }
}
