use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Repository(String),
    Hash(String),
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

#[derive(Clone, Debug)]
pub struct AuthRecord {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
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

#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<AuthRecord>>;
    async fn find_by_id(
        &self,
        user_id: &str,
    ) -> Result<Option<AuthRecord>>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String>;
    fn verify(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool>;
}

#[derive(Clone)]
pub struct ProviderImpl {
    repo: Arc<dyn Repository>,
    hasher: Arc<dyn PasswordHasher>,
}

impl ProviderImpl {
    pub fn new(
        repo: Arc<dyn Repository>,
        hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self { repo, hasher }
    }
}

#[async_trait]
impl Provider for ProviderImpl {
    async fn authenticate(
        &self,
        credentials: Credentials,
    ) -> Result<Option<AuthenticatedUser>> {
        let record = match self.repo.find_by_email(&credentials.email).await? {
            Some(record) => record,
            None => return Ok(None),
        };

        let verified = self.hasher.verify(
            credentials.password.expose_secret(),
            &record.password_hash,
        )?;

        if !verified {
            return Ok(None);
        }

        Ok(Some(AuthenticatedUser {
            id: record.id,
            username: record.username,
            email: record.email,
            session_hash: record.password_hash,
        }))
    }

    async fn get_user(
        &self,
        user_id: &str,
    ) -> Result<Option<AuthenticatedUser>> {
        let record = match self.repo.find_by_id(user_id).await? {
            Some(record) => record,
            None => return Ok(None),
        };

        Ok(Some(AuthenticatedUser {
            id: record.id,
            username: record.username,
            email: record.email,
            session_hash: record.password_hash,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRepo {
        record: Option<AuthRecord>,
    }

    #[async_trait]
    impl Repository for TestRepo {
        async fn find_by_email(
            &self,
            _email: &str,
        ) -> Result<Option<AuthRecord>> {
            Ok(self.record.clone())
        }

        async fn find_by_id(
            &self,
            _user_id: &str,
        ) -> Result<Option<AuthRecord>> {
            Ok(self.record.clone())
        }
    }

    struct TestHasher {
        ok: bool,
    }

    impl PasswordHasher for TestHasher {
        fn hash(&self, _password: &str) -> Result<String> {
            Ok("hash".to_string())
        }

        fn verify(
            &self,
            _password: &str,
            _password_hash: &str,
        ) -> Result<bool> {
            Ok(self.ok)
        }
    }

    #[tokio::test]
    async fn authenticate_returns_user_on_valid_password() {
        let repo = Arc::new(TestRepo {
            record: Some(AuthRecord {
                id: "user-1".to_string(),
                username: "user".to_string(),
                email: "user@example.com".to_string(),
                password_hash: "hash".to_string(),
            }),
        });
        let hasher = Arc::new(TestHasher { ok: true });
        let provider = ProviderImpl::new(repo, hasher);

        let user = provider
            .authenticate(Credentials {
                email: "user@example.com".to_string(),
                password: SecretString::new("pw".into()),
            })
            .await
            .unwrap();

        assert!(user.is_some());
    }

    #[tokio::test]
    async fn authenticate_returns_none_on_invalid_password() {
        let repo = Arc::new(TestRepo {
            record: Some(AuthRecord {
                id: "user-1".to_string(),
                username: "user".to_string(),
                email: "user@example.com".to_string(),
                password_hash: "hash".to_string(),
            }),
        });
        let hasher = Arc::new(TestHasher { ok: false });
        let provider = ProviderImpl::new(repo, hasher);

        let user = provider
            .authenticate(Credentials {
                email: "user@example.com".to_string(),
                password: SecretString::new("pw".into()),
            })
            .await
            .unwrap();

        assert!(user.is_none());
    }
}
