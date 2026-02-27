mod error;

use std::sync::Arc;

use async_trait::async_trait;
use bon::Builder;
use secrecy::{ExposeSecret, SecretString};

use domain::user;
pub use error::{Error, Result};

#[derive(Clone, Debug, Builder)]
pub struct RegisterUser {
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
        password_hash: &crate::auth::PasswordHash,
    ) -> Result<()>;
}

#[derive(Clone)]
pub struct Service {
    users: Arc<dyn Repository>,
    hasher: Arc<dyn crate::auth::PasswordHasher>,
}

impl Service {
    pub fn new(
        users: Arc<dyn Repository>,
        hasher: Arc<dyn crate::auth::PasswordHasher>,
    ) -> Self {
        Self { users, hasher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn register_user(&self, command: RegisterUser) -> Result<user::Id> {
        if self.users.find_by_email(&command.email).await?.is_some() {
            return Err(Error::EmailTaken);
        }

        let new_user = user::User {
            id: user::Id::new_v4(),
            username: command.username,
            email: command.email,
        };

        let password_hash = self
            .hasher
            .hash(command.password.expose_secret())
            .map_err(Error::Hashing)?;

        self.users
            .create_with_credentials(&new_user, &password_hash)
            .await?;

        Ok(new_user.id)
    }

    pub async fn find_by_email(&self, email: user::Email) -> Result<Option<user::User>> {
        self.users.find_by_email(&email).await
    }
}

pub fn validate_input(
    username: &str,
    email: &str,
) -> Result<(user::Username, user::Email)> {
    let username = user::Username::try_new(username).map_err(domain::user::Error::from)?;
    let email = user::Email::try_new(email).map_err(domain::user::Error::from)?;
    Ok((username, email))
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
            _password_hash: &crate::auth::PasswordHash,
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

    impl crate::auth::PasswordHasher for TestHasher {
        fn hash(&self, _password: &str) -> crate::auth::Result<crate::auth::PasswordHash> {
            match self.hash_outcome {
                HashOutcome::Ok => Ok(crate::auth::PasswordHash::new("hash")),
                HashOutcome::Fail => Err(crate::auth::Error::Hash(
                    crate::auth::HashErrorText::new("hash failed"),
                )),
            }
        }

        fn verify(
            &self,
            _password: &str,
            _password_hash: &crate::auth::PasswordHash,
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
                RegisterUser::builder()
                    .username(valid_username())
                    .email(valid_email())
                    .password(SecretString::new("password".to_string().into()))
                    .build(),
            )
            .await;

        assert!(matches!(result, Err(Error::EmailTaken)));
    }

    #[test]
    fn validate_input_rejects_invalid_email() {
        assert!(validate_input("person", "not-an-email").is_err());
    }

    #[tokio::test]
    async fn register_user_succeeds_on_valid_unique_input() {
        let service = service(None, CreateOutcome::Ok, HashOutcome::Ok);
        let result = service
            .register_user(
                RegisterUser::builder()
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
                RegisterUser::builder()
                    .username(valid_username())
                    .email(valid_email())
                    .password(SecretString::new("password".to_string().into()))
                    .build(),
            )
            .await;

        assert!(matches!(
            result,
            Err(Error::Hashing(crate::auth::Error::Hash(_)))
        ));
    }
}
