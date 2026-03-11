use axum::response::IntoResponse;
use statum::{machine, state, transition};

use crate::paths::Route;
use crate::types::Text;
use crate::views::{self, pages};

#[state]
pub enum ProtectedGateState {
    Incoming,
    Guest,
    UserPrepared,
}

#[machine]
pub(super) struct ProtectedGateFlow<ProtectedGateState> {
    user: Option<crate::auth::User>,
    username: Option<Text>,
    email: Option<Text>,
}

impl ProtectedGateFlow<Incoming> {
    pub(super) fn from_session(auth_session: crate::auth::Session) -> Self {
        ProtectedGateFlow::<Incoming>::builder()
            .maybe_user(auth_session.user)
            .maybe_username(None)
            .maybe_email(None)
            .build()
    }

    pub(super) fn classify(self) -> ProtectedGateOutcome {
        if self.user.is_some() {
            ProtectedGateOutcome::Render(self.mark_user_prepared())
        } else {
            ProtectedGateOutcome::Guest(self.mark_guest())
        }
    }
}

#[transition]
impl ProtectedGateFlow<Incoming> {
    fn mark_guest(self) -> ProtectedGateFlow<Guest> {
        self.transition()
    }

    fn mark_user_prepared(mut self) -> ProtectedGateFlow<UserPrepared> {
        if let Some(user) = self.user.as_ref() {
            self.username = Some(Text::from(user.username.to_string()));
            self.email = Some(Text::from(user.email.to_string()));
        }
        self.transition()
    }
}

impl ProtectedGateFlow<Guest> {
    pub(super) fn into_response(self) -> axum::response::Response {
        axum::response::Redirect::to(Route::Login.as_str()).into_response()
    }
}

impl ProtectedGateFlow<UserPrepared> {
    pub(super) fn into_response(self) -> axum::response::Response {
        let username = self.username.unwrap_or_else(|| Text::from(""));
        let email = self.email.unwrap_or_else(|| Text::from(""));
        let user_nav = crate::views::page::UserNav::builder()
            .username(username.clone())
            .email(email.clone())
            .build();

        views::render(
            pages::Protected::builder()
                .username(username)
                .email(email)
                .user(user_nav)
                .build(),
        )
        .into_response()
    }
}

pub(super) enum ProtectedGateOutcome {
    Guest(ProtectedGateFlow<Guest>),
    Render(ProtectedGateFlow<UserPrepared>),
}

impl ProtectedGateOutcome {
    pub(super) fn into_response(self) -> axum::response::Response {
        match self {
            Self::Guest(guest) => guest.into_response(),
            Self::Render(render) => render.into_response(),
        }
    }
}

pub(super) type IncomingFlow = ProtectedGateFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_classification_routes_to_guest_variant() {
        let incoming = IncomingFlow::builder()
            .maybe_user(None)
            .maybe_username(None)
            .maybe_email(None)
            .build();

        assert!(matches!(
            incoming.classify(),
            ProtectedGateOutcome::Guest(_)
        ));
    }
}
