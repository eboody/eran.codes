# Architecture Map

## Workspace crates

- `crates/domain`
  - Owns domain invariants and entities (`user`, `chat`).
  - No HTTP or persistence mechanics.
- `crates/app`
  - Owns use-case orchestration and policy.
  - Defines service-level behavior and trait-shaped boundaries.
- `crates/infra`
  - Owns SQL and concrete adapters (repositories, auth backing, migrations).
  - Implements persistence and mechanism details.
- `crates/http`
  - Owns transport layer, routes, handlers, view composition, SSE endpoints, and trace/log presentation.
- `crates/utils`
  - Shared utility helpers (env/base64 wrappers).
- `src/` (binary root)
  - Wires services, config, and runtime startup.

## Dependency direction

- `domain` -> none.
- `app` -> `domain`.
- `infra` -> `app` + `domain`.
- `http` -> `app` + `domain` (selective transport-level use).
- binary root -> all above for composition.

## View composition map (`crates/http/src/views`)

- `pages/*`:
  - top-level HTML page builders (home, login, register, protected, chat views).
- `partials/demo/layout/*`:
  - reusable layout surfaces (`SectionHeader`, `FeatureGallery`, `TabbedShowcase`, etc.).
- `partials/demo/chat/*`:
  - chat interaction surfaces (`ChatPanel`, `ChatWindow`, `ChatMessage`).
- `partials/demo/log/*`:
  - live log and network log rendering.
- `partials/components/*`:
  - cross-cutting reusable presentational components (`CodeBlock`).

## Current high-level runtime seams

- HTTP routing and middleware: `crates/http/src/router/*`.
- SSE session lifecycle: `crates/http/src/handlers/sse.rs`, `crates/http/src/sse/*`.
- Chat use-case + moderation/rate-limit path:
  - handler: `crates/http/src/handlers/demo/chat.rs`
  - app policy: `crates/app/src/chat/mod.rs`
  - persistence: `crates/infra/src/repo/chat.rs`
