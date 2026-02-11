use bon::Builder;
use maud::Render;

use crate::views::partials::{StatusCard, TraceLog};
use crate::types::Text;

#[derive(Clone, Copy, Debug)]
enum AuthStatusLabel {
    Authenticated,
    Anonymous,
}

impl From<AuthStatusLabel> for Text {
    fn from(value: AuthStatusLabel) -> Self {
        match value {
            AuthStatusLabel::Authenticated => Text::from("Authenticated"),
            AuthStatusLabel::Anonymous => Text::from("Anonymous"),
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct AuthStatus {
    pub user_id: Option<Text>,
    pub username: Option<Text>,
    pub email: Option<Text>,
    pub session_id: Option<Text>,
    pub expiry: Option<Text>,
    pub trace: Vec<crate::trace_log::TraceEntry>,
}

impl Render for AuthStatus {
    fn render(&self) -> maud::Markup {
        let status = if self.user_id.is_some() {
            AuthStatusLabel::Authenticated
        } else {
            AuthStatusLabel::Anonymous
        };
        let items = vec![
            (
                Text::from("user_id"),
                self.user_id.clone().unwrap_or_else(|| Text::from("none")),
            ),
            (
                Text::from("username"),
                self.username.clone().unwrap_or_else(|| Text::from("none")),
            ),
            (
                Text::from("email"),
                self.email.clone().unwrap_or_else(|| Text::from("none")),
            ),
            (
                Text::from("session_id"),
                self.session_id
                    .clone()
                    .unwrap_or_else(|| Text::from("none")),
            ),
            (
                Text::from("expiry"),
                self.expiry
                    .clone()
                    .unwrap_or_else(|| Text::from("none")),
            ),
        ];

        maud::html! {
            article id="auth-status-target" {
                (StatusCard::builder()
                    .title(Text::from(status))
                    .items(items)
                    .build()
                    .render())
                (TraceLog::builder().entries(&self.trace).build().render())
            }
        }
    }
}
