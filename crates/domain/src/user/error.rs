use std::fmt;

use derive_more::From;

use crate::user::{EmailError, UsernameError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Username(UsernameError),
    Email(EmailError),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::Username(e) => {
                write!(f, "invalid username: {}", e)
            }
            Error::Email(e) => {
                write!(f, "invalid email: {}", e)
            }
        }
    }
}

impl std::error::Error for Error {}
