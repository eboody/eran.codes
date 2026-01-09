use maud::{Markup, Render};

pub struct Layout<'a> {
    pub title: &'a str,
    pub content: Markup,
}

impl Render for Layout<'_> {
    fn render(&self) -> Markup {
        maud::html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    title { (self.title) }
                    link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png";
                    link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png";
                    link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
                    link rel="icon" href="/static/favicon.ico";
                    link rel="stylesheet" href="/static/app.css";
                    script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js" {}
                    script src="/static/css-scope-inline.js" {}
                    script src="/static/surreal.js" {}
                }
                body {
                    div id="error-target" {}
                    (self.content.clone())
                }
            }
        }
    }
}
