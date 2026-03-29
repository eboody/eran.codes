# Datastar Reactive Signals Notes

Use this as a concise reference when building Datastar-driven UI in this repo.

## Core Principles
- Backend drives state; frontend is a projection of server truth.
- Patch elements and signals rather than manually mutating DOM.
- Use Datastar signals for request-coupled, short-lived UI state.
- Use a small local controller for purely local presentation behavior such as shared tabs or other DOM-only selectors.

## Signals
- Signals are reactive variables prefixed with `$`.
- Signals can be created via:
  - `data-bind` on inputs
  - `data-computed` for derived values
  - `data-signals` to patch one or more signals

## Data Attributes (Common)
- `data-bind:foo` or `data-bind="foo"`: two-way bind input to `$foo`.
- `data-text="$foo"`: set element text from a signal/expression.
- `data-computed:bar="..."`: computed, read-only signal.
- `data-show="expr"`: show/hide element based on expression.
- `data-class:success="expr"` or `data-class="{...}"`: conditional class list.
- `data-attr:aria-hidden="expr"` or `data-attr="{...}"`: bind attributes.
- `data-signals:foo="1"` or `data-signals="{...}"`: patch signals.
- `data-on:click="..."`: bind events to expressions/actions.

## Patch Signals From Backend
- `application/json` responses are treated as “Patch Signals” events.
- `text/event-stream` responses may include `datastar-patch-signals` SSE events.

## Patch Elements From Backend
- `text/html` responses patch DOM elements by ID using morphing.
- SSE streams can emit `datastar-patch-elements` events for live updates.

## Practical Guidance
- Prefer `@get()`/`@post()` actions for backend interaction.
- Use `data-show` with initial `style="display: none"` to avoid flash.
- Keep signals narrow and intentional; avoid treating the page-global signal store as a generic local state bag.
- Don’t manage browser history; use normal links and redirects.

## Local Controller Ownership

Use a local controller when the behavior is purely DOM-local and does not need
to round-trip through the backend-owned signal model.

- `crates/http/static/local-tabs.js`
  Owner: `LocalTabRoot` and `LocalTabPanel`
  Why local: tab selection, keyboard roving focus, and hash-to-panel reveal are
  DOM-only behavior with no backend truth surface.
- `crates/http/static/chat-demo.js`
  Owner: `chat::Surface`
  Why local: demo chat windows only need local autoscroll after message patches;
  Datastar still owns the request-coupled message state.
- `crates/http/static/request-burst.js`
  Owner: `RequestBurstDemo`
  Why local: the burst harness is browser-observed latency and throughput UI,
  not backend-owned application state.
- `crates/http/static/operational-timeline-scroll.js`
  Owner: `OperationalRequestFilter`
  Why local: the filter request is Datastar-owned, but the follow-up scroll
  behavior is a view affordance scoped to the operations timeline.
- `crates/http/static/log-auto-scroll.js`
  Owner: `logs::primitives::AutoScroll`
  Why local: each log panel emits its own root/selector pair, but the behavior
  now runs through a repo-owned asset instead of per-instance inline JS.
- `crates/http/static/transport-errors.js`
  Owner: page transport shell in `crates/http/src/views/page.rs`
  Why not local-only: it translates Datastar transport events into the shared
  page error surface, so it belongs with the page-level request contract rather
  than per-component local controllers.
