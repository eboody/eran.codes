use infra::crypto::KeyMaterial;
use nutype::nutype;
use serde::Deserialize;
use snafu::prelude::*;

const SESSION_CLEANUP_INTERVAL_SECS_DEFAULT: u64 = 3600;
const INTEGRATION_TOKEN_REFRESH_INTERVAL_SECS_DEFAULT: u64 = 900;
const INTEGRATION_SYNC_INTERVAL_SECS_DEFAULT: u64 = 1200;
const ENCRYPTION_ROTATION_INTERVAL_SECS_DEFAULT: u64 = 1800;
const ENCRYPTION_ROTATION_BATCH_SIZE_DEFAULT: usize = 25;
const SENSITIVE_SANDBOX_TIMEOUT_SECS_DEFAULT: u64 = 10;
const SENSITIVE_SANDBOX_RETRY_BACKOFF_SECS_DEFAULT: u64 = 45;
const SESSION_SECRET_MIN_BYTES: usize = 64;
const DATA_ENCRYPTION_KEY_LEN: usize = 32;
const LEGACY_DATA_KEY_ID: &str = "legacy_data_key";

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
    pub sensitive: SensitiveConfig,
}

#[derive(Clone, Debug)]
pub struct HttpConfig {
    pub host: HostName,
    pub port: u16,
    pub session_secret: Vec<u8>,
    pub session_cleanup_interval_secs: u64,
}

#[derive(Clone, Debug)]
pub struct SensitiveConfig {
    pub data_encryption_keys: Vec<KeyMaterial>,
    pub active_data_key_id: domain::sensitive::KeyId,
    pub disabled_data_key_ids: Vec<domain::sensitive::KeyId>,
    pub provider_mode: SensitiveProviderRuntimeMode,
    pub token_refresh_interval_secs: u64,
    pub sync_interval_secs: u64,
    pub rotation_interval_secs: u64,
    pub rotation_batch_size: usize,
    pub reader_emails: Vec<domain::user::Email>,
    pub operator_emails: Vec<domain::user::Email>,
    pub provider_stub_port: u16,
    pub provider_stub_failure_mode: SensitiveProviderStubFailureMode,
    pub sandbox: SensitiveSandboxConfig,
}

#[derive(Clone, Debug)]
pub struct SensitiveSandboxConfig {
    pub base_url: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub timeout_secs: u64,
    pub retry_backoff_secs: u64,
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

impl SensitiveConfig {
    pub fn from_env(http_port: u16) -> Result<Self> {
        let (data_encryption_keys, active_data_key_id) = data_encryption_config_from_env()?;
        let disabled_data_key_ids = key_id_list_from_env("DISABLED_DATA_KEY_IDS")?;
        let provider_mode = provider_mode_from_env()?;
        let token_refresh_interval_secs = integration_interval_secs_from_env(
            "INTEGRATION_TOKEN_REFRESH_INTERVAL_SECS",
            INTEGRATION_TOKEN_REFRESH_INTERVAL_SECS_DEFAULT,
        )?;
        let sync_interval_secs = integration_interval_secs_from_env(
            "INTEGRATION_SYNC_INTERVAL_SECS",
            INTEGRATION_SYNC_INTERVAL_SECS_DEFAULT,
        )?;
        let rotation_interval_secs = integration_interval_secs_from_env(
            "ENCRYPTION_ROTATION_INTERVAL_SECS",
            ENCRYPTION_ROTATION_INTERVAL_SECS_DEFAULT,
        )?;
        let rotation_batch_size = batch_size_from_env(
            "ENCRYPTION_ROTATION_BATCH_SIZE",
            ENCRYPTION_ROTATION_BATCH_SIZE_DEFAULT,
        )?;
        let reader_emails = email_list_from_env("SENSITIVE_READER_EMAILS")?;
        let operator_emails = email_list_from_env("SENSITIVE_OPERATOR_EMAILS")?;
        let provider_stub_port = provider_stub_port_from_env(http_port)?;
        let provider_stub_failure_mode = provider_stub_failure_mode_from_env()?;
        let sandbox = SensitiveSandboxConfig {
            base_url: optional_env("SENSITIVE_PROVIDER_BASE_URL")?,
            client_id: optional_env("SENSITIVE_SANDBOX_CLIENT_ID")?,
            client_secret: optional_env("SENSITIVE_SANDBOX_CLIENT_SECRET")?,
            timeout_secs: integration_interval_secs_from_env(
                "SENSITIVE_SANDBOX_TIMEOUT_SECS",
                SENSITIVE_SANDBOX_TIMEOUT_SECS_DEFAULT,
            )?,
            retry_backoff_secs: integration_interval_secs_from_env(
                "SENSITIVE_SANDBOX_RETRY_BACKOFF_SECS",
                SENSITIVE_SANDBOX_RETRY_BACKOFF_SECS_DEFAULT,
            )?,
        };

        Ok(Self {
            data_encryption_keys,
            active_data_key_id,
            disabled_data_key_ids,
            provider_mode,
            token_refresh_interval_secs,
            sync_interval_secs,
            rotation_interval_secs,
            rotation_batch_size,
            reader_emails,
            operator_emails,
            provider_stub_port,
            provider_stub_failure_mode,
            sandbox,
        })
    }

