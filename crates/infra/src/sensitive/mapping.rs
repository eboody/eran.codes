use super::*;

pub(super) fn parse_key_id(value: String) -> sensitive::Result<sensitive_domain::KeyId> {
    sensitive_domain::KeyId::try_new(value.clone())
        .map_err(|_| sensitive::failure::Error::InvalidStoredKeyId { key_id: value })
}

pub(super) fn parse_u32_count(value: i64, field: &'static str) -> sensitive::Result<u32> {
    u32::try_from(value)
        .map_err(|_| sensitive::failure::Error::InvalidStoredRotationCount { field, value })
}

pub(super) fn ciphertext_evidence(
    key_id: sensitive_domain::KeyId,
    ciphertext: &[u8],
) -> sensitive::CiphertextEvidence {
    let preview_len = ciphertext.len().min(10);
    let preview = ciphertext[..preview_len]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    sensitive::CiphertextEvidence::builder()
        .key_id(key_id)
        .preview(preview)
        .bytes(ciphertext.len())
        .build()
}

pub(super) fn payload_fingerprint(
    record: &sensitive_domain::Record,
    authorized_json: &[u8],
) -> String {
    let mut digest = Sha256::new();
    digest.update(record.external_id.to_string());
    digest.update(record.label.to_string());
    digest.update(record.last4.to_string());
    digest.update(authorized_json);
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(super) fn access_grant_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::AccessGrant> {
    Ok(sensitive_domain::AccessGrant::builder()
        .user_id(user_domain::Id::from(row.get::<uuid::Uuid, _>("user_id")))
        .capability(row.get::<String, _>("capability").parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredAccessCapability {
                capability: row.get("capability"),
            }
        })?)
        .granted_at(from_offset_datetime(row.get("granted_at")))
        .build())
}

pub(super) fn access_event_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::AccessEvent> {
    let capability_raw = row.get::<String, _>("capability");
    let outcome_raw = row.get::<String, _>("outcome");
    Ok(sensitive_domain::AccessEvent::builder()
        .maybe_user_id(
            row.get::<Option<uuid::Uuid>, _>("user_id")
                .map(user_domain::Id::from),
        )
        .capability(capability_raw.parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredAccessCapability {
                capability: capability_raw.clone(),
            }
        })?)
        .maybe_record_id(
            row.get::<Option<uuid::Uuid>, _>("target_record_id")
                .map(sensitive_domain::Id::from),
        )
        .outcome(outcome_raw.parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredAccessOutcome {
                outcome: outcome_raw.clone(),
            }
        })?)
        .detail(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("detail"))
                .map_err(sensitive::failure::Error::decode_detail_text)?,
        )
        .occurred_at(from_offset_datetime(row.get("occurred_at")))
        .build())
}

pub(super) fn record_proof_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive::RecordProof> {
    Ok(sensitive::RecordProof::builder()
        .id(sensitive_domain::Id::from(row.get::<uuid::Uuid, _>("id")))
        .label(
            sensitive_domain::Label::try_new(row.get::<String, _>("redacted_label"))
                .map_err(sensitive::failure::Error::decode_label)?,
        )
        .last4(
            sensitive_domain::Last4::try_new(row.get::<String, _>("redacted_last4"))
                .map_err(sensitive::failure::Error::decode_last4)?,
        )
        .synced_at(from_offset_datetime(row.get("synced_at")))
        .ciphertext(ciphertext_evidence(
            parse_key_id(row.get::<String, _>("authorized_key_id"))?,
            &row.get::<Vec<u8>, _>("authorized_ciphertext"),
        ))
        .build())
}

pub(super) fn sync_run_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::SyncRun> {
    let provider_raw = row.get::<String, _>("provider");
    let outcome_raw = row.get::<String, _>("outcome");
    let records_seen = row.get::<i32, _>("records_seen");
    let records_upserted = row.get::<i32, _>("records_upserted");
    Ok(sensitive_domain::SyncRun::builder()
        .provider(provider_raw.parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredProvider {
                provider: provider_raw.clone(),
            }
        })?)
        .outcome(outcome_raw.parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredSyncOutcome {
                outcome: outcome_raw.clone(),
            }
        })?)
        .records_seen(parse_u32_count(records_seen.into(), "records_seen")?)
        .records_upserted(parse_u32_count(
            records_upserted.into(),
            "records_upserted",
        )?)
        .detail(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("detail"))
                .map_err(sensitive::failure::Error::decode_detail_text)?,
        )
        .started_at(from_offset_datetime(row.get("started_at")))
        .finished_at(from_offset_datetime(row.get("finished_at")))
        .build())
}

pub(super) fn rotation_run_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::KeyRotationRun> {
    let active_key_id = parse_key_id(row.get::<String, _>("active_key_id"))?;
    let outcome_raw = row.get::<String, _>("outcome");
    let rows_scanned = row.get::<i32, _>("rows_scanned");
    let rows_rewrapped = row.get::<i32, _>("rows_rewrapped");
    let rows_already_current = row.get::<i32, _>("rows_already_current");
    let rows_failed = row.get::<i32, _>("rows_failed");
    Ok(sensitive_domain::KeyRotationRun::builder()
        .active_key_id(active_key_id)
        .outcome(outcome_raw.parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredRotationOutcome {
                outcome: outcome_raw.clone(),
            }
        })?)
        .rows_scanned(parse_u32_count(rows_scanned.into(), "rows_scanned")?)
        .rows_rewrapped(parse_u32_count(rows_rewrapped.into(), "rows_rewrapped")?)
        .rows_already_current(parse_u32_count(
            rows_already_current.into(),
            "rows_already_current",
        )?)
        .rows_failed(parse_u32_count(rows_failed.into(), "rows_failed")?)
        .detail(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("detail"))
                .map_err(sensitive::failure::Error::decode_detail_text)?,
        )
        .started_at(from_offset_datetime(row.get("started_at")))
        .finished_at(from_offset_datetime(row.get("finished_at")))
        .build())
}

