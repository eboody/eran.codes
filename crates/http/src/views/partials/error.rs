use maud::Render;

#[derive(Debug)]
pub struct Error {
    pub message: &'static str,
}

impl Render for Error {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="error-target" class="ui-error-alert" role="alert" {
                strong { "Something went wrong" }
                p { (self.message) }
            }
        }
    }
}