    pub fn provider_base_url(&self) -> Option<String> {
        match self.provider_mode {
            SensitiveProviderRuntimeMode::Stub => {
                Some(format!("http://127.0.0.1:{}/", self.provider_stub_port))
            }
            SensitiveProviderRuntimeMode::SandboxHttp => self.sandbox.base_url.clone(),
        }
    }

    pub fn provider_stub_addr(&self) -> String {
        format!("127.0.0.1:{}", self.provider_stub_port)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SensitiveProviderRuntimeMode {
    Stub,
    SandboxHttp,
}

impl SensitiveProviderRuntimeMode {
    fn from_str(value: &str) -> Result<Self> {
        match value {
            "stub" => Ok(Self::Stub),
            "sandbox_http" => Ok(Self::SandboxHttp),
            _ => Err(Error::InvalidEnv {
                name: "SENSITIVE_PROVIDER_MODE",
                reason: "must be one of: stub, sandbox_http",
            }),
        }
    }
}

fn required_env(name: &'static str) -> Result<String> {
    utils::envs::get_env(name).map_err(|_| Error::MissingEnv { name })
}

fn optional_env(name: &'static str) -> Result<Option<String>> {
    match std::env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name,
            reason: "must be valid unicode text",
        }),
    }
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

fn parse_data_encryption_key(value: &str) -> Result<[u8; DATA_ENCRYPTION_KEY_LEN]> {
    decode_key_material("DATA_ENCRYPTION_KEY", value)
}

fn decode_key_material(
    name: &'static str,
    value: &str,
) -> Result<[u8; DATA_ENCRYPTION_KEY_LEN]> {
    let key = utils::b64::b64u_decode(value).map_err(|_| Error::InvalidEnv {
        name,
        reason: "must be base64url without padding",
    })?;

    key.try_into().map_err(|_| Error::InvalidEnv {
        name,
        reason: "must decode to exactly 32 bytes",
    })
}

fn data_encryption_config_from_env() -> Result<(Vec<KeyMaterial>, domain::sensitive::KeyId)>
{
    match std::env::var("DATA_ENCRYPTION_KEYS_JSON") {
        Ok(value) => {
            let keys = parse_data_encryption_keys_json(&value)?;
            let active_key_id =
                parse_key_id("ACTIVE_DATA_KEY_ID", &required_env("ACTIVE_DATA_KEY_ID")?)?;
            Ok((keys, active_key_id))
        }
        Err(std::env::VarError::NotPresent) => Ok((
            vec![KeyMaterial {
                key_id: legacy_data_key_id(),
                key: parse_data_encryption_key(&required_env("DATA_ENCRYPTION_KEY")?)?,
            }],
            legacy_data_key_id(),
        )),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name: "DATA_ENCRYPTION_KEYS_JSON",
            reason: "must be valid JSON",
        }),
    }
}

fn legacy_data_key_id() -> domain::sensitive::KeyId {
    domain::sensitive::KeyId::try_new(LEGACY_DATA_KEY_ID)
        .expect("legacy data key id should stay valid")
}

