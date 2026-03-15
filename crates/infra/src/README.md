# infra::src

This directory is the concrete mechanism surface for the workspace.

## Read order
- `config.rs`
- `auth.rs`
- [repo/README.md](./repo/README.md)
- `chat.rs`

## Rules
- Own SQL and migrations.
- Map DB rows into domain types.
- Keep transport concerns out of infra code.
