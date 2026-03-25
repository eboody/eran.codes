use super::*;

impl Repository {
    pub(super) async fn load_access_grants_impl(
        &self,
        user_id: &user_domain::Id,
    ) -> sensitive::Result<Vec<sensitive_domain::AccessGrant>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, capability, granted_at
            FROM sensitive_access_grants
            WHERE user_id = $1
            ORDER BY capability ASC
            "#,
        )
        .bind(user_id.as_ref())
        .fetch_all(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::LoadAccessGrants,
                source,
            )
        })?;

        rows.into_iter()
            .map(mapping::access_grant_from_row)
            .collect()
    }

    pub(super) async fn upsert_access_grants_impl(
        &self,
        user_id: &user_domain::Id,
        capabilities: &[sensitive_domain::AccessCapability],
        granted_at: SystemTime,
    ) -> sensitive::Result<()> {
        for capability in capabilities {
            sqlx::query(
                r#"
                INSERT INTO sensitive_access_grants (
                    user_id,
                    capability,
                    granted_at
                )
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id, capability) DO NOTHING
                "#,
            )
            .bind(user_id.as_ref())
            .bind(capability.as_ref())
            .bind(mapping::to_offset_datetime(granted_at))
            .execute(&self.pg)
            .await
            .map_err(|source| {
                sensitive::failure::Error::query_repository(
                    RepositoryOperation::UpsertAccessGrants,
                    source,
                )
            })?;
        }

        Ok(())
    }

    pub(super) async fn record_access_event_impl(
        &self,
        event: &sensitive_domain::AccessEvent,
    ) -> sensitive::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sensitive_access_events (
                user_id,
                capability,
                target_record_id,
                outcome,
                detail,
                occurred_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(event.user_id.map(uuid::Uuid::from))
        .bind(event.capability.as_ref())
        .bind(event.record_id.map(uuid::Uuid::from))
        .bind(event.outcome.as_ref())
        .bind(event.detail.to_string())
        .bind(mapping::to_offset_datetime(event.occurred_at))
        .execute(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::RecordAccessEvent,
                source,
            )
        })?;

        Ok(())
    }

    pub(super) async fn list_recent_access_events_impl(
        &self,
        limit: usize,
    ) -> sensitive::Result<Vec<sensitive_domain::AccessEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, capability, target_record_id, outcome, detail, occurred_at
            FROM sensitive_access_events
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pg)
        .await
        .map_err(|source| {
            sensitive::failure::Error::query_repository(
                RepositoryOperation::ListAccessEvents,
                source,
            )
        })?;

        rows.into_iter()
            .map(mapping::access_event_from_row)
            .collect()
    }
}
