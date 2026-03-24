# Portfolio Demo Concepts (Consolidated)

Use this list when building or explaining demos in this repo. The goal is conceptual clarity, not demo count.

These demos now back the secure-systems thesis at a bounded but honest runtime slice: session durability, encrypted sensitive-record storage, provider-token lifecycle, bounded external-record sync, typed boundaries, and inspectable runtime behavior.

## Sensitive Data + Provider Sync
- Encrypted storage: provider tokens and authorized record payloads are stored encrypted at rest in Postgres.
- Background jobs: startup bootstrapping plus repeating token-refresh and sync loops keep the proof surface current.
- Redacted vs authorized reads: `/lab` shows guest-safe redacted summaries, denied signed-in reads without grants, and audited authorized detail from the same stored records.
- Scope guard: stub mode stays deterministic by default, sandbox mode is opt-in, and the records are sanitized. The repo proves secure mechanics and transport boundaries, not production healthcare or vendor integrations.

## Identity & Session Durability
- Auth flow: axum-login provider + AuthSession extractor.
- Sessions: tower-sessions + SQLx Postgres store with inactivity expiry and cleanup task.
- Security posture: signed, HTTP-only session cookies with SameSite Lax.
- Passwords: Argon2 hashing with a separate credentials table.
- Migrations: schema is defined and evolved in SQL migrations.

## Trust Boundaries + Error Strategy
- Layered boundaries: domain/app/infra/http with explicit traits.
- Flow map: HTTP DTO → app command → domain newtypes → infra SQL.
- Centralized error mapping for page + Datastar partial responses.

## Inspectable Runtime Behavior
- Tracing: request spans with request_id, session_id, user_id, route, latency.
- Live logs: backend + network logs streamed via SSE.
- Realtime transport: single EventSource per visitor; Datastar patches for signals/fragments.
- Demo value: reviewers can inspect request, backend, transport, encrypted-storage evidence, and denied-vs-authorized read behavior directly instead of relying on static architecture claims.

## Live Chat System (Supporting Proof)
- Embedded chat demo showing request → persist → broadcast.
- Persistence: messages/rooms/memberships stored in Postgres.
- Controls: rate limiting, moderation queue, audit trail.
- SSE fanout: message broadcast via Datastar append.

## Request Burst Handling
- Embedded burst demo on `/` with a range slider and one-click execution.
- Client sends a high-volume burst to a lightweight probe endpoint.
- Live log + network panels show request throughput and failures as they happen.
- Demonstrates that inspection surfaces and request handling still hold under stress.
