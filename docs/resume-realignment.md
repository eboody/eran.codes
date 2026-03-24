# Resume Realignment Status

As of March 23, 2026, the repo has completed the original proof pivot and the narrow secure-data proof slice that this document first argued for.

This file is no longer a gap list. It is the status note that explains what changed, what is now honestly proved, and what still remains outside the current claim surface.

## Original Mismatch

The original issue was straightforward:

- the repo proved architecture, realtime delivery, and observability better than it proved security-sensitive systems work
- the public story leaned too heavily on older case studies instead of a current Rust artifact that showed encrypted storage, token lifecycle, and bounded sync behavior
- the runtime had no public proof surface for encrypted records, integration credentials, or background synchronization

## What Shipped Since That Audit

- The repo thesis and public copy now lead with secure backend systems, trust boundaries, and inspectable runtime behavior.
- The workspace now has a `sensitive` bounded context across `domain`, `app`, `infra`, and `http`.
- Provider tokens and authorized record payloads are stored encrypted at rest.
- Ciphertext now carries explicit key IDs, the runtime can read mixed-key ciphertext through a configured keyring, and bounded background passes re-seal stale ciphertext to the active key.
- The runtime now persists `integration_credentials`, `sensitive_records`, and `sync_runs`.
- Background token refresh and bounded record sync now run inside the normal app runtime.
- `/lab#sensitive-proof` exposes token state, sync state, ciphertext evidence, key custody evidence, redacted records, denied reads without grants, and audited authorized detail.
- `/work/sensitive-sync` now exists as the current shipped proof case instead of relying only on historical case-study pages.
- [Sensitive Sync Architecture](./sensitive-sync-architecture.md) now documents the trust boundaries and scope limits for that slice.

## What The Repo Now Honestly Proves

- durable auth and session handling
- encrypted-at-rest storage for sensitive records and provider credentials
- application-level key rotation with active and readable legacy keys
- background token refresh and bounded stub-or-sandbox HTTP-backed record sync
- explicit cross-layer trust boundaries
- guest-safe redacted reads versus persisted-grant authorized reads with audit evidence
- reviewer-visible runtime inspection surfaces

## What The Repo Still Does Not Claim

- production third-party vendor connectivity
- real PHI handling
- broader RBAC beyond the narrow sensitive-access capability model
- HSM or cloud-KMS-backed key custody
- a full healthcare platform rebuild

Those boundaries are intentional. They keep the proof surface honest.

## What Still Remains For Resume Alignment

- The repo thesis is now correct and the public content root is unified, but legacy `/work/*` routes still remain as optional future redirect candidates.
- The secure-data slice now proves a deterministic local stub path and an opt-in sandbox HTTP path, but not production third-party operations.
- The secure-data slice now proves application-level key rotation and custody evidence, but not external key-management infrastructure.
- The shared content root is now the source of truth, but future edits should stay disciplined about what belongs in authored content versus runtime-owned code paths.

## Recommended Next Move

The packaging branch is now shipped. The next move should be a focused choice, not another broad pass:

- route and redirect policy for supporting-proof pages
- incremental content-root ergonomics cleanup if editing pain shows up
- a separate stronger runtime-proof branch if the resume needs deeper authorization, production vendor evidence, or external key-management proof

Use [Resume Alignment Packaging Status](./refactor-plan.md) for the current post-refactor state, and [Site Content Authoring Guide](./site-content-authoring.md) for the rules around the shared content root.
