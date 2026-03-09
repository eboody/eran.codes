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
        let email_value = email.to_string();
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, username, email FROM users WHERE email = $1",
            db_bind_1 = email_value.clone()
        );
        let record = sqlx::query(
            r#"
            SELECT id, username, email
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email_value)
        .fetch_optional(&self.pg)
        .await
        .map_err(Self::map_sqlx_error)?;
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
        let user_id = user.id.as_uuid().to_string();
        let username = user.username.to_string();
        let email = user.email.to_string();
        let password_hash_value = password_hash.to_string();
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO users (id, username, email) VALUES ($1, $2, $3)",
            db_bind_1 = user_id.clone(),
            db_bind_2 = username.clone(),
            db_bind_3 = email.clone()
        );
        let mut tx = self.pg.begin().await.map_err(Self::map_sqlx_error)?;

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(user.id.as_uuid())
        .bind(username)
        .bind(email)
        .execute(&mut *tx)
        .await
        .map_err(Self::map_sqlx_error)?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO credentials (user_id, password_hash) VALUES ($1, $2)",
            db_bind_1 = user_id,
            db_bind_2 = password_hash_value.clone()
        );
        let start = std::time::Instant::now();
        sqlx::query(
            r#"
            INSERT INTO credentials (user_id, password_hash)
            VALUES ($1, $2)
            "#,
        )
        .bind(user.id.as_uuid())
        .bind(password_hash_value)
        .execute(&mut *tx)
        .await
        .map_err(Self::map_sqlx_error)?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        tx.commit().await.map_err(Self::map_sqlx_error)?;

        Ok(())
    }
}

impl SqlxUserRepository {
    pub fn new(pg: sqlx::PgPool) -> Self {
        Self { pg }
    }

    fn map_sqlx_error(error: sqlx::Error) -> Error {
        if let sqlx::Error::Database(db_error) = &error {
            let is_unique_violation = db_error.code().as_deref() == Some("23505");
            let is_email_constraint = db_error.constraint() == Some("users_email_key");
            if is_unique_violation && is_email_constraint {
                return Error::EmailTaken;
            }
        }

        Error::Repo(error.to_string().into())
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
