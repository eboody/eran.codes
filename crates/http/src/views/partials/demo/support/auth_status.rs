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
            partials::StatusCardItem::text(
                "user_id",
                super::authenticated_redacted(&self.user_id),
            ),
            partials::StatusCardItem::optional("username", self.username.clone()),
            partials::StatusCardItem::optional("email", self.email.clone()),
            partials::StatusCardItem::text(
                "session_id",
                super::present_redacted(&self.session_id),
            ),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_internal_identifiers_but_keeps_self_identity_fields() {
        let markup = AuthStatus::builder()
            .maybe_user_id(Some(Text::from("user-1")))
            .maybe_username(Some(Text::from("demo")))
            .maybe_email(Some(Text::from("demo@example.com")))
            .maybe_session_id(Some(Text::from("session-1")))
            .maybe_expiry(Some(Text::from("soon")))
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("authenticated (redacted)"));
        assert!(markup.contains("present (redacted)"));
        assert!(markup.contains("demo"));
        assert!(markup.contains("demo@example.com"));
        assert!(!markup.contains("user-1"));
        assert!(!markup.contains("session-1"));
    }
}
