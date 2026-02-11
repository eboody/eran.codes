# infra::src

Concrete implementations for persistence and external mechanisms.

## Modules
- `repo/` SQL repositories for app traits.
- `auth.rs` hashing + auth repository.
- `chat.rs` chat repo + infra helpers.

## Rules
- Own SQL and migrations.
- Map DB rows into domain types.
