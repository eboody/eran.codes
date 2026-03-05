---
name: mds-datastar-architecture
version: 0.1.0
description: Canonical command + global SSE architecture for Datastar components in this repo.
scope: project
---

# mds-datastar-architecture

## Purpose
Enforce one Datastar architecture across specs and generated code:
- Commands mutate server state.
- Commands return `204` (or `202` if queued) and never return JSON state.
- App-authority convergence happens through one global SSE stream (`/events`).
- Server emits `datastar-patch-signals` for app-state updates.
- Specs model app-state updates through SSE mappings only.

## Required Rules
- Classify interaction scope before protocol:
  - `presentation`/`session` interactions default to `ui_local`.
  - `app` interactions use command + SSE.
- Command handlers used by Datastar UI intents must be marked:
  - `// ci: datastar-command <handler_name>`
- Command handlers must return `StatusCode`:
  - `StatusCode::NO_CONTENT` or `StatusCode::ACCEPTED`
- Command handlers must not return `Json<...>` state payloads.
- App-authority state fields must not be mutated by UI transitions.
- Tabs/selectors are presentation interactions by default and should not require backend commands unless explicit app-level semantics exist.
- Datastar component specs must satisfy:
  - `events.app_mappings.backend_responses == []`
  - app event trigger: `sse:datastar-patch-signals`
  - SSE mapping event: `event_name: datastar-patch-signals`

## Bad vs Good

Bad (for Datastar command handlers):
```rust
pub async fn sync_counter(...) -> Json<CounterAppPatch> {
    Json(CounterAppPatch { server_count: 7, server_connected: true })
}
```

Good:
```rust
// ci: datastar-command sync_counter
pub async fn sync_counter(...) -> StatusCode {
    // mutate canonical server state
    // emit patch via global /events stream
    StatusCode::NO_CONTENT
}
```

Bad (spec):
```json
{
  "app_mappings": {
    "backend_responses": [{ "action_id": "sync_counter", "updates": [] }],
    "sse_events": [{ "event_name": "counter_update", "updates": [] }]
  }
}
```

Good (spec):
```json
{
  "app_mappings": {
    "backend_responses": [],
    "sse_events": [
      { "event_name": "datastar-patch-signals", "updates": [] }
    ]
  }
}
```

## Notes
- JSON responses are fine for non-Datastar APIs.
- This skill applies only to Datastar command paths that converge app-authority state through SSE.
