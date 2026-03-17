# Component Checklist

Review these before shipping styling work:

- Base state reads clearly at default viewport.
- Hover, focus-visible, active, and disabled states exist where relevant.
- Empty, read-only, and error states remain coherent when the component supports them.
- Type, spacing, radius, and shadow choices come from semantic aliases or justified local aliases.
- Typography literals were audited: no stray `font-size`, `line-height`, or `letter-spacing` values remain when a semantic or local alias should own that decision.
- Direct rhythm and spacing literals were removed from declarations unless the file is defining a token or a deliberate component-owned metric.
- Local aliases and selectors stayed proportional: no pass-through variable farm and no new one-off class when an existing utility, package, local token, or `data-*` hook would do.
- Single-use pass-through locals were removed: a local `--_...` alias should not exist just to rename one shared token for one selector.
- Shared packages do not encode component-specific content assumptions.
- Scoped selectors stay explicit enough for `css-scope-inline`.
- Responsive behavior holds at the repo's active mobile and desktop breakpoints.
- Motion is meaningful and restrained.
- Modern native CSS features are used intentionally and improve clarity or behavior rather than novelty alone.
- Contrast remains acceptable for body copy, muted copy, and controls.

Require visual QA for:
- shared `ui-*` package changes
- semantic token changes
- major composed components

Allow lighter-weight validation for local scoped-only tweaks, but still review responsive and interactive states.
