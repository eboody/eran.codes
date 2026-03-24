ALTER TABLE integration_sync_state
    DROP CONSTRAINT IF EXISTS integration_sync_state_mode_check,
    DROP CONSTRAINT IF EXISTS integration_sync_state_last_error_category_check;

ALTER TABLE integration_sync_state
    ADD CONSTRAINT integration_sync_state_mode_check CHECK (
        mode IN ('local_stub')
    ),
    ADD CONSTRAINT integration_sync_state_last_error_category_check CHECK (
        last_error_category IN (
            'unauthorized',
            'rate_limited',
            'malformed_payload',
            'transport'
        )
    );

ALTER TABLE integration_sync_state
    DROP COLUMN IF EXISTS auth_mode,
    DROP COLUMN IF EXISTS last_auth_outcome,
    DROP COLUMN IF EXISTS last_remote_status_code,
    DROP COLUMN IF EXISTS retry_backoff_secs,
    DROP COLUMN IF EXISTS last_successful_mode;
