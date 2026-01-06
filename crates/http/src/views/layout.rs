pub struct PageLayout<'a> {
    pub title: &'a str,
    pub content: maud::Markup,
}

impl maud::Render for PageLayout<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    title { (self.title) }
                    link rel="stylesheet" href="/static/app.css";
                    script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js" {}
                    script src="/static/css-scope-inline.js" {}
                    script src="/static/surreal.js" {}
                }
                body { (self.content.clone()) }
            }
        }
    }
}
