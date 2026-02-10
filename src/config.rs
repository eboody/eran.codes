use infra::config::InfraConfig;
use nutype::nutype;

use crate::error::{EnvErrorReason, Error, Result};

#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub infra: InfraConfig,
    pub http: HttpConfig,
}

#[derive(Clone, Debug)]
pub struct HttpConfig {
    pub host: HostName,
    pub port: u16,
    pub session_secret: Vec<u8>,
    pub session_cleanup_interval_secs: u64,
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
                    reason: EnvErrorReason::new(
                        "must be base64url without padding",
                    ),
                })?;
        if session_secret.len() < 64 {
            return Err(Error::InvalidEnv {
                key: "SESSION_SECRET",
                reason: EnvErrorReason::new("must be at least 64 bytes"),
            });
        }

        let session_cleanup_interval_secs =
            match std::env::var("SESSION_CLEANUP_INTERVAL_SECS") {
                Ok(value) => value.parse().map_err(|_| {
                    Error::InvalidEnv {
                        key: "SESSION_CLEANUP_INTERVAL_SECS",
                        reason: EnvErrorReason::new(
                            "must be a valid u64 integer",
                        ),
                    }
                })?,
                Err(std::env::VarError::NotPresent) => 3600,
                Err(_) => {
                    return Err(Error::InvalidEnv {
                        key: "SESSION_CLEANUP_INTERVAL_SECS",
                        reason: EnvErrorReason::new(
                            "must be a valid u64 integer",
                        ),
                    })
                }
            };

        if session_cleanup_interval_secs == 0 {
            return Err(Error::InvalidEnv {
                key: "SESSION_CLEANUP_INTERVAL_SECS",
                reason: EnvErrorReason::new("must be greater than 0"),
            });
        }

        Ok(Self {
            host: HostName::new(host),
            port: port_str.parse().map_err(|_| {
                Error::InvalidEnv {
                    key: "PORT",
                    reason: EnvErrorReason::new(
                        "must be a valid u16 integer",
                    ),
                }
            })?,
            session_secret,
            session_cleanup_interval_secs,
        })
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Display)
)]
pub struct HostName(String);
impl Config {
    pub fn load() -> Result<Self> {
        let infra = InfraConfig::from_env()
            .map_err(Error::Infra)?;
        let http = HttpConfig::from_env()?;

        Ok(Self { infra, http })
    }
}