pub(super) fn integration_state_from_row(
    row: sqlx::postgres::PgRow,
) -> sensitive::Result<sensitive_domain::IntegrationState> {
    let provider_raw = row.get::<String, _>("provider");
    let mode_raw = row.get::<String, _>("mode");
    let auth_mode_raw = row.get::<Option<String>, _>("auth_mode");
    let cursor_raw = row.get::<Option<String>, _>("cursor");
    let last_error_category_raw = row.get::<Option<String>, _>("last_error_category");
    let last_auth_outcome_raw = row.get::<Option<String>, _>("last_auth_outcome");
    let last_successful_mode_raw = row.get::<Option<String>, _>("last_successful_mode");
    let last_remote_status_code = row.get::<Option<i64>, _>("last_remote_status_code");
    let retry_backoff_secs = row.get::<Option<i64>, _>("retry_backoff_secs");
    let failure_count = row.get::<i64, _>("failure_count");

    Ok(sensitive_domain::IntegrationState::builder()
        .provider(provider_raw.parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredProvider {
                provider: provider_raw.clone(),
            }
        })?)
        .mode(mode_raw.parse().map_err(|_| {
            sensitive::failure::Error::InvalidStoredProviderMode {
                mode: mode_raw.clone(),
            }
        })?)
        .endpoint(
            sensitive_domain::DetailText::try_new(row.get::<String, _>("endpoint"))
                .map_err(sensitive::failure::Error::decode_detail_text)?,
        )
        .maybe_auth_mode(
            auth_mode_raw
                .map(|mode| {
                    mode.parse().map_err(|_| {
                        sensitive::failure::Error::InvalidStoredProviderAuthMode {
                            mode: mode.clone(),
                        }
                    })
                })
                .transpose()?,
        )
        .maybe_cursor(
            cursor_raw
                .map(|cursor| {
                    sensitive_domain::SyncCursor::try_new(cursor.clone()).map_err(|_| {
                        sensitive::failure::Error::InvalidStoredSyncCursor { cursor }
                    })
                })
                .transpose()?,
        )
        .last_fetch_outcome(row.get::<String, _>("last_fetch_outcome").parse().map_err(
            |_| sensitive::failure::Error::InvalidStoredFetchOutcome {
                outcome: row.get("last_fetch_outcome"),
            },
        )?)
        .token_strategy(
            row.get::<String, _>("token_strategy")
                .parse()
                .map_err(|_| sensitive::failure::Error::InvalidStoredTokenStrategy {
                    strategy: row.get("token_strategy"),
                })?,
        )
        .maybe_last_error_category(
            last_error_category_raw
                .map(|category| {
                    category.parse().map_err(|_| {
                        sensitive::failure::Error::InvalidStoredRemoteErrorCategory {
                            category: category.clone(),
                        }
                    })
                })
                .transpose()?,
        )
        .maybe_last_auth_outcome(
            last_auth_outcome_raw
                .map(|outcome| {
                    outcome.parse().map_err(|_| {
                        sensitive::failure::Error::InvalidStoredFetchOutcome {
                            outcome: outcome.clone(),
                        }
                    })
                })
                .transpose()?,
        )
        .maybe_last_remote_status_code(
            last_remote_status_code
                .map(|status_code| {
                    u16::try_from(status_code).map_err(|_| {
                        sensitive::failure::Error::InvalidStoredRemoteStatusCode {
                            status_code,
                        }
                    })
                })
                .transpose()?,
        )
        .maybe_retry_backoff_secs(
            retry_backoff_secs
                .map(|backoff| {
                    u32::try_from(backoff).map_err(|_| {
                        sensitive::failure::Error::InvalidStoredFailureCount {
                            failure_count: backoff,
                        }
                    })
                })
                .transpose()?,
        )
        .maybe_last_successful_mode(
            last_successful_mode_raw
                .map(|mode| {
                    mode.parse().map_err(|_| {
                        sensitive::failure::Error::InvalidStoredProviderMode {
                            mode: mode.clone(),
                        }
                    })
                })
                .transpose()?,
        )
        .maybe_last_successful_fetch_at(
            row.get::<Option<OffsetDateTime>, _>("last_successful_fetch_at")
                .map(from_offset_datetime),
        )
        .last_attempted_fetch_at(from_offset_datetime(row.get("last_attempted_fetch_at")))
        .failure_count(u32::try_from(failure_count).map_err(|_| {
            sensitive::failure::Error::InvalidStoredFailureCount { failure_count }
        })?)
        .build())
}

pub(super) fn to_offset_datetime(value: SystemTime) -> OffsetDateTime {
    value.into()
}

pub(super) fn from_offset_datetime(value: OffsetDateTime) -> SystemTime {
    value.into()
}
