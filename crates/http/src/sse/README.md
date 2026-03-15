# http::sse

`http::sse` owns the server-side stream registry and the event helpers that make Datastar convergence and live operational updates possible.

## What it owns
- session plus optional tab keyed stream identity
- subscription management and fanout helpers
- patch event construction for elements and signals

## Read it like this
- [mod.rs](./mod.rs)
- [session.rs](./session.rs)
- [../handlers/sse/mod.rs](../handlers/sse/mod.rs)

## Why it matters
The project's realtime story is not "open a socket and hope." Streams are tied to session identity, handlers emit explicit patch events, and the rest of the app can reason about delivery in typed terms.