#[derive(Deserialize)]
struct DataEncryptionKeyJsonEntry {
    key_id: String,
    key: String,
}

fn parse_data_encryption_keys_json(value: &str) -> Result<Vec<KeyMaterial>> {
    let entries: Vec<DataEncryptionKeyJsonEntry> =
        serde_json::from_str(value).map_err(|_| Error::InvalidEnv {
            name: "DATA_ENCRYPTION_KEYS_JSON",
            reason: "must be a JSON array of {key_id,key} entries",
        })?;

    if entries.is_empty() {
        return Err(Error::InvalidEnv {
            name: "DATA_ENCRYPTION_KEYS_JSON",
            reason: "must include at least one encryption key entry",
        });
    }

    let mut keys = Vec::new();
    for entry in entries {
        let key_id = parse_key_id("DATA_ENCRYPTION_KEYS_JSON", &entry.key_id)?;
        if keys
            .iter()
            .any(|existing: &KeyMaterial| existing.key_id == key_id)
        {
            return Err(Error::InvalidEnv {
                name: "DATA_ENCRYPTION_KEYS_JSON",
                reason: "must not contain duplicate key_id values",
            });
        }
        keys.push(KeyMaterial {
            key_id,
            key: decode_key_material("DATA_ENCRYPTION_KEYS_JSON", &entry.key)?,
        });
    }

    Ok(keys)
}

fn parse_key_id(name: &'static str, value: &str) -> Result<domain::sensitive::KeyId> {
    domain::sensitive::KeyId::try_new(value).map_err(|_| Error::InvalidEnv {
        name,
        reason: "must be a non-empty key id up to 64 characters",
    })
}

fn key_id_list_from_env(name: &'static str) -> Result<Vec<domain::sensitive::KeyId>> {
    match std::env::var(name) {
        Ok(value) => parse_key_id_list(name, &value),
        Err(std::env::VarError::NotPresent) => Ok(Vec::new()),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name,
            reason: "must be a comma-separated list of valid key ids",
        }),
    }
}

fn parse_key_id_list(
    name: &'static str,
    value: &str,
) -> Result<Vec<domain::sensitive::KeyId>> {
    let mut key_ids = Vec::new();
    for raw_key_id in value.split(',') {
        let trimmed = raw_key_id.trim();
        if trimmed.is_empty() {
            continue;
        }
        let key_id = parse_key_id(name, trimmed)?;
        if !key_ids.contains(&key_id) {
            key_ids.push(key_id);
        }
    }
    Ok(key_ids)
}

fn email_list_from_env(name: &'static str) -> Result<Vec<domain::user::Email>> {
    match std::env::var(name) {
        Ok(value) => parse_email_list(name, &value),
        Err(std::env::VarError::NotPresent) => Ok(Vec::new()),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name,
            reason: "must be a comma-separated list of valid email addresses",
        }),
    }
}

fn parse_email_list(name: &'static str, value: &str) -> Result<Vec<domain::user::Email>> {
    let mut emails = Vec::new();
    for raw_email in value.split(',') {
        let trimmed = raw_email.trim();
        if trimmed.is_empty() {
            continue;
        }
        let email =
            domain::user::Email::try_new(trimmed).map_err(|_| Error::InvalidEnv {
                name,
                reason: "must be a comma-separated list of valid email addresses",
            })?;
        if !emails.contains(&email) {
            emails.push(email);
        }
    }
    Ok(emails)
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
    parse_positive_u64_env(
        "SESSION_CLEANUP_INTERVAL_SECS",
        value,
        "must be a valid u64 integer",
    )
}

fn integration_interval_secs_from_env(name: &'static str, default: u64) -> Result<u64> {
    match std::env::var(name) {
        Ok(value) => parse_positive_u64_env(name, &value, "must be a valid u64 integer"),
        Err(std::env::VarError::NotPresent) => Ok(default),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name,
            reason: "must be a valid u64 integer",
        }),
    }
}

