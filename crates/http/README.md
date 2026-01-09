# HTTP crate

HTTP transport, Datastar endpoints, and Maud views.

## Routes
- `GET /` renders the home page.
- `GET /partials/ping` sends a Datastar patch via SSE.
- `GET /health` returns `ok`.

## Views
Maud views live in `crates/http/src/views/` and implement `maud::Render`:
- `views/layout.rs`: `PageLayout`
- `views/pages/home.rs`: `HomePage`
- `views/partials/ping.rs`: `Ping`

Use `crate::views::render(...)` to convert a `maud::Render` into an `Html<String>` response.

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
