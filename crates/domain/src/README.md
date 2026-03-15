# domain::src

This directory is the pure business core of the workspace.

## Read order
- [user/README.md](./user/README.md)
- [chat/README.md](./chat/README.md)
- [error.rs](./error.rs)

## Rules
- No serde derives.
- No HTTP or DB concepts.
- No transport-specific naming leaking into core types.
