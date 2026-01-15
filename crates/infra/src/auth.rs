use app::auth::{AuthRecord, Error, PasswordHasher, Repository, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher as _, PasswordVerifier};
use async_trait::async_trait;
use rand_core::OsRng;
use sqlx::{PgPool, Row};

pub struct AuthRepository {
    pg: PgPool,
}

impl AuthRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    fn map_error(error: sqlx::Error) -> Error {
        Error::Repository(error.to_string())
    }
}

#[async_trait]
impl Repository for AuthRepository {
    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<AuthRecord>> {
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT u.id, u.username, u.email, c.password_hash FROM users u JOIN credentials c ON c.user_id = u.id WHERE u.email = $1"
        );
        let record = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, c.password_hash
            FROM users u
            JOIN credentials c ON c.user_id = u.id
            WHERE u.email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pg)
        .await
        .map_err(Self::map_error)?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        Ok(record.map(|row| {
            AuthRecord::builder()
                .id(row.get::<uuid::Uuid, _>("id").to_string())
                .username(row.get::<String, _>("username"))
                .email(row.get::<String, _>("email"))
                .password_hash(row.get::<String, _>("password_hash"))
                .build()
        }))
    }

    async fn find_by_id(
        &self,
        user_id: &str,
    ) -> Result<Option<AuthRecord>> {
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT u.id, u.username, u.email, c.password_hash FROM users u JOIN credentials c ON c.user_id = u.id WHERE u.id = $1"
        );
        let user_id = user_id
            .parse::<uuid::Uuid>()
            .map_err(|error| Error::Repository(error.to_string()))?;

        let record = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, c.password_hash
            FROM users u
            JOIN credentials c ON c.user_id = u.id
            WHERE u.id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pg)
        .await
        .map_err(Self::map_error)?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        Ok(record.map(|row| {
            AuthRecord::builder()
                .id(row.get::<uuid::Uuid, _>("id").to_string())
                .username(row.get::<String, _>("username"))
                .email(row.get::<String, _>("email"))
                .password_hash(row.get::<String, _>("password_hash"))
                .build()
        }))
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

impl PasswordHasher for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String> {
        let salt = password_hash::SaltString::generate(&mut OsRng);
        let hash = self
            .inner
            .hash_password(password.as_bytes(), &salt)
            .map_err(|error| Error::Hash(error.to_string()))?
            .to_string();
        Ok(hash)
    }

    fn verify(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool> {
        let parsed = PasswordHash::new(password_hash)
            .map_err(|error| Error::Hash(error.to_string()))?;
        Ok(self
            .inner
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}
