use secrecy::SecretString;
use statum::{machine, state, transition};

use super::{Error, RegisterUser};
use domain::user;

#[derive(Clone, Debug, PartialEq)]
pub struct UserMaterializedData {
    user: user::User,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PasswordHashedData {
    user: user::User,
    password_hash: crate::auth::PasswordHash,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PersistedData {
    user_id: user::Id,
}

#[state]
pub enum RegisterUserState {
    Incoming,
    EmailAvailable,
    EmailTaken,
    UserMaterialized(UserMaterializedData),
    PasswordHashed(PasswordHashedData),
    Persisted(PersistedData),
}

#[machine]
pub(super) struct RegisterUserFlow<RegisterUserState> {
    username: user::Username,
    email: user::Email,
    password: SecretString,
}

#[transition]
impl RegisterUserFlow<Incoming> {
    fn mark_email_available(self) -> RegisterUserFlow<EmailAvailable> {
        self.transition()
    }

    fn mark_email_taken(self) -> RegisterUserFlow<EmailTaken> {
        self.transition()
    }
}

impl RegisterUserFlow<Incoming> {
    pub(super) fn classify_email_availability(
        self,
        existing_user: Option<user::User>,
    ) -> EmailAvailabilityOutcome {
        match existing_user {
            Some(_) => EmailAvailabilityOutcome::Taken(self.mark_email_taken()),
            None => EmailAvailabilityOutcome::Available(self.mark_email_available()),
        }
    }
}

#[transition]
impl RegisterUserFlow<EmailAvailable> {
    pub(super) fn materialize_user(
        self,
        user_id: user::Id,
    ) -> RegisterUserFlow<UserMaterialized> {
        let data = UserMaterializedData {
            user: user::User {
                id: user_id,
                username: self.username.clone(),
                email: self.email.clone(),
            },
        };
        self.transition_with(data)
    }
}

#[transition]
impl RegisterUserFlow<UserMaterialized> {
    pub(super) fn attach_password_hash(
        self,
        password_hash: crate::auth::PasswordHash,
    ) -> RegisterUserFlow<PasswordHashed> {
        let data = PasswordHashedData {
            user: self.state_data.user.clone(),
            password_hash,
        };
        self.transition_with(data)
    }
}

#[transition]
impl RegisterUserFlow<PasswordHashed> {
    pub(super) fn mark_persisted(self) -> RegisterUserFlow<Persisted> {
        let data = PersistedData {
            user_id: self.state_data.user.id,
        };
        self.transition_with(data)
    }
}

impl RegisterUserFlow<Incoming> {
    pub(super) fn from_command(command: RegisterUser) -> Self {
        let RegisterUser {
            username,
            email,
            password,
        } = command;

        RegisterUserFlow::<Incoming>::builder()
            .username(username)
            .email(email)
            .password(password)
            .build()
    }
}

impl<S: RegisterUserStateTrait> RegisterUserFlow<S> {
    pub(super) fn email(&self) -> &user::Email {
        &self.email
    }

    pub(super) fn password(&self) -> &SecretString {
        &self.password
    }
}

impl RegisterUserFlow<PasswordHashed> {
    pub(super) fn user(&self) -> &user::User {
        &self.state_data.user
    }

    pub(super) fn password_hash(&self) -> &crate::auth::PasswordHash {
        &self.state_data.password_hash
    }
}

impl RegisterUserFlow<Persisted> {
    pub(super) fn user_id(&self) -> user::Id {
        self.state_data.user_id
    }
}

pub(super) enum EmailAvailabilityOutcome {
    Available(RegisterUserFlow<EmailAvailable>),
    Taken(RegisterUserFlow<EmailTaken>),
}

impl EmailAvailabilityOutcome {
    pub(super) fn require_available(
        self,
    ) -> Result<RegisterUserFlow<EmailAvailable>, Error> {
        match self {
            Self::Available(available) => Ok(available),
            Self::Taken(taken) => {
                let _ = taken;
                Err(Error::EmailTaken)
            }
        }
    }
}

pub(super) type IncomingFlow = RegisterUserFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn build_command() -> RegisterUser {
        RegisterUser::builder()
            .username(user::Username::try_new("person").expect("valid username"))
            .email(user::Email::try_new("person@example.com").expect("valid email"))
            .password(SecretString::new("password".to_string().into()))
            .build()
    }

    fn build_existing_user() -> user::User {
        user::User::builder()
            .id(user::Id::new_v4())
            .username(user::Username::try_new("person").expect("valid username"))
            .email(user::Email::try_new("person@example.com").expect("valid email"))
            .build()
    }

    #[test]
    fn classify_email_availability_rejects_existing_user() {
        let incoming = RegisterUserFlow::<Incoming>::from_command(build_command());
        let result = incoming
            .classify_email_availability(Some(build_existing_user()))
            .require_available();
        assert!(matches!(result, Err(Error::EmailTaken)));
    }

    #[test]
    fn happy_path_materializes_hashes_and_persists() {
        let incoming = RegisterUserFlow::<Incoming>::from_command(build_command());
        let available = incoming
            .classify_email_availability(None)
            .require_available()
            .expect("email available");
        let materialized = available.materialize_user(user::Id::new_v4());
        let expected_user_id = materialized.state_data.user.id;

        let hashed =
            materialized.attach_password_hash(crate::auth::PasswordHash::new("hash"));

        assert_eq!(hashed.user().id, expected_user_id);
        assert_eq!(hashed.password_hash().to_string(), "hash");

        let persisted = hashed.mark_persisted();
        assert_eq!(persisted.user_id(), expected_user_id);
    }
}
