# app::src

This directory holds the main use-case surfaces for the workspace.

## Read order
- [auth.rs](./auth.rs)
- [user/README.md](./user/README.md)
- [chat/README.md](./chat/README.md)

## Rules
- Accept domain types, not HTTP DTOs.
- Define traits for infra to implement.
- Keep transport and persistence details outside this layer.
