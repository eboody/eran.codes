use maud::Render;

pub struct SessionStatus<'a> {
    pub session_id: Option<&'a str>,
    pub expiry: Option<&'a str>,
}

impl Render for SessionStatus<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article id="session-status-target" class="demo-result" {
                p { strong { "Session details" } }
                ul {
                    li { "session_id: " (self.session_id.unwrap_or("none")) }
                    li { "expiry: " (self.expiry.unwrap_or("none")) }
                }
            }
        }
    }
}
