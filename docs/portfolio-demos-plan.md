# Demo Catalog (Consolidated)

Use this file to guide portfolio demos and implementation priorities. The goal is conceptual clarity, not demo count.

## Demo A: Identity & Session Durability
- Path: `/register` → `/login` → `/protected`
- Shows: auth enforcement, session cookie settings, Argon2 + credentials table.
- Durability: SQLx Postgres session store + expiry cleanup; migrations as schema source of truth.
- Quality cues: secure cookie flags, redirect behavior, password hashing, session persistence.

## Demo B: Architecture Boundaries + Error Strategy
- Path: boundary map + error trigger.
- Shows: DTO → app command → domain types → infra repo/SQL.
- Error handling: centralized error mapping with page/partial responses.

## Demo C: Observability + Realtime Delivery
- Path: request metadata panel + SSE/Datastar panel + live logs.
- Shows: request_id, user_id, route, latency; single EventSource per visitor.
- Realtime: Datastar patches for signals/fragments; SSE request → broadcast flow.

## Demo D: Live Chat System (Capstone)
- Path: embedded on `/` under `#chat-demo`.
- Shows: session-scoped identity, SSE fanout, request → persist → broadcast flow.
- Enterprise: persisted history, rate limits, moderation queue, audit trail.
- Flow: POST → app validate + rate limit → persist + audit → SSE `chat.message` → Datastar append.
- Failures: 400/401/429 map to partials; moderation returns pending; DB errors via centralized error.
- Builders: use Bon typestate builders for chat router/service configuration so required steps read explicitly.

## Supporting Assets
- Domain newtypes: `RoomId`, `RoomName`, `MessageId`, `MessageBody`, `MessageStatus`, `UserId`.
- App surface: commands (`PostMessage`, `ListMessages`, `CreateRoom`, `JoinRoom`, `ModerateMessage`) and traits (`ChatRepository`, `ModerationQueue`, `RateLimiter`, `AuditLog`, `Clock`, `IdGenerator`).
- Migrations: `chat_rooms`, `chat_messages`, `chat_room_memberships`, `chat_moderation_queue`, `chat_audit_log`, `chat_rate_limits`.
- Indexing: `chat_messages` by `(room_id, created_at)` and `chat_room_memberships` by `(room_id, user_id)`.
