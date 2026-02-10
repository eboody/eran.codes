use std::sync::Arc;

use async_trait::async_trait;
use bon::Builder;
use nutype::nutype;
use secrecy::{ExposeSecret, SecretString};

use domain::user;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Repository(RepositoryErrorText),
    Hash(HashErrorText),
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

#[derive(Clone, Debug, Builder)]
pub struct Credentials {
    pub email: user::Email,
    pub password: SecretString,
}

#[derive(Clone, Debug, Builder)]
pub struct AuthenticatedUser {
    pub id: user::Id,
    pub username: user::Username,
    pub email: user::Email,
    pub session_hash: SessionHash,
}

#[derive(Clone, Debug, Builder)]
pub struct AuthRecord {
    pub id: user::Id,
    pub username: user::Username,
    pub email: user::Email,
    pub password_hash: PasswordHash,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn authenticate(
        &self,
        credentials: Credentials,
    ) -> Result<Option<AuthenticatedUser>>;
    async fn get_user(
        &self,
        user_id: &user::Id,
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
        user_id: &user::Id,
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
        _user_id: &user::Id,
    ) -> Result<Option<AuthenticatedUser>> {
        Ok(None)
    }
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_email(
        &self,
        email: &user::Email,
    ) -> Result<Option<AuthRecord>>;
    async fn find_by_id(
        &self,
        user_id: &user::Id,
    ) -> Result<Option<AuthRecord>>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<PasswordHash>;
    fn verify(
        &self,
        password: &str,
        password_hash: &PasswordHash,
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

        Ok(Some(
            AuthenticatedUser::builder()
                .id(record.id)
                .username(record.username)
                .email(record.email)
                .session_hash(SessionHash::from_password_hash(
                    &record.password_hash,
                ))
                .build(),
        ))
    }

    async fn get_user(
        &self,
        user_id: &user::Id,
    ) -> Result<Option<AuthenticatedUser>> {
        let record = match self.repo.find_by_id(user_id).await? {
            Some(record) => record,
            None => return Ok(None),
        };

        Ok(Some(
            AuthenticatedUser::builder()
                .id(record.id)
                .username(record.username)
                .email(record.email)
                .session_hash(SessionHash::from_password_hash(
                    &record.password_hash,
                ))
                .build(),
        ))
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct RepositoryErrorText(String);

impl From<String> for RepositoryErrorText {
    fn from(value: String) -> Self {
        RepositoryErrorText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct HashErrorText(String);

impl From<String> for HashErrorText {
    fn from(value: String) -> Self {
        HashErrorText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct PasswordHash(String);

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct SessionHash(String);

impl SessionHash {
    pub fn from_password_hash(value: &PasswordHash) -> Self {
        SessionHash::new(value.to_string())
            
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
            record: Some(
                AuthRecord::builder()
                    .id("user-1".to_string())
                    .username("user".to_string())
                    .email("user@example.com".to_string())
                    .password_hash("hash".to_string())
                    .build(),
            ),
        });
        let hasher = Arc::new(TestHasher { ok: true });
        let provider = ProviderImpl::new(repo, hasher);

        let user = provider
            .authenticate(
                Credentials::builder()
                    .email("user@example.com".to_string())
                    .password(SecretString::new("pw".into()))
                    .build(),
            )
            .await
            .unwrap();

        assert!(user.is_some());
    }

    #[tokio::test]
    async fn authenticate_returns_none_on_invalid_password() {
        let repo = Arc::new(TestRepo {
            record: Some(
                AuthRecord::builder()
                    .id("user-1".to_string())
                    .username("user".to_string())
                    .email("user@example.com".to_string())
                    .password_hash("hash".to_string())
                    .build(),
            ),
        });
        let hasher = Arc::new(TestHasher { ok: false });
        let provider = ProviderImpl::new(repo, hasher);

        let user = provider
            .authenticate(
                Credentials::builder()
                    .email("user@example.com".to_string())
                    .password(SecretString::new("pw".into()))
                    .build(),
            )
            .await
            .unwrap();

        assert!(user.is_none());
    }
}
