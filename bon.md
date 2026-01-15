# bon usage guidance

Use `bon` to make multi-step construction readable and self-documenting. Builders should
encode intent so callers can read a high-level flow without opening the implementation.

## When to reach for bon
- A constructor requires more than 3-4 arguments.
- The call site chains multiple `.layer(...)` or setup steps that are not self-evident.
- There are optional features or flags that otherwise become bool soup.

## What to look up before implementing
- The exact `bon` derive/macro syntax for your version (check `Cargo.toml`/`Cargo.lock`).
- How to set default values and optional fields.
- How to customize setter names so the builder reads like prose.
- How to enforce required fields or validation in `build()`.

## Builder conventions
- Prefer `with_` or `enable_` method names for clarity.
- Keep `build()` small and predictable; avoid side effects.
- Keep the builder type close to the module it configures (router, state, SSE, etc.).
- Expose only the builder methods you want read at the call site; hide noisy defaults.

## Suggested targets in this repo
- HTTP router wiring (`crates/http/src/router.rs`).
- Trace/logging configuration blocks with multiple layers.
- SSE registry/session configuration if it grows beyond simple defaults.
