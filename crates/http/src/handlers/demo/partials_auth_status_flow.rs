use axum::http::StatusCode;
use maud::Render;
use statum::{machine, state, transition};
use tower_sessions::Session;

use crate::types::Text;

#[state]
pub enum AuthStatusPartialState {
    Incoming,
    SnapshotPrepared,
}

#[machine]
pub(super) struct AuthStatusPartialFlow<AuthStatusPartialState> {
    user_id: Option<Text>,
    username: Option<Text>,
    email: Option<Text>,
    session_id: Option<Text>,
    expiry: Option<Text>,
    trace: Vec<crate::trace_log::TraceEntry>,
}

impl AuthStatusPartialFlow<Incoming> {
    pub(super) fn new() -> Self {
        AuthStatusPartialFlow::<Incoming>::builder()
            .maybe_user_id(None)
            .maybe_username(None)
            .maybe_email(None)
            .maybe_session_id(None)
            .maybe_expiry(None)
            .trace(Vec::new())
            .build()
    }
}

#[transition]
impl AuthStatusPartialFlow<Incoming> {
    pub(super) fn prepare_snapshot(
        mut self,
        auth_session: &crate::auth::Session,
        session: &Session,
        state: &crate::State,
    ) -> AuthStatusPartialFlow<SnapshotPrepared> {
        let user = auth_session.user.as_ref();
        self.user_id = user.map(|value| Text::from(value.id.to_string()));
        self.username = user.map(|value| Text::from(value.username.to_string()));
        self.email = user.map(|value| Text::from(value.email.to_string()));
        self.session_id = session.id().map(|value| Text::from(value.to_string()));
        self.expiry = session
            .expiry()
            .map(|value| Text::from(format!("{value:?}")));
        self.trace = super::partials::trace_snapshot(state);
        self.transition()
    }
}

impl AuthStatusPartialFlow<SnapshotPrepared> {
    pub(super) fn into_response(self) -> (StatusCode, axum::response::Html<String>) {
        let partial = crate::views::partials::AuthStatus::builder()
            .maybe_user_id(self.user_id)
            .maybe_username(self.username)
            .maybe_email(self.email)
            .maybe_session_id(self.session_id)
            .maybe_expiry(self.expiry)
            .trace(self.trace)
            .build();
        (
            StatusCode::OK,
            axum::response::Html(partial.render().into_string()),
        )
    }
}

pub(super) type IncomingFlow = AuthStatusPartialFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepared_flow_renders_auth_status_target() {
        let prepared = AuthStatusPartialFlow::<SnapshotPrepared>::builder()
            .user_id(Text::from("user-1"))
            .username(Text::from("demo"))
            .email(Text::from("demo@example.com"))
            .session_id(Text::from("session-1"))
            .expiry(Text::from("in one hour"))
            .trace(Vec::new())
            .build();

        let response = prepared.into_response();
        assert_eq!(response.0, StatusCode::OK);
        assert!(response.1.0.contains("auth-status-target"));
    }
}
