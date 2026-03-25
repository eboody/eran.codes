use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

use super::Results;

#[derive(Clone, Copy, Debug)]
enum Label {
    Authenticated,
    Anonymous,
}

impl From<Label> for Text {
    fn from(value: Label) -> Self {
        match value {
            Label::Authenticated => Text::from("Authenticated"),
            Label::Anonymous => Text::from("Anonymous"),
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
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for AuthStatus {
    fn render(&self) -> maud::Markup {
        let status = if self.user_id.is_some() {
            Label::Authenticated
        } else {
            Label::Anonymous
        };
        let items = vec![
            partials::StatusCardItem::optional("user_id", self.user_id.clone()),
            partials::StatusCardItem::optional("username", self.username.clone()),
            partials::StatusCardItem::optional("email", self.email.clone()),
            partials::StatusCardItem::optional("session_id", self.session_id.clone()),
            partials::StatusCardItem::optional("expiry", self.expiry.clone()),
        ];

        Results::builder()
            .target_id(Text::from("auth-status-target"))
            .summary(
                partials::StatusCard::builder()
                    .title(Text::from(status))
                    .items(items)
                    .build()
                    .render(),
            )
            .trace(
                partials::RequestTraceLog::builder()
                    .entries(&self.trace)
                    .build()
                    .render(),
            )
            .build()
            .render()
    }
}
