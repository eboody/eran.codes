# infra::src

Read this directory to confirm that mechanism code stays concrete: configuration, auth persistence, queries, and row mapping all live here instead of bleeding upward.

## What this directory proves
- SQL and persistence mechanics stay local to `infra`
- `app` contracts are implemented without moving policy into the mechanism layer
- rows and config are translated into typed workspace values at the boundary

## Read order
- `config.rs`
- `auth.rs`
- [repo/README.md](./repo/README.md)
- `chat.rs`

## Rules
- Own SQL and migrations.
- Map DB rows into domain types.
- Keep transport concerns out of infra code.
