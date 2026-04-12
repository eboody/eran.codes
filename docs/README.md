# Docs

This repo only keeps the docs that help a reviewer understand the shipped work.

Start here:

- [../README.md](../README.md)
- [Sensitive Sync Architecture](./sensitive-sync-architecture.md)
- [Auth + Sessions](./auth-sessions.md)
- [Portfolio Demo Notes](./portfolio-demos.md)

Crate read order:

- [domain](../crates/domain/README.md)
- [app](../crates/app/README.md)
- [infra](../crates/infra/README.md)
- [http](../crates/http/README.md)
- [utils](../crates/utils/README.md)

Validation:

```bash
cargo test --workspace --lib --tests
bash scripts/check_portfolio_browser_smoke.sh
bash scripts/check_docker_runtime_smoke.sh
```
