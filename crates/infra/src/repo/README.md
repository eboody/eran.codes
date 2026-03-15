# infra::repo

SQL repositories implementing `app` traits live here.

## What to inspect
- `user.rs` for account lookup and persistence
- `chat.rs` for rooms, messages, moderation, rate limits, and audit log persistence
- `mod.rs` for the public repository surface

## What it proves
Storage concerns stay concrete here: query shape, joins, and row mapping are local, while the rest of the system speaks in app and domain types.
