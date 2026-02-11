# eran_codes

## Quickstart
- `HOST`, `PORT`, `DATABASE_URL`, `SESSION_SECRET` (base64url, no padding, 64 bytes)
- Optional: `SESSION_CLEANUP_INTERVAL_SECS` (defaults to 3600)
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`
- `cargo run`

## Structure
- `crates/domain`, `crates/app`, `crates/infra`, `crates/http`, `crates/utils`
- `maud-extensions` (external crate)
- `crates/http` handles SSE with one `/events` stream per visitor, keyed by a signed `session_id` cookie
- `crates/http` integrates `axum-login` with `tower-sessions` for auth sessions (SQLx Postgres store)
- Live chat demo is part of the home page, with Datastar-driven SSE updates
- Trace logging is split between live log (SSE) and diagnostic log (non-SSE)
- Auth routes: `GET /register`, `POST /register`, `GET /login`, `POST /login`, `POST /logout`, `GET /protected`

## Commands
- `cargo build`
- `cargo test`
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`
- `scripts/ci/stringy-check.sh`
- `scripts/ci/no-string-fields.sh`
- `scripts/ci/partials-render.sh`

## Tracing
- Use `RUST_LOG` to control output; see `.cargo/config.toml` for defaults.
- New endpoints and use-cases must follow `docs/tracing.md`.
- Classify new tracing events as live-log vs diagnostic; diagnostics should not emit SSE.

## Docs
- `crates/http/README.md`
- `maud-extensions/README.md`
- `bon.md`
- `docs/auth-sessions.md`
- `docs/tracing.md`
- `docs/datastar-tao.md`
- `docs/datastar-signals.md`
- `docs/datastar-expressions.md`
- `docs/datastar-backend-requests.md`

## README hierarchy
- `README.md` (workspace overview)
- `crates/http/README.md` (HTTP crate overview)
- `crates/http/src/README.md` (HTTP internals)
- `crates/http/src/handlers/README.md` (handler responsibilities)
- `crates/http/src/views/README.md` (Maud view structure)
