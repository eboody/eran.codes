pub fn page(title: &str, content: maud::Markup) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                link rel="stylesheet" href="/static/app.css";
                script src="https://unpkg.com/htmx.org@1.9.12" {}
                script src="/static/css-scope-inline.js" {}
                script src="/static/surreal.js" {}
            }
            body {
                (content)
            }
        }
    }
}
