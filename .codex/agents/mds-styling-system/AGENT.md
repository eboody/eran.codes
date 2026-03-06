# mds-styling-system

## Purpose
Apply the hybrid styling policy at the end of component generation: extract reusable package styles to global `app.css` and keep only justified component-specific scoped styles.

## Inputs It Expects
- `component_spec.ui`
- `component_spec.content`
- `component_spec.design`
- Generated or updated view component files
- Existing global stylesheet (`crates/http/static/app.css`)

## Outputs It Must Produce
- `component_spec.styling`
- Global package style updates in `crates/http/static/app.css` (when reusable patterns are introduced)
- Scoped-style exception list for component-specific rules that remain inline

## Non-Goals / Forbidden Behaviors
- Must not change backend contracts or state authority semantics.
- Must not introduce request-specific naming in reusable style classes.
- Must not keep duplicated package patterns inline across multiple components.

## Checklist Of Required Invariants
- Styling mode is `hybrid`.
- Reusable patterns (tabs/panels/cards/cta/layout primitives) are represented in global `app.css` package classes.
- Scoped inline CSS is kept only for non-reusable, component-specific behavior and documented in `scoped_exceptions`.
- Reusable class names are generic and library-oriented (`ui-tabs`, `ui-panel`, `ui-cta`, etc.).
- Reusable package rules prefer OpenProps/shared tokens (`--size-*`, `--radius-*`, `--border-size-*`) instead of raw numeric literals.
- A styling-system review pass must happen before the prompt is considered complete.

## Minimal Valid Output Snippet
```json
{
  "styling": {
    "mode": "hybrid",
    "global_packages": ["ui-tabs", "ui-tab", "ui-panel", "ui-cta"],
    "scoped_exceptions": ["tab_set_showcase: panel-specific media query tweak"],
    "tokens_used": ["--ui-border-soft", "--ui-text-muted", "--portfolio-surface-border"]
  }
}
```
