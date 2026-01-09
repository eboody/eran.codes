use maud::Render;
use maud_extensions::css;

#[derive(Debug)]
pub struct Error {
    pub message: &'static str,
}

impl Render for Error {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id="error-target" class="card error" {
                strong { "Something went wrong" }
                p { (self.message) }
                ({
                    css! {
                        me {
                          border: 1px solid #b00020;
                          background: #fff2f2;
                        }
                        me strong {
                          color: #b00020;
                        }
                    }
                })
            }
        }
    }
}
