use snafu::prelude::*;
use strum_macros::{AsRefStr, Display};

pub type Result<T> = core::result::Result<T, Error>;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    Domain { source: domain::sensitive::Error },
    #[snafu(display("{source}"))]
    Repository { source: RepositoryError },
    #[snafu(display(
        "sensitive provider request failed while {operation} ({kind}): {source}"
    ))]
    Provider {
        operation: ProviderOperation,
        kind: ProviderFailureKind,
        source: BoxError,
    },
    #[snafu(display("sensitive repository is missing a provider token after refresh"))]
    MissingProviderToken,
    #[snafu(display("invalid stored provider: {provider}"))]
    InvalidStoredProvider { provider: String },
    #[snafu(display("invalid stored provider mode: {mode}"))]
    InvalidStoredProviderMode { mode: String },
    #[snafu(display("invalid stored sync cursor: {cursor}"))]
    InvalidStoredSyncCursor { cursor: String },
    #[snafu(display("invalid stored sync outcome: {outcome}"))]
    InvalidStoredSyncOutcome { outcome: String },
    #[snafu(display("invalid stored fetch outcome: {outcome}"))]
    InvalidStoredFetchOutcome { outcome: String },
    #[snafu(display("invalid stored token strategy: {strategy}"))]
    InvalidStoredTokenStrategy { strategy: String },
    #[snafu(display("invalid stored remote error category: {category}"))]
    InvalidStoredRemoteErrorCategory { category: String },
    #[snafu(display("invalid stored failure count: {failure_count}"))]
    InvalidStoredFailureCount { failure_count: i64 },
    #[snafu(display("invalid stored encryption key id: {key_id}"))]
    InvalidStoredKeyId { key_id: String },
    #[snafu(display("invalid stored rotation outcome: {outcome}"))]
    InvalidStoredRotationOutcome { outcome: String },
    #[snafu(display("invalid stored rotation count for {field}: {value}"))]
    InvalidStoredRotationCount { field: &'static str, value: i64 },
    #[snafu(display("invalid stored access capability: {capability}"))]
    InvalidStoredAccessCapability { capability: String },
    #[snafu(display("invalid stored access outcome: {outcome}"))]
    InvalidStoredAccessOutcome { outcome: String },
}

#[derive(Clone, Copy, Debug, Display)]
pub enum RepositoryOperation {
    #[strum(serialize = "load sensitive snapshot")]
    LoadSnapshot,
    #[strum(serialize = "load authorized sensitive record")]
    LoadAuthorizedRecord,
    #[strum(serialize = "load integration sync state")]
    LoadIntegrationState,
    #[strum(serialize = "load key custody state")]
    LoadKeyCustody,
    #[strum(serialize = "load sensitive access grants")]
    LoadAccessGrants,
    #[strum(serialize = "load provider token")]
    LoadToken,
    #[strum(serialize = "upsert provider token")]
    UpsertToken,
    #[strum(serialize = "upsert sensitive records")]
    UpsertRecords,
    #[strum(serialize = "upsert integration sync state")]
    UpsertIntegrationState,
    #[strum(serialize = "rotate sensitive ciphertext to the active key")]
    RotateCiphertext,
    #[strum(serialize = "upsert sensitive access grants")]
    UpsertAccessGrants,
    #[strum(serialize = "record sync run")]
    RecordSyncRun,
    #[strum(serialize = "record key rotation run")]
    RecordKeyRotationRun,
    #[strum(serialize = "record sensitive access event")]
    RecordAccessEvent,
    #[strum(serialize = "list sensitive access events")]
    ListAccessEvents,
}

#[derive(Clone, Copy, Debug, Display)]
pub enum ProviderOperation {
    #[strum(serialize = "refresh token")]
    RefreshToken,
    #[strum(serialize = "fetch provider records")]
    FetchRecords,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum ProviderFailureKind {
    Unauthorized,
    RateLimited,
    MalformedPayload,
    Transport,
}

#[derive(Debug, Snafu)]
pub enum RepositoryError {
    #[snafu(display("sensitive repository query failed while {operation}: {source}"))]
    Query {
        operation: RepositoryOperation,
        source: BoxError,
    },
    #[snafu(display("failed to decode sensitive label: {source}"))]
    DecodeLabel {
        source: domain::sensitive::LabelError,
    },
    #[snafu(display("failed to decode sensitive last4: {source}"))]
    DecodeLast4 {
        source: domain::sensitive::Last4Error,
    },
    #[snafu(display("failed to decode sensitive external id: {source}"))]
    DecodeExternalId {
        source: domain::sensitive::ExternalIdError,
    },
    #[snafu(display("failed to decode sensitive detail text: {source}"))]
    DecodeDetailText {
        source: domain::sensitive::DetailTextError,
    },
    #[snafu(display("failed to encode authorized fields for storage: {source}"))]
    EncodeAuthorizedFields { source: BoxError },
    #[snafu(display("failed to decode authorized fields from storage: {source}"))]
    DecodeAuthorizedFields { source: BoxError },
    #[snafu(display("failed to encrypt provider token for storage: {source}"))]
    EncryptToken { source: BoxError },
    #[snafu(display("failed to decrypt provider token from storage: {source}"))]
    DecryptToken { source: BoxError },
    #[snafu(display("failed to encrypt sensitive record for storage: {source}"))]
    EncryptRecord { source: BoxError },
    #[snafu(display("failed to decrypt sensitive record from storage: {source}"))]
    DecryptRecord { source: BoxError },
}

fn box_error(source: impl std::error::Error + Send + Sync + 'static) -> BoxError {
    Box::new(source)
}

impl From<domain::sensitive::Error> for Error {
    fn from(source: domain::sensitive::Error) -> Self {
        Self::Domain { source }
    }
}

impl Error {
    pub fn query_repository(
        operation: RepositoryOperation,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Repository {
            source: RepositoryError::Query {
                operation,
                source: box_error(source),
            },
        }
    }

    pub fn provider_request(
        operation: ProviderOperation,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Provider {
            operation,
            kind: ProviderFailureKind::Transport,
            source: box_error(source),
        }
    }

    pub fn provider_failure(
        operation: ProviderOperation,
        kind: ProviderFailureKind,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Provider {
            operation,
            kind,
            source: box_error(source),
        }
    }

    pub fn decode_label(source: domain::sensitive::LabelError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeLabel { source },
        }
    }

    pub fn decode_last4(source: domain::sensitive::Last4Error) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeLast4 { source },
        }
    }

    pub fn decode_external_id(source: domain::sensitive::ExternalIdError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeExternalId { source },
        }
    }

    pub fn decode_detail_text(source: domain::sensitive::DetailTextError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeDetailText { source },
        }
    }

    pub fn encode_authorized_fields(
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Repository {
            source: RepositoryError::EncodeAuthorizedFields {
                source: box_error(source),
            },
        }
    }

    pub fn decode_authorized_fields(
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeAuthorizedFields {
                source: box_error(source),
            },
        }
    }

    pub fn encrypt_token(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Repository {
            source: RepositoryError::EncryptToken {
                source: box_error(source),
            },
        }
    }

    pub fn decrypt_token(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Repository {
            source: RepositoryError::DecryptToken {
                source: box_error(source),
            },
        }
    }

    pub fn encrypt_record(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Repository {
            source: RepositoryError::EncryptRecord {
                source: box_error(source),
            },
        }
    }

    pub fn decrypt_record(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Repository {
            source: RepositoryError::DecryptRecord {
                source: box_error(source),
            },
        }
    }
}
