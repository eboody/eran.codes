# http::views::partials

`partials` is the reusable view surface for both page composition and Datastar patch responses.

## Structure
- `components/` for library-style primitives, composed components, and log UI building blocks
- `demo/` for demo-specific sections, support panels, and operational surfaces
- [error.rs](./error.rs) for the transport error fragment
- [mod.rs](./mod.rs) for the public partial surface exported to the rest of the crate

## Read order
1. `components/`
2. `demo/`
3. [error.rs](./error.rs)

## Rule of thumb
Public partials should render cleanly on their own, carry typed inputs, and stay honest about whether they are reusable components or demo-only composition.
