# domain::chat

Chat entities, ids, statuses, and value objects live here.

## What to inspect
- `room.rs` for room and membership vocabulary
- `message.rs` for message ids, bodies, and status
- `error.rs` for invalid chat input

## What it proves
The chat system is built on typed room/message concepts, not loosely coordinated literals.
