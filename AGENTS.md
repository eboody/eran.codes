# Repository Guidelines

## Project Structure & Module Organization
- `src/` holds the main binary (`src/main.rs`) plus a small helper binary in `src/bin/with_db.rs`.
- `crates/` contains workspace crates by layer: `domain`, `app`, `infra`, `http`, `utils`.
- `maud-extensions` provides `css!`, `js!`, and font macros for Maud templates.
- `crates/http/src/views/` holds Maud views (pages, partials, layout) and uses `maud::Render`.
- `crates/http/static/` serves CSS and frontend helper scripts (css-scope-inline, surreal, datastar).
- `crates/infra/migrations/` contains SQL migrations (`001_users.up.sql`, `002_users.down.sql`).
- `docker-compose.yml` provides a local Postgres instance for development.
- `crates/http` owns SSE and session cookies (single SSE connection per visitor) using `tower-cookies`.

## SSE + Session Notes
- A single SSE connection per visitor is served at `/events`.
- Session identity is stored in a signed `session_id` cookie (HTTP-only, SameSite Lax) in `crates/http`.
- SSE events carry Datastar payloads; the client keeps one `EventSource` and applies patches.
- TODOs in code track future work: signed cookies and per-tab SSE IDs.

## Auth + Session Notes
- `crates/http` integrates `axum-login` with `tower-sessions` for auth sessions, using the SQLx Postgres session store.
- Session cookies are encrypted using `SESSION_SECRET` (base64url) and user ids are attached to tracing via `http::request::set_user_id`.
- Auth backend uses `app::auth::ProviderImpl` with infra `AuthRepository` and `Argon2Hasher`.
 - Auth uses a separate `credentials` table with `password_hash` (argon2 PHC string); app owns traits, infra implements hashing.

## Error Handling Strategy
- Enterprise-level default: a centralized HTTP error type that maps domain/app errors to user-facing HTML pages for normal requests and Datastar patch responses for `datastar-request` requests.
- Use middleware + task-local request kind so handlers can return `Result<T, http::Error>` without extra parameters.
- Simpler option: always render a full error page and add a TODO comment in handlers to switch to Datastar-aware error patches later.

## Tracing Strategy
- Follow `docs/tracing.md` for required spans, context fields, and per-layer responsibilities.
- When adding or modifying routes/handlers, ensure request spans record `route`, `path`, `method`, `request_id`, `session_id`, `user_id`, and `kind` (via `MatchedPath` when available), and update tracing if a new request flow is introduced.
- Classify new tracing events as live-log vs diagnostic; use `LogTargetKnown`/`LogMessageKnown` and keep diagnostic-only events behind `DiagnosticTraceLogLayer` (no SSE).

## Bon Builders
- Prefer the `bon` crate for configuration/builders whenever a constructor has many steps or the call site reads as a pipeline of abstract methods.
- Use builder methods that are self-documenting (e.g., `with_session_store`, `enable_trace_layers`, `with_state`) so the high-level modules stay readable without diving into implementation.
- Keep builders close to the module they configure (e.g., router builders in `crates/http/src/router.rs` or a sibling module).
- See `bon.md` for what to look up before implementing a new builder.

## Updating This File
- Keep `AGENTS.md` updated when we introduce new architectural decisions, cross-cutting mechanisms, or boundary changes (e.g., SSE/session handling).
- Prefer module-scoped naming and re-exports for readability (e.g., `sse::Event`, `sse::Registry`, `views::pages::Home`, `views::partials::Ping`, `views::page::Layout`), avoiding deep paths in call sites.
- For page-level shared UI, prefer `views::page::*` (e.g., `views::page::Layout`, `views::page::Error`) over a `layout` module.
- Avoid redundant suffixes on view types; prefer concise names like `views::partials::Ping`.
- Prompt to update `README.md` and make a commit when changes warrant documentation or a logical checkpoint.
- When in doubt, explicitly ask before making commits.
- When multiple implementation paths exist, choose the most sensible default and proceed without asking unless a decision blocks progress.
- Reference `docs/portfolio-demos-plan.md` and `docs/portfolio-demos.md` when deciding which demo UX to implement next.
- When designing a feature, present both a simpler baseline (with a TODO placeholder) and an enterprise-level option so the user can choose.
- Never use string literal comparisons or stringly-typed checks; define enums/newtypes (prefer `strum`/`nutype`) and match on those instead.
- Avoid `String` fields in structs; use enums or newtypes instead (enforced by `scripts/ci/no-string-fields.sh`).
- When a submodule name matches its primary type, keep the submodule private and re-export the type (e.g., `mod ping; pub use ping::Ping` → `views::partials::Ping`).
- Prefer explicit re-exports over `pub use module::*` unless the module is intentionally a flat API surface.
- Use `moddef::moddef!` for module declarations when it reduces repetition and aligns with the above naming conventions.
- For Datastar-related work, follow `docs/datastar-tao.md` and keep Datastar-specific guidance there.
- For frontend reactivity and signals, reference `docs/datastar-signals.md`.
- For Datastar expressions and script execution, reference `docs/datastar-expressions.md`.
- For backend requests and SSE guidance, reference `docs/datastar-backend-requests.md`.

