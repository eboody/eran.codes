use app::auth;
use argon2::{
    Argon2, PasswordHash as ArgonPasswordHash, PasswordHasher as _, PasswordVerifier,
};
use async_trait::async_trait;
use domain::user;
use rand_core::OsRng;
use snafu::{ResultExt, Snafu};
use sqlx::{PgPool, Row, postgres::PgRow, types::time};

pub struct Repository {
    pg: PgPool,
}

#[derive(Debug)]
struct PasswordHashError(password_hash::Error);

impl core::fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for PasswordHashError {}

type RepositoryResult<T> = std::result::Result<T, RepositoryError>;
#[derive(Debug, Snafu)]
enum RepositoryError {
    #[snafu(display("could not query auth record by email"))]
    FindByEmail { source: sqlx::Error },
    #[snafu(display("could not query auth record by id"))]
    FindById { source: sqlx::Error },
    #[snafu(display("could not decode auth username"))]
    DecodeUsername { source: user::UsernameError },
    #[snafu(display("could not decode auth email"))]
    DecodeEmail { source: user::EmailError },
}

fn map_repository_error(error: RepositoryError) -> app::auth::Error {
    match error {
        RepositoryError::FindByEmail { source } => app::auth::Error::query_repository(
            app::auth::repository::Operation::FindByEmail,
            source,
        ),
        RepositoryError::FindById { source } => app::auth::Error::query_repository(
            app::auth::repository::Operation::FindById,
            source,
        ),
        RepositoryError::DecodeUsername { source } => {
            app::auth::Error::decode_username(source)
        }
        RepositoryError::DecodeEmail { source } => app::auth::Error::decode_email(source),
    }
}

impl Repository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    fn session_hash_from_credentials(
        user_id: &uuid::Uuid,
        updated_at: time::OffsetDateTime,
    ) -> app::auth::SessionHash {
        let version = updated_at.unix_timestamp_nanos();
        app::auth::SessionHash::new(format!("auth-v1:{user_id}:{version}"))
    }

    fn auth_record_from_row(row: PgRow) -> RepositoryResult<auth::Record> {
        let user_id = row.get::<uuid::Uuid, _>("id");
        let username = user::Username::try_new(row.get::<String, _>("username"))
            .context(DecodeUsernameSnafu)?;
        let email = user::Email::try_new(row.get::<String, _>("email"))
            .context(DecodeEmailSnafu)?;
        let password_hash =
            auth::password::Hash::new(row.get::<String, _>("password_hash"));
        let credential_updated_at =
            row.get::<time::OffsetDateTime, _>("credential_updated_at");

        Ok(auth::Record::builder()
            .id(user::Id::from(user_id))
            .username(username)
            .email(email)
            .password_hash(password_hash)
            .session_hash(Self::session_hash_from_credentials(
                &user_id,
                credential_updated_at,
            ))
            .build())
    }

    async fn find_by_email_record(
        &self,
        email: &user::Email,
    ) -> RepositoryResult<Option<auth::Record>> {
        let record = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, c.password_hash, c.updated_at AS credential_updated_at
            FROM users u
            JOIN credentials c ON c.user_id = u.id
            WHERE u.email = $1
            "#,
        )
        .bind(email.to_string())
        .fetch_optional(&self.pg)
        .await
        .context(FindByEmailSnafu)?;

        record.map(Self::auth_record_from_row).transpose()
    }

    async fn find_by_id_record(
        &self,
        user_id: &user::Id,
    ) -> RepositoryResult<Option<auth::Record>> {
        let record = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, c.password_hash, c.updated_at AS credential_updated_at
            FROM users u
            JOIN credentials c ON c.user_id = u.id
            WHERE u.id = $1
            "#,
        )
        .bind(user_id.as_ref())
        .fetch_optional(&self.pg)
        .await
        .context(FindByIdSnafu)?;

        record.map(Self::auth_record_from_row).transpose()
    }
}

#[async_trait]
impl auth::Repository for Repository {
    async fn find_by_email(
        &self,
        email: &user::Email,
    ) -> auth::Result<Option<auth::Record>> {
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT u.id, u.username, u.email, c.password_hash, c.updated_at FROM users u JOIN credentials c ON c.user_id = u.id WHERE u.email = $1"
        );
        let result = self.find_by_email_record(email).await;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        result.map_err(map_repository_error)
    }

    async fn find_by_id(&self, user_id: &user::Id) -> auth::Result<Option<auth::Record>> {
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT u.id, u.username, u.email, c.password_hash, c.updated_at FROM users u JOIN credentials c ON c.user_id = u.id WHERE u.id = $1"
        );
        let result = self.find_by_id_record(user_id).await;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        result.map_err(map_repository_error)
    }
}

#[derive(Default)]
pub struct Argon2Hasher {
    inner: Argon2<'static>,
}

impl Argon2Hasher {
    pub fn new() -> Self {
        Self::default()
    }
}

impl auth::password::Hasher for Argon2Hasher {
    fn hash(&self, password: &str) -> auth::Result<auth::password::Hash> {
        let salt = password_hash::SaltString::generate(&mut OsRng);
        let hash = self
            .inner
            .hash_password(password.as_bytes(), &salt)
            .map_err(PasswordHashError)
            .map_err(auth::Error::hash_password)?
            .to_string();
        Ok(auth::password::Hash::new(hash))
    }

    fn verify(
        &self,
        password: &str,
        password_hash: &auth::password::Hash,
    ) -> auth::Result<bool> {
        let hash_text = password_hash.to_string();
        let parsed = ArgonPasswordHash::new(&hash_text)
            .map_err(PasswordHashError)
            .map_err(auth::Error::parse_stored_password_hash)?;
        Ok(self
            .inner
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}
