use bon::Builder;
use maud::Render;

use crate::views::partials::{StatusCard, TraceLog};

#[derive(Clone, Copy, Debug)]
enum AuthStatusLabel {
    Authenticated,
    Anonymous,
}

impl AuthStatusLabel {
    fn as_str(self) -> &'static str {
        match self {
            AuthStatusLabel::Authenticated => "Authenticated",
            AuthStatusLabel::Anonymous => "Anonymous",
        }
    }
}

#[derive(Builder)]
pub struct AuthStatus<'a> {
    pub user_id: Option<&'a str>,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    pub session_id: Option<String>,
    pub expiry: Option<String>,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for AuthStatus<'_> {
    fn render(&self) -> maud::Markup {
        let status = if self.user_id.is_some() {
            AuthStatusLabel::Authenticated
        } else {
            AuthStatusLabel::Anonymous
        };
        let items = vec![
            ("user_id".to_string(), self.user_id.unwrap_or("none").to_string()),
            ("username".to_string(), self.username.unwrap_or("none").to_string()),
            ("email".to_string(), self.email.unwrap_or("none").to_string()),
            (
                "session_id".to_string(),
                self.session_id.as_deref().unwrap_or("none").to_string(),
            ),
            (
                "expiry".to_string(),
                self.expiry.as_deref().unwrap_or("none").to_string(),
            ),
        ];

        maud::html! {
            article id="auth-status-target" {
                (StatusCard::builder()
                    .title(status.as_str().to_string())
                    .items(items)
                    .build()
                    .render())
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
