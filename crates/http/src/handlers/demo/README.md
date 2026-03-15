# http::handlers::demo

This module powers the interactive proof surfaces behind `/lab` and the home page demos.

## What it owns
- chat message posting and moderation flows
- support partials for auth status, session status, request metadata, boundary checks, and DB checks
- endpoints that help the UI surface runtime behavior instead of hiding it

## Read it like this
- [chat/mod.rs](./chat/mod.rs)
- [partials/mod.rs](./partials/mod.rs)

## Boundaries
Business policy still lives in `app`; these handlers exist to expose that behavior clearly through pages, Datastar partials, and SSE-backed interactions.
