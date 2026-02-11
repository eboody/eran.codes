# eran_codes

## Quickstart
- `HOST`, `PORT`, `DATABASE_URL`, `SESSION_SECRET` (base64url, no padding, 64 bytes)
- Optional: `SESSION_CLEANUP_INTERVAL_SECS` (defaults to 3600)
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`
- `cargo run`

## Structure
- Workspace crates: `domain`, `app`, `infra`, `http`, `utils`
- HTTP demo surface is on the home page with SSE-driven chat updates
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
- `crates/domain/README.md`
- `crates/app/README.md`
- `crates/infra/README.md`
- `crates/utils/README.md`
- `crates/http/src/README.md`
- `crates/http/src/router/README.md`
- `crates/http/src/sse/README.md`
- `crates/http/src/views/README.md`
- `crates/http/src/views/pages/README.md`
- `crates/http/src/views/partials/README.md`
- `crates/http/src/handlers/README.md`
- `crates/http/src/handlers/demo/README.md`
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
- `crates/domain/README.md` (domain invariants)
- `crates/app/README.md` (use-case orchestration)
- `crates/infra/README.md` (DB + external mechanisms)
- `crates/utils/README.md` (shared helpers)
- `crates/http/README.md` (HTTP crate overview)
- `crates/http/src/README.md` (HTTP internals)
- `crates/http/src/router/README.md` (router wiring)
- `crates/http/src/sse/README.md` (SSE registry)
- `crates/http/src/handlers/README.md` (handler responsibilities)
- `crates/http/src/handlers/demo/README.md` (demo handlers)
- `crates/http/src/views/README.md` (Maud view structure)
- `crates/http/src/views/pages/README.md` (page-level views)
- `crates/http/src/views/partials/README.md` (component library)
