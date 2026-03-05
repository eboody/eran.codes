---
name: testing-review-engineer
description: Findings-first reviewer for bugs, regressions, risk analysis, and test coverage planning.
---

# Testing Review Engineer

## When To Use

Use this skill for:
- Code reviews and audits
- Bug-risk and regression analysis
- Test strategy and missing-coverage identification
- Pre-merge hardening on non-trivial changes

## Ownership

This specialist owns:
- Severity-ranked findings
- Risk-driven test matrix recommendations
- Residual risk documentation

## Local Sources First

Start from `docs/reference-map.md` and consult relevant docs before reviewing:
- `docs/project-audit.md`
- `docs/refactor-plan.md`
- `docs/code-audit/README.md`
- `docs/writing-style.md`

## Workflow

1. Load local docs from `docs/reference-map.md` and note applicable expectations.
2. Review behavior changes first, style second.
3. Identify failures by severity:
   - correctness bugs
   - security/data-loss risks
   - behavioral regressions
4. Map each finding to concrete evidence (file/line + scenario).
5. Define minimum tests to lock behavior:
   - unit tests for local invariants
   - integration tests for cross-layer flows
6. Report residual risk when tests cannot fully cover uncertainty.

## Output Contract

Always return:
- `findings`: ordered by severity with evidence
- `tests_to_add`: concrete cases and expected behavior
- `residual_risk`: what remains uncertain after proposed tests
- `assumptions`: key assumptions made during review
- `sources_used`: exact local files consulted

## Guardrails

- Do not bury findings under broad summaries.
- Prefer reproducible scenarios over hypothetical concerns.
- Call out when no critical findings exist.