fn parse_positive_u64_env(
    name: &'static str,
    value: &str,
    invalid_reason: &'static str,
) -> Result<u64> {
    let parsed = value.parse().map_err(|_| Error::InvalidEnv {
        name,
        reason: invalid_reason,
    })?;

    if parsed == 0 {
        return Err(Error::InvalidEnv {
            name,
            reason: "must be greater than 0",
        });
    }

    Ok(parsed)
}

fn batch_size_from_env(name: &'static str, default: usize) -> Result<usize> {
    match std::env::var(name) {
        Ok(value) => parse_positive_usize_env(name, &value),
        Err(std::env::VarError::NotPresent) => Ok(default),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name,
            reason: "must be a valid usize integer",
        }),
    }
}

fn parse_positive_usize_env(name: &'static str, value: &str) -> Result<usize> {
    let parsed = value.parse().map_err(|_| Error::InvalidEnv {
        name,
        reason: "must be a valid usize integer",
    })?;

    if parsed == 0 {
        return Err(Error::InvalidEnv {
            name,
            reason: "must be greater than 0",
        });
    }

    Ok(parsed)
}

fn provider_stub_port_from_env(http_port: u16) -> Result<u16> {
    match std::env::var("SENSITIVE_PROVIDER_PORT") {
        Ok(value) => value.parse().map_err(|_| Error::InvalidEnv {
            name: "SENSITIVE_PROVIDER_PORT",
            reason: "must be a valid u16 integer",
        }),
        Err(std::env::VarError::NotPresent) => Ok(http_port.saturating_add(1000)),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name: "SENSITIVE_PROVIDER_PORT",
            reason: "must be a valid u16 integer",
        }),
    }
}

fn provider_mode_from_env() -> Result<SensitiveProviderRuntimeMode> {
    match std::env::var("SENSITIVE_PROVIDER_MODE") {
        Ok(value) => SensitiveProviderRuntimeMode::from_str(&value),
        Err(std::env::VarError::NotPresent) => Ok(SensitiveProviderRuntimeMode::Stub),
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name: "SENSITIVE_PROVIDER_MODE",
            reason: "must be one of: stub, sandbox_http",
        }),
    }
}

fn provider_stub_failure_mode_from_env() -> Result<SensitiveProviderStubFailureMode> {
    match std::env::var("SENSITIVE_PROVIDER_STUB_FAILURE_MODE") {
        Ok(value) => SensitiveProviderStubFailureMode::from_str(&value),
        Err(std::env::VarError::NotPresent) => {
            Ok(SensitiveProviderStubFailureMode::Healthy)
        }
        Err(std::env::VarError::NotUnicode(_)) => Err(Error::InvalidEnv {
            name: "SENSITIVE_PROVIDER_STUB_FAILURE_MODE",
            reason: "must be one of: healthy, unauthorized_once, malformed_page, rate_limited",
        }),
    }
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Eq, Display))]
pub struct HostName(String);

