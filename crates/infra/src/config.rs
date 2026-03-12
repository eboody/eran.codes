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
        let database_url =
            utils::envs::get_env("DATABASE_URL").map_err(|_| Error::MissingEnv {
                name: "DATABASE_URL",
            })?;
        let max_connections = max_connections_from_env()?;

        Ok(Self::builder()
            .db(DbConfig::builder()
                .url(DbUrl::new(database_url))
                .max_connections(max_connections)
                .build())
            .build())
    }
}

fn max_connections_from_env() -> Result<u32> {
    const NAME: &str = "INFRA_DB_MAX_CONNECTIONS";
    let value = match std::env::var(NAME) {
        Ok(value) => value,
        Err(std::env::VarError::NotPresent) => return Ok(10),
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(Error::InvalidEnv {
                name: NAME,
                reason: "must be a valid u32 integer",
            });
        }
    };

    parse_max_connections_env_value(NAME, &value)
}

fn parse_max_connections_env_value(name: &'static str, value: &str) -> Result<u32> {
    let parsed = value.parse::<u32>().map_err(|_| Error::InvalidEnv {
        name,
        reason: "must be a valid u32 integer",
    })?;

    if parsed == 0 {
        return Err(Error::InvalidEnv {
            name,
            reason: "must be greater than 0",
        });
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::parse_max_connections_env_value;
    use crate::error::Error;

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
    fn parses_positive_max_connections() {
        let parsed = parse_max_connections_env_value("INFRA_DB_MAX_CONNECTIONS", "25")
            .expect("value should parse");
        assert_eq!(parsed, 25);
    }

    #[test]
    fn rejects_zero_max_connections() {
        let error = parse_max_connections_env_value("INFRA_DB_MAX_CONNECTIONS", "0")
            .expect_err("value should be rejected");
        assert_invalid_env(error, "INFRA_DB_MAX_CONNECTIONS", "must be greater than 0");
    }

    #[test]
    fn rejects_non_numeric_max_connections() {
        let error = parse_max_connections_env_value("INFRA_DB_MAX_CONNECTIONS", "abc")
            .expect_err("value should be rejected");
        assert_invalid_env(
            error,
            "INFRA_DB_MAX_CONNECTIONS",
            "must be a valid u32 integer",
        );
    }
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Eq, Display, AsRef))]
pub struct DbUrl(String);
