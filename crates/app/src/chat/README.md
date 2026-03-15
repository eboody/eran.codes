# app::chat

Chat use cases and policy live here.

## What to inspect
- `mod.rs` for the service and command surface
- `post_message_flow.rs` for the capstone path
- `moderate_message_flow.rs` for moderation policy
- `error.rs` for chat-specific contextual failures

## What it proves
Posting, moderation, rate limiting, and membership checks are modeled as explicit application workflows instead of transport glue.
