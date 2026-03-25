use snafu::Snafu;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("missing required environment variable `{name}`"))]
    MissingEnv { name: &'static str },
    #[snafu(display("invalid value for `{name}`: {reason}"))]
    InvalidEnv {
        name: &'static str,
        reason: &'static str,
    },
    #[snafu(display("failed to connect to postgres at {database_url}: {source}"))]
    ConnectDb {
        database_url: String,
        source: sqlx::Error,
    },
    #[snafu(display("failed to verify postgres connectivity at {database_url}: {source}"))]
    CheckDbConnection {
        database_url: String,
        source: sqlx::Error,
    },
    #[snafu(display("failed to run database migrations: {source}"))]
    RunMigrations { source: sqlx::migrate::MigrateError },
    #[snafu(display("failed to build shared HTTP client: {source}"))]
    BuildHttpClient { source: reqwest::Error },
}
