// main/src/error.rs
use std::{error::Error as StdError, fmt};

use derive_more::From;
use nutype::nutype;

#[derive(Debug, From)]
pub enum Error {
    Infra(infra::Error),
    MissingEnv {
        key: &'static str,
    },
    InvalidEnv {
        key: &'static str,
        reason: EnvErrorReason,
    },
    #[from]
    Io(std::io::Error),
    // Http(http::error::Error),
    // Service(service::error::Error),
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Display)
)]
pub struct EnvErrorReason(String);

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::Infra(e) => {
                write!(f, "infra config error: {e}")
            }
            Error::MissingEnv { key } => write!(
                f,
                "missing environment variable: {key}"
            ),
            Error::InvalidEnv { key, reason } => {
                write!(
                    f,
                    "invalid environment variable {key}: {reason}"
                )
            }
            Error::Io(e) => write!(f, "io error: {e}"),
            // Error::Http(e) => write!(f, "http config error: {e}"),
            // Error::Service(e) => write!(f, "service config error: {e}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Infra(e) => Some(e),
            Error::MissingEnv { .. } => None,
            Error::InvalidEnv { .. } => None,
            Error::Io(e) => Some(e),
            // Error::Http(e) => Some(e),
            // Error::Service(e) => Some(e),
        }
    }
}
