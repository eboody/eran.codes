---
name: lint-guardrail-engineer
description: Convert repeated mistakes into enforceable static checks and low-friction CI guardrails.
---

# Lint Guardrail Engineer

## When To Use

Use this skill when:
- The same convention miss appears repeatedly
- A review identifies a preventable pattern regression
- A rule is currently policy-only and should become enforceable
- A CI script/lint should be added or refined

## Ownership

This specialist owns:
- Gap analysis for why a miss was possible
- Guardrail design (lint/script/test) with minimal false positives
- Adoption and rollout guidance

## Local Sources First

Start with `docs/reference-map.md`, then consult the narrowest relevant docs:
- `docs/writing-style.md`
- `docs/project-audit.md`
- `docs/refactor-plan.md`

## Workflow

1. Load local docs from `docs/reference-map.md` and capture which rules apply.
2. Identify recurrence:
   - what failed
   - why existing checks did not catch it
3. Choose enforcement layer:
   - compile-time lint
   - CI script check
   - skill/documentation update (only if code enforcement is not practical)
4. Implement the smallest reliable guardrail.
5. Validate behavior on representative pass/fail examples.
6. Document the rule in the closest policy surface.

## Output Contract

Always return:
- `gap_analysis`: root cause of the miss
- `guardrail_change`: exact enforcement added/updated
- `false_positive_risk`: expected edge cases
- `rollout_notes`: where the new rule is documented and how to use it
- `sources_used`: exact local files consulted

## Guardrails

- Prefer deterministic checks over subjective style checks.
- Avoid broad rules that block valid patterns.
- Keep error messages actionable and specific.
