mod error;

use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};

use domain::user;
pub use error::{Error, Result};

#[derive(Clone, Debug)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: SecretString,
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_email(
        &self,
        email: &user::Email,
    ) -> Result<Option<user::User>>;
    async fn create_with_credentials(
        &self,
        user: &user::User,
        password_hash: &str,
    ) -> Result<()>;
}

#[allow(unused)]
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
    pub async fn register_user(
        &self,
        command: RegisterUser,
    ) -> Result<user::Id> {
        let username = user::Username::try_new(command.username)
            .map_err(domain::user::Error::from)?;
        let email = user::Email::try_new(command.email)
            .map_err(domain::user::Error::from)?;

        if self.users.find_by_email(&email).await?.is_some()
        {
            return Err(Error::EmailTaken);
        }

        let new_user = user::User {
            id: user::Id::new_v4(),
            username,
            email,
        };

        let password_hash = self
            .hasher
            .hash(command.password.expose_secret())
            .map_err(|error| Error::Repo(error.to_string()))?;

        self.users
            .create_with_credentials(&new_user, &password_hash)
            .await?;

        Ok(new_user.id)
    }

    pub async fn find_by_email(
        &self,
        email: String,
    ) -> Result<Option<user::User>> {
        let email =
            user::Email::try_new(email).map_err(domain::user::Error::from)?;
        self.users.find_by_email(&email).await
    }
}

pub fn validate_input(
    username: &str,
    email: &str,
) -> Result<(user::Username, user::Email)> {
    let username =
        user::Username::try_new(username.to_owned())
            .map_err(domain::user::Error::from)?;
    let email =
        user::Email::try_new(email.to_owned())
            .map_err(domain::user::Error::from)?;
    Ok((username, email))
}
