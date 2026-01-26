# HTTP internals

This directory is the HTTP crate's implementation detail. `crates/http/src/lib.rs` is
intentionally thin and re-exports the public surface so the high-level entry stays readable.

## Module map
- `state.rs`: shared HTTP state and demo-only state (`State`, `DemoState`, `SurrealState`).
- `router/`: Bon-powered router assembly split into routes + middleware layers.
- `handlers/`: request handlers grouped by concern.
- `request.rs`: request context extraction and task-local context helpers.
- `sse/`: session-keyed SSE registry and Datastar event wrappers.
- `trace_log.rs`: in-memory trace store + SSE live log integration (Bon builder).
- `views/`: Maud view components (pages, partials, layout).
- `error.rs`: HTTP error mapping to page or partial responses.
- `trace.rs`: route span enrichment middleware.

## Design goals
- Keep `lib.rs` and `router/` readable without drilling into submodules.
- Keep state ownership explicit: production state in `State`, demo-only state in `DemoState`.
- Treat `handlers/` as the only place that knows about request/response mechanics.
 - Use `Extension<State>` for handler access to shared HTTP state.

## Router flow
```
Routes::new()
  -> base routes
  -> page routes (with optional live-reload)
  -> route tracing middleware
  -> request layers (trace, audit, context, ids, cookies)
  -> auth/session layer
  -> state extension
```
