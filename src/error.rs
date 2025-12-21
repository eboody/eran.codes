// main/src/error.rs
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    Infra(infra::error::Error),
    // Http(http::error::Error),
    // Service(service::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Infra(e) => write!(f, "infra config error: {e}"),
            // Error::Http(e) => write!(f, "http config error: {e}"),
            // Error::Service(e) => write!(f, "service config error: {e}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Infra(e) => Some(e),
            // Error::Http(e) => Some(e),
            // Error::Service(e) => Some(e),
        }
    }
}
