# HTTP crate

HTTP transport, Datastar endpoints, and Maud views.

## Routes
- `GET /` renders the home page.
- `GET /partials/ping` sends a Datastar patch via SSE.
- `GET /health` returns `ok`.

## Sessions
- Auth sessions use `tower-sessions` with the SQLx Postgres store.
- Session cleanup runs on an interval configured by `SESSION_CLEANUP_INTERVAL_SECS` (default: 3600).
- Migrations for sessions live in `crates/infra/migrations/004_sessions.*.sql`.

## Views
Maud views live in `crates/http/src/views/` and implement `maud::Render`.

## Router builder
Router wiring is assembled via Bon to keep the call site self-documenting:

```rust
http::router(state, session_store);
```

Under the hood this flows through named builder steps in `crates/http/src/router/`.
Use Bon's typestate builder patterns for router/state configuration so call sites read like a checklist.

## Internal docs
- `crates/http/src/README.md`
- `crates/http/src/router/README.md`
- `crates/http/src/sse/README.md`
- `crates/http/src/handlers/README.md`
- `crates/http/src/handlers/demo/README.md`
- `crates/http/src/views/README.md`
- `crates/http/src/views/pages/README.md`
- `crates/http/src/views/partials/README.md`

## Static assets
Static files are served from `crates/http/static/` at `/static`.

Included scripts:
- `datastar` via CDN in layout
- `css-scope-inline` via `/static/css-scope-inline.js`
- `surreal` via `/static/surreal.js`

## Inline CSS/JS
Use macros from `maud_extensions` inside Maud views:

```rust
use maud_extensions::{css, js};

maud::html! {
    div class="card" {
        (css! {
            me { border: 1px solid var(--accent); }
        })
        button { "Click" }
        (js! {
            me('-').on('click', () => { me('-').textContent = 'Clicked.' })
        })
    }
}
```

Inline `<style>` tags are scoped by css-scope-inline. Inline `<script>` tags can use Surreal helpers.
