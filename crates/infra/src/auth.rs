use app::auth::{
    AuthRecord, Error, PasswordHash, PasswordHasher, Repository, Result,
};
use argon2::{
    Argon2, PasswordHash as ArgonPasswordHash, PasswordHasher as _,
    PasswordVerifier,
};
use async_trait::async_trait;
use domain::user;
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
        Error::Repository(
            app::auth::RepositoryErrorText::new(error.to_string())
                ,
        )
    }
}

#[async_trait]
impl Repository for AuthRepository {
    async fn find_by_email(
        &self,
        email: &user::Email,
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
        .bind(email.to_string())
        .fetch_optional(&self.pg)
        .await
        .map_err(Self::map_error)?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        Ok(record.map(|row| {
            let username = user::Username::try_new(
                row.get::<String, _>("username"),
            )
            .map_err(|error| {
                Error::Repository(
                    app::auth::RepositoryErrorText::new(
                        error.to_string(),
                    )
                    ,
                )
            })
            .expect("username");
            let email = user::Email::try_new(row.get::<String, _>("email"))
                .map_err(|error| {
                    Error::Repository(
                        app::auth::RepositoryErrorText::new(
                            error.to_string(),
                        )
                        ,
                    )
                })
                .expect("email");
            let password_hash =
                PasswordHash::new(row.get::<String, _>("password_hash"));

            AuthRecord::builder()
                .id(user::Id::from_uuid(
                    row.get::<uuid::Uuid, _>("id"),
                ))
                .username(username)
                .email(email)
                .password_hash(password_hash)
                .build()
        }))
    }

    async fn find_by_id(
        &self,
        user_id: &user::Id,
    ) -> Result<Option<AuthRecord>> {
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT u.id, u.username, u.email, c.password_hash FROM users u JOIN credentials c ON c.user_id = u.id WHERE u.id = $1"
        );
        let record = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, c.password_hash
            FROM users u
            JOIN credentials c ON c.user_id = u.id
            WHERE u.id = $1
            "#,
        )
        .bind(user_id.as_uuid())
        .fetch_optional(&self.pg)
        .await
        .map_err(Self::map_error)?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        Ok(record.map(|row| {
            let username = user::Username::try_new(
                row.get::<String, _>("username"),
            )
            .map_err(|error| {
                Error::Repository(
                    app::auth::RepositoryErrorText::new(
                        error.to_string(),
                    )
                    ,
                )
            })
            .expect("username");
            let email = user::Email::try_new(row.get::<String, _>("email"))
                .map_err(|error| {
                    Error::Repository(
                        app::auth::RepositoryErrorText::new(
                            error.to_string(),
                        )
                        ,
                    )
                })
                .expect("email");
            let password_hash =
                PasswordHash::new(row.get::<String, _>("password_hash"));

            AuthRecord::builder()
                .id(user::Id::from_uuid(
                    row.get::<uuid::Uuid, _>("id"),
                ))
                .username(username)
                .email(email)
                .password_hash(password_hash)
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
    fn hash(&self, password: &str) -> Result<PasswordHash> {
        let salt = password_hash::SaltString::generate(&mut OsRng);
        let hash = self
            .inner
            .hash_password(password.as_bytes(), &salt)
            .map_err(|error| {
                Error::Hash(
                    app::auth::HashErrorText::new(error.to_string()),
                )
            })?
            .to_string();
        Ok(PasswordHash::new(hash))
    }

    fn verify(
        &self,
        password: &str,
        password_hash: &PasswordHash,
    ) -> Result<bool> {
        let hash_text = password_hash.to_string();
        let parsed = ArgonPasswordHash::new(&hash_text)
            .map_err(|error| {
                Error::Hash(
                    app::auth::HashErrorText::new(error.to_string()),
                )
            })?;
        Ok(self
            .inner
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}
