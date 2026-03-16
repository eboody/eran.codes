# app::src

Read this directory to confirm that use-case orchestration stays transport-free and persistence-agnostic.

## What this directory proves
- requests should already be translated into `domain` values before they arrive here
- application services coordinate policy and contracts, not HTTP or SQL details
- `infra` should be swappable because this layer depends on traits, not concrete mechanisms

## Read order
- [auth.rs](./auth.rs)
- [user/README.md](./user/README.md)
- [chat/README.md](./chat/README.md)

## Rules
- Accept domain types, not HTTP DTOs.
- Define traits for infra to implement.
- Keep transport and persistence details outside this layer.
