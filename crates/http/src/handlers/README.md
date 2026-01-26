# HTTP handlers

Handlers are grouped by concern to keep the router readable and navigation predictable.
Each module focuses on a single HTTP surface.

## Modules
- `pages.rs`: full-page HTML handlers (`/`, `/login`, `/register`, `/protected`, `/logout`).
- `auth.rs`: auth form + session handlers (login/register/logout/protected).
- `demo/partials.rs`: Datastar fragment handlers used by the demos.
- `demo/chat.rs`: live chat demo page + message handlers.
- `sse.rs`: SSE stream and Datastar signal demo handlers.

## Guidelines
- Keep handlers small and IO-focused; push validation and policy to `app`.
- Prefer `views::render` for full pages and explicit `Html` for partials.
- Keep request DTOs scoped to the module that owns the route.

## Readme map
- `crates/http/README.md`
- `crates/http/src/README.md`
- `crates/http/src/views/README.md`
