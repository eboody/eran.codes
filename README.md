# eran.codes

Rust workspace behind <https://eran.codes>.

The repo is organized around one public thesis:

- `statum` is the flagship crate.
- `/open-source` compares the published crates.
- `/lab` is the live application proof.
- `/work/sensitive-sync` is the single written case study.
- `/work` is a short archive for older shipped systems.

## What is here

The workspace keeps the usual boundary split:

- [`crates/domain`](./crates/domain/README.md): domain types and invariants
- [`crates/app`](./crates/app/README.md): workflows and policy
- [`crates/infra`](./crates/infra/README.md): Postgres, repositories, hashing, config
- [`crates/http`](./crates/http/README.md): routes, handlers, SSE, Maud views, live proof surface
- [`crates/utils`](./crates/utils/README.md): developer tooling

If you want the short version, start with:

1. [`/`](https://eran.codes/)
2. [`/lab`](https://eran.codes/lab)
3. [`/work/sensitive-sync`](https://eran.codes/work/sensitive-sync)
4. [`/open-source`](https://eran.codes/open-source)

## Docs

- [docs/README.md](./docs/README.md)
- [Sensitive Sync Architecture](./docs/sensitive-sync-architecture.md)
- [Auth + Sessions](./docs/auth-sessions.md)
- [Portfolio Demo Notes](./docs/portfolio-demos.md)

## Run It

Required env:

- `HOST`
- `PORT`
- `DATABASE_URL`
- `SESSION_SECRET`
- `DATA_ENCRYPTION_KEYS_JSON`
- `ACTIVE_DATA_KEY_ID`

Optional env:

- `DISABLED_DATA_KEY_IDS`
- `SENSITIVE_PROVIDER_MODE`
- `SENSITIVE_PROVIDER_BASE_URL`
- `SENSITIVE_SANDBOX_CLIENT_ID`
- `SENSITIVE_SANDBOX_CLIENT_SECRET`

Local boot:

```bash
docker-compose up -d
cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations
cargo run
```

## Validation

```bash
cargo test --workspace --lib --tests
bash scripts/check_portfolio_browser_smoke.sh
bash scripts/check_docker_runtime_smoke.sh
```
