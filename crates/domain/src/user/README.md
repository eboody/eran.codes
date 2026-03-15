# domain::user

User identity types live here.

## What to inspect
- `mod.rs` for the public surface
- `entity.rs` for the user entity shape
- `error.rs` for invalid input cases

## What it proves
User-facing identity data enters the system as typed values with explicit invariants instead of raw strings that get rechecked everywhere else.
