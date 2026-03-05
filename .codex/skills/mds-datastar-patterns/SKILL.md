---
name: mds-datastar-patterns
version: 0.1.0
description: Datastar attribute, signal, event, and SSE patch patterns used in this repo.
scope: project
---

# mds-datastar-patterns

## Purpose
Apply repo-approved Datastar signal, attribute, expression, and SSE interaction patterns to events/state/bindings design.

## Usage
- Use before defining `ui.bindings`, `events.handlers`, `events.effects`, and Datastar-related codegen decisions.
- Prefer docs-backed attribute and SSE rules over priors.
- Apply `.codex/skills/mds-datastar-architecture/SKILL.md` as the architecture baseline for command handlers and SSE convergence.
- Route edge cases through `mds-docs-librarian`.
- Classify interaction scope before choosing protocol:
  - `presentation`/`session` interactions default to `ui_local`.
  - `app` interactions use `command_sse` with SSE convergence.
- Respect state authority split:
  - `authority = "ui"` fields may use local `data-on:*` expressions.
  - `authority = "app"` fields must be updated only via SSE mappings (`datastar-patch-signals`), never by local UI expressions.
- Tabs/selectors are presentation concerns by default; do not require backend commands unless explicit app-level semantics are requested.
