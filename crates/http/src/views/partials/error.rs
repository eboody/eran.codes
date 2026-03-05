use maud::Render;
use maud_extensions::css;

#[derive(Debug)]
pub struct Error {
    pub message: &'static str,
}

impl Render for Error {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="error-target" role="alert" {
                strong { "Something went wrong" }
                p { (self.message) }
                ({
                    css! {
                        me {
                          border: 1px solid var(--ui-text-danger);
                          background: var(--ui-surface-danger);
                        }
                        me strong {
                          color: var(--ui-text-danger);
                        }
                    }
                })
            }
        }
    }
}
