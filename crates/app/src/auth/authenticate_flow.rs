use secrecy::{ExposeSecret, SecretString};
use statum::{machine, state, transition};

use super::{
    AuthRecord, AuthenticatedUser, Credentials, PasswordHash, PasswordHasher, Result,
};
use domain::user;

#[derive(Clone, Debug)]
pub struct RecordData {
    record: AuthRecord,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedData {
    user: AuthenticatedUser,
}

#[state]
pub enum AuthenticateState {
    Incoming,
    RecordMissing,
    RecordFound(RecordData),
    PasswordRejected,
    PasswordVerified(RecordData),
    Authenticated(AuthenticatedData),
}

#[machine]
pub(super) struct AuthenticateFlow<AuthenticateState> {
    credentials: Credentials,
}

impl AuthenticateFlow<Incoming> {
    pub(super) fn from_credentials(credentials: Credentials) -> Self {
        AuthenticateFlow::<Incoming>::builder()
            .credentials(credentials)
            .build()
    }

    pub(super) fn classify_lookup(self, record: Option<AuthRecord>) -> LookupOutcome {
        match record {
            Some(record) => LookupOutcome::Found(self.mark_record_found(record)),
            None => {
                let _ = self.mark_record_missing();
                LookupOutcome::Missing
            }
        }
    }
}

impl<S: AuthenticateStateTrait> AuthenticateFlow<S> {
    pub(super) fn email(&self) -> &user::Email {
        &self.credentials.email
    }

    pub(super) fn password(&self) -> &SecretString {
        &self.credentials.password
    }
}

#[transition]
impl AuthenticateFlow<Incoming> {
    fn mark_record_found(self, record: AuthRecord) -> AuthenticateFlow<RecordFound> {
        self.transition_with(RecordData { record })
    }

    fn mark_record_missing(self) -> AuthenticateFlow<RecordMissing> {
        self.transition()
    }
}

impl AuthenticateFlow<RecordFound> {
    pub(super) fn password_hash(&self) -> &PasswordHash {
        &self.state_data.record.password_hash
    }

    pub(super) fn verify_password(
        self,
        hasher: &dyn PasswordHasher,
    ) -> Result<PasswordCheckOutcome> {
        let verified =
            hasher.verify(self.password().expose_secret(), self.password_hash())?;
        Ok(self.apply_password_check(verified))
    }

    fn apply_password_check(self, verified: bool) -> PasswordCheckOutcome {
        if verified {
            PasswordCheckOutcome::Verified(self.mark_password_verified())
        } else {
            let _ = self.mark_password_rejected();
            PasswordCheckOutcome::Rejected
        }
    }
}

#[transition]
impl AuthenticateFlow<RecordFound> {
    fn mark_password_verified(self) -> AuthenticateFlow<PasswordVerified> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }

    fn mark_password_rejected(self) -> AuthenticateFlow<PasswordRejected> {
        self.transition()
    }
}

#[transition]
impl AuthenticateFlow<PasswordVerified> {
    pub(super) fn mark_authenticated(self) -> AuthenticateFlow<Authenticated> {
        let record = self.state_data.record.clone();
        let user = AuthenticatedUser::builder()
            .id(record.id)
            .username(record.username)
            .email(record.email)
            .session_hash(record.session_hash)
            .build();
        self.transition_with(AuthenticatedData { user })
    }
}

impl AuthenticateFlow<Authenticated> {
    pub(super) fn into_user(self) -> AuthenticatedUser {
        self.state_data.user
    }
}

pub(super) enum LookupOutcome {
    Found(AuthenticateFlow<RecordFound>),
    Missing,
}

pub(super) enum PasswordCheckOutcome {
    Verified(AuthenticateFlow<PasswordVerified>),
    Rejected,
}

impl LookupOutcome {
    pub(super) fn authenticate(
        self,
        hasher: &dyn PasswordHasher,
    ) -> Result<Option<AuthenticatedUser>> {
        match self {
            Self::Found(found) => Ok(found.verify_password(hasher)?.into_user_option()),
            Self::Missing => Ok(None),
        }
    }
}

impl PasswordCheckOutcome {
    fn into_user_option(self) -> Option<AuthenticatedUser> {
        match self {
            Self::Verified(verified) => Some(verified.mark_authenticated().into_user()),
            Self::Rejected => None,
        }
    }
}

pub(super) type IncomingFlow = AuthenticateFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_email() -> user::Email {
        user::Email::try_new("person@example.com").expect("valid email")
    }

    fn test_username() -> user::Username {
        user::Username::try_new("person").expect("valid username")
    }

    fn test_record() -> AuthRecord {
        AuthRecord::builder()
            .id(user::Id::new_v4())
            .username(test_username())
            .email(test_email())
            .password_hash(PasswordHash::new("hash"))
            .session_hash(super::super::SessionHash::new("session"))
            .build()
    }

    fn credentials() -> Credentials {
        Credentials::builder()
            .email(test_email())
            .password(SecretString::new("pw".into()))
            .build()
    }

    struct TestHasher {
        ok: bool,
    }

    impl PasswordHasher for TestHasher {
        fn hash(&self, _password: &str) -> Result<PasswordHash> {
            Ok(PasswordHash::new("unused"))
        }

        fn verify(&self, _password: &str, _password_hash: &PasswordHash) -> Result<bool> {
            Ok(self.ok)
        }
    }

    #[test]
    fn classify_lookup_returns_missing_branch() {
        let incoming = AuthenticateFlow::<Incoming>::from_credentials(credentials());
        let outcome = incoming.classify_lookup(None);
        assert!(matches!(outcome, LookupOutcome::Missing));
    }

    #[test]
    fn verified_password_path_materializes_authenticated_user() {
        let incoming = AuthenticateFlow::<Incoming>::from_credentials(credentials());
        let found = match incoming.classify_lookup(Some(test_record())) {
            LookupOutcome::Found(found) => found,
            LookupOutcome::Missing => panic!("expected found branch"),
        };

        let checked = found
            .verify_password(&TestHasher { ok: true })
            .expect("password check");
        let verified = match checked {
            PasswordCheckOutcome::Verified(verified) => verified,
            PasswordCheckOutcome::Rejected => panic!("expected verified branch"),
        };
        let user = verified.mark_authenticated().into_user();

        assert_eq!(user.email, test_email());
    }

    #[test]
    fn rejected_password_path_returns_rejected_branch() {
        let incoming = AuthenticateFlow::<Incoming>::from_credentials(credentials());
        let found = match incoming.classify_lookup(Some(test_record())) {
            LookupOutcome::Found(found) => found,
            LookupOutcome::Missing => panic!("expected found branch"),
        };
        let checked = found
            .verify_password(&TestHasher { ok: false })
            .expect("password check");

        assert!(matches!(checked, PasswordCheckOutcome::Rejected));
    }
}
