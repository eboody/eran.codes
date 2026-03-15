# utils::src

Shared helpers with minimal dependencies live here.

## What to inspect
- `envs.rs` for environment helpers
- `b64.rs` for encoding support
- `bin/visual_snapshot.rs` for screenshot tooling

## Rule of thumb
If a helper would force business, transport, or persistence concerns into this crate, it belongs somewhere else.
