# eran_codes

## Quickstart
- `HOST`, `PORT`, `DATABASE_URL`, `SESSION_SECRET` (base64url, no padding, 64 bytes)
- `cargo run`

## Structure
- `crates/domain`, `crates/app`, `crates/infra`, `crates/http`, `crates/utils`
- `maud-extensions` (external crate)
- `crates/http` handles SSE with one `/events` stream per visitor, keyed by an unsigned `session_id` cookie
- `crates/http` integrates `axum-login` with `tower-sessions` for auth sessions

## Commands
- `cargo build`
- `cargo test`
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`

## Tracing
- Use `RUST_LOG` to control output; see `.cargo/config.toml` for defaults.
- New endpoints and use-cases must follow `docs/tracing.md`.

## Docs
- `crates/http/README.md`
- `maud-extensions/README.md`
- `docs/tracing.md`
