pub use SqlxUserRepository as Repository;

use app::user::{Error, Repository as UserRepository, Result};
use async_trait::async_trait;
use domain::user;
use sqlx::Row;

#[allow(unused)]
pub struct SqlxUserRepository {
    pg: sqlx::PgPool,
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn find_by_email(
        &self,
        email: &user::Email,
    ) -> Result<Option<user::User>> {
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
        .map_err(|error| Error::Repo(error.to_string()))?;

        let Some(row) = record else {
            return Ok(None);
        };

        let id = row.get::<uuid::Uuid, _>("id");
        let username = row.get::<String, _>("username");
        let email = row.get::<String, _>("email");

        let username = user::Username::try_new(username)
            .map_err(|error| Error::Repo(error.to_string()))?;
        let email = user::Email::try_new(email)
            .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(Some(user::User {
            id: user::Id::from_uuid(id),
            username,
            email,
        }))
    }

    async fn create_with_credentials(
        &self,
        user: &user::User,
        password_hash: &str,
    ) -> Result<()> {
        let mut tx = self
            .pg
            .begin()
            .await
            .map_err(|error| Error::Repo(error.to_string()))?;

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
        .map_err(|error| Error::Repo(error.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO credentials (user_id, password_hash)
            VALUES ($1, $2)
            "#,
        )
        .bind(user.id.as_uuid())
        .bind(password_hash)
        .execute(&mut *tx)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        tx.commit()
            .await
            .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(())
    }
}

impl SqlxUserRepository {
    pub fn new(pg: sqlx::PgPool) -> Self {
        Self { pg }
    }

    #[tracing::instrument(
        skip(self),
        fields(
            db.statement = "SELECT COUNT(*)::bigint AS count FROM users",
            db.rows = tracing::field::Empty
        )
    )]
    pub async fn count(&self) -> crate::error::Result<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*)::bigint AS count FROM users",
        )
        .fetch_one(&self.pg)
        .await
        .map_err(crate::error::Error::Pgsql)?;

        let count = row.get::<i64, _>("count");
        tracing::Span::current().record("db.rows", 1);
        Ok(count)
    }
}