````md
# Agent: Architecture Boundary Enforcer

## Mission
Keep the codebase aligned with the workspace architecture by enforcing clean boundaries between domain, app, http, and infra. Prevent transport and persistence concerns from leaking inward. Design and review flows so that domain models remain stable, app orchestrates use cases and policy, and outer layers implement mechanisms.

This agent is the default reviewer for:
- New endpoints
- New persistence features
- Auth and security flows
- DTO and model design
- Trait placement and dependency direction

## Workspace Mental Model
Think in three rings, not a layer cake:

### Core
- `domain`
- Pure business concepts and invariants
- No HTTP, no DB, no framework crates

### Central
- `app`
- Use cases, orchestration, policy
- Defines abstractions (traits) needed from the outside world

### Outer
- `http`, `infra`, and the binary composition root
- `http`: transport, serde, request parsing, response mapping
- `infra`: implementations that talk to external systems (DB, hashing algorithms, email, caches)
- main: wiring (dependency injection), startup, config

Dependency direction:
- `domain` depends on nothing
- `app` depends on `domain`
- `http` depends on `app` (and optionally `domain` types if deliberate)
- `infra` depends on `domain` and `app` traits it implements
- main depends on everything to wire it together

Call flow is separate from dependency direction.

## Boundary Rules

### Domain rules
- Domain entities and value objects must not derive `Serialize` or `Deserialize`.
- Domain types must not mention HTTP concepts (requests, cookies, headers, status codes).
- Domain types must not mention infra concepts (SQL, rows, migrations, DB IDs).
- Put stable invariants here using newtypes (e.g. `Username`, `Email`) and constructors that validate and sanitize.

### App rules
- App services enforce use-case policy and orchestration.
- App defines traits for mechanisms it needs from the outside world.
  - Example: `PasswordHasher`, `UserRepository`, `EmailSender`, `Clock`, `SessionStore`
- App types must not depend on `http` DTOs or serde types.
- App converts untrusted input into domain types early and performs contextual checks (uniqueness, authorization, rate limits).

### HTTP rules
- HTTP DTOs exist because the network exists.
- HTTP DTOs should be plain serde shapes, typically `String` fields and optional fields that match the API contract.
- HTTP parses and validates cheap things (basic format, length) if helpful, but does not perform expensive work.
- HTTP maps DTOs to app commands, and app errors to responses.
- HTTP renders Maud views using `maud::Render` components; pages and partials live under `crates/http/src/views/`.
- Inline styles/scripts use `css!` and `js!` macros (from `maud_extensions`) and are scoped by `css-scope-inline` and `surreal` in `crates/http/static/`.

### Infra rules
- Infra implements app-defined traits using concrete crates.
- Infra maps DB rows to domain entities.
- Infra should not accept plaintext passwords from app for persistence.
- Infra owns schema, migrations, SQL, query efficiency, and concrete security mechanisms (argon2, bcrypt, token signing).

## When to Create Separate Types
Introduce a new type when a boundary changes ownership of concerns.

Common shapes in a flow:
- `http::dto::*Request` for deserialization and API contract
- `app::*Command` for use-case input
- `domain::*` for invariants and entities
- `infra::*Row` for DB representation
- `http::dto::*Response` for public output

Do not create duplicates for ceremony. Create them when:
- serde or API evolution would otherwise leak into app/domain
- persistence details would otherwise leak into app/domain
- security data would otherwise be accidentally exposed

## Validation Strategy
Two kinds of validation:

### Pure, local validation
- Does not require I/O
- Stable business invariants, normalization, syntax checks
- Belongs in `domain` newtypes and constructors (or in app when domain has no concept for it)

Examples:
- `Username` trimming, lowercase, max length, not empty
- `Email` trimming, lowercase, basic format

### Contextual validation
- Requires repositories or policy context
- Belongs in `app`

Examples:
- Username uniqueness
- Email uniqueness
- Invite code validity
- Authorization rules

## Auth and Password Handling

### Plaintext password handling
- HTTP request typically contains plaintext password over TLS.
- Client-side "hash then send" is not a replacement for server hashing. It becomes a replayable password-equivalent secret.

