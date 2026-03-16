# http::views

The view layer is split so the repo can distinguish reusable UI contracts from demo-specific composition as the UI grows.

## What this split proves
- `page.rs` owns the shared page shell and layout vocabulary
- `pages/` assembles full documents from existing shells and partials
- `partials/` carries reusable component contracts and Datastar patch targets
- demo composition can stay local without pretending to be a library surface

## Read order
1. [page.rs](./page.rs) for layout and site-wide shell components
2. [pages/README.md](./pages/README.md) for full documents
3. [partials/README.md](./partials/README.md) for reusable components and patch surfaces

## What lives here
- `page.rs` for shared layout and page-level helpers
- `pages/` for full page documents like home, lab, login, and work pages
- `partials/` for reusable render components, demo sections, and Datastar patch targets
- `scoped.rs` and `proper_theme.rs` for scoped styling and theme behavior

## Why it matters
This repo treats frontend composition as real engineering work: typed render inputs, reusable shells, and explicit boundaries between library-style components and demo-specific surfaces.
