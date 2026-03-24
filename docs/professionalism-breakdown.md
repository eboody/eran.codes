# Professionalism In Practice

This page explains how engineering judgment shows up in this codebase, with concrete examples.

It is part of the repo's current proof surface: durable auth and session handling, encrypted-record storage, bounded sync behavior, explicit trust boundaries, typed invariants, and runtime inspection.

## 1) Trust-boundary modeling
Professional code keeps policy and mechanisms in the right place. In this repo, app code defines contracts and infra code implements them.

```rust
// crates/app/src/chat/mod.rs
pub trait ChatRepository: Send + Sync {
    async fn create_room(
        &self,
        name: room::RoomName,
        created_by: room::UserId,
    ) -> Result<room::Room>;
}
```

Why this matters:
- The app layer is stable even if storage changes.
- Infra can evolve independently (SQL tuning, schema migrations, etc.).
- Testing app behavior is easier because traits can be mocked.

## 2) Typed invariants over stringly logic
Professional systems encode invariant sets as enums/newtypes so the compiler participates in correctness.

```rust
// crates/domain/src/chat/message.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString)]
pub enum MessageStatus {
    #[strum(serialize = "visible")]
    Visible,
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "removed")]
    Removed,
}
```

Why this matters:
- Fewer runtime bugs from typo-prone literal checks.
- Safer refactors across layers.
- Better readability at call sites (`MessageStatus::Visible` vs `"visible"`).

## 3) Centralized failure contracts
Professional APIs are predictable under failure. Error mapping is centralized so handlers stay focused on workflow, not response edge-cases.

```rust
// crates/http/src/error.rs
pub enum Error {
    Internal,
    Unauthorized,
    Validation(Text),
    Chat(app::chat::Error),
    User(app::user::Error),
    Auth(app::auth::Error),
}
```

Why this matters:
- Consistent user-facing behavior for both page and partial requests.
- Easier to add new error types without duplicating response logic.
- Cleaner handler implementations.

## 4) Runtime inspection as supporting proof
Professional systems distinguish reviewer-facing inspection surfaces from deeper diagnostic events.

```rust
// crates/http/src/trace_log.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum LogTargetKnown {
    #[strum(serialize = "demo.request")]
    DemoRequest,
    #[strum(serialize = "demo.db")]
    DemoDb,
    #[strum(serialize = "demo.sse")]
    DemoSse,
    #[strum(serialize = "http::router::layers")]
    RouterLayers,
}
```

Why this matters:
- The live UI exposes enough signal to inspect request and transport behavior directly.
- Diagnostic data remains available without polluting the reviewer-facing surface.
- Trace semantics are explicit and testable.

## 5) Reusable view components with typed inputs
Professional frontends reduce duplication by using composable, typed view primitives.

```rust
// crates/http/src/views/partials/demo/log/row.rs
#[derive(Clone, Debug, Builder)]
pub struct Row {
    pub timestamp: Text,
    pub message: Text,
    #[builder(default)]
    pub pills: Vec<Pill>,
}
```

Why this matters:
- One component change updates every log surface.
- New log views are composed, not rebuilt.
- UI consistency is enforced by construction.

## 6) Builders as readable wiring
Professional systems optimize for maintainers. Builder pipelines make composition roots self-documenting, especially where auth, chat, SSE, and tracing have to stay explicit.

```rust
// crates/http/src/state.rs
#[builder]
pub fn from_parts(
    #[builder(setters(name = with_user))] user: app::user::Service,
    #[builder(setters(name = with_auth))] auth: app::auth::ProviderImpl,
    #[builder(setters(name = with_chat))] chat: app::chat::Service,
    #[builder(setters(name = with_sse))] sse: crate::sse::Registry,
) -> Self
```

Why this matters:
- Required dependencies are obvious at the call site.
- Wiring changes stay readable as systems grow.
- Fewer constructor mistakes from ambiguous parameter order.
