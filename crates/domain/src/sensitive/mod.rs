moddef::moddef!(mod { error });

pub use error::{Error, Result};

use bon::Builder;
use nutype::nutype;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::user;

fn is_valid_last4(value: &str) -> bool {
    value.len() == 4 && value.chars().all(|value| value.is_ascii_digit())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl From<uuid::Uuid> for Id {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<Id> for uuid::Uuid {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl AsRef<uuid::Uuid> for Id {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.0
    }
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 128),
    derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)
)]
pub struct ExternalId(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 80),
    derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)
)]
pub struct Label(String);

#[nutype(
    sanitize(trim),
    validate(predicate = is_valid_last4),
    derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)
)]
pub struct Last4(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 120),
    derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)
)]
pub struct DetailText(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 64),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Display,
        Serialize,
        Deserialize
    )
)]
pub struct KeyId(String);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Provider {
    SyntheticSecureFeed,
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)
)]
pub struct SyncCursor(String);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum ProviderMode {
    LocalStub,
    SandboxHttp,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum ProviderAuthMode {
    StubIssuedToken,
    ClientCredentials,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum SyncOutcome {
    Success,
    Failed,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum FetchOutcome {
    Success,
    Failed,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum TokenStrategy {
    CachedToken,
    RefreshedToken,
    RetryAfterUnauthorized,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum RemoteErrorCategory {
    Configuration,
    Unauthorized,
    Forbidden,
    RateLimited,
    MalformedPayload,
    Timeout,
    ServerError,
    Transport,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum AccessCapability {
    AuthorizedRecordRead,
    TokenStatusRead,
    AccessAuditRead,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum CipherKeyStatus {
    Active,
    ReadOnlyLegacy,
    Disabled,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumString,
    AsRefStr,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum AccessOutcome {
    Allowed,
    Denied,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizedFields {
    pub subject_name: DetailText,
    pub classification: DetailText,
    pub note: DetailText,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct Record {
    pub external_id: ExternalId,
    pub label: Label,
    pub last4: Last4,
    pub authorized: AuthorizedFields,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct AuthorizedRecord {
    pub id: Id,
    pub label: Label,
    pub last4: Last4,
    pub authorized: AuthorizedFields,
    pub synced_at: std::time::SystemTime,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct SyncRun {
    pub provider: Provider,
    pub outcome: SyncOutcome,
    pub records_seen: u32,
    pub records_upserted: u32,
    pub detail: DetailText,
    pub started_at: std::time::SystemTime,
    pub finished_at: std::time::SystemTime,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct TokenStatus {
    pub provider: Provider,
    pub expires_at: std::time::SystemTime,
    pub refreshed_at: std::time::SystemTime,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct ConfiguredKey {
    pub key_id: KeyId,
    pub status: CipherKeyStatus,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct KeyedCiphertextCount {
    pub key_id: KeyId,
    pub count: u32,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct IntegrationState {
    pub provider: Provider,
    pub mode: ProviderMode,
    pub endpoint: DetailText,
    pub auth_mode: Option<ProviderAuthMode>,
    pub cursor: Option<SyncCursor>,
    pub last_fetch_outcome: FetchOutcome,
    pub token_strategy: TokenStrategy,
    pub last_error_category: Option<RemoteErrorCategory>,
    pub last_auth_outcome: Option<FetchOutcome>,
    pub last_remote_status_code: Option<u16>,
    pub retry_backoff_secs: Option<u32>,
    pub last_successful_mode: Option<ProviderMode>,
    pub last_successful_fetch_at: Option<std::time::SystemTime>,
    pub last_attempted_fetch_at: std::time::SystemTime,
    pub failure_count: u32,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct AccessGrant {
    pub user_id: user::Id,
    pub capability: AccessCapability,
    pub granted_at: std::time::SystemTime,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct AccessEvent {
    pub user_id: Option<user::Id>,
    pub capability: AccessCapability,
    pub record_id: Option<Id>,
    pub outcome: AccessOutcome,
    pub detail: DetailText,
    pub occurred_at: std::time::SystemTime,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum RotationOutcome {
    Success,
    Failed,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct KeyRotationRun {
    pub active_key_id: KeyId,
    pub outcome: RotationOutcome,
    pub rows_scanned: u32,
    pub rows_rewrapped: u32,
    pub rows_already_current: u32,
    pub rows_failed: u32,
    pub detail: DetailText,
    pub started_at: std::time::SystemTime,
    pub finished_at: std::time::SystemTime,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct KeyCustodyState {
    pub active_key_id: KeyId,
    pub configured_keys: Vec<ConfiguredKey>,
    pub token_counts: Vec<KeyedCiphertextCount>,
    pub record_counts: Vec<KeyedCiphertextCount>,
    pub stale_token_count: u32,
    pub stale_record_count: u32,
    pub last_rotation_run: Option<KeyRotationRun>,
}

#[cfg(test)]
mod tests {
    use super::{
        AccessCapability, AccessOutcome, CipherKeyStatus, KeyId, Last4, Provider,
        ProviderAuthMode, ProviderMode, RemoteErrorCategory, RotationOutcome,
        TokenStrategy,
    };

    #[test]
    fn rejects_invalid_last4() {
        assert!(Last4::try_new("12").is_err());
        assert!(Last4::try_new("12ab").is_err());
    }

    #[test]
    fn provider_round_trips_snake_case_strings() {
        let parsed = "synthetic_secure_feed"
            .parse::<Provider>()
            .expect("provider should parse");
        assert_eq!(parsed.as_ref(), "synthetic_secure_feed");
    }

    #[test]
    fn access_capability_round_trips_snake_case_strings() {
        let parsed = "authorized_record_read"
            .parse::<AccessCapability>()
            .expect("capability should parse");
        assert_eq!(parsed.as_ref(), "authorized_record_read");
    }

    #[test]
    fn access_outcome_round_trips_snake_case_strings() {
        let parsed = "denied"
            .parse::<AccessOutcome>()
            .expect("outcome should parse");
        assert_eq!(parsed.as_ref(), "denied");
    }

    #[test]
    fn provider_mode_round_trips_snake_case_strings() {
        let parsed = "local_stub"
            .parse::<ProviderMode>()
            .expect("provider mode should parse");
        assert_eq!(parsed.as_ref(), "local_stub");
    }

    #[test]
    fn provider_auth_mode_round_trips_snake_case_strings() {
        let parsed = "client_credentials"
            .parse::<ProviderAuthMode>()
            .expect("provider auth mode should parse");
        assert_eq!(parsed.as_ref(), "client_credentials");
    }

    #[test]
    fn token_strategy_round_trips_snake_case_strings() {
        let parsed = "retry_after_unauthorized"
            .parse::<TokenStrategy>()
            .expect("token strategy should parse");
        assert_eq!(parsed.as_ref(), "retry_after_unauthorized");
    }

    #[test]
    fn remote_error_category_round_trips_snake_case_strings() {
        let parsed = "server_error"
            .parse::<RemoteErrorCategory>()
            .expect("error category should parse");
        assert_eq!(parsed.as_ref(), "server_error");
    }

    #[test]
    fn key_id_rejects_blank_values() {
        assert!(KeyId::try_new("   ").is_err());
    }

    #[test]
    fn custody_enums_round_trip_snake_case_strings() {
        let status = "read_only_legacy"
            .parse::<CipherKeyStatus>()
            .expect("key status should parse");
        let outcome = "failed"
            .parse::<RotationOutcome>()
            .expect("rotation outcome should parse");

        assert_eq!(status.as_ref(), "read_only_legacy");
        assert_eq!(outcome.as_ref(), "failed");
    }
}
