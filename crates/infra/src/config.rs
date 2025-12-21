use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct InfraConfig {
    pub database_url: String,
}

impl InfraConfig {
    pub fn from_env() -> Result<Self> {
        let database_url = utils::envs::get_env("DATABASE_URL").map_err(|_| Error::MissingEnv {
            key: "DATABASE_URL",
        })?;

        Ok(Self { database_url })
    }
}
