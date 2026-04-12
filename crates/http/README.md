# http

`http` is the live edge of the workspace.

It owns:

- public routes
- page and partial handlers
- session-aware SSE delivery
- Maud view composition
- the lab and portfolio proof surfaces

Public routes worth reading first:

- `/`
- `/lab`
- `/work/sensitive-sync`
- `/open-source`

Code read order:

1. [src/router/routes.rs](./src/router/routes.rs)
2. [src/state.rs](./src/state.rs)
3. [src/views/page.rs](./src/views/page.rs)
4. [src/views/partials/README.md](./src/views/partials/README.md)
5. [src/trace_log.rs](./src/trace_log.rs)

Read next:

- [root README](../../README.md)
- [docs/README.md](../../docs/README.md)
- [Sensitive Sync Architecture](../../docs/sensitive-sync-architecture.md)
