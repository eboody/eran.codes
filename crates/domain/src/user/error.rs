use snafu::Snafu;

use crate::user::{EmailError, UsernameError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid username: {source}"))]
    Username {
        source: UsernameError,
    },
    #[snafu(display("invalid email: {source}"))]
    Email {
        source: EmailError,
    },
}

impl From<UsernameError> for Error {
    fn from(source: UsernameError) -> Self {
        Self::Username {
            source,
        }
    }
}

impl From<EmailError> for Error {
    fn from(source: EmailError) -> Self {
        Self::Email {
            source,
        }
    }
}
