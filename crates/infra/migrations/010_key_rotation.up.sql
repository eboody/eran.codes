ALTER TABLE integration_credentials
    ADD COLUMN token_key_id TEXT;

UPDATE integration_credentials
SET token_key_id = 'legacy_data_key'
WHERE token_key_id IS NULL;

ALTER TABLE integration_credentials
    ALTER COLUMN token_key_id SET NOT NULL;

CREATE INDEX integration_credentials_token_key_id_idx
    ON integration_credentials (token_key_id);

ALTER TABLE sensitive_records
    ADD COLUMN authorized_key_id TEXT;

UPDATE sensitive_records
SET authorized_key_id = 'legacy_data_key'
WHERE authorized_key_id IS NULL;

ALTER TABLE sensitive_records
    ALTER COLUMN authorized_key_id SET NOT NULL;

CREATE INDEX sensitive_records_authorized_key_id_idx
    ON sensitive_records (authorized_key_id);

CREATE TABLE key_rotation_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    active_key_id TEXT NOT NULL,
    outcome TEXT NOT NULL CHECK (outcome IN ('success', 'failed')),
    rows_scanned INTEGER NOT NULL CHECK (rows_scanned >= 0),
    rows_rewrapped INTEGER NOT NULL CHECK (rows_rewrapped >= 0),
    rows_already_current INTEGER NOT NULL CHECK (rows_already_current >= 0),
    rows_failed INTEGER NOT NULL CHECK (rows_failed >= 0),
    detail TEXT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX key_rotation_runs_finished_at_idx
    ON key_rotation_runs (finished_at DESC, created_at DESC);
