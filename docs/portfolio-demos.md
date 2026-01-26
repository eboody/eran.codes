# Portfolio Demo Concepts

Use this list when building or explaining demos in this repo.

## Auth and Sessions
- axum-login authentication flow with backend provider and AuthSession extractor.
- Persistent sessions via tower-sessions + SQLx Postgres store.
- Signed, HTTP-only session cookies with SameSite Lax and inactivity expiry.
- Background session cleanup task for expired records.
- Argon2 password hashing and separate credentials table.

## UI and Interaction
- Server-rendered HTML with Maud layouts and view modules.
- SSE support with per-visitor signed session cookie.
- Datastar-compatible request path (signals/patches available in codebase).
- Live chat room demo: single EventSource, chat stream updates, and form-driven message posts.
  - Flow: POST message → validate + persist → SSE `chat.message` → Datastar append.

## Architecture and Ops
- Layered boundaries: domain/app/infra/http with explicit traits.
- Repo traits + infra implementations for persistence.
- Centralized HTTP error mapping to page/partial responses.
- Tracing strategy with request spans and user-id injection.
- SQL migrations as the schema source of truth.
- Composition root wiring in src/main.rs for services and stores.
- Chat room shows request → broadcast flow with persisted history, rate limits, and abuse controls.
- Chat room enterprise checklist: messages + rooms schema, moderation queue, audit trail.
  - Boundaries: domain newtypes, app commands/traits, infra SQLx, http DTOs/views.
