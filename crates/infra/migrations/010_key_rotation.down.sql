DROP INDEX IF EXISTS key_rotation_runs_finished_at_idx;
DROP TABLE IF EXISTS key_rotation_runs;

DROP INDEX IF EXISTS sensitive_records_authorized_key_id_idx;
ALTER TABLE sensitive_records
    DROP COLUMN IF EXISTS authorized_key_id;

DROP INDEX IF EXISTS integration_credentials_token_key_id_idx;
ALTER TABLE integration_credentials
    DROP COLUMN IF EXISTS token_key_id;
