---
name: mds-css-conventions
version: 0.1.0
description: CSS conventions for generated components (class naming, layout expectations, minimal styling rules).
scope: project
---

# mds-css-conventions

## Purpose
Apply repo CSS conventions and css-scope-inline usage patterns to class design and style scoping decisions used by generated components.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic CSS structure.

## Usage
- Use before finalizing class names and style scoping assumptions.
- Enforce local conventions from `crates/http/static/app.css`, current component code, and the live repo styling surface instead of the old mirrored `/docs` corpus.
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
- Log package classes (`ui-log-*`) used by reusable log components (surface, panel, table, grouped feed, empty state).
- Flow tracing packages (`ui-log-flow-*`) used by request timeline components (flow shell, selector list, detail lane).

## What May Stay Scoped
- One-off component-specific styling with no reuse value.
- Highly local visual adjustments that would pollute global packages.
- Any scoped rule left inline must be documented as a scoped exception in `component_spec.styling.scoped_exceptions`.

## Extraction Rules
- If a style pattern appears in two or more components, extract to global package classes.
- Prefer consuming tokens/aliases already present in `app.css`.
- Avoid raw magic values when equivalent tokens exist.
- Prefer OpenProps token aliases (`--size-*`, `--radius-*`, `--border-size-*`, etc.) in reusable package classes.
- Do not leave inline `css!` blocks in reusable log components; log styling should be consumed via global `ui-log-*` classes.

## Modern CSS Adoption Rule
- Use modern CSS/HTML features when they exist and they simplify behavior otherwise requiring JS.
- Apply progressive enhancement:
  - Ship a stable baseline first.
  - Layer modern behavior inside `@supports` (or equivalent capability checks).
  - Keep accessibility and keyboard behavior intact if enhancements are unsupported.
- Prefer these features when relevant:
  - Popover/dialog controls: `command`/`commandfor`, `popover="hint"`, `interestfor`, dialog light-dismiss (`closedby`).
  - Scroll/carousel primitives: `::scroll-button`, `::scroll-marker`, `scroll-target-group`, `:target-current`, scroll-state queries.
  - Query/dynamic syntax: typed `attr`, `if`, `@function`, expanded range syntax, anchored/container queries.
  - Presentation primitives: customizable select pseudo-elements, `text-box-*`, `corner-shape`, `shape`, `stretch`, `sibling-index`/`sibling-count`.
