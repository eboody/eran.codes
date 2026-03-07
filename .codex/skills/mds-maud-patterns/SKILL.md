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
- Prefer typed composition edges for reusable parent props (for example `Vec<Panel>`), not generic `Vec<maud::Markup>`.
- Avoid raw `maud::Markup` slots for variant-bearing children when a typed child component can encode invariants.
- If a reusable component must expose a `maud::Markup` slot (for rich content/cells), annotate the file with `// ci: markup-slot-exempt <reason>`.
- Reuse primitives composition-first (for example, compose `Icon` inside `Tab`) rather than duplicating leaf markup.
- Keep behavior variants typed (enum-driven render branches) instead of raw string flags.
- For semantic finite sets (`*mode`, `*variant`, `*kind`, `*role`, `*type`, etc.), prefer enums/newtypes over `Text`/`String` fields.
- Prefer `bon` builders for component construction APIs:
  - Use typestate builder/state-machine style when required construction order matters.
  - Use regular `#[derive(Builder)]` when named setters improve clarity and typestate would add noise.
- Prefer module-scoped render APIs and qualified calls (`tab_set::pane::Body`) over flat prefixed type names.
- For log surfaces, split reusable render pieces by tier:
  - primitive building blocks under `views/partials/components/logs/primitives`
  - composed log views under `views/partials/components/logs/composed`
  - keep trace-entry mapping in a separate VM layer (`views/partials/demo/log/vm`).
- For action-trace UX, prefer a request-flow timeline panel (grouped by `request_id`) alongside raw HTTP/SSE tables so users can follow request -> backend -> SSE causality.
- Keep trace/event mapping code lean by splitting dense files into focused module families (`kind`, `event_builder`, `helpers`) instead of one large mapper file.
- When space is constrained, keep the flow timeline as the primary visible log surface and treat raw tables/secondary logs as optional or hidden by default.
- Preserve stable patch-target roots (for example `network-log-target`) when refactoring internals so SSE `patch_elements` convergence does not drift.
