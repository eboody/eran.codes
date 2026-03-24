ALTER TABLE integration_sync_state
    ADD COLUMN auth_mode TEXT NULL CHECK (
        auth_mode IN ('stub_issued_token', 'client_credentials')
    ),
    ADD COLUMN last_auth_outcome TEXT NULL CHECK (
        last_auth_outcome IN ('success', 'failed')
    ),
    ADD COLUMN last_remote_status_code BIGINT NULL CHECK (
        last_remote_status_code >= 100 AND last_remote_status_code <= 599
    ),
    ADD COLUMN retry_backoff_secs BIGINT NULL CHECK (retry_backoff_secs >= 0),
    ADD COLUMN last_successful_mode TEXT NULL CHECK (
        last_successful_mode IN ('local_stub', 'sandbox_http')
    );

ALTER TABLE integration_sync_state
    DROP CONSTRAINT IF EXISTS integration_sync_state_mode_check,
    DROP CONSTRAINT IF EXISTS integration_sync_state_last_error_category_check;

ALTER TABLE integration_sync_state
    ADD CONSTRAINT integration_sync_state_mode_check CHECK (
        mode IN ('local_stub', 'sandbox_http')
    ),
    ADD CONSTRAINT integration_sync_state_last_error_category_check CHECK (
        last_error_category IN (
            'configuration',
            'unauthorized',
            'forbidden',
            'rate_limited',
            'malformed_payload',
            'timeout',
            'server_error',
            'transport'
        )
    );
