// infra/src/error.rs
use std::fmt;

use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    MissingEnv {
        key: &'static str,
    },
    InvalidEnv {
        key: &'static str,
        value: String,
        reason: &'static str,
    },
    Io(std::io::Error),
    Pgsql(sqlx::Error),
    HttpClient(reqwest::Error),
    #[from]
    Migrate(sqlx::migrate::MigrateError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingEnv { key } => write!(f, "missing required environment variable `{key}`"),
            Error::InvalidEnv { key, reason, .. } => {
                write!(f, "invalid value for `{key}`: {reason}")
            }
            Error::Io(e) => write!(f, "io error: {e}"),
            Error::Pgsql(e) => write!(f, "postgresql error: {e}"),
            Error::HttpClient(e) => write!(f, "http client error: {e}"),
            Error::Migrate(e) => write!(f, "database migration error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}
