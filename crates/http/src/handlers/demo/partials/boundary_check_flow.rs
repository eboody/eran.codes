use axum::http;
use maud::Render;
use statum::{machine, state, transition};

use crate::types::Text;

#[state]
pub enum BoundaryCheckPartialState {
    Incoming,
    CaseResolved,
    ValidationEvaluated,
}

#[machine]
pub(super) struct BoundaryCheckPartialFlow<BoundaryCheckPartialState> {
    case_text: Option<String>,
    label: Option<Text>,
    username: Option<Text>,
    email: Option<Text>,
    result: Option<Text>,
    trace: Vec<crate::trace_log::TraceEntry>,
}

impl BoundaryCheckPartialFlow<Incoming> {
    pub(super) fn from_query(case: Option<Text>) -> Self {
        BoundaryCheckPartialFlow::<Incoming>::builder()
            .case_text(case.map(|value| value.to_string()))
            .label(None)
            .username(None)
            .email(None)
            .result(None)
            .trace(Vec::new())
            .build()
    }
}

#[transition]
impl BoundaryCheckPartialFlow<Incoming> {
    pub(super) fn resolve_case(mut self) -> BoundaryCheckPartialFlow<CaseResolved> {
        let (label, username, email) = match self.case_text.as_deref() {
            Some("invalid") => ("Invalid input", " ", "not-an-email"),
            _ => ("Valid input", "demo_user", "demo@example.com"),
        };
        self.label = Some(Text::from(label));
        self.username = Some(Text::from(username));
        self.email = Some(Text::from(email));
        self.transition()
    }
}

#[transition]
impl BoundaryCheckPartialFlow<CaseResolved> {
    pub(super) fn evaluate(
        mut self,
        state: &crate::State,
    ) -> BoundaryCheckPartialFlow<ValidationEvaluated> {
        let username = self
            .username
            .clone()
            .unwrap_or_else(|| Text::from(""))
            .to_string();
        let email = self
            .email
            .clone()
            .unwrap_or_else(|| Text::from(""))
            .to_string();
        let result = match app::user::validate_input(&username, &email) {
            Ok(_) => "ok",
            Err(err) => {
                tracing::debug!(?err, "boundary validation failed");
                "error"
            }
        };
        self.result = Some(Text::from(result));
        self.trace = super::trace_snapshot(state);
        self.transition()
    }
}

impl BoundaryCheckPartialFlow<ValidationEvaluated> {
    pub(super) fn into_response(self) -> (http::StatusCode, axum::response::Html<String>) {
        let partial = crate::views::partials::BoundaryCheck::builder()
            .label(self.label.unwrap_or_else(|| Text::from("Boundary check")))
            .username(self.username.unwrap_or_else(|| Text::from("none")))
            .email(self.email.unwrap_or_else(|| Text::from("none")))
            .result(self.result.unwrap_or_else(|| Text::from("unknown")))
            .trace(self.trace)
            .build();
        (
            http::StatusCode::OK,
            axum::response::Html(partial.render().into_string()),
        )
    }
}

pub(super) type IncomingFlow = BoundaryCheckPartialFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluated_flow_renders_boundary_target() {
        let evaluated = BoundaryCheckPartialFlow::<ValidationEvaluated>::builder()
            .case_text(None)
            .label(Some(Text::from("Valid input")))
            .username(Some(Text::from("demo_user")))
            .email(Some(Text::from("demo@example.com")))
            .result(Some(Text::from("ok")))
            .trace(Vec::new())
            .build();

        let response = evaluated.into_response();
        assert_eq!(response.0, http::StatusCode::OK);
        assert!(response.1.0.contains("boundary-target"));
    }
}
