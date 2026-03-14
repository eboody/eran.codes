# mds-styling-system

## Purpose
Apply the hybrid styling policy at the end of component generation: extract reusable package styles to global `app.css` and keep only justified component-specific scoped styles.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic styling structure.

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
- Must not prefer flashy or dense styling tricks over clearer reusable structure without explicit justification.

## Checklist Of Required Invariants
- Styling mode is `hybrid`.
- Reusable patterns (tabs/panels/cards/cta/layout primitives) are represented in global `app.css` package classes.
- Scoped inline CSS is kept only for non-reusable, component-specific behavior and documented in `scoped_exceptions`.
- Reusable class names are generic and library-oriented (`ui-tabs`, `ui-panel`, `ui-cta`, etc.).
- Reusable package rules prefer OpenProps/shared tokens (`--size-*`, `--radius-*`, `--border-size-*`) instead of raw numeric literals.
- Existing parent or page-level theming hooks stay intact across scoped-CSS refactors; component roots must consume public tokens via fallback values instead of redefining them.
- Styling refactors against an existing rendered surface must capture a before-state screenshot or live reference before changing the shell contract.
- Non-trivial visual changes must be verified with a before/after comparison, not just code inspection.
- Modern CSS capabilities should be used when they exist and improve clarity/reduce JS, with progressive enhancement fallbacks.
- A styling-system review pass must happen before the prompt is considered complete.

## Modern CSS Preference (Use If Available)
- Prefer declarative CSS/HTML capabilities over JS workarounds when support exists in target browsers.
- Keep a baseline style first, then add modern enhancements behind `@supports` (or equivalent feature gates).
- Favor these features when relevant to the component:
  - Popover/dialog declarative controls: command invokers, `popover="hint"`, `interestfor`, dialog light-dismiss (`closedby`).
  - Carousel/scroll UI primitives: `::scroll-button`, `::scroll-marker`, `scroll-target-group`, `:target-current`, scroll-state queries.
  - Positioning/query primitives: anchor positioning, anchored container queries, container/style queries.
  - Typed/dynamic CSS expressions: typed `attr`, `if`, `@function`, expanded range syntax.
  - UI polish primitives: customizable `<select>`, `text-box-*`, `corner-shape`, `shape`, `stretch`, `sibling-index`/`sibling-count`.
- Do not force these features where they add risk or complexity; fallback must preserve usable behavior.

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
