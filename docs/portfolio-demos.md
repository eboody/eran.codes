# Portfolio Demo Concepts (Consolidated)

Use this list when building or explaining demos in this repo. The goal is conceptual clarity, not demo count.

## Identity & Session Durability
- Auth flow: axum-login provider + AuthSession extractor.
- Sessions: tower-sessions + SQLx Postgres store with inactivity expiry and cleanup task.
- Security posture: signed, HTTP-only session cookies with SameSite Lax.
- Passwords: Argon2 hashing with a separate credentials table.
- Migrations: schema is defined and evolved in SQL migrations.

## Architecture Boundaries + Error Strategy
- Layered boundaries: domain/app/infra/http with explicit traits.
- Flow map: HTTP DTO → app command → domain newtypes → infra SQL.
- Centralized error mapping for page + Datastar partial responses.

## Observability + Realtime Delivery
- Tracing: request spans with request_id, session_id, user_id, route, latency.
- Live logs: backend + network logs streamed via SSE.
- Realtime: single EventSource per visitor; Datastar patches for signals/fragments.

## Live Chat System (Capstone)
- Embedded chat demo (two senders) showing request → persist → broadcast.
- Persistence: messages/rooms/memberships stored in Postgres.
- Controls: rate limiting, moderation queue, audit trail.
- SSE fanout: message broadcast via Datastar append.
