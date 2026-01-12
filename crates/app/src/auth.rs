use std::sync::Arc;

use async_trait::async_trait;
use secrecy::SecretString;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Provider,
}

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: SecretString,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub session_hash: String,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn authenticate(
        &self,
        credentials: Credentials,
    ) -> Result<Option<AuthenticatedUser>>;
    async fn get_user(
        &self,
        user_id: &str,
    ) -> Result<Option<AuthenticatedUser>>;
}

#[derive(Clone)]
pub struct Service {
    provider: Arc<dyn Provider>,
}

impl Service {
    pub fn new(provider: Arc<dyn Provider>) -> Self {
        Self { provider }
    }

    pub fn disabled() -> Self {
        Self {
            provider: Arc::new(DisabledProvider),
        }
    }

    pub async fn authenticate(
        &self,
        credentials: Credentials,
    ) -> Result<Option<AuthenticatedUser>> {
        self.provider.authenticate(credentials).await
    }

    pub async fn get_user(
        &self,
        user_id: &str,
    ) -> Result<Option<AuthenticatedUser>> {
        self.provider.get_user(user_id).await
    }
}

struct DisabledProvider;

#[async_trait]
impl Provider for DisabledProvider {
    async fn authenticate(
        &self,
        _credentials: Credentials,
    ) -> Result<Option<AuthenticatedUser>> {
        Ok(None)
    }

    async fn get_user(
        &self,
        _user_id: &str,
    ) -> Result<Option<AuthenticatedUser>> {
        Ok(None)
    }
}
