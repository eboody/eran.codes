use infra::config::InfraConfig;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub infra: InfraConfig,
    pub http: HttpConfig,
}

#[derive(Clone, Debug)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

impl HttpConfig {
    pub fn from_env() -> Result<Self> {
        let host =
            utils::envs::get_env("HOST").map_err(|_| {
                Error::MissingEnv { key: "HOST" }
            })?;
        let port_str = utils::envs::get_env("PORT")
            .map_err(|_| Error::MissingEnv {
                key: "PORT",
            })?;

        Ok(Self {
            host,
            port: port_str.parse().map_err(|_| {
                Error::InvalidEnv {
                    key: "PORT",
                    reason: "must be a valid u16 integer"
                        .to_string(),
                }
            })?,
        })
    }
}
impl Config {
    pub fn load() -> Result<Self> {
        let infra = InfraConfig::from_env()
            .map_err(Error::Infra)?;
        let http = HttpConfig::from_env()?;

        Ok(Self { infra, http })
    }
}
