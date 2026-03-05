---
name: mds-repo-docs-index
version: 0.1.0
description: Docs-first policy and curated lookup index for /docs, including allowlist/denylist and conflict resolution rules.
scope: project
---

# mds-repo-docs-index

## Purpose
Define a single docs-first retrieval policy for the repo. Treat `/docs` as evergreen source of truth for Maud, Datastar, Axum, CSS conventions, writing style, visual signoff, and code-audit practices.

## Source Of Truth
- Primary corpus: `/docs` (read-only)
- Conflict rule: `/docs` overrides agent priors and defaults.
- Tie-breaker: when two docs conflict, use the most specific path for the current decision area; if still ambiguous, escalate to `mds-verifier` as a blocking warning.

## Authoritative Doc Roots (Allowlist)
Only these roots are authoritative for the Maud + Datastar component system:
- `/docs/maud/`
- `/docs/datastar/`
- `/docs/axum/`
- `/docs/css-scope-inline/`
- `/docs/code-audit/`
- `/docs/visual-signoff/latest/`
- `/docs/writing-style.md`
- `/docs/reference-map.md`

## Historical/Stale Docs (Denylist, Non-Authoritative)
Do not use these files to drive `component_spec` or verification decisions:
- `/docs/nestum/Agents.md` (legacy agent guidance for a different crate context)
- `/docs/statum/AGENTS.md` (legacy repository guidelines for a different crate context)

If a path is in this denylist, treat it as informational history only, not as policy input.

## Curated Map Of Doc Areas
- `maud`
  - `/docs/maud/index.md`
  - `/docs/maud/elements-attributes.md`
  - `/docs/maud/partials.md`
  - `/docs/maud/web-frameworks.md`
- `datastar`
  - `/docs/datastar/index.md`
  - `/docs/datastar/guide.md`
  - `/docs/datastar/reference.md`
  - `/docs/datastar/reference/attributes.md`
  - `/docs/datastar/reference/sse_events.md`
  - `/docs/datastar/route-map.md`
- `axum`
  - `/docs/axum/index.md`
  - `/docs/axum/routing/index.md`
  - `/docs/axum/response/sse/index.md`
  - `/docs/axum/extract/index.md`
- `css-scope-inline`
  - `/docs/css-scope-inline/index.md`
  - `/docs/css-scope-inline/03-how-it-works.md`
  - `/docs/css-scope-inline/06-workflow-tips.md`
- `writing-style`
  - `/docs/writing-style.md`
- `visual-signoff`
  - `/docs/visual-signoff/latest/ui-signoff.md`
  - `/docs/visual-signoff/latest/ux-signoff.md`
- `code-audit`
  - `/docs/code-audit/README.md`
  - `/docs/code-audit/01-architecture-map.md`
  - `/docs/code-audit/02-request-and-data-flows.md`

## Lookup Protocol
1. Classify request into one or more areas (`maud`, `datastar`, `axum`, `css-scope-inline`, `writing-style`, `visual-signoff`, `code-audit`).
2. Retrieve only mapped docs for those areas.
3. Extract concrete constraints in normalized form: `rule_id`, `source_path`, `rule_text`.
4. Attach resolved rules to working context before any design decision.
5. If no applicable rule exists, explicitly record `rule_text: "no direct corpus rule found"`.

## Conflict Resolution Rule
- If proposed output contradicts docs-backed rules, docs win.
- Exception is allowed only with `component_spec.override` present and populated with a reason and approved-by identity.
- `mds-verifier` must fail when contradiction exists without valid `override` metadata.
