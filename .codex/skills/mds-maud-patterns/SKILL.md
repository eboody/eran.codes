---
name: mds-maud-patterns
version: 0.1.0
description: Maud patterns and conventions used in this repo (view composition, escaping, partials, render usage).
scope: project
---

# mds-maud-patterns

## Purpose
Apply repo-approved Maud composition, escaping, and structure patterns when building `component_spec.ui` and generated Maud templates.

## Usage
- Use before defining `ui.nodes`, slot strategy, and render patterns.
- Prefer documented Maud idioms over agent priors.
- Defer unresolved ambiguities to `mds-docs-librarian`.

## Render Composition Contract
- Prefer typed render components over large inline render blocks.
- Where repeated UI exists, model child component structs that each `impl Render`.
- Parent components should accept child component collections as props and compose them directly in Maud.
- Reuse primitives composition-first (for example, compose `Icon` inside `Tab`) rather than duplicating leaf markup.
- Keep behavior variants typed (enum-driven render branches) instead of raw string flags.
- Prefer module-scoped render APIs and qualified calls (`tab_set::pane::Body`) over flat prefixed type names.
- For log surfaces, keep render primitives under `views/partials/components/logs` and keep trace-entry mapping in a separate VM layer (`views/partials/demo/log/vm`).
- Preserve stable patch-target roots (`live-log-target`, `network-log-target`) when refactoring internals so SSE `patch_elements` convergence does not drift.
