use snafu::prelude::*;
use strum_macros::Display;

use domain::user;

pub type Result<T> = core::result::Result<T, Error>;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    Domain { source: domain::user::Error },
    #[snafu(display("user password hashing failed: {source}"))]
    Hashing { source: crate::auth::Error },
    #[snafu(display("{source}"))]
    Repo { source: RepositoryError },
    #[snafu(display("email already in use"))]
    EmailTaken,
}

#[derive(Clone, Copy, Debug, Display)]
pub enum RepositoryOperation {
    #[strum(serialize = "find user by email")]
    FindByEmail,
    #[strum(serialize = "begin create user transaction")]
    BeginCreateWithCredentials,
    #[strum(serialize = "insert user row")]
    InsertUser,
    #[strum(serialize = "insert credentials row")]
    InsertCredentials,
    #[strum(serialize = "commit create user transaction")]
    CommitCreateWithCredentials,
}

#[derive(Debug, Snafu)]
pub enum RepositoryError {
    #[snafu(display("user repository query failed while {operation}: {source}"))]
    Query {
        operation: RepositoryOperation,
        source: BoxError,
    },
    #[snafu(display("failed to decode username from row: {source}"))]
    DecodeUsername { source: user::UsernameError },
    #[snafu(display("failed to decode email from row: {source}"))]
    DecodeEmail { source: user::EmailError },
}

impl From<domain::user::Error> for Error {
    fn from(source: domain::user::Error) -> Self {
        Self::Domain { source }
    }
}

fn box_error(source: impl std::error::Error + Send + Sync + 'static) -> BoxError {
    Box::new(source)
}

impl From<crate::auth::Error> for Error {
    fn from(source: crate::auth::Error) -> Self {
        Self::Hashing { source }
    }
}

impl Error {
    pub fn query(
        operation: RepositoryOperation,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Repo {
            source: RepositoryError::Query {
                operation,
                source: box_error(source),
            },
        }
    }

    pub fn decode_username(source: user::UsernameError) -> Self {
        Self::Repo {
            source: RepositoryError::DecodeUsername { source },
        }
    }

    pub fn decode_email(source: user::EmailError) -> Self {
        Self::Repo {
            source: RepositoryError::DecodeEmail { source },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn repo_error_preserves_source() {
        let error = Error::query(
            RepositoryOperation::FindByEmail,
            std::io::Error::other("db down"),
        );

        assert_eq!(
            error
                .source()
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some("user repository query failed while find user by email: db down"),
        );
        assert_eq!(
            error
                .source()
                .and_then(|source| source.source())
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some("db down"),
        );
    }
}
