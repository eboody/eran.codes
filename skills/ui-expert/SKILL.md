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

## Core Workflow

1. Establish visual intent and reference style target.
2. Audit layout hierarchy:
   - spacing rhythm
   - visual grouping
   - emphasis and scannability
3. Audit component state quality:
   - base, hover, focus, active, disabled
   - tab/selection indicators and transitions
4. Enforce token-first styling:
   - use design tokens/Open Props variables
   - avoid magic numbers
   - keep rules component-local unless shared on purpose
5. Validate responsive behavior at key breakpoints.
6. Validate accessibility-facing UI qualities:
   - focus visibility
   - contrast sufficiency
   - readable type scale and line lengths

## Output Contract

Always return:
- `ui_findings`: prioritized visual/design defects
- `token_changes`: token-level decisions and rationale
- `component_changes`: concrete CSS/markup updates
- `visual_qa`: screenshot/baseline status and follow-ups

## Guardrails

- Preserve existing design language unless change is intentional.
- Prefer reusable component/token abstractions over one-off overrides.
- For visual tasks, run in tandem with `$ux-expert` so polish and usability move together.
- If visual output intentionally changes, refresh and report visual baseline status.
