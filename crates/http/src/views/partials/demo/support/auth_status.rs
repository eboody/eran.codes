use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

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
            partials::StatusCardItem::optional("user_id", self.user_id.clone()),
            partials::StatusCardItem::optional("username", self.username.clone()),
            partials::StatusCardItem::optional("email", self.email.clone()),
            partials::StatusCardItem::optional("session_id", self.session_id.clone()),
            partials::StatusCardItem::optional("expiry", self.expiry.clone()),
        ];

        maud::html! {
            article id="auth-status-target" {
                (partials::StatusCard::builder()
                    .title(Text::from(status))
                    .items(items)
                    .build())
                (partials::RequestTraceLog::builder().entries(&self.trace).build())
            }
        }
    }
}
