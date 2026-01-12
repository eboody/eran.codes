use maud::Render;

pub struct AuthStatus<'a> {
    pub user_id: Option<&'a str>,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    pub session_id: Option<String>,
    pub expiry: Option<String>,
}

impl Render for AuthStatus<'_> {
    fn render(&self) -> maud::Markup {
        let status = if self.user_id.is_some() {
            "Authenticated"
        } else {
            "Anonymous"
        };

        maud::html! {
            article id="auth-status-target" class="demo-result" {
                p { strong { (status) } }
                ul {
                    li { "user_id: " (self.user_id.unwrap_or("none")) }
                    li { "username: " (self.username.unwrap_or("none")) }
                    li { "email: " (self.email.unwrap_or("none")) }
                    li { "session_id: " (self.session_id.as_deref().unwrap_or("none")) }
                    li { "expiry: " (self.expiry.as_deref().unwrap_or("none")) }
                }
            }
        }
    }
}
