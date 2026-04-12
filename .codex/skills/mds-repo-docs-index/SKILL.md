---
name: mds-repo-docs-index
version: 0.2.0
description: Docs-first policy for the lean repo docs set, plus routing rules for when to leave the repo and use installed skills or upstream docs.
scope: project
---

# mds-repo-docs-index

## Purpose
Define the current docs-first policy for `eran_codes` after the repo-doc cleanup.

The repo no longer mirrors framework docs for Maud, Datastar, Axum, Bon, or Statum.
This skill now governs:

- which repo docs are actually authoritative
- when repo docs stop and installed skills or upstream docs should take over
- how to avoid treating deleted mirrors as if they still exist

## Source Of Truth
- Primary repo docs corpus:
  - `/README.md`
  - `/docs/README.md`
  - `/docs/auth-sessions.md`
  - `/docs/sensitive-sync-architecture.md`
  - `/docs/portfolio-demos.md`
  - crate READMEs under `/crates/*/README.md`
- Conflict rule:
  - for repo-specific portfolio/product claims, current repo docs beat agent memory
  - for framework semantics, repo docs are no longer authoritative unless they describe repo-owned behavior

## Authoritative Repo Doc Roots
- `/README.md`
- `/docs/README.md`
- `/docs/auth-sessions.md`
- `/docs/sensitive-sync-architecture.md`
- `/docs/portfolio-demos.md`
- `/crates/domain/README.md`
- `/crates/app/README.md`
- `/crates/infra/README.md`
- `/crates/http/README.md`
- `/crates/utils/README.md`

## Non-Authoritative Or Removed Surfaces
- Any deleted mirror path under `/docs/axum`, `/docs/bon`, `/docs/datastar`, `/docs/maud`, `/docs/statum`, `/docs/css-scope-inline`, `/docs/tracing`, and similar trees
- Any deleted plan/audit doc from the pre-cleanup repo
- Any removed CI script under `/scripts/ci`

If a skill or note still points at one of those paths, treat that as stale guidance and fix or ignore it rather than pretending the path exists.

## Lookup Protocol
1. Classify the question:
   - `repo-story`
   - `portfolio-surface`
   - `auth`
   - `sensitive-sync`
   - `crate-boundary`
   - `framework-semantics`
2. For the first five classes, start in the authoritative repo docs listed above.
3. For `framework-semantics`, don't search deleted repo mirrors.
4. Route framework questions to the matching installed skill or official docs:
   - Axum: `mds-axum-integration` plus official Axum docs
   - Datastar: `mds-datastar-idiom-audit`, `mds-datastar-patterns`, and the installed `datastar` skill
   - Maud: `mds-maud-patterns` plus official Maud docs
   - Bon: `mds-bon-patterns` plus `bon-workspace` or official Bon docs
   - Statum: `mds-statum-patterns` plus current crate usage or upstream docs
5. If no repo doc exists for a repo-specific claim, say so explicitly instead of inventing one from memory.

## Curated Map
- `repo-story`
  - `/README.md`
  - `/docs/README.md`
- `auth`
  - `/docs/auth-sessions.md`
  - `/crates/http/README.md`
- `sensitive-sync`
  - `/docs/sensitive-sync-architecture.md`
  - `/docs/portfolio-demos.md`
- `crate-boundary`
  - `/crates/domain/README.md`
  - `/crates/app/README.md`
  - `/crates/infra/README.md`
  - `/crates/http/README.md`
  - `/crates/utils/README.md`

## Rule
- Don't cite or rely on deleted repo mirrors.
- Don't keep a repo skill pointing at removed docs after a repo cleanup.
- When the repo intentionally becomes leaner, the skills must narrow with it.
