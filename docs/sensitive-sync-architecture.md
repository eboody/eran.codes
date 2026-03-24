# Sensitive Sync Architecture

This note explains what the new secure-data slice proves, where the trust boundaries are, and what the implementation still does not claim.

## What This Slice Proves

- Provider tokens are stored encrypted at rest before any sync work uses them.
- Authorized record payloads are stored encrypted at rest while redacted labels and last-four evidence remain queryable for guest-safe proof surfaces.
- Ciphertext now carries explicit key IDs, and the runtime can read mixed-key ciphertext through a configured keyring while new writes use the active key.
- Background token refresh and cursor-based record sync run inside the normal app runtime.
- The app crosses a real local HTTP boundary through `reqwest` instead of calling an in-process provider helper directly.
- Unauthorized reads, operator-only token state, and recent access-audit evidence are all visible on `/lab#sensitive-proof`.
- Operator view now shows provider mode, endpoint provenance, cursor state, token strategy, last fetch outcome, recent remote failure category, active key ID, mixed-key ciphertext counts, stale ciphertext counts, and latest rotation outcome.
- `/work/sensitive-sync` frames that implementation as the current shipped proof slice.

## Boundary Layout

- `domain::sensitive`
  - Owns provider, sync, boundary-state, grant, and authorized-field types.
- `app::sensitive`
  - Owns the use-case contract: load snapshot, refresh token, run sync, resolve persisted sensitive-access grants, retry after remote unauthorized responses, and gate authorized reads behind that policy.
- `infra::crypto`
  - Owns AES-256-GCM keyring mechanics, active-key writes, readable legacy-key decrypts, and disabled-key fail-closed behavior.
- `infra::sensitive`
  - Owns Postgres persistence, ciphertext evidence shaping, key-versioned storage, token decryption for sync work, integration cursor/error state, authorized-record decryption for the explicit authorized-read path, and bounded rewrap passes.
- `infra::sensitive_boundary`
  - Owns the HTTP-backed provider adapter, wire payload decoding, and malformed/unauthorized/rate-limited remote failure classification.
- `src::sensitive_provider_stub`
  - Owns the local stub listener that simulates an external token + records service over HTTP.
- `http`
  - Owns the lab partial, the proof panel, and the current case-study route.

## Encrypted vs Cleartext Fields

- `integration_credentials`
  - `token_ciphertext` and `token_nonce` are encrypted/token-only storage.
  - `token_key_id` stores which configured key sealed the ciphertext.
  - `expires_at` and `refreshed_at` stay cleartext so the runtime can show token lifecycle state.
- `sensitive_records`
  - `authorized_ciphertext` and `authorized_nonce` are encrypted authorized payload storage.
  - `authorized_key_id` stores which configured key sealed the ciphertext.
  - `redacted_label` and `redacted_last4` stay cleartext so guest-safe proof surfaces can render without decrypting.
  - `payload_fingerprint` supports idempotent sync upserts.
- `sync_runs`
  - Stores sync outcome and counts in cleartext because those fields describe runtime operations rather than record payloads.
- `integration_sync_state`
  - Stores cursor, endpoint, token strategy, fetch outcome, last remote error category, and failure count in cleartext because those fields describe boundary behavior rather than sensitive payloads.
- `key_rotation_runs`
  - Stores active key id, scanned/rewrapped/current/failed counts, and bounded failure detail so operator surfaces can show custody progress without exposing payloads.

## Where Decryption Is Allowed

- Token decryption is allowed only in the repo path that loads the provider token for refresh/sync work.
- Authorized record decryption is allowed only in the repo path that loads a specific authorized record.
- Decryption may use any configured readable key, but encryption always uses the configured active key.
- If ciphertext references an unknown or disabled key ID, the read fails closed and the rotation surface records failure evidence instead of exposing payloads.
- Guest lab snapshots record a denied authorized-read decision and do not call the decrypt path.
- Signed-in viewers without grants also record a denied decision and still do not call the decrypt path.
- Reader and operator viewers may call the authorized-record load path for the sample authorized view.

## Authorization And Audit Scope

- `sensitive_access_grants` persists the narrow capability model for this slice:
  - `authorized_record_read`
  - `token_status_read`
  - `access_audit_read`
- `/lab#sensitive-proof` now distinguishes guest, signed-in-without-grant, reader, and operator states.
- `sensitive_access_events` records allowed and denied attempts for the authorized-read path so the proof surface can show policy outcomes, not just payload access.
- Bootstrap grants may be reconciled from configured emails, but the enforcement point is the persisted grant table in the normal runtime path.

## Local Integration Boundary

- The app talks to a local stub over HTTP for both token refresh and record fetches.
- The stub supports paginated record fetch, deterministic cursors, one-shot unauthorized responses, malformed payload responses, and rate-limited responses.
- The app persists enough state to prove what happened at that boundary:
  - current cursor
  - last fetch outcome
  - token strategy used for the last successful or failed sync
  - last remote error category
  - failure count
- Unauthorized record fetches invalidate the cached token path, refresh once, and retry once.
- Malformed payloads and rate-limited responses fail closed and record structured failed sync state.

## Key Rotation And Custody

- The runtime now loads a keyring with one active write key plus any readable legacy keys.
- Existing single-key deployments still boot through a compatibility path that maps `DATA_ENCRYPTION_KEY` to the repo-local legacy key id `legacy_data_key`.
- New token writes and authorized-record writes stamp the active key id beside ciphertext.
- `key_rotation_runs` records bounded rewrap passes that scan a limited number of token and record rows, re-seal stale ciphertext to the active key, and record any row-level failures without exposing plaintext.
- Mixed-key ciphertext is expected during rotation. Refresh, sync, and authorized reads continue to work as long as the referenced key id stays readable in the configured keyring.
- `/lab#sensitive-proof` shows custody evidence only to operators:
  - active key id
  - configured keys and statuses
  - ciphertext counts by key id
  - stale ciphertext counts awaiting rewrap
  - latest rotation outcome and detail

## Scope Limits

- The provider is still a local stub, not a live third-party system.
- The records are sanitized runtime-proof fixtures, not real healthcare or production vendor data.
- The slice proves secure storage, application-level key rotation, transport-boundary handling, bounded sync behavior, persisted grants, and access audit.
- It does not prove live third-party connectivity, HSM/cloud-KMS custody, broad org/team RBAC, or real PHI handling.