impl Config {
    pub fn load() -> Result<Self> {
        let infra = infra::config::Infra::from_env().context(LoadInfraConfigSnafu)?;
        let http = HttpConfig::from_env()?;
        let sensitive = SensitiveConfig::from_env(http.port)?;

        Ok(Self {
            infra,
            http,
            sensitive,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SensitiveProviderStubFailureMode {
    Healthy,
    UnauthorizedOnce,
    MalformedPage,
    RateLimited,
}

impl SensitiveProviderStubFailureMode {
    fn from_str(value: &str) -> Result<Self> {
        match value {
            "healthy" => Ok(Self::Healthy),
            "unauthorized_once" => Ok(Self::UnauthorizedOnce),
            "malformed_page" => Ok(Self::MalformedPage),
            "rate_limited" => Ok(Self::RateLimited),
            _ => Err(Error::InvalidEnv {
                name: "SENSITIVE_PROVIDER_STUB_FAILURE_MODE",
                reason: "must be one of: healthy, unauthorized_once, malformed_page, rate_limited",
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, OnceLock};

    use super::{
        Error, SensitiveProviderRuntimeMode, SensitiveProviderStubFailureMode,
        data_encryption_config_from_env, parse_data_encryption_key,
        parse_data_encryption_keys_json, parse_email_list, parse_key_id_list, parse_port,
        parse_positive_u64_env, parse_positive_usize_env, parse_session_secret,
        provider_mode_from_env, provider_stub_failure_mode_from_env,
        provider_stub_port_from_env,
    };

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock")
    }

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
        let error = parse_positive_u64_env(
            "SESSION_CLEANUP_INTERVAL_SECS",
            "abc",
            "must be a valid u64 integer",
        )
        .expect_err("interval should be rejected");
        assert_invalid_env(
            error,
            "SESSION_CLEANUP_INTERVAL_SECS",
            "must be a valid u64 integer",
        );
    }

    #[test]
    fn rejects_zero_cleanup_interval() {
        let error = parse_positive_u64_env(
            "SESSION_CLEANUP_INTERVAL_SECS",
            "0",
            "must be a valid u64 integer",
        )
        .expect_err("interval should be rejected");
        assert_invalid_env(
            error,
            "SESSION_CLEANUP_INTERVAL_SECS",
            "must be greater than 0",
        );
    }

    #[test]
    fn parses_valid_data_encryption_key() {
        let expected = [11_u8; 32];
        let encoded = utils::b64::b64u_encode(expected);

        let key = parse_data_encryption_key(&encoded).expect("key should parse");

        assert_eq!(key, expected);
    }

    #[test]
    fn rejects_invalid_data_encryption_key_base64() {
        let error = parse_data_encryption_key("%%%").expect_err("key should be rejected");
        assert_invalid_env(
            error,
            "DATA_ENCRYPTION_KEY",
            "must be base64url without padding",
        );
    }

    #[test]
    fn rejects_short_data_encryption_key() {
        let encoded = utils::b64::b64u_encode([11_u8; 31]);
        let error =
            parse_data_encryption_key(&encoded).expect_err("short key should be rejected");
        assert_invalid_env(
            error,
            "DATA_ENCRYPTION_KEY",
            "must decode to exactly 32 bytes",
        );
    }

    #[test]
    fn parses_keyring_json_and_preserves_key_ids() {
        let encoded = utils::b64::b64u_encode([11_u8; 32]);
        let keys = parse_data_encryption_keys_json(&format!(
            r#"[{{"key_id":"legacy_data_key","key":"{encoded}"}},{{"key_id":"active_data_key","key":"{encoded}"}}]"#
        ))
        .expect("keyring json should parse");

        assert_eq!(keys.len(), 2);
        assert_eq!(keys[0].key_id.to_string(), "legacy_data_key");
        assert_eq!(keys[1].key_id.to_string(), "active_data_key");
    }

    #[test]
    fn rejects_duplicate_key_ids_in_keyring_json() {
        let encoded = utils::b64::b64u_encode([11_u8; 32]);
        let error = parse_data_encryption_keys_json(&format!(
            r#"[{{"key_id":"dup","key":"{encoded}"}},{{"key_id":"dup","key":"{encoded}"}}]"#
        ))
        .expect_err("duplicate key ids should be rejected");

        assert_invalid_env(
            error,
            "DATA_ENCRYPTION_KEYS_JSON",
            "must not contain duplicate key_id values",
        );
    }

    #[test]
    fn compatibility_path_loads_legacy_single_key_config() {
        let _lock = env_lock();
        let encoded = utils::b64::b64u_encode([11_u8; 32]);
        unsafe {
            std::env::remove_var("DATA_ENCRYPTION_KEYS_JSON");
            std::env::remove_var("ACTIVE_DATA_KEY_ID");
            std::env::set_var("DATA_ENCRYPTION_KEY", &encoded);
        }

        let (keys, active_key_id) =
            data_encryption_config_from_env().expect("legacy config should load");

        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].key_id.to_string(), "legacy_data_key");
        assert_eq!(active_key_id.to_string(), "legacy_data_key");
        unsafe {
            std::env::remove_var("DATA_ENCRYPTION_KEY");
        }
    }

    #[test]
    fn parses_email_list_and_deduplicates_entries() {
        let emails = parse_email_list(
            "SENSITIVE_OPERATOR_EMAILS",
            " reader@example.com,operator@example.com,reader@example.com ",
        )
        .expect("email list should parse");

        assert_eq!(emails.len(), 2);
        assert_eq!(emails[0].to_string(), "reader@example.com");
        assert_eq!(emails[1].to_string(), "operator@example.com");
    }

    #[test]
    fn rejects_invalid_email_list_entries() {
        let error =
            parse_email_list("SENSITIVE_READER_EMAILS", "ok@example.com, not-an-email")
                .expect_err("email list should be rejected");

        assert_invalid_env(
            error,
            "SENSITIVE_READER_EMAILS",
            "must be a comma-separated list of valid email addresses",
        );
    }

    #[test]
    fn parses_key_id_list_and_deduplicates_entries() {
        let key_ids = parse_key_id_list("DISABLED_DATA_KEY_IDS", "legacy,active,legacy")
            .expect("key id list should parse");

        assert_eq!(key_ids.len(), 2);
        assert_eq!(key_ids[0].to_string(), "legacy");
        assert_eq!(key_ids[1].to_string(), "active");
    }

    #[test]
    fn rejects_zero_rotation_batch_size() {
        let error = parse_positive_usize_env("ENCRYPTION_ROTATION_BATCH_SIZE", "0")
            .expect_err("batch size should be rejected");
        assert_invalid_env(
            error,
            "ENCRYPTION_ROTATION_BATCH_SIZE",
            "must be greater than 0",
        );
    }

    #[test]
    fn defaults_provider_stub_port_from_http_port() {
        let _lock = env_lock();
        unsafe {
            std::env::remove_var("SENSITIVE_PROVIDER_PORT");
        }
        let port = provider_stub_port_from_env(3002).expect("default port");
        assert_eq!(port, 4002);
    }

    #[test]
    fn defaults_provider_mode_to_stub() {
        let _lock = env_lock();
        unsafe {
            std::env::remove_var("SENSITIVE_PROVIDER_MODE");
        }
        let mode = provider_mode_from_env().expect("default mode");
        assert_eq!(mode, SensitiveProviderRuntimeMode::Stub);
    }

    #[test]
    fn parses_provider_mode() {
        let _lock = env_lock();
        unsafe {
            std::env::set_var("SENSITIVE_PROVIDER_MODE", "sandbox_http");
        }
        let mode = provider_mode_from_env().expect("mode should parse");
        assert_eq!(mode, SensitiveProviderRuntimeMode::SandboxHttp);
        unsafe {
            std::env::remove_var("SENSITIVE_PROVIDER_MODE");
        }
    }

    #[test]
    fn rejects_invalid_provider_mode() {
        let _lock = env_lock();
        unsafe {
            std::env::set_var("SENSITIVE_PROVIDER_MODE", "broken");
        }
        let error = provider_mode_from_env().expect_err("mode should be rejected");
        assert_invalid_env(
            error,
            "SENSITIVE_PROVIDER_MODE",
            "must be one of: stub, sandbox_http",
        );
        unsafe {
            std::env::remove_var("SENSITIVE_PROVIDER_MODE");
        }
    }

    #[test]
    fn parses_provider_stub_failure_mode() {
        let _lock = env_lock();
        unsafe {
            std::env::set_var("SENSITIVE_PROVIDER_STUB_FAILURE_MODE", "unauthorized_once");
        }
        let mode = provider_stub_failure_mode_from_env().expect("mode should parse");
        assert_eq!(mode, SensitiveProviderStubFailureMode::UnauthorizedOnce);
        unsafe {
            std::env::remove_var("SENSITIVE_PROVIDER_STUB_FAILURE_MODE");
        }
    }

    #[test]
    fn rejects_invalid_provider_stub_failure_mode() {
        let _lock = env_lock();
        unsafe {
            std::env::set_var("SENSITIVE_PROVIDER_STUB_FAILURE_MODE", "broken");
        }
        let error =
            provider_stub_failure_mode_from_env().expect_err("mode should be rejected");
        assert_invalid_env(
            error,
            "SENSITIVE_PROVIDER_STUB_FAILURE_MODE",
            "must be one of: healthy, unauthorized_once, malformed_page, rate_limited",
        );
        unsafe {
            std::env::remove_var("SENSITIVE_PROVIDER_STUB_FAILURE_MODE");
        }
    }
}
