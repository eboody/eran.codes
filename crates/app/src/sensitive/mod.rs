mod bootstrap;
mod disabled;
#[path = "error.rs"]
pub mod failure;
mod rotation;
mod snapshot;
mod sync;

#[cfg(test)]
mod tests;

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use bon::{Builder, bon};
use secrecy::SecretString;
use strum_macros::{AsRefStr, Display};

use domain::{sensitive, user};
pub use failure::{ProviderFailureKind, ProviderOperation, RepositoryOperation, Result};

const PROVIDER: sensitive::Provider = sensitive::Provider::SyntheticSecureFeed;
const TOKEN_REFRESH_SKEW: Duration = Duration::from_secs(30);
const ACCESS_EVENT_LIMIT: usize = 6;

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct AuthenticatedViewer {
    pub user_id: user::Id,
    pub email: user::Email,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Viewer {
    Guest,
    Authenticated(AuthenticatedViewer),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum ViewerTier {
    Guest,
    Authenticated,
    SensitiveReader,
    SensitiveOperator,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct ViewerState {
    pub tier: ViewerTier,
    pub capabilities: Vec<sensitive::AccessCapability>,
}

impl ViewerState {
    pub fn guest() -> Self {
        Self::builder()
            .tier(ViewerTier::Guest)
            .capabilities(Vec::new())
            .build()
    }

    pub fn authenticated(capabilities: Vec<sensitive::AccessCapability>) -> Self {
        Self::builder()
            .tier(snapshot::viewer_tier_for_capabilities(&capabilities))
            .capabilities(capabilities)
            .build()
    }

    pub fn is_authenticated(&self) -> bool {
        self.tier != ViewerTier::Guest
    }

    pub fn has_capability(&self, capability: sensitive::AccessCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn allows_authorized_record(&self) -> bool {
        self.has_capability(sensitive::AccessCapability::AuthorizedRecordRead)
    }

    pub fn allows_token_status(&self) -> bool {
        self.has_capability(sensitive::AccessCapability::TokenStatusRead)
    }

    pub fn allows_access_audit(&self) -> bool {
        self.has_capability(sensitive::AccessCapability::AccessAuditRead)
    }
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct CiphertextEvidence {
    pub key_id: sensitive::KeyId,
    pub preview: String,
    pub bytes: usize,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct RecordProof {
    pub id: sensitive::Id,
    pub label: sensitive::Label,
    pub last4: sensitive::Last4,
    pub synced_at: SystemTime,
    pub ciphertext: CiphertextEvidence,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct TokenProof {
    pub status: sensitive::TokenStatus,
    pub ciphertext: CiphertextEvidence,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct ProviderBoundaryMeta {
    pub mode: sensitive::ProviderMode,
    pub endpoint: sensitive::DetailText,
    pub auth_mode: Option<sensitive::ProviderAuthMode>,
    pub retry_backoff_secs: Option<u32>,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct ProviderRecords {
    pub records: Vec<sensitive::Record>,
    pub cursor: Option<sensitive::SyncCursor>,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct StoredSnapshot {
    pub token: Option<TokenProof>,
    pub latest_sync: Option<sensitive::SyncRun>,
    pub integration_state: Option<sensitive::IntegrationState>,
    pub records: Vec<RecordProof>,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct KeyRotationProgress {
    pub active_key_id: sensitive::KeyId,
    pub rows_scanned: u32,
    pub rows_rewrapped: u32,
    pub rows_already_current: u32,
    pub rows_failed: u32,
    pub detail: sensitive::DetailText,
}

#[derive(Clone, Debug, Builder)]
pub struct ProviderToken {
    pub status: sensitive::TokenStatus,
    pub access_token: SecretString,
}

#[derive(Clone, Debug, Builder, PartialEq, Eq)]
pub struct Snapshot {
    pub viewer: ViewerState,
    pub token: Option<TokenProof>,
    pub latest_sync: Option<sensitive::SyncRun>,
    pub integration_state: Option<sensitive::IntegrationState>,
    pub key_custody: Option<sensitive::KeyCustodyState>,
    pub records: Vec<RecordProof>,
    pub authorized_record: Option<sensitive::AuthorizedRecord>,
    pub access_events: Vec<sensitive::AccessEvent>,
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn load_snapshot(&self) -> Result<StoredSnapshot>;
    async fn load_authorized_record(
        &self,
        record_id: &sensitive::Id,
    ) -> Result<Option<sensitive::AuthorizedRecord>>;
    async fn load_integration_state(
        &self,
        provider: sensitive::Provider,
    ) -> Result<Option<sensitive::IntegrationState>>;
    async fn load_key_custody(&self) -> Result<sensitive::KeyCustodyState>;
    async fn load_access_grants(
        &self,
        user_id: &user::Id,
    ) -> Result<Vec<sensitive::AccessGrant>>;
    async fn load_token(
        &self,
        provider: sensitive::Provider,
    ) -> Result<Option<ProviderToken>>;
    async fn upsert_token(&self, token: &ProviderToken) -> Result<()>;
    async fn upsert_records(
        &self,
        records: &[sensitive::Record],
        synced_at: SystemTime,
    ) -> Result<usize>;
    async fn upsert_integration_state(
        &self,
        state: &sensitive::IntegrationState,
    ) -> Result<()>;
    async fn rotate_ciphertext_to_active_key(
        &self,
        limit: usize,
        rotated_at: SystemTime,
    ) -> Result<KeyRotationProgress>;
    async fn record_sync_run(&self, run: &sensitive::SyncRun) -> Result<()>;
    async fn record_key_rotation_run(&self, run: &sensitive::KeyRotationRun) -> Result<()>;
    async fn upsert_access_grants(
        &self,
        user_id: &user::Id,
        capabilities: &[sensitive::AccessCapability],
        granted_at: SystemTime,
    ) -> Result<()>;
    async fn record_access_event(&self, event: &sensitive::AccessEvent) -> Result<()>;
    async fn list_recent_access_events(
        &self,
        limit: usize,
    ) -> Result<Vec<sensitive::AccessEvent>>;
}

#[async_trait]
pub trait ProviderClient: Send + Sync {
    fn boundary_meta(&self, provider: sensitive::Provider) -> ProviderBoundaryMeta;

    async fn refresh_token(
        &self,
        provider: sensitive::Provider,
        now: SystemTime,
        current_token: Option<&SecretString>,
    ) -> Result<ProviderToken>;

    async fn fetch_records(
        &self,
        provider: sensitive::Provider,
        token: &ProviderToken,
        cursor: Option<&sensitive::SyncCursor>,
        now: SystemTime,
    ) -> Result<ProviderRecords>;
}

pub trait Clock: Send + Sync {
    fn now(&self) -> SystemTime;
}

#[derive(Clone, Debug, Default)]
pub struct BootstrapGrants {
    reader_emails: Vec<user::Email>,
    operator_emails: Vec<user::Email>,
}

#[derive(Clone)]
pub struct Service {
    repo: Arc<dyn Repository>,
    provider: Arc<dyn ProviderClient>,
    clock: Arc<dyn Clock>,
    bootstrap: Arc<BootstrapGrants>,
}

impl Service {
    fn new(
        repo: Arc<dyn Repository>,
        provider: Arc<dyn ProviderClient>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            repo,
            provider,
            clock,
            bootstrap: Arc::new(BootstrapGrants::default()),
        }
    }

    pub fn with_bootstrap_grants(self, bootstrap: BootstrapGrants) -> Self {
        Self {
            repo: self.repo,
            provider: self.provider,
            clock: self.clock,
            bootstrap: Arc::new(bootstrap),
        }
    }

    pub fn disabled() -> Self {
        Self::new(
            Arc::new(disabled::Repository),
            Arc::new(disabled::Provider),
            Arc::new(disabled::Clock),
        )
    }

    pub fn bootstrap_grants(&self) -> &BootstrapGrants {
        &self.bootstrap
    }
}

#[bon]
impl Service {
    #[builder]
    pub fn builder(
        #[builder(setters(name = with_repo))] repo: Arc<dyn Repository>,
        #[builder(setters(name = with_provider))] provider: Arc<dyn ProviderClient>,
        #[builder(setters(name = with_clock))] clock: Arc<dyn Clock>,
    ) -> Self {
        Self::new(repo, provider, clock)
    }
}

fn bounded_detail_text(
    message: impl Into<String>,
    fallback: &'static str,
) -> sensitive::DetailText {
    let mut detail = message.into().trim().chars().take(120).collect::<String>();
    if detail.is_empty() {
        detail = fallback.to_string();
    }

    sensitive::DetailText::try_new(detail)
        .expect("detail text should be normalized to a valid bounded string")
}
