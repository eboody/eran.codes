---
name: mds-datastar-idiom-audit
version: 0.1.0
description: Docs-backed Datastar idiom audit workflow for this repo.
scope: project
---

# mds-datastar-idiom-audit

## Purpose
Audit `eran_codes` Datastar usage against:
- the local Datastar docs corpus
- the repo's Datastar architecture rules
- the repo's Datastar attribute/signal/SSE house patterns

Use this skill when reviewing whether an existing Datastar surface is idiomatic, not just functional.

## Required Inputs
- `/home/eran/.codex/skills/datastar/SKILL.md`
- `.codex/skills/mds-datastar-architecture/SKILL.md`
- `.codex/skills/mds-datastar-patterns/SKILL.md`
- `.codex/skills/mds-axum-integration/SKILL.md` when handler/extractor/SSE details matter

## Audit Priority
1. Correctness against Datastar semantics
2. Correctness against repo architecture rules
3. Idiomatic Datastar design
4. Readable, maintainable Rust + Maud composition

## Audit Workflow
1. Classify each interaction before judging implementation:
   - presentation/session concerns should usually stay `ui_local`
   - canonical app-state changes should usually be command + SSE
2. Verify authority boundaries:
   - app-owned state should converge through SSE patches, not local UI mutation
   - local UI signals should stay presentation-scoped unless there is clear app meaning
3. Verify transport shape:
   - Datastar command handlers should return status codes, not JSON state payloads
   - backend convergence should use `datastar-patch-signals` where the repo requires it
4. Verify signal and expression usage:
   - signals are minimal, purposeful, and not carrying unnecessary sensitive or canonical state
   - expressions are readable and not simulating backend workflow locally
5. Verify view composition:
   - Datastar bindings are attached at stable, intentional nodes
   - reusable tab/chat/surface primitives are preferred over one-off markup
6. Distinguish actual problems from deliberate local exceptions:
   - demo/log surfaces may be intentionally more explicit or instrumented
   - do not flag repo-sanctioned architecture as non-idiomatic just because it is more explicit than a toy Datastar example

## Findings Format
- Findings first, ordered by severity
- Each finding should include:
  - what is wrong
  - why it is non-idiomatic or incorrect
  - the relevant file reference
  - the governing Datastar/repo rule
- After findings, include:
  - `Strengths`
  - `Open Questions`
  - `Residual Risk`

## Things To Reject
- calling something non-idiomatic without tying it to Datastar docs or repo rules
- inventing Rust SDK APIs not documented in the local corpus
- collapsing architecture review into generic frontend taste
- treating every local signal as suspicious without first classifying authority and scope
- recommending JSON-response state sync for Datastar command flows in this repo

## Typical Good Outcomes
- no findings, but a short note that the surface is idiomatic and why
- a few high-signal findings around:
  - wrong authority split
  - overused local signals
  - command handlers returning state directly
  - SSE/event drift
  - expressions or bindings doing too much work locally
