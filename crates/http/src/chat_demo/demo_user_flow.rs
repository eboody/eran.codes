use secrecy::SecretString;
use statum::{machine, state, transition};

#[derive(Clone, Debug)]
pub struct DemoIdentityData {
    email: domain::user::Email,
    username: domain::user::Username,
}

#[derive(Clone, Debug)]
pub struct DemoUserData {
    user: domain::user::User,
}

#[state]
pub enum DemoUserEnsureState {
    Incoming,
    IdentityPrepared(DemoIdentityData),
    ExistingFound(DemoUserData),
    Missing(DemoIdentityData),
    RegistrationAccepted(DemoIdentityData),
    Resolved(DemoUserData),
}

#[machine]
pub(super) struct DemoUserEnsureFlow<DemoUserEnsureState> {}

impl DemoUserEnsureFlow<Incoming> {
    pub(super) fn new() -> Self {
        DemoUserEnsureFlow::<Incoming>::builder().build()
    }

    pub(super) async fn ensure(
        self,
        state: &crate::State,
        email_literal: &str,
        username_literal: &str,
    ) -> crate::Result<domain::user::User> {
        let identity = self.prepare_identity(email_literal, username_literal)?;
        let existing = state.user.find_by_email(identity.email().clone()).await?;

        identity
            .classify_existing(existing)
            .resolve_user(state)
            .await
    }
}

#[transition]
impl DemoUserEnsureFlow<Incoming> {
    fn mark_identity_prepared(
        self,
        data: DemoIdentityData,
    ) -> DemoUserEnsureFlow<IdentityPrepared> {
        self.transition_with(data)
    }
}

impl DemoUserEnsureFlow<Incoming> {
    pub(super) fn prepare_identity(
        self,
        email_literal: &str,
        username_literal: &str,
    ) -> crate::Result<DemoUserEnsureFlow<IdentityPrepared>> {
        let email = domain::user::Email::try_new(email_literal).map_err(|error| {
            tracing::error!(%error, email_literal, "demo user email literal is invalid");
            crate::Error::Internal
        })?;
        let username = domain::user::Username::try_new(username_literal).map_err(|error| {
            tracing::error!(%error, username_literal, "demo user username literal is invalid");
            crate::Error::Internal
        })?;
        Ok(self.mark_identity_prepared(DemoIdentityData { email, username }))
    }
}

#[transition]
impl DemoUserEnsureFlow<IdentityPrepared> {
    fn mark_existing(self, user: domain::user::User) -> DemoUserEnsureFlow<ExistingFound> {
        self.transition_with(DemoUserData { user })
    }

    fn mark_missing(self) -> DemoUserEnsureFlow<Missing> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl DemoUserEnsureFlow<IdentityPrepared> {
    pub(super) fn email(&self) -> &domain::user::Email {
        &self.state_data.email
    }

    pub(super) fn classify_existing(
        self,
        existing: Option<domain::user::User>,
    ) -> DemoUserLookupOutcome {
        match existing {
            Some(user) => DemoUserLookupOutcome::Found(self.mark_existing(user)),
            None => DemoUserLookupOutcome::Missing(self.mark_missing()),
        }
    }
}

#[transition]
impl DemoUserEnsureFlow<Missing> {
    fn accept_registration(self) -> DemoUserEnsureFlow<RegistrationAccepted> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl DemoUserEnsureFlow<Missing> {
    pub(super) fn registration_command(
        &self,
        password: SecretString,
    ) -> app::user::Register {
        app::user::Register::builder()
            .username(self.state_data.username.clone())
            .email(self.state_data.email.clone())
            .password(password)
            .build()
    }

    pub(super) fn apply_registration_result(
        self,
        result: app::user::Result<domain::user::Id>,
    ) -> crate::Result<DemoUserEnsureFlow<RegistrationAccepted>> {
        match result {
            Ok(_) | Err(app::user::failure::Error::EmailTaken) => {
                Ok(self.accept_registration())
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn register_and_resolve(
        self,
        state: &crate::State,
    ) -> crate::Result<domain::user::User> {
        let password = secrecy::SecretString::new(uuid::Uuid::new_v4().to_string().into());
        let registration = state
            .user
            .register_user(self.registration_command(password))
            .await;
        let accepted = self.apply_registration_result(registration)?;
        let demo_email = accepted.email().clone();
        let resolved =
            accepted.resolve_lookup(state.user.find_by_email(demo_email).await?)?;
        Ok(resolved.into_user())
    }
}

#[transition]
impl DemoUserEnsureFlow<RegistrationAccepted> {
    fn mark_resolved(self, user: domain::user::User) -> DemoUserEnsureFlow<Resolved> {
        self.transition_with(DemoUserData { user })
    }
}

impl DemoUserEnsureFlow<RegistrationAccepted> {
    pub(super) fn email(&self) -> &domain::user::Email {
        &self.state_data.email
    }

    pub(super) fn resolve_lookup(
        self,
        user: Option<domain::user::User>,
    ) -> crate::Result<DemoUserEnsureFlow<Resolved>> {
        match user {
            Some(user) => Ok(self.mark_resolved(user)),
            None => Err(crate::Error::Internal),
        }
    }
}

impl DemoUserEnsureFlow<ExistingFound> {
    pub(super) fn into_user(self) -> domain::user::User {
        self.state_data.user
    }
}

impl DemoUserEnsureFlow<Resolved> {
    pub(super) fn into_user(self) -> domain::user::User {
        self.state_data.user
    }
}

pub(super) enum DemoUserLookupOutcome {
    Found(DemoUserEnsureFlow<ExistingFound>),
    Missing(DemoUserEnsureFlow<Missing>),
}

impl DemoUserLookupOutcome {
    pub(super) async fn resolve_user(
        self,
        state: &crate::State,
    ) -> crate::Result<domain::user::User> {
        match self {
            Self::Found(found) => Ok(found.into_user()),
            Self::Missing(missing) => missing.register_and_resolve(state).await,
        }
    }
}

pub(super) type IncomingFlow = DemoUserEnsureFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_identity_rejects_invalid_email() {
        let incoming = IncomingFlow::new();
        let result = incoming.prepare_identity("not-an-email", "Demo Bot");
        assert!(matches!(result, Err(crate::Error::Internal)));
    }
}
