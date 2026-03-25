use secrecy::{ExposeSecret, SecretString};
use statum::{machine, state, transition};

use super::{Register, failure};
use domain::user;

#[derive(Clone, Debug, PartialEq)]
pub struct UserMaterializedData {
    user: user::User,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PasswordHashedData {
    user: user::User,
    password_hash: crate::auth::password::Hash,
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

    pub(super) async fn register(
        self,
        service: &super::Service,
    ) -> Result<RegisterUserFlow<Persisted>, failure::Error> {
        let email_available = self.verify_email_availability(service).await?;
        let materialized = email_available.materialize_user(user::Id::new_v4());
        let hashed = materialized.hash_password(service.hasher.as_ref())?;

        hashed.persist(service.users.as_ref()).await
    }

    async fn verify_email_availability(
        self,
        service: &super::Service,
    ) -> Result<RegisterUserFlow<EmailAvailable>, failure::Error> {
        let existing_user = service.users.find_by_email(self.email()).await?;
        self.classify_email_availability(existing_user)
            .require_available()
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
        password_hash: crate::auth::password::Hash,
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
    pub(super) fn from_command(command: Register) -> Self {
        let Register {
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

    pub(super) fn password_hash(&self) -> &crate::auth::password::Hash {
        &self.state_data.password_hash
    }

    async fn persist(
        self,
        repo: &dyn super::Repository,
    ) -> Result<RegisterUserFlow<Persisted>, failure::Error> {
        repo.create_with_credentials(self.user(), self.password_hash())
            .await?;
        Ok(self.mark_persisted())
    }
}

impl RegisterUserFlow<Persisted> {
    pub(super) fn user_id(&self) -> user::Id {
        self.state_data.user_id
    }
}

impl RegisterUserFlow<UserMaterialized> {
    fn hash_password(
        self,
        hasher: &dyn crate::auth::password::Hasher,
    ) -> Result<RegisterUserFlow<PasswordHashed>, failure::Error> {
        let password_hash = hasher
            .hash(self.password().expose_secret())
            .map_err(failure::Error::hash_password)?;
        Ok(self.attach_password_hash(password_hash))
    }
}

pub(super) enum EmailAvailabilityOutcome {
    Available(RegisterUserFlow<EmailAvailable>),
    Taken(RegisterUserFlow<EmailTaken>),
}

impl EmailAvailabilityOutcome {
    pub(super) fn require_available(
        self,
    ) -> Result<RegisterUserFlow<EmailAvailable>, failure::Error> {
        match self {
            Self::Available(available) => Ok(available),
            Self::Taken(taken) => {
                let _ = taken;
                Err(failure::Error::EmailTaken)
            }
        }
    }
}

pub(super) type IncomingFlow = RegisterUserFlow<Incoming>;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;

    use super::*;

    fn build_command() -> Register {
        Register::builder()
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
        assert!(matches!(result, Err(failure::Error::EmailTaken)));
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
            materialized.attach_password_hash(crate::auth::password::Hash::new("hash"));

        assert_eq!(hashed.user().id, expected_user_id);
        assert_eq!(hashed.password_hash().to_string(), "hash");

        let persisted = hashed.mark_persisted();
        assert_eq!(persisted.user_id(), expected_user_id);
    }

    struct TestRepository {
        existing_user: Option<user::User>,
        created_users: Mutex<Vec<user::Id>>,
        create_outcome: CreateOutcome,
    }

    impl TestRepository {
        fn created_users(&self) -> Vec<user::Id> {
            self.created_users
                .lock()
                .expect("created_users lock")
                .clone()
        }
    }

    #[async_trait]
    impl super::super::Repository for TestRepository {
        async fn find_by_email(
            &self,
            _email: &user::Email,
        ) -> super::super::Result<Option<user::User>> {
            Ok(self.existing_user.clone())
        }

        async fn create_with_credentials(
            &self,
            user: &user::User,
            _password_hash: &crate::auth::password::Hash,
        ) -> super::super::Result<()> {
            self.created_users
                .lock()
                .expect("created_users lock")
                .push(user.id);
            match self.create_outcome {
                CreateOutcome::Ok => Ok(()),
                CreateOutcome::EmailTaken => Err(super::super::failure::Error::EmailTaken),
            }
        }
    }

    #[derive(Clone, Copy)]
    enum CreateOutcome {
        Ok,
        EmailTaken,
    }

    struct TestHasher {
        hash_outcome: HashOutcome,
    }

    impl crate::auth::password::Hasher for TestHasher {
        fn hash(
            &self,
            _password: &str,
        ) -> crate::auth::Result<crate::auth::password::Hash> {
            match self.hash_outcome {
                HashOutcome::Ok => Ok(crate::auth::password::Hash::new("hash")),
                HashOutcome::Fail => Err(crate::auth::Error::hash_password(
                    std::io::Error::other("hash failed"),
                )),
            }
        }

        fn verify(
            &self,
            _password: &str,
            _password_hash: &crate::auth::password::Hash,
        ) -> crate::auth::Result<bool> {
            unimplemented!("not used in this test")
        }
    }

    #[derive(Clone, Copy)]
    enum HashOutcome {
        Ok,
        Fail,
    }

    fn test_service(
        repo: Arc<TestRepository>,
        hasher: Arc<TestHasher>,
    ) -> super::super::Service {
        super::super::Service::new(repo, hasher)
    }

    #[tokio::test]
    async fn register_runs_full_path_to_persisted_user() {
        let repo = Arc::new(TestRepository {
            existing_user: None,
            created_users: Mutex::new(Vec::new()),
            create_outcome: CreateOutcome::Ok,
        });
        let hasher = Arc::new(TestHasher {
            hash_outcome: HashOutcome::Ok,
        });
        let service = test_service(repo.clone(), hasher);

        let persisted = RegisterUserFlow::<Incoming>::from_command(build_command())
            .register(&service)
            .await
            .expect("persisted");

        assert_eq!(repo.created_users(), vec![persisted.user_id()]);
    }

    #[tokio::test]
    async fn register_surfaces_persistence_error_after_hash() {
        let repo = Arc::new(TestRepository {
            existing_user: None,
            created_users: Mutex::new(Vec::new()),
            create_outcome: CreateOutcome::EmailTaken,
        });
        let hasher = Arc::new(TestHasher {
            hash_outcome: HashOutcome::Ok,
        });
        let service = test_service(repo.clone(), hasher);

        let result = RegisterUserFlow::<Incoming>::from_command(build_command())
            .register(&service)
            .await;

        assert!(matches!(result, Err(failure::Error::EmailTaken)));
        assert_eq!(repo.created_users().len(), 1);
    }

    #[tokio::test]
    async fn register_surfaces_hashing_error_before_persist() {
        let repo = Arc::new(TestRepository {
            existing_user: None,
            created_users: Mutex::new(Vec::new()),
            create_outcome: CreateOutcome::Ok,
        });
        let hasher = Arc::new(TestHasher {
            hash_outcome: HashOutcome::Fail,
        });
        let service = test_service(repo.clone(), hasher);

        let result = RegisterUserFlow::<Incoming>::from_command(build_command())
            .register(&service)
            .await;

        assert!(matches!(
            result,
            Err(failure::Error::HashPassword {
                source: crate::auth::Error::HashPassword { .. },
            })
        ));
        assert!(repo.created_users().is_empty());
    }
}
