# domain::src

Read this directory to confirm that business meaning survives without HTTP, database, or framework naming.

## What this directory proves
- core values become trustworthy by validating at construction time
- `user` and `chat` vocabulary stays transport-independent
- invalid input fails as domain errors instead of leaking outward as generic runtime noise

## Read order
- [user/README.md](./user/README.md)
- [chat/README.md](./chat/README.md)
- [error.rs](./error.rs)

## Rules
- No serde derives.
- No HTTP or DB concepts.
- No transport-specific naming leaking into core types.
