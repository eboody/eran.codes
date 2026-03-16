use axum::response::IntoResponse;
use secrecy::SecretString;
use statum::{machine, state, transition};

use super::{NextPath, RegisterForm};
use crate::paths::Route;

#[derive(Clone, Debug)]
pub struct AuthenticatedData {
    user: crate::auth::User,
}

#[state]
pub enum RegisterFlowState {
    Incoming,
    RegistrationSucceeded,
    EmailTaken,
    InvalidInput,
    Authenticated(AuthenticatedData),
}

#[machine]
pub(super) struct RegisterFlow<RegisterFlowState> {
    next: Option<String>,
    command: app::user::Register,
    credentials: app::auth::Credentials,
}

impl RegisterFlow<Incoming> {
    pub(super) fn from_form(form: RegisterForm) -> crate::Result<Self> {
        let next = NextPath::sanitize(form.next.clone());
        let email = domain::user::Email::try_new(form.email.to_string())
            .map_err(domain::user::Error::from)
            .map_err(app::user::Error::from)
            .map_err(crate::Error::from)?;
        let username = domain::user::Username::try_new(form.username.to_string())
            .map_err(domain::user::Error::from)
            .map_err(app::user::Error::from)
            .map_err(crate::Error::from)?;
        let password = SecretString::new(form.password.to_string().into());

        let command = app::user::Register::builder()
            .username(username)
            .email(email.clone())
            .password(password.clone())
            .build();
        let credentials = app::auth::Credentials::builder()
            .email(email)
            .password(password)
            .build();

        Ok(RegisterFlow::<Incoming>::builder()
            .next(next)
            .command(command)
            .credentials(credentials)
            .build())
    }

    pub(super) async fn register(
        self,
        state: &crate::State,
    ) -> crate::Result<RegistrationOutcome> {
        let result = state.user.register_user(self.command.clone()).await;
        self.apply_registration_result(result)
    }

    pub(super) fn apply_registration_result(
        self,
        result: app::user::Result<domain::user::Id>,
    ) -> crate::Result<RegistrationOutcome> {
        match result {
            Ok(_) => Ok(RegistrationOutcome::Succeeded(
                self.mark_registration_succeeded(),
            )),
            Err(app::user::Error::EmailTaken) => {
                Ok(RegistrationOutcome::EmailTaken(self.mark_email_taken()))
            }
            Err(app::user::Error::Domain { .. }) => {
                Ok(RegistrationOutcome::InvalidInput(self.mark_invalid_input()))
            }
            Err(error) => Err(error.into()),
        }
    }
}

#[transition]
impl RegisterFlow<Incoming> {
    fn mark_registration_succeeded(self) -> RegisterFlow<RegistrationSucceeded> {
        self.transition()
    }

    fn mark_email_taken(self) -> RegisterFlow<EmailTaken> {
        self.transition()
    }

    fn mark_invalid_input(self) -> RegisterFlow<InvalidInput> {
        self.transition()
    }
}

#[transition]
impl RegisterFlow<RegistrationSucceeded> {
    fn mark_authenticated(self, user: crate::auth::User) -> RegisterFlow<Authenticated> {
        self.transition_with(AuthenticatedData { user })
    }
}

impl RegisterFlow<RegistrationSucceeded> {
    pub(super) async fn authenticate(
        self,
        auth_session: &mut crate::auth::Session,
    ) -> crate::Result<RegisterFlow<Authenticated>> {
        let user = auth_session.authenticate(self.credentials.clone()).await?;
        self.require_authentication(user)
    }

    pub(super) fn require_authentication(
        self,
        user: Option<crate::auth::User>,
    ) -> crate::Result<RegisterFlow<Authenticated>> {
        match user {
            Some(user) => Ok(self.mark_authenticated(user)),
            None => Err(crate::Error::Internal),
        }
    }
}

impl<S: RegisterFlowStateTrait> RegisterFlow<S> {
    pub(super) fn into_next(self) -> Option<String> {
        self.next
    }
}

impl RegisterFlow<Authenticated> {
    pub(super) fn user(&self) -> &crate::auth::User {
        &self.state_data.user
    }
}

pub(super) enum RegistrationOutcome {
    Succeeded(RegisterFlow<RegistrationSucceeded>),
    EmailTaken(RegisterFlow<EmailTaken>),
    InvalidInput(RegisterFlow<InvalidInput>),
}

impl RegistrationOutcome {
    pub(super) async fn into_response(
        self,
        auth_session: &mut crate::auth::Session,
    ) -> crate::Result<axum::response::Response> {
        match self {
            Self::Succeeded(succeeded) => {
                let authenticated = succeeded.authenticate(auth_session).await?;
                auth_session.login(authenticated.user()).await?;
                let target = authenticated
                    .into_next()
                    .unwrap_or_else(|| Route::Protected.as_str().to_string());
                Ok(axum::response::Redirect::to(&target).into_response())
            }
            Self::EmailTaken(duplicate) => {
                let next = duplicate.into_next();
                Ok(crate::views::render(
                    crate::views::pages::Register::builder()
                        .message("Email already in use.")
                        .maybe_next(next.as_deref())
                        .build(),
                )
                .into_response())
            }
            Self::InvalidInput(invalid) => {
                let next = invalid.into_next();
                Ok(crate::views::render(
                    crate::views::pages::Register::builder()
                        .message("Invalid input.")
                        .maybe_next(next.as_deref())
                        .build(),
                )
                .into_response())
            }
        }
    }
}

pub(super) type IncomingFlow = RegisterFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Text;

    fn register_form() -> RegisterForm {
        RegisterForm::builder()
            .username(Text::from("person"))
            .email(Text::from("person@example.com"))
            .password(Text::from("pw"))
            .next(Text::from("/protected"))
            .build()
    }

    #[test]
    fn registration_result_routes_email_taken_branch() {
        let incoming =
            RegisterFlow::<Incoming>::from_form(register_form()).expect("incoming");

        let outcome = incoming
            .apply_registration_result(Err(app::user::Error::EmailTaken))
            .expect("classified");
        assert!(matches!(outcome, RegistrationOutcome::EmailTaken(_)));
    }

    #[test]
    fn registration_result_routes_invalid_input_branch() {
        let incoming =
            RegisterFlow::<Incoming>::from_form(register_form()).expect("incoming");

        let outcome = incoming
            .apply_registration_result(Err(app::user::Error::Domain {
                source: domain::user::Error::Email {
                    source: domain::user::Email::try_new("not-an-email")
                        .expect_err("invalid email"),
                },
            }))
            .expect("classified");
        assert!(matches!(outcome, RegistrationOutcome::InvalidInput(_)));
    }

    #[test]
    fn from_form_drops_unsafe_next() {
        let form = RegisterForm::builder()
            .username(Text::from("person"))
            .email(Text::from("person@example.com"))
            .password(Text::from("pw"))
            .next(Text::from("//evil.example"))
            .build();

        let incoming = RegisterFlow::<Incoming>::from_form(form).expect("incoming");

        assert!(incoming.into_next().is_none());
    }
}
