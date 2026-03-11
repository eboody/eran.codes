# mds-linting-governor

## Purpose
Decide whether a repo rule belongs in hard linting/CI at all, and if so, choose the lightest enforcement mechanism that preserves signal.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic enforcement surface.

## Inputs It Expects
- User request for a lint, CI contract, or architectural enforcement rule
- Existing repo policies in `.codex/AGENTS.md`
- Existing scripts under `scripts/ci/`
- Current implementation context for the rule being discussed

## Outputs It Must Produce
- An explicit decision: `add`, `extend_existing`, `defer_to_review`, `defer_to_tests`, or `reject`
- Chosen enforcement layer
- Scope of enforcement
- Escape hatch or removal path when needed
- If implementation is approved, the actual script/policy/config changes

## Non-Goals / Forbidden Behaviors
- Must not turn every design preference into permanent CI.
- Must not duplicate compiler, clippy, rustfmt, type-system, or test coverage without a concrete gap.
- Must not add slow, noisy, or high-false-positive checks just because a rule sounds desirable.
- Must not create repo-wide mandatory lints for migration-only or unstable patterns.

## Decision Order
1. Ask whether the rule should exist at all.
2. If yes, prefer the cheapest correct layer:
   - type system / compiler
   - clippy / rustfmt
   - existing CI contract script
   - new narrow CI contract script
   - review or docs guidance only
3. If the rule is temporary, use an audit/migration mechanism instead of permanent CI.

## Checklist Of Required Invariants
- The rule is objective and mechanically testable.
- The failure message can point to a concrete fix.
- The rule targets a stable pattern, not an active design exploration.
- The likely false-positive rate is low enough that engineers will keep trusting the signal.
- The enforcement scope is as narrow as possible.
- Reuse or extend an existing `scripts/ci/*` script before creating a new one.
- If a new script is required, it should have one clear responsibility and fast runtime.
- Prefer opt-in markers when only some files or modules need the rule.
- If the rule needs frequent exceptions, it probably belongs in review guidance, not mandatory CI.
- If the rule duplicates a testable behavioral guarantee, prefer tests.
- If the rule duplicates a compiler or clippy guarantee, prefer those tools.
- If the rule mainly protects architecture drift, ensure the failure text names the violated policy and the intended alternative.
- Every permanent lint should have a plausible deletion path if the codebase or architecture changes.

## Heuristics
- Good candidates for hard CI:
  - stable architectural contracts
  - narrow marker-based invariants
  - cheap structural checks with obvious fixes
- Bad candidates for hard CI:
  - aesthetic preferences
  - rules that require deep semantic inference
  - checks likely to churn during refactors
  - one-off cleanup campaigns

## Minimal Valid Output Snippet
```json
{
  "lint_decision": {
    "decision": "extend_existing",
    "layer": "scripts/ci/descriptive-module-imports.sh",
    "scope": "modules marked with the descriptive-module-import comment",
    "reason": "The rule is stable, objective, and already has a close existing contract.",
    "escape_hatch": "opt-in marker only; do not apply repo-wide",
    "rejected_alternatives": [
      "new repo-wide script",
      "manual review only"
    ]
  }
}
```

## Failure Modes To Avoid
- CI bloat from one script per preference
- rigid rules that block refactors for no architectural gain
- lints that engineers cargo-cult around instead of understanding
- checks so noisy that people stop trusting red CI
