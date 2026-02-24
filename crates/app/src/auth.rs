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
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
    pub session_hash: SessionHash,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn authenticate(
        &self,
        credentials: Credentials,
    ) -> Result<Option<AuthenticatedUser>>;
    async fn get_user(&self, user_id: &user::Id) -> Result<Option<AuthenticatedUser>>;
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

    pub async fn get_user(&self, user_id: &user::Id) -> Result<Option<AuthenticatedUser>> {
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

    async fn get_user(&self, _user_id: &user::Id) -> Result<Option<AuthenticatedUser>> {
        Ok(None)
    }
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_email(&self, email: &user::Email) -> Result<Option<AuthRecord>>;
    async fn find_by_id(&self, user_id: &user::Id) -> Result<Option<AuthRecord>>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<PasswordHash>;
    fn verify(&self, password: &str, password_hash: &PasswordHash) -> Result<bool>;
}

#[derive(Clone)]
pub struct ProviderImpl {
    repo: Arc<dyn Repository>,
    hasher: Arc<dyn PasswordHasher>,
}

impl ProviderImpl {
    pub fn new(repo: Arc<dyn Repository>, hasher: Arc<dyn PasswordHasher>) -> Self {
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

        let verified = self
            .hasher
            .verify(credentials.password.expose_secret(), &record.password_hash)?;

        if !verified {
            return Ok(None);
        }

        Ok(Some(
            AuthenticatedUser::builder()
                .id(record.id)
                .username(record.username)
                .email(record.email)
                .session_hash(record.session_hash)
                .build(),
        ))
    }

    async fn get_user(&self, user_id: &user::Id) -> Result<Option<AuthenticatedUser>> {
        let record = match self.repo.find_by_id(user_id).await? {
            Some(record) => record,
            None => return Ok(None),
        };

        Ok(Some(
            AuthenticatedUser::builder()
                .id(record.id)
                .username(record.username)
                .email(record.email)
                .session_hash(record.session_hash)
                .build(),
        ))
    }
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct RepositoryErrorText(String);

impl From<String> for RepositoryErrorText {
    fn from(value: String) -> Self {
        RepositoryErrorText::new(value)
    }
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct HashErrorText(String);

impl From<String> for HashErrorText {
    fn from(value: String) -> Self {
        HashErrorText::new(value)
    }
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct PasswordHash(String);

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct SessionHash(String);

#[cfg(test)]
mod tests {
    use super::*;

    fn test_email() -> user::Email {
        user::Email::try_new("user@example.com".to_owned()).unwrap()
    }

    fn test_username() -> user::Username {
        user::Username::try_new("user".to_owned()).unwrap()
    }

    fn test_user_id() -> user::Id {
        user::Id::from_uuid(uuid::Uuid::new_v4())
    }

    fn test_password_hash() -> PasswordHash {
        PasswordHash::new("hash")
    }

    fn test_session_hash() -> SessionHash {
        SessionHash::new("session-hash")
    }

    struct TestRepo {
        record: Option<AuthRecord>,
    }

    #[async_trait]
    impl Repository for TestRepo {
        async fn find_by_email(&self, _email: &user::Email) -> Result<Option<AuthRecord>> {
            Ok(self.record.clone())
        }

        async fn find_by_id(&self, _user_id: &user::Id) -> Result<Option<AuthRecord>> {
            Ok(self.record.clone())
        }
    }

    struct TestHasher {
        ok: bool,
    }

    impl PasswordHasher for TestHasher {
        fn hash(&self, _password: &str) -> Result<PasswordHash> {
            Ok(test_password_hash())
        }

        fn verify(&self, _password: &str, _password_hash: &PasswordHash) -> Result<bool> {
            Ok(self.ok)
        }
    }

    #[tokio::test]
    async fn authenticate_returns_user_on_valid_password() {
        let repo = Arc::new(TestRepo {
            record: Some(
                AuthRecord::builder()
                    .id(test_user_id())
                    .username(test_username())
                    .email(test_email())
                    .password_hash(test_password_hash())
                    .session_hash(test_session_hash())
                    .build(),
            ),
        });
        let hasher = Arc::new(TestHasher { ok: true });
        let provider = ProviderImpl::new(repo, hasher);

        let user = provider
            .authenticate(
                Credentials::builder()
                    .email(test_email())
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
                    .id(test_user_id())
                    .username(test_username())
                    .email(test_email())
                    .password_hash(test_password_hash())
                    .session_hash(test_session_hash())
                    .build(),
            ),
        });
        let hasher = Arc::new(TestHasher { ok: false });
        let provider = ProviderImpl::new(repo, hasher);

        let user = provider
            .authenticate(
                Credentials::builder()
                    .email(test_email())
                    .password(SecretString::new("pw".into()))
                    .build(),
            )
            .await
            .unwrap();

        assert!(user.is_none());
    }

    #[tokio::test]
    async fn authenticate_uses_dedicated_session_hash() {
        let session_hash = SessionHash::new("session-version");
        let repo = Arc::new(TestRepo {
            record: Some(
                AuthRecord::builder()
                    .id(test_user_id())
                    .username(test_username())
                    .email(test_email())
                    .password_hash(test_password_hash())
                    .session_hash(session_hash.clone())
                    .build(),
            ),
        });
        let hasher = Arc::new(TestHasher { ok: true });
        let provider = ProviderImpl::new(repo, hasher);

        let user = provider
            .authenticate(
                Credentials::builder()
                    .email(test_email())
                    .password(SecretString::new("pw".into()))
                    .build(),
            )
            .await
            .unwrap()
            .expect("authenticated user");

        assert_eq!(user.session_hash, session_hash);
        assert_ne!(
            user.session_hash.to_string(),
            test_password_hash().to_string()
        );
    }
}
