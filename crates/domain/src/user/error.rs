use std::fmt;

use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Username,
    Email,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Username => write!(f, "invalid username"),
            Error::Email => write!(f, "invalid email"),
        }
    }
}

impl std::error::Error for Error {}
