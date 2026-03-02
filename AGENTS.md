# Repository Guidelines

## Coordinator Purpose
- This file is intentionally thin and acts as the main coordinator policy.
- It defines non-negotiables plus delegation rules.
- Domain-specific details now live in specialist skills under `skills/*/SKILL.md`.

## Project Structure
- `src/` holds binaries (`src/main.rs`, `src/bin/with_db.rs`).
- `crates/` is layered: `domain`, `app`, `infra`, `http`, `utils`.
- `crates/http/src/views/` contains Maud pages/partials.
- `crates/http/static/` contains shared CSS and frontend helpers.
- `crates/infra/migrations/` contains SQL migrations.

## Core Non-Negotiables
- Respect dependency direction: `domain <- app <- (http, infra) <- main`.
- Keep transport and persistence concerns out of `domain` and `app` core models.
- Avoid stringly invariants: prefer enums/newtypes (`strum`/`nutype`) over raw `String`.
- In app services, keep hasher failures typed; do not collapse to stringly errors.
- Public view partials must implement `maud::Render`.
- For marked descriptive namespaces, follow `scripts/ci/descriptive-module-imports.sh`.
- For visual changes in `crates/http/src/views/**` or `crates/http/static/**`, visual signoff is mandatory.
- For non-trivial design/review guidance, use local docs first via `docs/reference-map.md` and cite the files used.

## Delegation Model
- For non-trivial requests, run `router` first and classify ownership.
- During router classification, always evaluate `bon`, `statum`, and `nestum`.
- Delegate in parallel when ownership does not overlap.
- Keep small/single-scope tasks in the coordinator to avoid unnecessary handoff latency.
- Require source-grounding from local docs and code paths in specialist outputs.

## When Not To Delegate
All conditions should be true:
- Single-layer change with clear ownership.
- No auth/session/security/persistence boundary change.
- No visual/UI behavior change.
- No new lint/guardrail design.
- No review/audit request that needs severity-ranked findings.

## Required Delegation Rubric
- Endpoints, DTO boundaries, auth/session, persistence mapping, trait placement:
  - delegate to `$architecture-boundary-enforcer`.
- Visual/UI work (layout, styling, interaction states, responsive behavior):
  - always include both `$ux-expert` and `$ui-expert`.
- Visual/UI implementation that touches markup/CSS/client behavior:
  - also include `$visual-snapshot-check`.
- Repeated convention misses or requests to prevent recurrence:
  - delegate to `$lint-guardrail-engineer`.
- Code review, risk audit, bug hunt, regression hardening, test strategy:
  - delegate to `$testing-review-engineer`.
- Builder-heavy/config pipeline work:
  - include `$bon`.
- Typestate/state-machine/invariant flow design:
  - include `$statum`.
- Nestum architecture/API/integration work:
  - include `$nestum`.

## Specialist Ownership
- `$architecture-boundary-enforcer`:
  - crate boundaries, DTO/command/entity/row separation, auth/password boundaries.
- `$ux-expert` + `$ui-expert`:
  - UX flow quality, visual system quality, accessibility and interaction quality.
- `$visual-snapshot-check`:
  - screenshot baseline workflow and visual signoff gate.
- `$lint-guardrail-engineer`:
  - convert recurring mistakes into enforceable checks.
- `$testing-review-engineer`:
  - findings-first review and risk-driven test coverage.

## Visual Gate (Hard Requirement)
When changing `crates/http/src/views/**` or `crates/http/static/**`:
1. Run `scripts/ci/visual-snapshot.sh` (or baseline refresh when intentional).
2. Ensure signoff artifacts exist:
   - `artifacts/visual/audits/latest/ux-signoff.md`
   - `artifacts/visual/audits/latest/ui-signoff.md`
   - `artifacts/visual/audits/latest/signoff.env`
3. Run `scripts/ci/visual-expert-signoff.sh`.

## Build, Test, and Development Commands
- `cargo build`
- `cargo test`
- `cargo check`
- `cargo run` (requires `HOST`, `PORT`, `DATABASE_URL`, `SESSION_SECRET`)
- `docker-compose up -d`
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations`

## Configuration Notes
- Required env vars: `HOST`, `PORT`, `DATABASE_URL`, `SESSION_SECRET`.
- Optional env vars: `SESSION_CLEANUP_INTERVAL_SECS`, `INFRA_DB_MAX_CONNECTIONS`.
- Never commit secrets.

## Maintenance Rule
- Keep this file as coordinator policy only.
- Put deep implementation guidance in specialist skills.
- Update this file when delegation boundaries or hard CI gates change.
