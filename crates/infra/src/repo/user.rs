pub use SqlxUserRepository as Repository;
use domain::user;

use crate::error::Result;

#[allow(unused)]
pub struct SqlxUserRepository {
    pg: sqlx::PgPool,
}

#[allow(unused)]
impl user::Repository for SqlxUserRepository {
    // Implement user repository methods here
    fn get_by_id(
        &self,
        id: &user::Id,
    ) -> domain::Result<Option<user::User>> {
        todo!()
    }

    fn save(
        &self,
        user: &user::User,
    ) -> domain::Result<()> {
        todo!()
    }

    fn get_by_email(
        &self,
        email: &user::Email,
    ) -> domain::Result<Option<user::User>> {
        todo!()
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
    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query!(
            "SELECT COUNT(*)::bigint AS count FROM users"
        )
        .fetch_one(&self.pg)
        .await
        .map_err(crate::error::Error::Pgsql)?;

        tracing::Span::current().record("db.rows", 1);
        Ok(row.count.unwrap_or(0))
    }
}
