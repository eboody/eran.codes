# Docs Hub

This repo has two documentation layers:
- the root [README.md](../README.md) explains why the project matters
- this hub explains where to go next, depending on what you want to inspect

## Start Here

- [Root README](../README.md) for the project thesis, runtime surfaces, and quick evaluation path
- [Professionalism In Practice](./professionalism-breakdown.md) for the engineering rationale behind the codebase
- [Portfolio Demo Concepts](./portfolio-demos.md) for what each major demo is proving

## Architecture And Boundaries

- [Project Audit](./project-audit.md)
- [Refactor Plan](./refactor-plan.md)
- [Auth + Sessions](./auth-sessions.md)
- [Tracing Plan](./tracing.md)
- [Reference Map](./reference-map.md) for docs-first source routing inside the repo

## Crate Guide

- [domain](../crates/domain/README.md) for business types and invariants
- [app](../crates/app/README.md) for use cases, policy, and contracts
- [infra](../crates/infra/README.md) for Postgres, hashing, migrations, and repositories
- [http](../crates/http/README.md) for routing, SSE, views, and operational surfaces
- [utils](../crates/utils/README.md) for shared helpers and developer tooling

## HTTP Deep Dive

- [http::src](../crates/http/src/README.md)
- [http::router](../crates/http/src/router/README.md)
- [http::handlers](../crates/http/src/handlers/README.md)
- [http::handlers::demo](../crates/http/src/handlers/demo/README.md)
- [http::sse](../crates/http/src/sse/README.md)
- [http::views](../crates/http/src/views/README.md)
- [http::views::pages](../crates/http/src/views/pages/README.md)
- [http::views::partials](../crates/http/src/views/partials/README.md)

## Domain And App Deep Dive

- [domain::src](../crates/domain/src/README.md)
- [domain::user](../crates/domain/src/user/README.md)
- [domain::chat](../crates/domain/src/chat/README.md)
- [app::src](../crates/app/src/README.md)
- [app::user](../crates/app/src/user/README.md)
- [app::chat](../crates/app/src/chat/README.md)
- [infra::src](../crates/infra/src/README.md)
- [infra::repo](../crates/infra/src/repo/README.md)
- [utils::src](../crates/utils/src/README.md)

## Frontend And Realtime Notes

- [Datastar Tao](./datastar-tao.md)
- [Datastar Signals](./datastar-signals.md)
- [Datastar Expressions](./datastar-expressions.md)
- [Datastar Backend Requests](./datastar-backend-requests.md)
- [Style System Index](./style-system/index.md)

## Suggested Reading Paths

- **Hiring manager**
  - [Root README](../README.md) -> [Professionalism In Practice](./professionalism-breakdown.md) -> [HTTP crate](../crates/http/README.md)

- **Senior engineer**
  - [Root README](../README.md) -> [Project Audit](./project-audit.md) -> [domain](../crates/domain/README.md) -> [app](../crates/app/README.md) -> [infra](../crates/infra/README.md) -> [http](../crates/http/README.md)

- **Frontend / realtime reviewer**
  - [Root README](../README.md) -> [Portfolio Demo Concepts](./portfolio-demos.md) -> [http::views](../crates/http/src/views/README.md) -> [http::sse](../crates/http/src/sse/README.md)
