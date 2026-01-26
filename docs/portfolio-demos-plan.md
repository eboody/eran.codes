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
