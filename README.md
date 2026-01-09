# eran_codes

## Quickstart
- `HOST`, `PORT`, `DATABASE_URL`
- `cargo run`

## Structure
- `crates/domain`, `crates/app`, `crates/infra`, `crates/http`, `crates/utils`
- `maud-extensions` (external crate)
- `crates/http` handles SSE with one `/events` stream per visitor, keyed by an unsigned `session_id` cookie

## Commands
- `cargo build`
- `cargo test`
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`

## Docs
- `crates/http/README.md`
- `maud-extensions/README.md`
