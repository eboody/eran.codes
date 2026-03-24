mod access;
mod clock;
mod mapping;
mod record_store;
mod rotation;
mod snapshot;
mod token_store;

use std::time::SystemTime;

use app::sensitive::{self, RepositoryOperation};
use async_trait::async_trait;
use domain::{sensitive as sensitive_domain, user as user_domain};
use secrecy::{ExposeSecret, SecretString};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row, types::time::OffsetDateTime};

use crate::crypto::{Keyring, SealedValue};

pub use clock::SystemClock;

pub struct Repository {
    pg: PgPool,
    crypto: Keyring,
}

impl Repository {
    pub fn new(pg: PgPool, crypto: Keyring) -> Self {
        Self { pg, crypto }
    }
}

#[async_trait]
impl sensitive::Repository for Repository {
    async fn load_snapshot(&self) -> sensitive::Result<sensitive::StoredSnapshot> {
        self.load_snapshot_impl().await
    }

    async fn load_authorized_record(
        &self,
        record_id: &sensitive_domain::Id,
    ) -> sensitive::Result<Option<sensitive_domain::AuthorizedRecord>> {
        self.load_authorized_record_impl(record_id).await
    }

    async fn load_integration_state(
        &self,
        provider: sensitive_domain::Provider,
    ) -> sensitive::Result<Option<sensitive_domain::IntegrationState>> {
        self.load_integration_state_impl(provider).await
    }

    async fn load_key_custody(
        &self,
    ) -> sensitive::Result<sensitive_domain::KeyCustodyState> {
        self.load_key_custody_impl().await
    }

    async fn load_access_grants(
        &self,
        user_id: &user_domain::Id,
    ) -> sensitive::Result<Vec<sensitive_domain::AccessGrant>> {
        self.load_access_grants_impl(user_id).await
    }

    async fn load_token(
        &self,
        provider: sensitive_domain::Provider,
    ) -> sensitive::Result<Option<sensitive::ProviderToken>> {
        self.load_token_impl(provider).await
    }

    async fn upsert_token(
        &self,
        token: &sensitive::ProviderToken,
    ) -> sensitive::Result<()> {
        self.upsert_token_impl(token).await
    }

    async fn upsert_records(
        &self,
        records: &[sensitive_domain::Record],
        synced_at: SystemTime,
    ) -> sensitive::Result<usize> {
        self.upsert_records_impl(records, synced_at).await
    }

    async fn upsert_integration_state(
        &self,
        state: &sensitive_domain::IntegrationState,
    ) -> sensitive::Result<()> {
        self.upsert_integration_state_impl(state).await
    }

    async fn rotate_ciphertext_to_active_key(
        &self,
        limit: usize,
        rotated_at: SystemTime,
    ) -> sensitive::Result<sensitive::KeyRotationProgress> {
        self.rotate_ciphertext_to_active_key_impl(limit, rotated_at)
            .await
    }

    async fn record_sync_run(
        &self,
        run: &sensitive_domain::SyncRun,
    ) -> sensitive::Result<()> {
        self.record_sync_run_impl(run).await
    }

    async fn record_key_rotation_run(
        &self,
        run: &sensitive_domain::KeyRotationRun,
    ) -> sensitive::Result<()> {
        self.record_key_rotation_run_impl(run).await
    }

    async fn upsert_access_grants(
        &self,
        user_id: &user_domain::Id,
        capabilities: &[sensitive_domain::AccessCapability],
        granted_at: SystemTime,
    ) -> sensitive::Result<()> {
        self.upsert_access_grants_impl(user_id, capabilities, granted_at)
            .await
    }

    async fn record_access_event(
        &self,
        event: &sensitive_domain::AccessEvent,
    ) -> sensitive::Result<()> {
        self.record_access_event_impl(event).await
    }

    async fn list_recent_access_events(
        &self,
        limit: usize,
    ) -> sensitive::Result<Vec<sensitive_domain::AccessEvent>> {
        self.list_recent_access_events_impl(limit).await
    }
}
