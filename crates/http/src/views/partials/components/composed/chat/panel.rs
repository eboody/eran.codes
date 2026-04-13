#[cfg(test)]
mod tests;

use maud::Render;

use crate::views::partials::components::chat;

#[derive(Clone, Debug)]
enum Footer {
    Composer(chat::Composer),
    Notice(chat::Notice),
}

#[derive(Clone, Debug)]
pub struct Panel {
    window: chat::Window,
    footer: Footer,
}

impl Panel {
    pub fn composer(window: chat::Window, composer: chat::Composer) -> Self {
        Self {
            window,
            footer: Footer::Composer(composer),
        }
    }

    pub fn notice(window: chat::Window, notice: chat::Notice) -> Self {
        Self {
            window,
            footer: Footer::Notice(notice),
        }
    }
}

impl Render for Panel {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-panel {
                (self.window.render())
                @match &self.footer {
                    Footer::Composer(composer) => {
                        (composer.render())
                    }
                    Footer::Notice(notice) => {
                        (notice.render())
                    }
                }
            }
        }
    }
}
