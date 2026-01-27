# Views Audit: Componentization + Invariant Strings

Date: 2026-01-27
Scope: `crates/http/src/views/` (pages + partials), plus related strings in demo UI.

## Summary
The chat demo is now well-componentized, but the rest of the UI still relies on inline markup and string literals for routes, labels, statuses, and action values that are invariant. There are clear opportunities to extract shared components (hero, cards, CTA rows, demo sections, status blocks) and replace stringly-typed invariants with enums/newtypes.

## Componentization Opportunities
### Page-level layout blocks
- **Home hero** (`views/pages/home.rs`): header with title, subcopy, tags, and a right-side card is a reusable pattern across the site.
  - Candidate partial: `HomeHero` with fields for title, subtitle, tags, and session card data.
- **Hero + aside card** (`views/pages/chat.rs`, `views/pages/home.rs`, `views/pages/chat_moderation.rs`): repeated header structure.
  - Candidate partial: `HeroCard` or `HeroSection` reused across pages.
- **CTA rows** (`home.rs`, `chat_moderation.rs`, `protected.rs`): repeated button row patterns.
  - Candidate partial: `CtaRow` with vector of buttons/links.
- **Demo cards / flow cards** (`home.rs`): multiple demo sections share structure: title, copy, bullets, actions, and result target.
  - Candidate partial: `DemoCard` with nested `DemoAction`/`DemoTarget`.

### Partial-level blocks
- **Demo result containers** (`home.rs`, demo partials): repeated `div id=... class="demo-result"` with placeholder text.
  - Candidate partial: `DemoResultPlaceholder` to standardize usage.
- **Meta lists** (`request_meta.rs`, `session_status.rs`, `auth_status.rs`, `boundary_check.rs`, `db_check.rs`): repeated `ul` list of key/value pairs.
  - Candidate partial: `KeyValueList` with list of `(label, value)`.
- **Status pills/tags** (`home.rs`, chat connection pill): repeated pill styles.
  - Candidate partial: `Pill`/`PillRow` with variant enum.

## String Invariants to Enum/Newtype-ize
### Routes / endpoints (in views)
Hard-coded paths should be centralized as enums/newtypes to avoid drift:
- `/login`, `/register`, `/logout`, `/protected`
- `/demo/chat`, `/demo/chat/messages`, `/demo/chat/messages/demo`, `/demo/chat/moderation`
- `/events`
- `/partials/*` endpoints used in home demo actions:
  - `/partials/auth-status`, `/partials/session-status`, `/partials/db-check`, `/partials/boundary-check`, `/partials/request-meta`, `/partials/ping`
  - `/error-test`

Candidate: `enum Route` with `as_str()` and (optionally) `with_query()` helpers.

### Moderation decisions
- `value="approve"` / `value="remove"` in `chat_moderation.rs` should be an enum (e.g., `ModerationAction`) shared with the handler to avoid mismatch.

### Demo request sources
- Network log uses `"/demo/chat/messages"` and `"/demo/chat/messages/demo"` literals. Already enum-ized locally; consider sharing a `ChatRequestRoute` enum with the chat panel roles.

### Auth status labels
- `"Authenticated"` / `"Anonymous"` in `auth_status.rs` should be an enum to avoid text drift.

### DB check status labels
- `"found"` / `"not found"` in `db_check.rs` are invariant and should be an enum (`DbLookupStatus`).

### SSE connection labels
- `"SSE connected"` / `"SSE disconnected"` in `chat_connection.rs` could be sourced from an enum or shared status component.

### Demo titles + CTA labels
- Demo section headings ("Demo 1: ...") and CTA labels ("Start demo", "Sign in", "Fetch request metadata", etc.) are fixed copy; consider grouping into a `DemoCatalog` enum or constants for reuse across the page.

## High-Value Quick Wins (suggested order)
1. **Route constants/enum** used by views + handlers (chat, auth, demo partials).
2. **Moderation action enum** shared across view + handler (`approve`/`remove`).
3. **Key-value list partial** for repeated session/request/auth/meta lists.
4. **Hero section partial** for home + chat + moderation.
5. **Demo card partial** for the home page demo list.

## Notes
- Avoid string duplication for DataStar endpoints in `data-on:click` and form actions.
- Existing chat panel role enum is a good pattern to extend to other demo variants.
- Prefer enums + `as_str()` over public string constants if you want compiler coverage.

