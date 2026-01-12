# Tracing Plan (Enterprise-Level)

This document defines how tracing must be handled across the workspace. Follow it for new
features, endpoints, and infra work. The goal is consistent, structured context with clear
ownership by layer.

## Ownership by Layer
- `domain`: no tracing; keep it pure.
- `app`: use-case spans and policy-level events.
- `http`: request spans, request/session/user context, response mapping.
- `infra`: I/O spans (DB, external services), retries, backoff, circuit breakers.
- `main`: subscriber configuration and environment defaults.

## Required Context Fields
Attach these fields whenever available:
- `request_id`: UUID from `tower_http::request_id`.
- `session_id`: from HTTP cookie (if present).
- `user_id`: from authenticated identity (if available).
- `route`: matched route (not raw URI).
- `method`, `status`, `latency_ms`.
- `client_ip`, `user_agent` (if available and safe).

Never log plaintext passwords or secrets. Avoid logging raw emails; use a hash if needed.

## HTTP Layer (Request Span + Middleware)
- Use `tower_http::trace::TraceLayer` with a custom `make_span` to emit a single
  `http.request` span for each request.
- Add response metadata in `on_response` and errors in `on_failure`.
- Add a `RequestContext` extension or task-local to carry `request_id`, `session_id`,
  `user_id`, `route`, and `is_datastar`.
- Auth middleware must call `http::request::set_user_id` once identity is known.

## App Layer (Use-Case Spans)
- Public app service methods must be `#[tracing::instrument]`.
- Use `skip(self, raw_input)` for large or sensitive inputs.
- Add derived fields manually (e.g., `user_id`, `email_hash`).
- App returns typed errors; no logging here unless the decision itself is a policy event.

## Infra Layer (I/O Spans)
- Wrap DB operations with `#[tracing::instrument]` or explicit spans.
- Include `db.statement` (sanitized or hashed), `db.rows`, `db.elapsed_ms`.
- Log connection failures/retries at `warn` or `error`.
- Keep all DB/driver-specific tracing in `infra`.

## Logging Levels
- `info`: request start/end, important state changes, successful use-case completion.
- `debug`: internal flow and low-level detail (sanitized).
- `warn`: retries, partial failures, unusual but expected conditions.
- `error`: failed requests, I/O failures, unexpected states.

## SSE Guidance
- Emit an `sse.connection` span for connect/disconnect.
- Log `session_id`, `request_id`, `event_name`, `payload_bytes`.
- Do not log raw payload content.

## Configuration
- Local: `RUST_LOG=info,http=debug,app=debug,infra=debug,sqlx=warn`
- Prod: `RUST_LOG=info,http=info,app=info,infra=info,sqlx=warn`
- Prefer JSON logs in production; pretty logs locally.

## Optional Export
If needed, add OpenTelemetry export (Jaeger/OTLP) behind a feature flag. Keep it wired in
`main` and ensure no direct dependencies in `app` or `domain`.

## Required for New Work
When adding new endpoints or use-cases:
1) Ensure HTTP spans include request/session/user context.
2) Ensure app services are instrumented.
3) Ensure infra I/O spans exist for DB/external calls.
4) Ensure errors are mapped once in `http` and logged at the edge.
