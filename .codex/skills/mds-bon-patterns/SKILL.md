---
name: mds-bon-patterns
version: 0.1.0
description: Bon builder usage patterns for this repo (optional members, conditional building, into/default semantics).
scope: project
---

# mds-bon-patterns

## Purpose
Enforce repo-standard `bon` usage so builders stay readable and consistent with the Bon guide.

## Source Docs (Authoritative)
- `/docs/bon/guide/basics.md`
- `/docs/bon/guide/basics/optional-members.md`
- `/docs/bon/guide/patterns/conditional-building.md`
- `/docs/bon/reference/builder/member/default.md`
- `/docs/bon/reference/builder/member/into.md`

## Required Setter Rules
- For optional builder members, use the direct setter when value is known:
  - Good: `.request_id(Text::from("request-1"))`
  - Bad: `.maybe_request_id(Some(Text::from("request-1")))`
- Use `maybe_` setters only when you already have an `Option<T>` at the callsite.
  - Good: `.maybe_request_id(request_id_opt)`
- Use `maybe_...None` only when intentionally clearing/omitting a value (or triggering default semantics).
  - If `into` is enabled and type inference fails, annotate `None::<T>`.

## Conditional Building Rules
- Prefer precomputing optional values, then pass them once via `maybe_`:
  - `let alias: Option<Text> = ...; builder.maybe_alias(alias)`
- Avoid branching that duplicates long builder chains.

## API Selection Rules
- `#[derive(Builder)]` for struct/view construction.
- `#[builder]`/`#[bon]` for function or method builders (including fallible builders).
- Avoid hand-rolled builder APIs when `bon` can express the same construction cleanly.

## Common Mistakes To Reject
- `maybe_*` with `Some(...)` literals.
- `maybe_*` used for non-optional source values.
- builder callsites that hide intent by mixing "set known value" and "pass optional value" patterns inconsistently.
