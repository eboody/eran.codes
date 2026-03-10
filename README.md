# eran_codes

This is my portfolio codebase. The point is not demo count, it is engineering quality: clear boundaries, typed invariants, readable composition, and observable runtime behavior.

## What this project demonstrates
- Domain/app/http/infra boundaries that are enforced in code, not just described in docs.
- Durable auth + session handling with Postgres-backed session storage.
- SSE + Datastar delivery for live UI updates.
- Reusable Maud components instead of one-off template markup.
- Trace/log architecture split into live user-facing signals vs diagnostic-only events.
- High-volume request burst demo showing throughput and log behavior under load.

## Quickstart
Required env vars:
- `HOST`
- `PORT`
- `DATABASE_URL`
- `SESSION_SECRET` (base64url, no padding, 64 bytes)

Optional env vars:
- `SESSION_CLEANUP_INTERVAL_SECS` (defaults to `3600`)
- `INFRA_DB_MAX_CONNECTIONS` (defaults to `10`)

Run locally:
1. `docker-compose up -d`
2. `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`
3. `cargo run`

## Core commands
- `cargo build`
- `cargo test`
- `cargo check`
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`

CI guardrails:
- `scripts/ci/stringy-check.sh`
- `scripts/ci/no-string-fields.sh`
- `scripts/ci/typed-hasher-errors.sh`
- `scripts/ci/bon-usage-contract.sh`
- `scripts/ci/descriptive-module-imports.sh`
- `scripts/ci/partials-render.sh`
- `scripts/ci/visual-snapshot.sh` (requires app running at `VISUAL_URL`, defaults to `http://127.0.0.1:3000/`)

Visual baseline workflow:
- Update baseline: `VISUAL_UPDATE_BASELINE=1 scripts/ci/visual-snapshot.sh`
- Verify current UI: `scripts/ci/visual-snapshot.sh`

## Docker
- Build: `docker build -t eran_codes .`
- Run: `docker run -p 3000:3000 -e DATABASE_URL=... -e SESSION_SECRET=... eran_codes`

## Architecture audit
Current architecture and quality audit:
- `docs/project-audit.md`

Prioritized refactor plan from that audit:
- `docs/refactor-plan.md`

## Workspace structure
- `crates/domain`: business types + invariants
- `crates/app`: use-cases + orchestration traits
- `crates/infra`: SQL/persistence + mechanism implementations
- `crates/http`: routing, handlers, SSE, Maud views
- `crates/utils`: shared helpers
- `src/main.rs`: composition root

## Tracing
- Configure output with `RUST_LOG`.
- Keep tracing conventions aligned with `docs/tracing.md`.
- New request flows must classify events as `live` (SSE-visible) or `diagnostic` (non-SSE).

## Documentation map
Top-level:
- `AGENTS.md`
- `bon.md`
- `docs/writing-style.md`
- `docs/auth-sessions.md`
- `docs/tracing.md`
- `docs/datastar-tao.md`
- `docs/datastar-signals.md`
- `docs/datastar-expressions.md`
- `docs/datastar-backend-requests.md`
- `docs/portfolio-demos.md`
- `docs/portfolio-demos-plan.md`
- `docs/project-audit.md`
- `docs/refactor-plan.md`
- `docs/professionalism-breakdown.md`

Crate-level:
- `crates/domain/README.md`
- `crates/app/README.md`
- `crates/infra/README.md`
- `crates/http/README.md`
- `crates/utils/README.md`

HTTP internals:
- `crates/http/src/README.md`
- `crates/http/src/router/README.md`
- `crates/http/src/handlers/README.md`
- `crates/http/src/handlers/demo/README.md`
- `crates/http/src/sse/README.md`
- `crates/http/src/views/README.md`
- `crates/http/src/views/pages/README.md`
- `crates/http/src/views/partials/README.md`
