# Auth + Sessions: How It Works

This project uses `axum-login` for authentication and `tower-sessions` for session storage. Think of it as:

- `axum-login`: auth orchestration (login/logout, current user, session auth hash).
- `tower-sessions`: session middleware (cookies, persistence, expiry).

## High-level flow

1. A request hits the router.
2. `tower-sessions` reads or creates a session based on the session cookie.
3. `axum-login` uses the session to resolve the current user.
4. Handlers read `AuthSession` and decide what to do.

## Where things live

- Auth backend: `crates/http/src/auth.rs`
  - Implements `axum_login::AuthnBackend`.
  - Bridges to `app::auth::Service`.
- Session middleware: `crates/http/src/lib.rs`
  - Builds `SessionManagerLayer` (secure cookie, SameSite, expiry).
  - Wraps the router with `AuthManagerLayerBuilder`.
- Auth use-case: `crates/app/src/auth.rs`
  - Validates credentials and fetches user data.
- Password hashing: `crates/infra/src/auth.rs`
  - Argon2 hashing and verification.
- Persistence: `crates/infra/src/auth.rs` + `crates/infra/migrations/003_credentials.*.sql`

## Session storage (Postgres)

We use `tower-sessions-sqlx-store` with Postgres:

- Store: `tower_sessions_sqlx_store::PostgresStore`
- Schema + table: `crates/infra/migrations/004_sessions.*.sql`
- Cleanup task: in `src/main.rs`
  - Interval configured by `SESSION_CLEANUP_INTERVAL_SECS` (default 3600).

## Cookie behavior

Session cookies are configured in `crates/http/src/lib.rs`:

- Name: `eran.sid`
- HttpOnly: `true`
- SameSite: `Lax`
- Secure: `true` in non-debug builds
- Expiry: `OnInactivity(7 days)`
- Signed/encrypted using `SESSION_SECRET`

## Login flow

1. `POST /login` accepts email + password.
2. `axum-login` calls `app::auth::Service::authenticate`.
3. `app::auth` uses `infra::auth::AuthRepository` and `Argon2Hasher`.
4. On success, `AuthSession::login` stores the user id and session auth hash.

## Registration flow

1. `POST /register` validates input (domain constructors).
2. `app::user::Service` hashes password and writes `users` + `credentials`.
3. The handler reuses the login flow to create a session.

## Why both axum-login and tower-sessions?

- `axum-login` does not store sessions itself; it depends on `tower-sessions`.
- `tower-sessions` does not know how to authenticate users; it only stores session data.
- Together they provide secure, persistent authentication.

## Demo hooks

The home page uses interactive demos that show:

- Current auth/session status.
- Request metadata and tracing logs.
- SQL statements + timings from infra repositories.

These are wired through partial endpoints in `crates/http/src/handlers.rs` and trace capture in `crates/http/src/trace_log.rs`.
