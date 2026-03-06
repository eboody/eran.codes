---
name: mds-css-conventions
version: 0.1.0
description: CSS conventions for generated components (class naming, layout expectations, minimal styling rules).
scope: project
---

# mds-css-conventions

## Purpose
Apply repo CSS conventions and css-scope-inline usage patterns to class design and style scoping decisions used by generated components.

## Usage
- Use before finalizing class names and style scoping assumptions.
- Enforce local conventions from `/docs/css-scope-inline` and writing/style signoff docs.
- Flag unresolved stylistic conflicts for explicit override review.
- Run a styling-system review before marking component work complete.

## Hybrid Styling Policy (Default)
- Use a hybrid model:
  - Global `app.css` for reusable package-deal patterns and shared design tokens.
  - Scoped `inline_css!` only for component-specific behavior that is not reusable.
- Reusable package classes should be generic and library-oriented (`ui-tabs`, `ui-tab`, `ui-panel`, `ui-preview-frame`, `ui-feature-list`, `ui-cta`).

## What Must Be Global (`app.css`)
- Foundation: tokens/aliases for spacing, type, radius, elevation, borders, and shared surface patterns.
- Reusable component packages: tabs, panel shells, preview frames, feature lists, CTA surfaces.
- Layout primitives that are shared across unrelated components/pages.

## What May Stay Scoped
- One-off component-specific styling with no reuse value.
- Highly local visual adjustments that would pollute global packages.
- Any scoped rule left inline must be documented as a scoped exception in `component_spec.styling.scoped_exceptions`.

## Extraction Rules
- If a style pattern appears in two or more components, extract to global package classes.
- Prefer consuming tokens/aliases already present in `app.css`.
- Avoid raw magic values when equivalent tokens exist.
- Prefer OpenProps token aliases (`--size-*`, `--radius-*`, `--border-size-*`, etc.) in reusable package classes.
