mod authenticate_flow;
mod get_user_flow;
pub mod password;
pub mod repository;

use std::sync::Arc;

use async_trait::async_trait;
use bon::Builder;
use nutype::nutype;
use secrecy::SecretString;
use snafu::prelude::*;

use domain::user;

pub type Result<T> = core::result::Result<T, Error>;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    Repository { source: repository::Error },
    #[snafu(display("auth password hashing failed: {source}"))]
    HashPassword { source: password::HashError },
    #[snafu(display("stored password hash parsing failed: {source}"))]
    ParseStoredPasswordHash { source: password::HashError },
    #[snafu(display("invalid authenticated user id: {source}"))]
    InvalidAuthenticatedUserId { source: uuid::Error },
}

fn box_error(source: impl std::error::Error + Send + Sync + 'static) -> BoxError {
    Box::new(source)
}

impl Error {
    pub fn query_repository(
        operation: repository::Operation,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Repository {
            source: repository::Error::Query {
                operation,
                source: box_error(source),
            },
        }
    }

    pub fn decode_username(source: user::UsernameError) -> Self {
        Self::Repository {
            source: repository::Error::DecodeUsername { source },
        }
    }

    pub fn decode_email(source: user::EmailError) -> Self {
        Self::Repository {
            source: repository::Error::DecodeEmail { source },
        }
    }

    pub fn hash_password(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::HashPassword {
            source: password::HashError(box_error(source)),
        }
    }

    pub fn parse_stored_password_hash(
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::ParseStoredPasswordHash {
            source: password::HashError(box_error(source)),
        }
    }
}

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
pub struct Record {
    pub id: user::Id,
    pub username: user::Username,
    pub email: user::Email,
    pub password_hash: password::Hash,
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
    async fn find_by_email(&self, email: &user::Email) -> Result<Option<Record>>;
    async fn find_by_id(&self, user_id: &user::Id) -> Result<Option<Record>>;
}

#[derive(Clone)]
pub struct ProviderImpl {
    repo: Arc<dyn Repository>,
    hasher: Arc<dyn password::Hasher>,
}

impl ProviderImpl {
    pub fn new(repo: Arc<dyn Repository>, hasher: Arc<dyn password::Hasher>) -> Self {
        Self { repo, hasher }
    }
}

#[async_trait]
impl Provider for ProviderImpl {
    async fn authenticate(
        &self,
        credentials: Credentials,
    ) -> Result<Option<AuthenticatedUser>> {
        let incoming = authenticate_flow::IncomingFlow::from_credentials(credentials);
        let record = self.repo.find_by_email(incoming.email()).await?;
        incoming
            .classify_lookup(record)
            .authenticate(self.hasher.as_ref())
    }

    async fn get_user(&self, user_id: &user::Id) -> Result<Option<AuthenticatedUser>> {
        let incoming = get_user_flow::IncomingFlow::new();
        let record = self.repo.find_by_id(user_id).await?;
        Ok(incoming.classify_lookup(record).into_user_option())
    }
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct SessionHash(String);

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    fn test_email() -> user::Email {
        user::Email::try_new("user@example.com".to_owned()).unwrap()
    }

    fn test_username() -> user::Username {
        user::Username::try_new("user".to_owned()).unwrap()
    }

    #[test]
    fn repository_error_preserves_source() {
        let error = Error::query_repository(
            repository::Operation::FindByEmail,
            std::io::Error::other("db unavailable"),
        );

        assert_eq!(
            error
                .source()
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some(
                "auth repository query failed while find auth record by email: db unavailable"
            ),
        );
        assert_eq!(
            error
                .source()
                .and_then(|source| source.source())
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some("db unavailable"),
        );
    }

    #[test]
    fn hash_password_error_preserves_source() {
        let error = Error::hash_password(std::io::Error::other("hash failed"));

        assert_eq!(
            error
                .source()
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some("hash failed"),
        );
        assert_eq!(
            error.to_string(),
            "auth password hashing failed: hash failed",
        );
    }

    #[test]
    fn parse_stored_password_hash_error_preserves_source() {
        let error =
            Error::parse_stored_password_hash(std::io::Error::other("invalid hash"));

        assert_eq!(
            error
                .source()
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some("invalid hash"),
        );
        assert_eq!(
            error.to_string(),
            "stored password hash parsing failed: invalid hash",
        );
    }

    fn test_user_id() -> user::Id {
        user::Id::from(uuid::Uuid::new_v4())
    }

    fn test_password_hash() -> password::Hash {
        password::Hash::new("hash")
    }

    fn test_session_hash() -> SessionHash {
        SessionHash::new("session-hash")
    }

    struct TestRepo {
        record: Option<Record>,
    }

    #[async_trait]
    impl Repository for TestRepo {
        async fn find_by_email(&self, _email: &user::Email) -> Result<Option<Record>> {
            Ok(self.record.clone())
        }

        async fn find_by_id(&self, _user_id: &user::Id) -> Result<Option<Record>> {
            Ok(self.record.clone())
        }
    }

    struct TestHasher {
        ok: bool,
    }

    impl password::Hasher for TestHasher {
        fn hash(&self, _password: &str) -> Result<password::Hash> {
            Ok(test_password_hash())
        }

        fn verify(&self, _password: &str, _password_hash: &password::Hash) -> Result<bool> {
            Ok(self.ok)
        }
    }

    #[tokio::test]
    async fn authenticate_returns_user_on_valid_password() {
        let repo = Arc::new(TestRepo {
            record: Some(
                Record::builder()
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
                Record::builder()
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
                Record::builder()
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
