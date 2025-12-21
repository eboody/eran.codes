use infra::config::InfraConfig;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct AppConfig {
    pub infra: InfraConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let infra = InfraConfig::from_env().map_err(Error::Infra)?;

        Ok(Self { infra })
    }
}
