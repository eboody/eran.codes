pub use SqlxUserRepository as Repository;

use app::auth::PasswordHash;
use app::user::{Error, Repository as UserRepository, Result};
use async_trait::async_trait;
use domain::user;
use sqlx::{Row, postgres::PgRow};

pub struct SqlxUserRepository {
    pg: sqlx::PgPool,
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn find_by_email(&self, email: &user::Email) -> Result<Option<user::User>> {
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, username, email FROM users WHERE email = $1"
        );
        let record = sqlx::query(
            r#"
            SELECT id, username, email
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.to_string())
        .fetch_optional(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string().into()))?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        record.map(Self::user_from_row).transpose()
    }

    async fn create_with_credentials(
        &self,
        user: &user::User,
        password_hash: &PasswordHash,
    ) -> Result<()> {
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO users (id, username, email) VALUES ($1, $2, $3)"
        );
        let mut tx = self
            .pg
            .begin()
            .await
            .map_err(|error| Error::Repo(error.to_string().into()))?;

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(user.id.as_uuid())
        .bind(user.username.to_string())
        .bind(user.email.to_string())
        .execute(&mut *tx)
        .await
        .map_err(|error| Error::Repo(error.to_string().into()))?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO credentials (user_id, password_hash) VALUES ($1, $2)"
        );
        let start = std::time::Instant::now();
        sqlx::query(
            r#"
            INSERT INTO credentials (user_id, password_hash)
            VALUES ($1, $2)
            "#,
        )
        .bind(user.id.as_uuid())
        .bind(password_hash.to_string())
        .execute(&mut *tx)
        .await
        .map_err(|error| Error::Repo(error.to_string().into()))?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        tx.commit()
            .await
            .map_err(|error| Error::Repo(error.to_string().into()))?;

        Ok(())
    }
}

impl SqlxUserRepository {
    pub fn new(pg: sqlx::PgPool) -> Self {
        Self { pg }
    }

    fn user_from_row(row: PgRow) -> Result<user::User> {
        let username = user::Username::try_new(row.get::<String, _>("username"))
            .map_err(|error| Error::Repo(error.to_string().into()))?;
        let email = user::Email::try_new(row.get::<String, _>("email"))
            .map_err(|error| Error::Repo(error.to_string().into()))?;

        Ok(user::User {
            id: user::Id::from_uuid(row.get::<uuid::Uuid, _>("id")),
            username,
            email,
        })
    }
}
