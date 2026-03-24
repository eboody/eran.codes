use super::*;

impl Service {
    #[tracing::instrument(skip(self))]
    pub async fn snapshot(&self, viewer: Viewer) -> Result<Snapshot> {
        let stored = self.repo.load_snapshot().await?;
        let viewer_state = self.resolve_viewer_state(&viewer).await?;
        let first_record_id = stored.records.first().map(|record| record.id);
        let authorized_record = self
            .load_authorized_record_for_viewer(&viewer, &viewer_state, first_record_id)
            .await?;
        let key_custody = if viewer_state.allows_token_status() {
            Some(self.repo.load_key_custody().await?)
        } else {
            None
        };
        let token = if viewer_state.allows_token_status() {
            stored.token
        } else {
            None
        };
        let latest_sync = if viewer_state.allows_token_status() {
            stored.latest_sync
        } else {
            None
        };
        let integration_state = if viewer_state.allows_token_status() {
            stored.integration_state
        } else {
            None
        };
        let access_events = if viewer_state.allows_access_audit() {
            self.repo
                .list_recent_access_events(ACCESS_EVENT_LIMIT)
                .await?
        } else {
            Vec::new()
        };

        Ok(Snapshot::builder()
            .viewer(viewer_state)
            .maybe_token(token)
            .maybe_latest_sync(latest_sync)
            .maybe_integration_state(integration_state)
            .maybe_key_custody(key_custody)
            .records(stored.records)
            .maybe_authorized_record(authorized_record)
            .access_events(access_events)
            .build())
    }

    async fn resolve_viewer_state(&self, viewer: &Viewer) -> Result<ViewerState> {
        match viewer {
            Viewer::Guest => Ok(ViewerState::guest()),
            Viewer::Authenticated(viewer) => {
                let _ = self
                    .reconcile_bootstrap_grants_for_user(viewer.user_id, &viewer.email)
                    .await?;
                let grants = self.repo.load_access_grants(&viewer.user_id).await?;
                let capabilities = sorted_capabilities(
                    grants.into_iter().map(|grant| grant.capability).collect(),
                );
                Ok(ViewerState::authenticated(capabilities))
            }
        }
    }

    async fn load_authorized_record_for_viewer(
        &self,
        viewer: &Viewer,
        viewer_state: &ViewerState,
        record_id: Option<sensitive::Id>,
    ) -> Result<Option<sensitive::AuthorizedRecord>> {
        let Some(record_id) = record_id else {
            return Ok(None);
        };

        if viewer_state.allows_authorized_record() {
            let authorized_record = match self.repo.load_authorized_record(&record_id).await
            {
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
            let (outcome, detail) = if authorized_record.is_some() {
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
            self.record_access_decision(viewer, outcome, Some(record_id), detail)
                .await?;
            return Ok(authorized_record);
        }

        self.record_access_decision(
            viewer,
            sensitive::AccessOutcome::Denied,
            Some(record_id),
            access_denial_detail(viewer),
        )
        .await?;
        Ok(None)
    }

    async fn record_access_decision(
        &self,
        viewer: &Viewer,
        outcome: sensitive::AccessOutcome,
        record_id: Option<sensitive::Id>,
        detail: impl Into<String>,
    ) -> Result<()> {
        let user_id = match viewer {
            Viewer::Guest => None,
            Viewer::Authenticated(viewer) => Some(viewer.user_id),
        };
        let event = sensitive::AccessEvent::builder()
            .maybe_user_id(user_id)
            .capability(sensitive::AccessCapability::AuthorizedRecordRead)
            .maybe_record_id(record_id)
            .outcome(outcome)
            .detail(access_detail_text(detail))
            .occurred_at(self.clock.now())
            .build();
        self.repo.record_access_event(&event).await
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

fn authorized_record_requires_denied_fallback(error: &Error) -> bool {
    matches!(
        error,
        Error::Repository {
            source: error::RepositoryError::DecryptRecord { .. },
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
