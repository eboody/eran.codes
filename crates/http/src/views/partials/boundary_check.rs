use maud::Render;

pub struct BoundaryCheck<'a> {
    pub label: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub result: &'a str,
}

impl Render for BoundaryCheck<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="boundary-target" class="demo-result" {
                p { strong { (self.label) } }
                ul {
                    li { "username: " (self.username) }
                    li { "email: " (self.email) }
                    li { "result: " (self.result) }
                }
            }
        }
    }
}
