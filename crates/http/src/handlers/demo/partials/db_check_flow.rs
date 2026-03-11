use axum::http::StatusCode;
use statum::{machine, state, transition};

use crate::types::Text;

#[state]
pub enum DbCheckPartialState {
    Incoming,
    EmailPrepared,
    LookupEvaluated,
}

#[machine]
pub(super) struct DbCheckPartialFlow<DbCheckPartialState> {
    email_text: Option<String>,
    result: Option<Text>,
}

impl DbCheckPartialFlow<Incoming> {
    pub(super) fn from_query(email: Option<Text>) -> Self {
        DbCheckPartialFlow::<Incoming>::builder()
            .maybe_email_text(email.map(|value| value.to_string()))
            .maybe_result(None)
            .build()
    }
}

#[transition]
impl DbCheckPartialFlow<Incoming> {
    pub(super) fn prepare_email(mut self) -> DbCheckPartialFlow<EmailPrepared> {
        let email = self
            .email_text
            .clone()
            .unwrap_or_else(|| "demo@example.com".to_string());
        self.email_text = Some(email);
        self.transition()
    }
}

impl DbCheckPartialFlow<EmailPrepared> {
    pub(super) async fn evaluate_lookup(
        self,
        state: &crate::State,
    ) -> DbCheckPartialFlow<LookupEvaluated> {
        let email_text = self.email_text.clone().unwrap_or_default();
        tracing::info!(target: "demo.db", "db check requested");

        let result = match domain::user::Email::try_new(&email_text) {
            Ok(email) => match state.user.find_by_email(email).await {
                Ok(Some(_)) => Text::from("found"),
                Ok(None) => Text::from("not found"),
                Err(error) => {
                    tracing::warn!(%error, email = email_text, "db check lookup failed");
                    Text::from("lookup failed")
                }
            },
            Err(error) => {
                tracing::debug!(%error, email = email_text, "db check received invalid email");
                Text::from("invalid email")
            }
        };
        self.mark_lookup_evaluated(result)
    }
}

#[transition]
impl DbCheckPartialFlow<EmailPrepared> {
    fn mark_lookup_evaluated(
        mut self,
        result: Text,
    ) -> DbCheckPartialFlow<LookupEvaluated> {
        self.result = Some(result);
        self.transition()
    }
}

impl DbCheckPartialFlow<LookupEvaluated> {
    pub(super) fn into_response(
        self,
        state: &crate::State,
    ) -> (StatusCode, axum::response::Html<String>) {
        let trace = super::trace_snapshot(state);
        let partial = crate::views::partials::DbCheck::builder()
            .email(Text::from(self.email_text.unwrap_or_default()))
            .result(self.result.unwrap_or_else(|| Text::from("unknown")))
            .trace(trace)
            .build();
        (
            StatusCode::OK,
            axum::response::Html(maud::Render::render(&partial).into_string()),
        )
    }
}

pub(super) type IncomingFlow = DbCheckPartialFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_email_uses_default_when_missing() {
        let prepared = IncomingFlow::from_query(None).prepare_email();
        assert_eq!(prepared.email_text.as_deref(), Some("demo@example.com"));
    }
}
