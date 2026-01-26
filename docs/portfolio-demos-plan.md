# Demo Catalog

Use this file to guide portfolio demos and implementation priorities.

## Demo 1: Auth Flow Walkthrough
- Path: `/register` → `/login` → `/protected`
- Shows: auth enforcement, session cookie settings, Argon2 + credentials table.
- Quality cues: secure cookie flags, redirect behavior, password hashing.

## Demo 2: Persistent Session Resilience
- Path: log in, restart server, stay authenticated.
- Shows: SQLx Postgres session store + expiry cleanup.

## Demo 3: Architecture Boundary Demo
- Path: doc panel or “flow map” UI.
- Shows: DTO → app command → domain types → infra repo/SQL.

## Demo 4: Error Handling Showcase
- Path: “trigger error” action to centralized error page.
- Shows: consistent error rendering, status + safe messaging.

## Demo 5: Tracing & Observability
- Path: “request metadata” panel or docs.
- Shows: request_id, user_id, route, latency fields.

## Demo 6: SSE + Datastar Patch Demo
- Path: live status feed with SSE updates and Datastar patches.
- Shows: single EventSource per visitor, session-scoped SSE.

## Demo 7: Migrations as Source of Truth
- Path: applied migrations list / schema snapshot.
- Shows: migrations applied at startup.

## Demo 8: Security Posture Summary
- Path: checklist UI on home/protected.
- Shows: signed cookies, SameSite, HttpOnly, inactivity expiry.

## Demo 9: Live Chat Room
- Path: `/demo/chat`
- Shows: session-scoped identity, SSE/Datastar message stream, request → broadcast flow.
- Enterprise: persisted history, rate limits, abuse controls, trace-friendly fanout.
- Enterprise checklist: schema for messages + rooms, rate limiting, moderation queue, audit trail.
- Enterprise plan:
  - Boundaries: domain newtypes (RoomId, MessageBody), app commands/traits, infra SQLx, http DTOs + views.
  - Data model: `chat_rooms`, `chat_messages` (status + client_id), `chat_room_memberships`, `chat_moderation_queue`, `chat_audit_log`.
  - Endpoints: `GET /demo/chat`, `POST /demo/chat/messages` (optional rooms/join).
  - Flow: POST → app validate + rate limit → persist + audit → SSE `chat.message` → Datastar append.
  - Failures: 400/401/429 map to partials; moderation returns pending; DB errors via centralized error.
  - Builders: use Bon typestate builders for chat router/service configuration so required steps read explicitly.
- App surface (sketch):
  - Commands: `PostMessage`, `ListMessages`, `CreateRoom`, `JoinRoom`, `ModerateMessage`.
  - Traits: `ChatRepository`, `ModerationQueue`, `RateLimiter`, `AuditLog`, `Clock`, `IdGenerator`.
- Domain newtypes (sketch):
  - `RoomId`, `RoomName`, `MessageId`, `MessageBody`, `MessageStatus`, `UserId`.
- Migration outline (sketch):
  - `chat_rooms` (id, name, created_at, created_by)
  - `chat_messages` (id, room_id, user_id, body, created_at, status, client_id)
  - `chat_room_memberships` (room_id, user_id, joined_at, role)
  - `chat_moderation_queue` (message_id, reason, status, reviewer_id, reviewed_at)
  - `chat_audit_log` (id, room_id, actor_user_id, action, metadata_json, created_at)
  - Indexing: `chat_messages` by `(room_id, created_at)` and `chat_room_memberships` by `(room_id, user_id)`.
