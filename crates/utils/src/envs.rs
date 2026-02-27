use std::{env, str::FromStr};

use crate::b64::b64u_decode;

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|source| Error::MissingEnv { name, source })
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T>
where
    T::Err: core::fmt::Display,
{
    let val = get_env(name)?;
    val.parse::<T>().map_err(|source| Error::WrongFormat {
        name,
        reason: source.to_string(),
    })
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|source| Error::WrongFormat {
        name,
        reason: source.to_string(),
    })
}

// region:    --- Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingEnv {
        name: &'static str,
        source: env::VarError,
    },
    WrongFormat {
        name: &'static str,
        reason: String,
    },
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::MissingEnv { name, .. } => {
                write!(fmt, "missing required environment variable `{name}`")
            }
            Error::WrongFormat { name, reason } => {
                write!(fmt, "invalid value for `{name}`: {reason}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::MissingEnv { source, .. } => Some(source),
            Error::WrongFormat { .. } => None,
        }
    }
}
// endregion: --- Error Boilerplate

// endregion: --- Error
