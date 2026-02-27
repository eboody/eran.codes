use bon::Builder;
use nutype::nutype;

use crate::error::{EnvValue, Error, Result};

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
                key: "DATABASE_URL",
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
    const KEY: &str = "INFRA_DB_MAX_CONNECTIONS";
    let value = match std::env::var(KEY) {
        Ok(value) => value,
        Err(std::env::VarError::NotPresent) => return Ok(10),
        Err(std::env::VarError::NotUnicode(_)) => {
            return Err(Error::InvalidEnv {
                key: KEY,
                value: EnvValue::new("<non-unicode>"),
                reason: "must be a valid u32 integer",
            });
        }
    };

    parse_max_connections_env_value(KEY, &value)
}

fn parse_max_connections_env_value(key: &'static str, value: &str) -> Result<u32> {
    let parsed = value.parse::<u32>().map_err(|_| Error::InvalidEnv {
        key,
        value: EnvValue::new(value),
        reason: "must be a valid u32 integer",
    })?;

    if parsed == 0 {
        return Err(Error::InvalidEnv {
            key,
            value: EnvValue::new(value),
            reason: "must be greater than 0",
        });
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::parse_max_connections_env_value;

    #[test]
    fn parses_positive_max_connections() {
        let parsed = parse_max_connections_env_value("INFRA_DB_MAX_CONNECTIONS", "25")
            .expect("value should parse");
        assert_eq!(parsed, 25);
    }

    #[test]
    fn rejects_zero_max_connections() {
        let result = parse_max_connections_env_value("INFRA_DB_MAX_CONNECTIONS", "0");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_numeric_max_connections() {
        let result = parse_max_connections_env_value("INFRA_DB_MAX_CONNECTIONS", "abc");
        assert!(result.is_err());
    }
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Eq, Display, AsRef))]
pub struct DbUrl(String);
