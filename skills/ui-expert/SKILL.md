---
name: ui-expert
description: UI specialist for visual hierarchy, component polish, and design-system consistency.
---

# UI Expert

## When To Use

Use this skill for:
- Visual refreshes and component polish
- Design token/theming and style consistency
- Spacing, typography, color, and contrast decisions
- Interactive states (hover/focus/active/disabled)
- Responsive layout and breakpoint refinement
- Any visual UI change in Maud/CSS components

## Local Sources First

Start from `docs/reference-map.md` and read the smallest relevant set:
- `docs/portfolio-demos-plan.md`
- `docs/portfolio-demos.md`
- `docs/datastar-signals.md`
- `docs/surreal/05-usage.md`
- `docs/css-scope-inline/index.md`

## Core Workflow

1. Load local docs from `docs/reference-map.md` and note applicable constraints.
2. Establish visual intent and reference style target.
3. Audit layout hierarchy:
   - spacing rhythm
   - visual grouping
   - emphasis and scannability
4. Audit component state quality:
   - base, hover, focus, active, disabled
   - tab/selection indicators and transitions
5. Enforce token-first styling:
   - use design tokens/Open Props variables
   - avoid magic numbers
   - keep rules component-local unless shared on purpose
6. Validate responsive behavior at key breakpoints.
7. Validate accessibility-facing UI qualities:
   - focus visibility
   - contrast sufficiency
   - readable type scale and line lengths

## Output Contract

Always return:
- `ui_findings`: prioritized visual/design defects
- `token_changes`: token-level decisions and rationale
- `component_changes`: concrete CSS/markup updates
- `visual_qa`: screenshot/baseline status and follow-ups
- `sources_used`: exact local files consulted

## Guardrails

- Preserve existing design language unless change is intentional.
- Prefer reusable component/token abstractions over one-off overrides.
- Flag decomposition when a single UI component mixes too many concerns (markup + large token/CSS surface + interaction logic) or exceeds practical review size; recommend splitting into focused subcomponents/modules.
- For visual tasks, run in tandem with `$ux-expert` so polish and usability move together.
- For Maud component changes, enforce `inline_css!` / `inline_js!` with `(css())` / `(js())` call-site splices; flag render-local `css!` / `js!` unless the block is tiny and justified.
- For interaction scripts, prefer Surreal helpers (`me()` / `any()` and chainable methods) and explicitly note when vanilla DOM APIs are intentionally retained.
- For stateful interactions, call out whether Datastar signals should be used or intentionally skipped for purely local presentation state.
- In `css-scope-inline`, avoid bare descendant selectors like `me [data-*]`; use explicit anchored chains (for example `me > [slot] [data-*]`) so scoping/minification does not collapse selector intent.
- If visual output intentionally changes, refresh and report visual baseline status.
