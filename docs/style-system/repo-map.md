# Repo Map

Inspect these files first for styling tasks in this repo:

## Doctrine

- `docs/style-system/index.md`
- `docs/style-system/token-layers.md`
- `docs/style-system/modern-css.md`
- `docs/style-system/promotion-rules.md`
- `docs/style-system/package-catalog.md`
- `docs/style-system/component-checklist.md`

## Existing Behavior

- `docs/reference-map.md`
- `docs/css-scope-inline/index.md`
- `crates/http/static/app.css`
- `crates/http/src/views/page.rs`
- shared components under `crates/http/src/views/partials/components/**`

## Enforcement

- `scripts/ci/style-system-consistency.sh`
- `scripts/style-system-audit.sh`
- generated specs under `generated/**`
- fixture specs under `tests/fixtures/**`

## Current Repo Facts

- The repo already uses a hybrid styling contract in component specs.
- Shared `ui-*` packages already exist in `app.css`.
- Live view code currently leans more on shared classes than active scoped CSS blocks.
