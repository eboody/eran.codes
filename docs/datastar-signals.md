# Datastar Reactive Signals Notes

Use this as a concise reference when building Datastar-driven UI in this repo.

## Core Principles
- Backend drives state; frontend is a projection of server truth.
- Patch elements and signals rather than manually mutating DOM.
- Use signals for user-driven, short-lived UI state.

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
- Don’t manage browser history; use normal links and redirects.
