use std::{env, str::FromStr};

use snafu::Snafu;

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

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("missing required environment variable `{name}`"))]
    MissingEnv {
        name: &'static str,
        source: env::VarError,
    },
    #[snafu(display("invalid value for `{name}`: {reason}"))]
    WrongFormat { name: &'static str, reason: String },
}

// endregion: --- Error
