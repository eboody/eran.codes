pub use SqlxUserRepository as Repository;

use app::auth;
use app::user::Repository as AppUserRepository;
use async_trait::async_trait;
use domain::user;
use snafu::prelude::*;
use sqlx::{Row, postgres::PgRow};

pub struct SqlxUserRepository {
    pg: sqlx::PgPool,
}

type RepositoryResult<T> = std::result::Result<T, UserRepositoryError>;

#[derive(Debug, Snafu)]
enum UserRepositoryError {
    #[snafu(display("could not find user by email"))]
    FindByEmail { source: sqlx::Error },
    #[snafu(display("could not begin create-user transaction"))]
    BeginCreateWithCredentials { source: sqlx::Error },
    #[snafu(display("could not insert user row"))]
    InsertUser { source: sqlx::Error },
    #[snafu(display("could not insert credentials row"))]
    InsertCredentials { source: sqlx::Error },
    #[snafu(display("could not commit create-user transaction"))]
    CommitCreateWithCredentials { source: sqlx::Error },
    #[snafu(display("could not decode username from row"))]
    DecodeUsername { source: user::UsernameError },
    #[snafu(display("could not decode email from row"))]
    DecodeEmail { source: user::EmailError },
}

fn is_email_taken(error: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_error) = error {
        let is_unique_violation = db_error.code().as_deref() == Some("23505");
        let is_email_constraint = db_error.constraint() == Some("users_email_key");
        is_unique_violation && is_email_constraint
    } else {
        false
    }
}

fn map_repository_error(error: UserRepositoryError) -> app::user::Error {
    match error {
        UserRepositoryError::FindByEmail { source } => app::user::Error::query_repository(
            app::user::RepositoryOperation::FindByEmail,
            source,
        ),
        UserRepositoryError::BeginCreateWithCredentials { source } => {
            app::user::Error::query_repository(
                app::user::RepositoryOperation::BeginCreateWithCredentials,
                source,
            )
        }
        UserRepositoryError::InsertUser { source } => {
            if is_email_taken(&source) {
                app::user::Error::EmailTaken
            } else {
                app::user::Error::query_repository(
                    app::user::RepositoryOperation::InsertUser,
                    source,
                )
            }
        }
        UserRepositoryError::InsertCredentials { source } => {
            app::user::Error::query_repository(
                app::user::RepositoryOperation::InsertCredentials,
                source,
            )
        }
        UserRepositoryError::CommitCreateWithCredentials { source } => {
            app::user::Error::query_repository(
                app::user::RepositoryOperation::CommitCreateWithCredentials,
                source,
            )
        }
        UserRepositoryError::DecodeUsername { source } => {
            app::user::Error::decode_username(source)
        }
        UserRepositoryError::DecodeEmail { source } => {
            app::user::Error::decode_email(source)
        }
    }
}

#[async_trait]
impl AppUserRepository for SqlxUserRepository {
    async fn find_by_email(
        &self,
        email: &user::Email,
    ) -> app::user::Result<Option<user::User>> {
        let email_value = email.to_string();
        let start = std::time::Instant::now();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, username, email FROM users WHERE email = $1",
            db_bind_1 = email_value.clone()
        );
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        self.find_by_email_record(email_value)
            .await
            .map_err(map_repository_error)
    }

    async fn create_with_credentials(
        &self,
        user: &user::User,
        password_hash: &auth::PasswordHash,
    ) -> app::user::Result<()> {
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
        self.create_with_credentials_inner(
            user,
            &user_id,
            username,
            email,
            password_hash_value,
            start,
        )
        .await
        .map_err(map_repository_error)
    }
}

impl SqlxUserRepository {
    pub fn new(pg: sqlx::PgPool) -> Self {
        Self { pg }
    }

    async fn find_by_email_record(
        &self,
        email_value: String,
    ) -> RepositoryResult<Option<user::User>> {
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
        .context(FindByEmailSnafu)?;

        record.map(Self::user_from_row).transpose()
    }

    async fn create_with_credentials_inner(
        &self,
        user: &user::User,
        user_id: &str,
        username: String,
        email: String,
        password_hash_value: String,
        start: std::time::Instant,
    ) -> RepositoryResult<()> {
        let mut tx = self
            .pg
            .begin()
            .await
            .context(BeginCreateWithCredentialsSnafu)?;

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
        .context(InsertUserSnafu)?;
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
        .context(InsertCredentialsSnafu)?;
        tracing::info!(
            target: "demo.db",
            message = "db query complete",
            db_duration_ms = start.elapsed().as_millis() as u64
        );

        tx.commit()
            .await
            .context(CommitCreateWithCredentialsSnafu)?;

        Ok(())
    }

    fn user_from_row(row: PgRow) -> RepositoryResult<user::User> {
        let username = user::Username::try_new(row.get::<String, _>("username"))
            .context(DecodeUsernameSnafu)?;
        let email = user::Email::try_new(row.get::<String, _>("email"))
            .context(DecodeEmailSnafu)?;

        Ok(user::User {
            id: user::Id::from_uuid(row.get::<uuid::Uuid, _>("id")),
            username,
            email,
        })
    }
}
