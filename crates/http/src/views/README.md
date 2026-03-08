# Views

Views are Maud components split by responsibility so templates stay small and reusable.

## Modules
- `page.rs`: shared layout + page-level helpers (e.g., `Layout`, `Error`, `UserNav`).
- `pages/`: full-page documents (`Home`, `Login`, `Register`, `Protected`).
- `partials/`: Datastar fragments and reusable UI blocks for demos and SSE patches.
- `partials/demo/`: demo-only fragments (status panels, trace logs, ping).

## Conventions
- Pages and partials implement `maud::Render`.
- Reusable styling and behavior live in global static assets (`/static/app.css`, `/static/*.js`).
- Pages should be composed from `page::Layout` to keep a consistent shell.

## Readme map
- `crates/http/README.md`
- `crates/http/src/README.md`
- `crates/http/src/handlers/README.md`
