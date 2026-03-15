<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./crates/http/static/eran.codes-dark.svg">
    <img alt="eran.codes logo" src="./crates/http/static/eran.codes-light.svg" width="420">
  </picture>
  <p><strong>A Rust portfolio application built to demonstrate architecture, realtime delivery, and operational clarity under load.</strong></p>
  <p>
    <a href="https://eran.codes">Live site</a>
    ·
    <a href="https://github.com/eboody/eran.codes">Source</a>
    ·
    <a href="./docs/README.md">Docs hub</a>
  </p>
  <p>
    <a href="https://github.com/eboody/eran.codes/actions/workflows/ci.yml"><img src="https://github.com/eboody/eran.codes/actions/workflows/ci.yml/badge.svg?branch=main&event=push" alt="CI status" /></a>
  </p>
</div>

# eran.codes

`eran_codes` is the codebase behind my portfolio site. It is designed to show how I build software when boundaries, runtime behavior, and maintainability matter more than shipping another generic web app.

The demos are not decoration. They are the architecture made inspectable.

![Runtime architecture overview](./docs/static/readme/runtime-architecture.svg)

## What This Repo Is Designed To Prove

- **Boundary discipline.** `http`, `app`, `domain`, and `infra` are separate crates with distinct responsibilities, not one module tree with polite comments.
- **Realtime delivery without hand-waving.** UI state converges through SSE + Datastar, with session-aware stream routing instead of ad hoc polling.
- **Observability as a product feature.** Request metadata, backend flow, and live operational timelines are visible in the UI, not buried in logs alone.
- **Operational thinking.** Sessions are durable, tracing is layered, and the request-burst surface stresses the app without abandoning clarity.
- **Typed view and domain contracts.** Maud components, newtypes, enums, and workflow surfaces replace template sprawl and brittle stringly logic.

## Evaluate It In Five Minutes

1. Visit [`/lab`](https://eran.codes/lab).
   This is the fastest proof surface. It combines live chat, request burst traffic, auth/session panels, and the operational timeline.

2. Visit the focused work pages.
   - [`/work/chat-realtime`](https://eran.codes/work/chat-realtime)
   - [`/work/command-sse`](https://eran.codes/work/command-sse)
   - [`/work/operational-visibility`](https://eran.codes/work/operational-visibility)

3. Read the engineering rationale.
   - [Docs hub](./docs/README.md)
   - [Professionalism In Practice](./docs/professionalism-breakdown.md)
   - [Portfolio Demo Concepts](./docs/portfolio-demos.md)
   - [Auth + Sessions](./docs/auth-sessions.md)
   - [Tracing Plan](./docs/tracing.md)

4. Inspect the crate boundaries.
   - [domain](./crates/domain/README.md)
   - [app](./crates/app/README.md)
   - [infra](./crates/infra/README.md)
   - [http](./crates/http/README.md)

## Architecture At A Glance

```text
browser
  -> axum router + handlers
  -> app services
  -> domain types and invariants
  -> infra repositories + Postgres + Argon2

global SSE stream
  -> session/tab keyed registry
  -> Datastar patches for UI convergence

trace pipeline
  -> live operational surface for the reader
  -> deeper diagnostic stream for engineering visibility
```

`[src/main.rs](./src/main.rs)` is the composition root. It wires tracing, Postgres-backed sessions, app services, the SSE registry, and the HTTP router into one runtime.

## Core Runtime Surfaces

| Surface | Path | What it demonstrates |
| --- | --- | --- |
| Lab | [`/lab`](https://eran.codes/lab) | Live chat, request burst traffic, auth/session panels, and the operational timeline |
| Chat system | [`/work/chat-realtime`](https://eran.codes/work/chat-realtime) | Persisted messaging, moderation, rate limiting, and SSE fanout |
| Command + SSE | [`/work/command-sse`](https://eran.codes/work/command-sse) | Datastar command flow, server-authoritative state, and SSE convergence |
| Operational visibility | [`/work/operational-visibility`](https://eran.codes/work/operational-visibility) | Request tracing, backend flow grouping, and UI-visible runtime behavior |
| Auth durability | [`/register`](https://eran.codes/register) -> [`/login`](https://eran.codes/login) -> [`/protected`](https://eran.codes/protected) | Session lifecycle, auth enforcement, and secure persistent sessions |

## Run It Locally

Required environment:
- `HOST`
- `PORT`
- `DATABASE_URL`
- `SESSION_SECRET` (`base64url`, no padding, 64 bytes)

Optional environment:
- `SESSION_CLEANUP_INTERVAL_SECS` (default `3600`)
- `INFRA_DB_MAX_CONNECTIONS` (default `10`)
- `LOG_FORMAT` (`pretty` or `json`)

Start the app:

```bash
docker-compose up -d
cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations
cargo run
```

Then open `http://127.0.0.1:3000/` or `http://127.0.0.1:3000/lab`.

## Codebase Guide

| Area | Role | Start here |
| --- | --- | --- |
| [domain](./crates/domain/README.md) | Pure business types and invariants | `user` and `chat` modules |
| [app](./crates/app/README.md) | Use cases, policy, and external contracts | auth and chat services |
| [infra](./crates/infra/README.md) | Postgres, hashing, repositories, and migrations | auth repo, chat repo, config |
| [http](./crates/http/README.md) | Router, handlers, SSE, Maud views, trace surfaces | router, handlers, views, `trace_log` |
| [utils](./crates/utils/README.md) | Small shared helpers and developer tooling | `visual_snapshot` and support utilities |

## Read The System Like This

- **Start at the docs hub**
  - [docs/README.md](./docs/README.md)

- **Why it is structured this way**
  - [Professionalism In Practice](./docs/professionalism-breakdown.md)
  - [Project Audit](./docs/project-audit.md)
  - [Refactor Plan](./docs/refactor-plan.md)

- **How the runtime behaves**
  - [Auth + Sessions](./docs/auth-sessions.md)
  - [Tracing Plan](./docs/tracing.md)
  - [Portfolio Demo Concepts](./docs/portfolio-demos.md)

- **How the HTTP surface is organized**
  - [HTTP crate](./crates/http/README.md)
  - [HTTP internals](./crates/http/src/README.md)
  - [Handlers](./crates/http/src/handlers/README.md)
  - [Views](./crates/http/src/views/README.md)

## Design Bias

This repo intentionally optimizes for:
- explicit boundaries over convenience coupling
- typed invariants over stringly state
- visible runtime behavior over hidden magic
- reusable render components over template duplication
- clear composition roots over implicit wiring

That bias is the point of the project.
