use bon::Builder;
use nutype::nutype;

use crate::error::{Error, Result};

#[derive(Clone, Debug, Builder)]
pub struct InfraConfig {
    pub db: DbConfig,
}

#[derive(Clone, Debug, Builder)]
pub struct DbConfig {
    pub url: DbUrl,
    pub max_connections: u32,
}

impl InfraConfig {
    pub fn from_env() -> Result<Self> {
        let database_url = utils::envs::get_env("DATABASE_URL")
        .map_err(|_| Error::MissingEnv {
            key: "DATABASE_URL",
        })?;

        Ok(
            Self::builder()
                .db(
                    DbConfig::builder()
                        .url(DbUrl::new(database_url))
                        .max_connections(10)
                        .build(),
                )
                .build(),
        )
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Display, AsRef)
)]
pub struct DbUrl(String);
