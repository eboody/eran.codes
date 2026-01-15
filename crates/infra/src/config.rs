use bon::Builder;

use crate::error::{Error, Result};

#[derive(Clone, Debug, Builder)]
pub struct InfraConfig {
    pub db: DbConfig,
}

#[derive(Clone, Debug, Builder)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
}

impl InfraConfig {
    pub fn from_env() -> Result<Self> {
        let database_url = utils::envs::get_env(
            "DATABASE_URL",
        )
        .map_err(|_| Error::MissingEnv {
            key: "DATABASE_URL",
        })?;

        Ok(
            Self::builder()
                .db(
                    DbConfig::builder()
                        .url(database_url)
                        .max_connections(10)
                        .build(),
                )
                .build(),
        )
    }
}
