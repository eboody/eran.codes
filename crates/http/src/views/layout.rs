use maud::{Markup, html};

pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                link rel="stylesheet" href="/static/app.css";
                script src="https://unpkg.com/htmx.org@1.9.12" {}
            }
            body {
                (content)
            }
        }
    }
}
