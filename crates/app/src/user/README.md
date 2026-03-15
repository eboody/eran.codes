# app::user

User registration and identity-policy orchestration live here.

## What to inspect
- `mod.rs` for the public service surface
- `register_user_flow.rs` for the registration workflow
- `error.rs` for contextual failures

## What it proves
Registration policy is handled as an application workflow, not embedded in HTTP handlers or persistence code.
