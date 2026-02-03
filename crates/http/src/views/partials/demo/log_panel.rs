use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct LogPanel {
    pub title: String,
    pub body: maud::Markup,
}

impl Render for LogPanel {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article class="demo-result network-log-panel" {
                p { strong { (&self.title) } }
                div class="network-log-scroll" {
                    (self.body.clone())
                }
            }
        }
    }
}
