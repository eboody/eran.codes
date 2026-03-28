# Docs Hub

This repo has two documentation layers:
- the root [README.md](../README.md) explains why the project matters
- this hub explains where to go next, depending on what you want to inspect

## Start Here

- [Root README](../README.md) for the current thesis, proof surfaces, and quick evaluation path
- [Presentation Remediation Plan](./presentation-remediation-plan.md) for the active work to close the remaining presentation audit findings
- [Presentation Verification Plan](./presentation-tightening-plan.md) for the earlier signed-in smoke and visual-regression hardening pass
- [Resume Realignment Status](./resume-realignment.md) for what the original proof pivot identified and what has now shipped
- [Resume Alignment Packaging Status](./refactor-plan.md) for the historical shared-content, IA-hardening, supporting-proof route-policy, and archive-collapse passes
- [Site Content Authoring Guide](./site-content-authoring.md) for what belongs in `site_content/` versus runtime-owned code
- [Sensitive Sync Architecture](./sensitive-sync-architecture.md) for the new encrypted-storage, token-lifecycle, and bounded-sync proof
- [Auth + Sessions](./auth-sessions.md) for the session/auth portion of the shipped security proof
- [Portfolio Demo Concepts](./portfolio-demos.md) for what each major demo is proving today

## Architecture And Boundaries

- [Presentation Remediation Plan](./presentation-remediation-plan.md)
- [Presentation Verification Plan](./presentation-tightening-plan.md)
- [Sensitive Sync Architecture](./sensitive-sync-architecture.md)
- [Professionalism In Practice](./professionalism-breakdown.md)
- [Project Audit](./project-audit.md)
- [Resume Alignment Packaging Status](./refactor-plan.md)
- [Site Content Authoring Guide](./site-content-authoring.md)
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

## Release Gate

- Before downstream publish or accept steps, run `bash scripts/check_publish_dry_run.sh`.
- That gate includes local workspace tests, a Docker runtime build-and-boot smoke check that hits `/health` and then renders the live portfolio routes, and a local `act -j repo-checks` run against the repo CI workflow.
- The portfolio browser smoke now compares the stable portfolio routes against committed baselines by default under `artifacts/visual/baseline/portfolio-smoke`, so visual regressions fail the local publish gate instead of staying advisory.
- The default Docker browser-smoke gate now runs both guest and signed-in coverage. Use `DOCKER_SMOKE_INCLUDE_SIGNED_IN=0` only when you intentionally need a guest-only pass.
- `/lab` stays in the smoke path for live route/assertion coverage, but its operations timeline is still treated as a volatile surface rather than a pixel-locked baseline.
- `repo-checks` also verifies the Docker runtime image keeps the expected `/health` probe and includes both `curl` and `wget` for host-platform healthchecks.
- This repo also ships a `.githooks/pre-push` hook for the same gate; enable it locally with `git config core.hooksPath .githooks`.
- To refresh the default baselines intentionally, run `PORTFOLIO_SMOKE_UPDATE_BASELINE=1 bash scripts/check_portfolio_browser_smoke.sh`.
- To refresh the signed-in baselines intentionally, run `PORTFOLIO_SMOKE_SESSION_MODE=signed-in PORTFOLIO_SMOKE_UPDATE_BASELINE=1 bash scripts/check_portfolio_browser_smoke.sh`.
- For a heavier guest visual pass, run `PORTFOLIO_SMOKE_MODE=matrix bash scripts/check_portfolio_browser_smoke.sh`, and add `PORTFOLIO_SMOKE_UPDATE_BASELINE=1` when you intentionally refresh the wider matrix under `artifacts/visual/baseline/portfolio-smoke`.
- For the heavier signed-in pass, add `PORTFOLIO_SMOKE_SESSION_MODE=signed-in` to the same matrix command.
- To run the full publish gate in its default guest-plus-signed-in shape, use `bash scripts/check_publish_dry_run.sh`.
- To force a guest-only publish pass temporarily, run `DOCKER_SMOKE_INCLUDE_SIGNED_IN=0 bash scripts/check_publish_dry_run.sh`.

## Suggested Reading Paths

- **Hiring manager**
  - [Root README](../README.md) -> [Sensitive Sync Architecture](./sensitive-sync-architecture.md) -> [Auth + Sessions](./auth-sessions.md) -> [HTTP crate](../crates/http/README.md)

- **Senior engineer**
  - [Root README](../README.md) -> [Sensitive Sync Architecture](./sensitive-sync-architecture.md) -> [Project Audit](./project-audit.md) -> [domain](../crates/domain/README.md) -> [app](../crates/app/README.md) -> [infra](../crates/infra/README.md) -> [http](../crates/http/README.md)

- **Runtime reviewer**
  - [Root README](../README.md) -> [Sensitive Sync Architecture](./sensitive-sync-architecture.md) -> [Portfolio Demo Concepts](./portfolio-demos.md) -> [http::views](../crates/http/src/views/README.md) -> [http::sse](../crates/http/src/sse/README.md)
