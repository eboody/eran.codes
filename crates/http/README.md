# http

`http` is where the portfolio becomes a working system: routing, auth/session transport, Datastar endpoints, SSE fanout, Maud views, and the live operational surface.

## What it owns
- router and middleware composition
- page handlers, Datastar partial handlers, and EventSource endpoints
- session-aware stream routing and event helpers
- page, partial, and component rendering surfaces
- trace surfaces that feed the operational UI

## Runtime Proof Surfaces
- [`/`](https://eran.codes/) for the portfolio landing page
- [`/lab`](https://eran.codes/lab) for the operational demo surface
- [`/events`](https://eran.codes/events) for the EventSource stream
- `/demo/chat/*` and `/partials/*` for the interactive support surfaces behind the demos

If you want to evaluate this crate quickly, start with `/lab` and the EventSource-backed surfaces. `http` is where transport behavior becomes inspectable.

## What to inspect in code
- [src/router/routes.rs](./src/router/routes.rs) for public route shape
- [src/state.rs](./src/state.rs) for HTTP-owned runtime state
- [src/trace_log.rs](./src/trace_log.rs) for live vs diagnostic tracing behavior
- [src/views/page.rs](./src/views/page.rs) for the site shell
- [src/views/partials/README.md](./src/views/partials/README.md) for the reusable component vs demo-composition split

## Fast read order
1. [src/README.md](./src/README.md)
2. [src/router/README.md](./src/router/README.md)
3. [src/handlers/README.md](./src/handlers/README.md)
4. [src/views/README.md](./src/views/README.md)
5. [src/sse/README.md](./src/sse/README.md)

## Why this crate exists
It keeps transport, rendering, and realtime delivery close together while still treating `app` and `domain` as the source of business truth. It is also where the repo proves that interactive pages can be assembled from reusable typed components instead of growing as route-local template sprawl.

## Read next
- [root README](../../README.md)
- [Auth + Sessions](../../docs/auth-sessions.md)
- [Tracing Plan](../../docs/tracing.md)
