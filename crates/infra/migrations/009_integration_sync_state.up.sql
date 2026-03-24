CREATE TABLE integration_sync_state (
    provider TEXT PRIMARY KEY,
    mode TEXT NOT NULL CHECK (mode IN ('local_stub')),
    endpoint TEXT NOT NULL,
    cursor TEXT NULL,
    last_fetch_outcome TEXT NOT NULL CHECK (
        last_fetch_outcome IN ('success', 'failed')
    ),
    token_strategy TEXT NOT NULL CHECK (
        token_strategy IN (
            'cached_token',
            'refreshed_token',
            'retry_after_unauthorized'
        )
    ),
    last_error_category TEXT NULL CHECK (
        last_error_category IN (
            'unauthorized',
            'rate_limited',
            'malformed_payload',
            'transport'
        )
    ),
    last_successful_fetch_at TIMESTAMPTZ NULL,
    last_attempted_fetch_at TIMESTAMPTZ NOT NULL,
    failure_count BIGINT NOT NULL CHECK (failure_count >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
