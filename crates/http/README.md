# http

`http` is where the portfolio becomes a working system: routing, auth/session transport, Datastar endpoints, SSE fanout, Maud views, and the live operational surface.

## What it owns
- router and middleware composition
- page handlers, Datastar partial handlers, and EventSource endpoints
- session-aware stream routing and event helpers
- page and partial rendering
- trace surfaces that feed the operational UI

## Fast read order
1. [src/README.md](./src/README.md)
2. [src/router/README.md](./src/router/README.md)
3. [src/handlers/README.md](./src/handlers/README.md)
4. [src/views/README.md](./src/views/README.md)
5. [src/sse/README.md](./src/sse/README.md)

## Runtime surfaces
- [`/`](https://eran.codes/) for the portfolio landing page
- [`/lab`](https://eran.codes/lab) for the operational demo surface
- [`/events`](https://eran.codes/events) for the EventSource stream
- `/demo/chat/*` and `/partials/*` for the interactive support surfaces behind the demos

## What to inspect in code
- [src/router/routes.rs](./src/router/routes.rs) for public route shape
- [src/state.rs](./src/state.rs) for HTTP-owned runtime state
- [src/trace_log.rs](./src/trace_log.rs) for live vs diagnostic tracing behavior
- [src/views/page.rs](./src/views/page.rs) for the site shell

## Why this crate exists
It keeps transport, rendering, and realtime delivery close together while still treating `app` and `domain` as the source of business truth.

## Read next
- [root README](../../README.md)
- [Auth + Sessions](../../docs/auth-sessions.md)
- [Tracing Plan](../../docs/tracing.md)
