use nutype::nutype;
use snafu::prelude::*;

const SESSION_CLEANUP_INTERVAL_SECS_DEFAULT: u64 = 3600;
const SESSION_SECRET_MIN_BYTES: usize = 64;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display("missing required environment variable `{name}`"))]
    MissingEnv { name: &'static str },
    #[snafu(display("invalid value for `{name}`: {reason}"))]
    InvalidEnv {
        name: &'static str,
        reason: &'static str,
    },
    #[snafu(display("failed to load infra configuration: {source}"))]
    LoadInfraConfig { source: infra::Error },
}

#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub infra: infra::config::Infra,
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
        let host = required_env("HOST")?;
        let port = parse_port(&required_env("PORT")?)?;
        let session_secret = parse_session_secret(&required_env("SESSION_SECRET")?)?;
        let session_cleanup_interval_secs = session_cleanup_interval_secs_from_env()?;

        Ok(Self {
            host: HostName::new(host),
            port,
            session_secret,
            session_cleanup_interval_secs,
        })
    }
}

fn required_env(name: &'static str) -> Result<String> {
    utils::envs::get_env(name).map_err(|_| Error::MissingEnv { name })
}

fn parse_port(value: &str) -> Result<u16> {
    value.parse().map_err(|_| Error::InvalidEnv {
        name: "PORT",
        reason: "must be a valid u16 integer",
    })
}

fn parse_session_secret(value: &str) -> Result<Vec<u8>> {
    let session_secret = utils::b64::b64u_decode(value).map_err(|_| Error::InvalidEnv {
        name: "SESSION_SECRET",
        reason: "must be base64url without padding",
    })?;

    if session_secret.len() < SESSION_SECRET_MIN_BYTES {
        return Err(Error::InvalidEnv {
            name: "SESSION_SECRET",
            reason: "must be at least 64 bytes",
        });
    }

    Ok(session_secret)
}

fn session_cleanup_interval_secs_from_env() -> Result<u64> {
    match std::env::var("SESSION_CLEANUP_INTERVAL_SECS") {
        Ok(value) => parse_session_cleanup_interval_secs(&value),
        Err(std::env::VarError::NotPresent) => Ok(SESSION_CLEANUP_INTERVAL_SECS_DEFAULT),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name: "SESSION_CLEANUP_INTERVAL_SECS",
            reason: "must be a valid u64 integer",
        }),
    }
}

fn parse_session_cleanup_interval_secs(value: &str) -> Result<u64> {
    let session_cleanup_interval_secs = value.parse().map_err(|_| Error::InvalidEnv {
        name: "SESSION_CLEANUP_INTERVAL_SECS",
        reason: "must be a valid u64 integer",
    })?;

    if session_cleanup_interval_secs == 0 {
        return Err(Error::InvalidEnv {
            name: "SESSION_CLEANUP_INTERVAL_SECS",
            reason: "must be greater than 0",
        });
    }

    Ok(session_cleanup_interval_secs)
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Eq, Display))]
pub struct HostName(String);

impl Config {
    pub fn load() -> Result<Self> {
        let infra = infra::config::Infra::from_env().context(LoadInfraConfigSnafu)?;
        let http = HttpConfig::from_env()?;

        Ok(Self { infra, http })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Error, parse_port, parse_session_cleanup_interval_secs, parse_session_secret,
    };

    fn assert_invalid_env(
        error: Error,
        expected_name: &'static str,
        expected_reason: &'static str,
    ) {
        match error {
            Error::InvalidEnv { name, reason } => {
                assert_eq!(name, expected_name);
                assert_eq!(reason, expected_reason);
            }
            other => panic!("expected invalid env error, got {other:?}"),
        }
    }

    #[test]
    fn parses_valid_port() {
        let port = parse_port("8080").expect("value should parse");
        assert_eq!(port, 8080);
    }

    #[test]
    fn rejects_invalid_port() {
        let error = parse_port("abc").expect_err("value should be rejected");
        assert_invalid_env(error, "PORT", "must be a valid u16 integer");
    }

    #[test]
    fn parses_valid_session_secret() {
        let expected = vec![7_u8; 64];
        let encoded = utils::b64::b64u_encode(&expected);

        let session_secret = parse_session_secret(&encoded).expect("secret should decode");

        assert_eq!(session_secret, expected);
    }

    #[test]
    fn rejects_invalid_session_secret_base64() {
        let error = parse_session_secret("%%%").expect_err("secret should be rejected");
        assert_invalid_env(error, "SESSION_SECRET", "must be base64url without padding");
    }

    #[test]
    fn rejects_short_session_secret() {
        let encoded = utils::b64::b64u_encode([7_u8; 63]);
        let error = parse_session_secret(&encoded).expect_err("secret should be rejected");
        assert_invalid_env(error, "SESSION_SECRET", "must be at least 64 bytes");
    }

    #[test]
    fn rejects_invalid_cleanup_interval_text() {
        let error = parse_session_cleanup_interval_secs("abc")
            .expect_err("interval should be rejected");
        assert_invalid_env(
            error,
            "SESSION_CLEANUP_INTERVAL_SECS",
            "must be a valid u64 integer",
        );
    }

    #[test]
    fn rejects_zero_cleanup_interval() {
        let error = parse_session_cleanup_interval_secs("0")
            .expect_err("interval should be rejected");
        assert_invalid_env(
            error,
            "SESSION_CLEANUP_INTERVAL_SECS",
            "must be greater than 0",
        );
    }
}
