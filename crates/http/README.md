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
