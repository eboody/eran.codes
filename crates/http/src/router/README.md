# http::router

`router` assembles the public HTTP surface and the middleware stack that makes the rest of the crate coherent.

## What it owns
- route registration
- middleware layering for tracing, request context, cookies, auth, sessions, and static assets
- app state and session store wiring into the axum router

## Read it like this
- [routes.rs](./routes.rs) for path-to-handler mapping
- [layers.rs](./layers.rs) for middleware and session/auth setup
- [mod.rs](./mod.rs) for the public router entrypoint

## Why it matters
A lot of the portfolio's quality proof lives here: secure session behavior, predictable request context, and a readable composition root instead of hidden framework magic.
