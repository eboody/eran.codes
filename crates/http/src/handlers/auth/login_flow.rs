use axum::response::IntoResponse;
use secrecy::SecretString;
use statum::{machine, state, transition};

use super::{LoginForm, NextPath};
use crate::paths::Route;

#[derive(Clone, Debug)]
pub struct AuthenticatedData {
    user: crate::auth::User,
}

#[state]
pub enum LoginFlowState {
    Incoming,
    Authenticated(AuthenticatedData),
    Rejected,
}

#[machine]
pub(super) struct LoginFlow<LoginFlowState> {
    next: Option<String>,
    credentials: app::auth::Credentials,
}

impl LoginFlow<Incoming> {
    pub(super) fn from_form(form: LoginForm) -> crate::Result<Self> {
        let next = NextPath::sanitize(form.next.clone());
        let email = domain::user::Email::try_new(form.email.to_string())
            .map_err(domain::user::Error::from)
            .map_err(app::user::Error::from)
            .map_err(crate::Error::from)?;
        let credentials = app::auth::Credentials::builder()
            .email(email)
            .password(SecretString::new(form.password.to_string().into()))
            .build();

        Ok(LoginFlow::<Incoming>::builder()
            .next(next)
            .credentials(credentials)
            .build())
    }

    pub(super) async fn authenticate(
        self,
        auth_session: &mut crate::auth::Session,
    ) -> crate::Result<LoginAuthResult> {
        let user = auth_session.authenticate(self.credentials.clone()).await?;
        Ok(self.apply_auth_result(user))
    }

    pub(super) fn apply_auth_result(
        self,
        user: Option<crate::auth::User>,
    ) -> LoginAuthResult {
        match user {
            Some(user) => LoginAuthResult::Authenticated(self.mark_authenticated(user)),
            None => LoginAuthResult::Rejected(self.mark_rejected()),
        }
    }
}

#[transition]
impl LoginFlow<Incoming> {
    fn mark_authenticated(self, user: crate::auth::User) -> LoginFlow<Authenticated> {
        self.transition_with(AuthenticatedData { user })
    }

    fn mark_rejected(self) -> LoginFlow<Rejected> {
        self.transition()
    }
}

impl<S: LoginFlowStateTrait> LoginFlow<S> {
    pub(super) fn into_next(self) -> Option<String> {
        self.next
    }
}

impl LoginFlow<Authenticated> {
    pub(super) fn user(&self) -> &crate::auth::User {
        &self.state_data.user
    }
}

pub(super) enum LoginAuthResult {
    Authenticated(LoginFlow<Authenticated>),
    Rejected(LoginFlow<Rejected>),
}

impl LoginAuthResult {
    pub(super) async fn into_response(
        self,
        auth_session: &mut crate::auth::Session,
    ) -> crate::Result<axum::response::Response> {
        match self {
            Self::Authenticated(authenticated) => {
                auth_session.login(authenticated.user()).await?;
                let target = authenticated
                    .into_next()
                    .unwrap_or_else(|| Route::Protected.as_str().to_string());
                Ok(axum::response::Redirect::to(&target).into_response())
            }
            Self::Rejected(rejected) => {
                let next = rejected.into_next();
                Ok(crate::views::render(
                    crate::views::pages::Login::builder()
                        .message("Invalid email or password.")
                        .maybe_next(next.as_deref())
                        .build(),
                )
                .into_response())
            }
        }
    }
}

pub(super) type IncomingFlow = LoginFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Text;

    fn login_form() -> LoginForm {
        LoginForm::builder()
            .email(Text::from("user@example.com"))
            .password(Text::from("pw"))
            .next(Text::from("/protected"))
            .build()
    }

    fn test_user() -> crate::auth::User {
        crate::auth::User::builder()
            .id(crate::auth::UserId::from(domain::user::Id::new_v4()))
            .username(domain::user::Username::try_new("person").expect("valid username"))
            .email(domain::user::Email::try_new("person@example.com").expect("valid email"))
            .session_hash_bytes(vec![1, 2, 3])
            .build()
    }

    #[test]
    fn apply_auth_result_routes_to_authenticated_state() {
        let incoming = LoginFlow::<Incoming>::from_form(login_form()).expect("incoming");

        let outcome = incoming.apply_auth_result(Some(test_user()));
        assert!(matches!(outcome, LoginAuthResult::Authenticated(_)));
    }

    #[test]
    fn apply_auth_result_routes_to_rejected_state() {
        let incoming = LoginFlow::<Incoming>::from_form(login_form()).expect("incoming");

        let outcome = incoming.apply_auth_result(None);
        assert!(matches!(outcome, LoginAuthResult::Rejected(_)));
    }

    #[test]
    fn from_form_drops_unsafe_next() {
        let form = LoginForm::builder()
            .email(Text::from("user@example.com"))
            .password(Text::from("pw"))
            .next(Text::from("//evil.example"))
            .build();

        let incoming = LoginFlow::<Incoming>::from_form(form).expect("incoming");

        assert!(incoming.into_next().is_none());
    }
}
