# http::src

HTTP transport, routers, handlers, SSE, and view composition.

## Modules
- `handlers/` request handlers and Datastar endpoints.
- `router/` router and middleware composition.
- `sse/` SSE registry and events.
- `views/` Maud view components.
- `trace_log.rs` live and diagnostic tracing stores.

## Rules
- Map DTOs to app commands early.
- No DB or infra logic here.
