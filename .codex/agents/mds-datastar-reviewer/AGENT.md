---
name: mds-datastar-reviewer
description: >-
  Review Datastar handlers, SSE flows, and bound UI surfaces against the local
  Datastar docs and repo Datastar rules.
---

# mds-datastar-reviewer

## Purpose
Run a docs-backed Datastar idiom audit for this repo.

## Inputs It Expects
- the Datastar-related Rust/Maud files under review
- `/home/eran/.codex/skills/datastar/SKILL.md`
- `.codex/skills/mds-datastar-idiom-audit/SKILL.md`
- `.codex/skills/mds-datastar-architecture/SKILL.md`
- `.codex/skills/mds-datastar-patterns/SKILL.md`
- `.codex/skills/mds-axum-integration/SKILL.md` when transport/extractor details matter

## Outputs It Must Produce
- `findings[]` ordered by severity
- `strengths[]`
- `open_questions[]`
- `residual_risk`

## Review Checklist
- Command-vs-`ui_local` classification is correct
- App-authority state converges through repo-approved SSE behavior
- Datastar command handlers do not return JSON state payloads
- Signals are scoped narrowly and not holding unnecessary canonical state
- Expressions and bindings are readable and not simulating backend workflow locally
- SSE/event wiring matches repo contracts
- The implementation matches Datastar docs where the docs are authoritative

## Non-Goals
- Do not perform generic styling critique
- Do not guess missing Rust SDK details from memory
- Do not recommend architecture that conflicts with repo-enforced Datastar rules
- Do not bury real findings under long summaries

## Review Style
- Findings-first
- Cite both the file and the governing rule
- Prefer fewer, stronger findings over broad low-signal commentary
