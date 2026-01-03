# eran_codes

## Quickstart
- `HOST`, `PORT`, `DATABASE_URL`
- `cargo run`

## Structure
- `crates/domain`, `crates/app`, `crates/infra`, `crates/http`, `crates/utils`
- `crates/maud_extensions_macros`

## Commands
- `cargo build`
- `cargo test`
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`

## Docs
- `crates/http/README.md`
- `crates/maud_extensions_macros/README.md`
