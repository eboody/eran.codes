use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials;

use super::Results;

#[derive(Clone, Debug, Builder)]
pub struct SessionStatus {
    pub session_id: Option<Text>,
    pub expiry: Option<Text>,
    pub trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl Render for SessionStatus {
    fn render(&self) -> maud::Markup {
        Results::builder()
            .target_id(Text::from("session-status-target"))
            .summary(
                partials::StatusCard::builder()
                    .title(Text::from("Session details"))
                    .items(vec![
                        partials::StatusCardItem::text(
                            "session_id",
                            super::present_redacted(&self.session_id),
                        ),
                        partials::StatusCardItem::optional("expiry", self.expiry.clone()),
                    ])
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
    fn redacts_session_id() {
        let markup = SessionStatus::builder()
            .maybe_session_id(Some(Text::from("session-1")))
            .maybe_expiry(Some(Text::from("soon")))
            .trace(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("present (redacted)"));
        assert!(!markup.contains("session-1"));
    }
}
