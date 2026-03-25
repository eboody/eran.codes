use super::*;

impl Service {
    #[tracing::instrument(skip(self))]
    pub async fn snapshot(&self, viewer: Viewer) -> Result<Snapshot> {
        let stored = self.repo.load_snapshot().await?;
        let viewer = self.resolve_viewer_access(viewer).await?;
        let visible = VisibleStoredSnapshot::from_stored(&viewer.state, stored);
        let authorized_access = self
            .evaluate_authorized_record_access(&viewer, visible.first_record_id())
            .await?;
        self.record_authorized_access(&authorized_access).await?;
        let key_custody = self.load_visible_key_custody(&viewer.state).await?;
        let access_events = self.load_visible_access_events(&viewer.state).await?;

        Ok(Snapshot::builder()
            .viewer(viewer.state)
            .maybe_token(visible.token)
            .maybe_latest_sync(visible.latest_sync)
            .maybe_integration_state(visible.integration_state)
            .maybe_key_custody(key_custody)
            .records(visible.records)
            .maybe_authorized_record(authorized_access.record)
            .access_events(access_events)
            .build())
    }

    async fn resolve_viewer_access(&self, viewer: Viewer) -> Result<ResolvedViewer> {
        let state = match &viewer {
            Viewer::Guest => ViewerState::guest(),
            Viewer::Authenticated(viewer) => {
                self.reconcile_bootstrap_grants_for_user(viewer.user_id, &viewer.email)
                    .await?;
                let grants = self.repo.load_access_grants(&viewer.user_id).await?;
                let capabilities = sorted_capabilities(
                    grants.into_iter().map(|grant| grant.capability).collect(),
                );
                ViewerState::authenticated(capabilities)
            }
        };

        Ok(ResolvedViewer { viewer, state })
    }

    async fn evaluate_authorized_record_access(
        &self,
        viewer: &ResolvedViewer,
        record_id: Option<sensitive::Id>,
    ) -> Result<AuthorizedRecordAccess> {
        let Some(record_id) = record_id else {
            return Ok(AuthorizedRecordAccess::empty());
        };

        if viewer.state.allows_authorized_record() {
            let record = match self.repo.load_authorized_record(&record_id).await {
                Ok(record) => record,
                Err(error) if authorized_record_requires_denied_fallback(&error) => {
                    tracing::warn!(
                        target: "demo.sensitive",
                        ?error,
                        record_id = %record_id.as_ref(),
                        "authorized record could not be decrypted under the configured keyring",
                    );
                    None
                }
                Err(error) => return Err(error),
            };
            let (outcome, detail) = if record.is_some() {
                (
                    sensitive::AccessOutcome::Allowed,
                    "authorized record decrypted under persisted grant",
                )
            } else {
                (
                    sensitive::AccessOutcome::Denied,
                    "authorized record was not available at decrypt time",
                )
            };

            return Ok(AuthorizedRecordAccess::new(
                record,
                Some(viewer.access_event(
                    outcome,
                    Some(record_id),
                    access_detail_text(detail),
                    self.clock.now(),
                )),
            ));
        }

        Ok(AuthorizedRecordAccess::new(
            None,
            Some(viewer.access_event(
                sensitive::AccessOutcome::Denied,
                Some(record_id),
                access_detail_text(access_denial_detail(&viewer.viewer)),
                self.clock.now(),
            )),
        ))
    }

    async fn record_authorized_access(
        &self,
        access: &AuthorizedRecordAccess,
    ) -> Result<()> {
        match &access.event {
            Some(event) => self.repo.record_access_event(event).await,
            None => Ok(()),
        }
    }

    async fn load_visible_key_custody(
        &self,
        viewer_state: &ViewerState,
    ) -> Result<Option<sensitive::KeyCustodyState>> {
        if viewer_state.allows_token_status() {
            return self.repo.load_key_custody().await.map(Some);
        }
        Ok(None)
    }

