mod error;
mod input;
mod register_user_flow;

use std::sync::Arc;

use async_trait::async_trait;
use bon::Builder;
use secrecy::SecretString;

use domain::user;
pub use error::{Error, RepositoryOperation, Result};
pub use input::Input;

#[derive(Clone, Debug, Builder)]
pub struct Register {
    pub username: user::Username,
    pub email: user::Email,
    pub password: SecretString,
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_email(&self, email: &user::Email) -> Result<Option<user::User>>;
    async fn create_with_credentials(
        &self,
        user: &user::User,
        password_hash: &crate::auth::password::Hash,
    ) -> Result<()>;
}

#[derive(Clone)]
pub struct Service {
    users: Arc<dyn Repository>,
    hasher: Arc<dyn crate::auth::password::Hasher>,
}

impl Service {
    pub fn new(
        users: Arc<dyn Repository>,
        hasher: Arc<dyn crate::auth::password::Hasher>,
    ) -> Self {
        Self { users, hasher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn register_user(&self, command: Register) -> Result<user::Id> {
        let persisted = register_user_flow::IncomingFlow::from_command(command)
            .register(self)
            .await?;
        Ok(persisted.user_id())
    }

    pub async fn find_by_email(&self, email: user::Email) -> Result<Option<user::User>> {
        self.users.find_by_email(&email).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    enum CreateOutcome {
        Ok,
        EmailTaken,
    }

    #[derive(Clone, Copy)]
    enum HashOutcome {
        Ok,
        Fail,
    }

    struct TestRepository {
        existing_user: Option<user::User>,
        create_outcome: CreateOutcome,
    }

    #[async_trait]
    impl Repository for TestRepository {
        async fn find_by_email(&self, _email: &user::Email) -> Result<Option<user::User>> {
            Ok(self.existing_user.clone())
        }

        async fn create_with_credentials(
            &self,
            _user: &user::User,
            _password_hash: &crate::auth::password::Hash,
        ) -> Result<()> {
            match self.create_outcome {
                CreateOutcome::Ok => Ok(()),
                CreateOutcome::EmailTaken => Err(Error::EmailTaken),
            }
        }
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
            Ok(true)
        }
    }

    fn valid_email() -> user::Email {
        user::Email::try_new("person@example.com").expect("valid email")
    }

    fn valid_username() -> user::Username {
        user::Username::try_new("person").expect("valid username")
    }

    fn service(
        existing_user: Option<user::User>,
        create_outcome: CreateOutcome,
        hash_outcome: HashOutcome,
    ) -> Service {
        Service::new(
            std::sync::Arc::new(TestRepository {
                existing_user,
                create_outcome,
            }),
            std::sync::Arc::new(TestHasher { hash_outcome }),
        )
    }

    #[tokio::test]
    async fn register_user_surfaces_email_taken_from_repository() {
        let service = service(None, CreateOutcome::EmailTaken, HashOutcome::Ok);
        let result = service
            .register_user(
                Register::builder()
                    .username(valid_username())
                    .email(valid_email())
                    .password(SecretString::new("password".to_string().into()))
                    .build(),
            )
            .await;

        assert!(matches!(result, Err(Error::EmailTaken)));
    }

    #[test]
    fn input_parse_rejects_invalid_email() {
        assert!(Input::parse("person", "not-an-email").is_err());
    }

    #[tokio::test]
    async fn register_user_succeeds_on_valid_unique_input() {
        let service = service(None, CreateOutcome::Ok, HashOutcome::Ok);
        let result = service
            .register_user(
                Register::builder()
                    .username(valid_username())
                    .email(valid_email())
                    .password(SecretString::new("password".to_string().into()))
                    .build(),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn register_user_surfaces_hashing_error() {
        let service = service(None, CreateOutcome::Ok, HashOutcome::Fail);
        let result = service
            .register_user(
                Register::builder()
                    .username(valid_username())
                    .email(valid_email())
                    .password(SecretString::new("password".to_string().into()))
                    .build(),
            )
            .await;

        assert!(matches!(
            result,
            Err(Error::HashPassword {
                source: crate::auth::Error::HashPassword { .. },
            })
        ));
    }
}
