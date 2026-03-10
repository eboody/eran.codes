# Modern Native CSS

Prefer modern native CSS features when they improve the result and are reasonable for the task's browser targets.

## Good Default Candidates

These are strong defaults when they help:

- nesting
- `color-mix()`
- relative color syntax
- logical properties
- `@starting-style`
- view-transition primitives
- newer native layout and animation features that reduce wrapper markup or JS hacks

## Use Deliberately

These can be valuable, but they should be chosen on purpose rather than by default:

- `@scope`
- CSS mixins and custom functions
- `if()`
- `contrast-color()`
- scroll-driven animations
- masonry and other newer layout APIs
- newer declarative UI features around dialogs, popovers, selects, scroll markers, or anchored/query-driven layout behavior

## Policy

- Prefer native CSS over extra abstraction when it makes the code clearer.
- Do not introduce a new feature only because it is new.
- If a feature materially shapes the design direction, call it out in the design brief or implementation summary.
- If support confidence is unclear, treat the feature as conditional on browser targets instead of assuming it is safe everywhere.
