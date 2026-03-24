CREATE TABLE integration_credentials (
    provider TEXT PRIMARY KEY,
    token_ciphertext BYTEA NOT NULL,
    token_nonce BYTEA NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    refreshed_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE sensitive_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    external_id TEXT NOT NULL UNIQUE,
    redacted_label TEXT NOT NULL,
    redacted_last4 TEXT NOT NULL,
    authorized_ciphertext BYTEA NOT NULL,
    authorized_nonce BYTEA NOT NULL,
    payload_fingerprint TEXT NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX sensitive_records_synced_at_idx ON sensitive_records (synced_at DESC);

CREATE TABLE sync_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider TEXT NOT NULL,
    outcome TEXT NOT NULL CHECK (outcome IN ('success', 'failed')),
    records_seen INTEGER NOT NULL,
    records_upserted INTEGER NOT NULL,
    detail TEXT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX sync_runs_provider_finished_at_idx
    ON sync_runs (provider, finished_at DESC);