### Where hashing happens
- App decides when to hash and verifies credentials as part of the use-case.
- Infra provides the hashing implementation behind a trait.

Recommended trait placement:
- Define `PasswordHasher` in `app` (or `domain` only if it is truly a domain concern).
- Implement it in `infra` using argon2/bcrypt.
- Inject it into app services from the composition root.

App should not import argon2/bcrypt crates.

### Avoid hashing in deserialization
Do not hash inside serde custom deserializers:
- hashing is intentionally expensive
- it runs before rate limits and abuse checks
- it turns operational failures into parse failures
- it forces HTTP to depend on hashing mechanisms

### Persistence shape for credentials
Preferred: separate credentials table from user table.

Rationale:
- Reduces accidental exposure via `SELECT *` or reuse of row structs
- Makes future features easier (multiple auth methods, rehash policies)
- Allows tighter DB permissions if needed

Minimal schema approach:
- Store a single encoded hash string that already contains algorithm and parameters (PHC string format for argon2).
- Example stored value shape: `$argon2id$v=19$m=65536,t=3,p=4$...`

## Rehash and Rotation Concepts
- Rehash-on-login: if stored parameters are weaker than current policy, verify then rehash and update.
- Rotation: forced reset or periodic upgrades.
- History policy: prevent reuse of last N passwords by storing previous hashes.

These are optional now, but separate credential storage makes them easier later.

## Decision Checklist for Reviews
When reviewing a change, answer:

### Types and boundaries
- Does any type in `domain` derive serde traits or mention HTTP/DB concepts?
- Does `app` accept `http` DTO types directly?
- Are newtypes used for stable invariants (`Username`, `Email`)?
- Are DB row structs confined to `infra`?

### Auth and secrets
- Is plaintext password confined to HTTP parsing and app service scope only?
- Does hashing happen in app via a trait, with implementation in infra?
- Does any repository accept plaintext password?
- Does any response DTO accidentally include credential data?

### Errors
- Are infra errors mapped into app errors before HTTP mapping?
- Are HTTP response codes and messages determined only in `http`?

## Templates

### HTTP DTO
```rust
#[derive(serde::Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
````

### App command

```rust
pub struct RegisterUser {
    pub username: domain::user::Username,
    pub email: domain::user::Email,
    pub password: SecretString,
}
```

### App traits

```rust
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<PasswordHash, HashError>;
    fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool, HashError>;
}
```

### Infra implementation rule

* `infra` implements `PasswordHasher` using a concrete crate.
* App never imports the concrete crate.

## Output Expectation

When asked to design or review a feature, produce:

* A boundary map (which crate owns which types)
* The minimal set of structs and traits needed
* A flow description from HTTP request to persistence and back
* A short list of failure modes and how they are handled across layers

```
```
## Build, Test, and Development Commands
- `cargo build` builds the workspace.
- `cargo test` runs unit tests (currently minimal/no tests).
- `cargo run` starts the HTTP server (requires `HOST`, `PORT`, `DATABASE_URL`).
- `docker-compose up -d` starts a local Postgres on `localhost:5432`.
- `cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations` runs DB migrations with `DATABASE_URL` injected.
- Always run at least `cargo check` (or `cargo test` when it adds coverage) after code changes to ensure the workspace compiles.

## Coding Style & Naming Conventions
- Rust 2024 edition; use standard `rustfmt` defaults.
- Indentation: 4 spaces; follow idiomatic Rust formatting.
- Naming: `snake_case` for functions/modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Layering: domain types and traits in `crates/domain`, business logic in `crates/app`, IO adapters in `crates/infra` and `crates/http`.
- No stringly-typed struct fields: replace `String` fields with enums (for invariant sets) or `nutype` newtypes (for unbounded text/ids). Prefer `strum` for enum string representations.

## Testing Guidelines
- Use `cargo test` for all tests; no special runner configured.
- Prefer unit tests colocated with modules (e.g., `mod tests` in the same file).
- If you add integration tests, place them in `tests/` at the workspace root.

## Commit & Pull Request Guidelines
- Current commit history uses short, lowercase, imperative messages (e.g., “added migrations”).
- Keep commits focused on a single change; include context in the PR description.
- PRs should include a brief summary, test status (e.g., `cargo test`), and any DB/migration steps.
- Exercise judgement on when changes warrant a commit; choose an appropriate message and commit without asking for confirmation.

## Configuration & Security Notes
- Required env vars: `HOST`, `PORT`, `DATABASE_URL`.
- Do not commit secrets; use local env files or shell exports.
