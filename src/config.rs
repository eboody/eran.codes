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
    pub session_secret: Vec<u8>,
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

        let session_secret =
            utils::envs::get_env_b64u_as_u8s("SESSION_SECRET")
                .map_err(|_| Error::InvalidEnv {
                    key: "SESSION_SECRET",
                    reason: "must be base64url without padding"
                        .to_string(),
                })?;
        if session_secret.len() < 32 {
            return Err(Error::InvalidEnv {
                key: "SESSION_SECRET",
                reason: "must be at least 32 bytes".to_string(),
            });
        }

        Ok(Self {
            host,
            port: port_str.parse().map_err(|_| {
                Error::InvalidEnv {
                    key: "PORT",
                    reason: "must be a valid u16 integer"
                        .to_string(),
                }
            })?,
            session_secret,
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
