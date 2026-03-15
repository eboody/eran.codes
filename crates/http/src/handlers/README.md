# http::handlers

Handlers are grouped by transport concern so the route map stays readable and the code matches the public surface.

## Modules
- [auth/mod.rs](./auth/mod.rs) for login, registration, logout, and page gating
- [pages/mod.rs](./pages/mod.rs) for full-page handlers and page-scoped Datastar commands
- [demo/README.md](./demo/README.md) for chat endpoints and interactive demo partials
- [sse/mod.rs](./sse/mod.rs) for the EventSource stream and surreal-message demo endpoints

## Read order
1. [pages/mod.rs](./pages/mod.rs)
2. [auth/mod.rs](./auth/mod.rs)
3. [demo/README.md](./demo/README.md)
4. [../request.rs](../request.rs) and [../request_context_flow.rs](../request_context_flow.rs)

## Guidelines
- Map transport inputs into `app` or `domain` types early.
- Keep response mapping at the HTTP edge.
- Keep orchestration small; long-lived business decisions belong in `app`.
