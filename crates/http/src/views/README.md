# Views

Views are Maud components split by responsibility so templates stay small and reusable.

## Modules
- `page.rs`: shared layout + page-level helpers (e.g., `Layout`, `Error`, `UserNav`).
- `pages/`: full-page documents (`Home`, `Login`, `Register`, `Protected`).
- `partials/`: Datastar fragments and reusable UI blocks for demos and SSE patches.

## Conventions
- Pages and partials implement `maud::Render`.
- Inline CSS/JS uses `maud_extensions::{css, js}` with scoped styling.
- Pages should be composed from `page::Layout` to keep a consistent shell.
