# http::views::partials

`partials` is the reusable view surface for both page composition and Datastar patch responses.

Read this directory to answer two questions:
- which partials define reusable UI contracts
- which partials expose runtime behavior for the demos and operational surfaces

## What this directory proves
- reusable UI should live behind typed partial/component boundaries instead of being copied across pages
- demo surfaces can stay honest about being demo composition instead of being prematurely promoted into shared UI
- Datastar patch targets can reuse the same rendering contracts the full pages use

## Inspection split
- `components/` for library-style primitives, composed components, and log UI building blocks
- `demo/` for demo-specific sections, support panels, and runtime-observability surfaces
- [error.rs](./error.rs) for the transport error fragment
- [mod.rs](./mod.rs) for the public partial surface exported to the rest of the crate

## Read order
1. `components/`
2. `demo/`
3. [error.rs](./error.rs)

## Rule of thumb
Public partials should render cleanly on their own, carry typed inputs, and stay honest about whether they are reusable components, promotion candidates, or demo-only composition.