    async fn load_visible_access_events(
        &self,
        viewer_state: &ViewerState,
    ) -> Result<Vec<sensitive::AccessEvent>> {
        if viewer_state.allows_access_audit() {
            return self
                .repo
                .list_recent_access_events(ACCESS_EVENT_LIMIT)
                .await;
        }
        Ok(Vec::new())
    }
}

struct ResolvedViewer {
    viewer: Viewer,
    state: ViewerState,
}

impl ResolvedViewer {
    fn access_event(
        &self,
        outcome: sensitive::AccessOutcome,
        record_id: Option<sensitive::Id>,
        detail: sensitive::DetailText,
        occurred_at: SystemTime,
    ) -> sensitive::AccessEvent {
        sensitive::AccessEvent::builder()
            .maybe_user_id(match &self.viewer {
                Viewer::Guest => None,
                Viewer::Authenticated(viewer) => Some(viewer.user_id),
            })
            .capability(sensitive::AccessCapability::AuthorizedRecordRead)
            .maybe_record_id(record_id)
            .outcome(outcome)
            .detail(detail)
            .occurred_at(occurred_at)
            .build()
    }
}

struct VisibleStoredSnapshot {
    token: Option<TokenProof>,
    latest_sync: Option<sensitive::SyncRun>,
    integration_state: Option<sensitive::IntegrationState>,
    records: Vec<RecordProof>,
}

impl VisibleStoredSnapshot {
    fn from_stored(viewer_state: &ViewerState, stored: StoredSnapshot) -> Self {
        let operator_view = viewer_state.allows_token_status();
        let StoredSnapshot {
            token,
            latest_sync,
            integration_state,
            records,
        } = stored;

        Self {
            token: if operator_view { token } else { None },
            latest_sync: if operator_view { latest_sync } else { None },
            integration_state: if operator_view { integration_state } else { None },
            records,
        }
    }

    fn first_record_id(&self) -> Option<sensitive::Id> {
        self.records.first().map(|record| record.id)
    }
}

struct AuthorizedRecordAccess {
    record: Option<sensitive::AuthorizedRecord>,
    event: Option<sensitive::AccessEvent>,
}

impl AuthorizedRecordAccess {
    fn empty() -> Self {
        Self {
            record: None,
            event: None,
        }
    }

    fn new(
        record: Option<sensitive::AuthorizedRecord>,
        event: Option<sensitive::AccessEvent>,
    ) -> Self {
        Self { record, event }
    }
}

pub(super) fn viewer_tier_for_capabilities(
    capabilities: &[sensitive::AccessCapability],
) -> ViewerTier {
    if capabilities.contains(&sensitive::AccessCapability::TokenStatusRead)
        || capabilities.contains(&sensitive::AccessCapability::AccessAuditRead)
    {
        return ViewerTier::SensitiveOperator;
    }
    if capabilities.contains(&sensitive::AccessCapability::AuthorizedRecordRead) {
        return ViewerTier::SensitiveReader;
    }
    ViewerTier::Authenticated
}

pub(super) fn sorted_capabilities(
    mut capabilities: Vec<sensitive::AccessCapability>,
) -> Vec<sensitive::AccessCapability> {
    capabilities.sort();
    capabilities.dedup();
    capabilities
}

fn authorized_record_requires_denied_fallback(error: &failure::Error) -> bool {
    matches!(
        error,
        failure::Error::Repository {
            source: failure::Repository::DecryptRecord { .. },
        }
    )
}

pub(super) fn access_detail_text(message: impl Into<String>) -> sensitive::DetailText {
    super::bounded_detail_text(message, "sensitive access decision")
}

fn access_denial_detail(viewer: &Viewer) -> &'static str {
    match viewer {
        Viewer::Guest => "sign in required before authorized record read",
        Viewer::Authenticated(_) => "viewer lacks authorized_record_read grant",
    }
}
