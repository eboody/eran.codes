# Reference Map

This file is the local source-of-truth router for tool and pattern documentation.

## Source Order

Use sources in this order:
1. Local docs listed in this map (`docs/**`).
2. Repository code for current behavior (`src/**`, `crates/**`, `scripts/**`).
3. Internet lookup only when local docs are missing or stale.

## Output Grounding Contract

For non-trivial implementation/review/design responses, include:
- `sources_used`: exact local file paths consulted.
- `unresolved_items`: items not found in local docs/code.

If a requested fact is not present in local sources, explicitly say it is not found and avoid guessing.

## Task Routing

### Architecture and Boundaries
- `docs/writing-style.md`
- `docs/project-audit.md`
- `docs/refactor-plan.md`
- `docs/code-audit/01-architecture-map.md`
- `docs/auth-sessions.md`

### Tracing and Observability
- `docs/tracing.md`
- `docs/tracing/route-map.md`
- `docs/tracing/index.md`

### Datastar and Frontend Reactivity
- `docs/datastar-tao.md`
- `docs/datastar-signals.md`
- `docs/datastar-expressions.md`
- `docs/datastar-backend-requests.md`
- `docs/datastar/guide.md`
- `docs/datastar/reference.md`

### Surreal and Interaction Scripting
- `docs/surreal/05-usage.md`
- `docs/surreal/08-no-surreal-needed.md`
- `docs/surreal/07-reference.md`

### CSS Scoping and Styling
- `docs/style-system/index.md`
- `docs/style-system/token-layers.md`
- `docs/style-system/modern-css.md`
- `docs/style-system/promotion-rules.md`
- `docs/style-system/package-catalog.md`
- `docs/style-system/component-checklist.md`
- `docs/style-system/repo-map.md`
- `docs/css-scope-inline/index.md`
- `docs/css-scope-inline/03-how-it-works.md`
- `docs/css-scope-inline/06-workflow-tips.md`

### Maud Templates and Rendering
- `docs/maud/index.md`
- `docs/maud/partials.md`
- `docs/maud/render-trait.md`
- `docs/maud/elements-attributes.md`

### Visual UX/UI and Showcase Direction
- `docs/portfolio-demos-plan.md`
- `docs/portfolio-demos.md`
- `docs/professionalism-breakdown.md`
- `artifacts/visual/audits/latest/ux-signoff.md`
- `artifacts/visual/audits/latest/ui-signoff.md`
- `artifacts/visual/audits/latest/fullstack-rust-hiring-manager-signoff.md`
- `artifacts/visual/audits/latest/rust-systems-hiring-manager-signoff.md`
- `artifacts/visual/audits/latest/signoff.env`

### Axum HTTP Patterns
- `docs/axum/index.md`
- `docs/axum/routing/index.md`
- `docs/axum/extract/index.md`
- `docs/axum/response/index.md`
- `docs/axum/middleware/index.md`

### Builder and Typestate Guidance
- `docs/bon/guide/overview.md`
- `docs/bon/guide/patterns/conditional-building.md`
- `docs/bon/guide/patterns/optional-generic-members.md`
- `docs/bon/guide/patterns/shared-configuration.md`
- `docs/bon/guide/patterns/fallible-builders.md`
- `docs/bon/guide/patterns/into-conversions-in-depth.md`
- `docs/statum/README.md`
- `docs/nestum/README.md`

### Modeling Utilities
- `docs/strum/index.md`
- `docs/nutype/index.md`

## Fast Lookup Commands

```bash
rg --line-number --ignore-case '<term>' docs/reference-map.md docs
rg --line-number --ignore-case '<term>' crates src scripts
```
