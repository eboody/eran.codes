CREATE TABLE sensitive_access_grants (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    capability TEXT NOT NULL CHECK (
        capability IN (
            'authorized_record_read',
            'token_status_read',
            'access_audit_read'
        )
    ),
    granted_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, capability)
);

CREATE INDEX sensitive_access_grants_user_id_idx
    ON sensitive_access_grants (user_id);

CREATE TABLE sensitive_access_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NULL REFERENCES users(id) ON DELETE SET NULL,
    target_record_id UUID NULL REFERENCES sensitive_records(id) ON DELETE SET NULL,
    capability TEXT NOT NULL CHECK (
        capability IN (
            'authorized_record_read',
            'token_status_read',
            'access_audit_read'
        )
    ),
    outcome TEXT NOT NULL CHECK (outcome IN ('allowed', 'denied')),
    detail TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX sensitive_access_events_occurred_at_idx
    ON sensitive_access_events (occurred_at DESC, created_at DESC);
