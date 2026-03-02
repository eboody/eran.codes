---
name: ux-expert
description: UX specialist for user flows, information architecture, usability, and accessibility decisions in product interfaces.
---

# UX Expert

## When To Use

Use this skill for:
- User-flow and navigation design
- Information architecture and content prioritization
- Form and onboarding usability improvements
- Interaction friction analysis
- Accessibility and inclusive UX reviews
- Visual feature work where workflow quality matters

## Local Sources First

Start from `docs/reference-map.md` and read the smallest relevant set:
- `docs/portfolio-demos-plan.md`
- `docs/portfolio-demos.md`
- `docs/professionalism-breakdown.md`
- `docs/datastar-signals.md`
- `docs/surreal/05-usage.md`

## Core Workflow

1. Load local docs from `docs/reference-map.md` and note applicable constraints.
2. Define user goal and critical task path.
3. Map the happy path and likely failure paths.
4. Identify friction points:
   - unnecessary steps
   - ambiguous labels/copy
   - hidden system status
   - weak error recovery
5. Run UX heuristics check:
   - visibility of system status
   - match with user mental model
   - consistency and standards
   - error prevention and recovery
6. Run accessibility UX check:
   - keyboard-only flow
   - focus order and visible focus
   - clear labels and instructions
   - semantic grouping and landmarks
7. Propose minimal, high-impact improvements first.

## Output Contract

Always return:
- `ux_findings`: prioritized issues ordered by severity
- `recommended_changes`: concrete implementation actions
- `acceptance_criteria`: testable UX outcomes
- `risk_notes`: tradeoffs or regressions to watch
- `sources_used`: exact local files consulted

## Guardrails

- Prefer simplification over adding controls.
- Keep language concrete and user-task oriented.
- Do not optimize visuals at the cost of task completion.
- Flag component decomposition when implementation size/complexity reduces auditability (e.g., one file owns markup, styling tokens, and interaction behavior for multiple UX concerns).
- When work is visual/UI-related, coordinate with `$ui-expert` in the same pass.
- Verify interaction implementation choices align with repo conventions: Maud `inline_css!` / `inline_js!`, Surreal helpers for DOM behavior, and a clear Datastar-signals decision for UI state.
- Flag missing rationale whenever plain DOM scripting is used where Surreal or Datastar patterns would improve consistency.
