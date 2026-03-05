---
name: view-transition-api
description: MDN-backed View Transition API implementation guidance with local mirrored docs for API methods/events, ViewTransition interfaces, and CSS selectors/properties (`::view-transition*`, `:active-view-transition*`, `view-transition-*`, `@view-transition`). Use when building, debugging, or reviewing same-document or cross-document view transitions.
---

# View Transition API

## Quick Start

1. Open `references/reference-map.md`.
2. Find the exact doc file(s) for the user request.
3. Search within mirrored docs first:

```bash
rg --line-number --ignore-case 'startviewtransition|viewtransition|pageswap|pagereveal|view-transition-name|::view-transition' references/mdn
```

4. If local docs are placeholders or stale, refresh from upstream MDN content:

```bash
scripts/refresh_mdn_docs.sh
```

## Workflow

### API and Event Questions

Use the `references/mdn/web/api/**` docs for:
- `Document.startViewTransition()`
- `Document.activeViewTransition`
- `ViewTransition` and `ViewTransitionTypeSet`
- cross-document events (`PageSwapEvent`, `PageRevealEvent`)
- CSS OM surface (`CSSViewTransitionRule`)

### CSS Questions

Use the `references/mdn/web/css/**` docs for:
- selectors: `::view-transition*`, `:active-view-transition*`
- properties: `view-transition-name`, `view-transition-class`
- at-rule: `@view-transition`
- guide-level patterns in `web/css/guides/view_transitions`

### Implementation Guidance Pattern

1. Confirm which transition type applies: same-document vs cross-document.
2. Confirm lifecycle hook points (start/update/ready/finished/pageswap/pagereveal).
3. Confirm CSS naming and pseudo-element targeting strategy.
4. Provide progressive enhancement fallback when View Transitions are unsupported.

## Guardrails

- Do not invent behavior not present in mirrored docs.
- Separate API mechanics from animation styling concerns.
- Call out browser support caveats when a request depends on newer cross-document pieces.
- Prefer small, runnable examples over abstract descriptions.
- Include `sources_used` with exact local file paths consulted.

## References

- `references/reference-map.md`
- `references/mdn-files.txt`
- `references/api-files.txt`
- `references/css-files.txt`
- `references/mdn/` (mirrored docs)
- `scripts/refresh_mdn_docs.sh`
