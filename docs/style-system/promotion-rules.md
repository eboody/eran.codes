# Promotion Rules

## Scoped First

Keep styles scoped to the component by default. Use local aliases and local selectors when the rule is specific to one component or one content shape.

## Promote To `app.css`

Promote a rule into shared `app.css` when one or more of these are true:

- the same structure or visual treatment appears in two or more components
- the rule defines a reusable primitive, surface, shell, or affordance
- the rule belongs to an existing `ui-*` package family
- the rule must remain visually consistent across routes or composed components

## Keep Local

Keep a rule local when it is:

- one-off layout glue
- content-shape-specific
- easier to understand as a local alias than a global package
- still being evaluated before broader promotion

## Spec Contract

When a component remains hybrid, record:
- shared packages in `styling.global_packages`
- local leftovers in `styling.scoped_exceptions`
- semantic tokens consumed in `styling.tokens_used`
