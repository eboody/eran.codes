# Datastar Backend Requests Notes

Use this as a concise reference for backend requests and SSE patterns in this repo.

## Sending Signals
- By default, all non-local signals (not starting with `_`) are sent with every request.
- `GET`: signals are sent as the `datastar` query param (JSON).
- Other methods: signals are sent as JSON in the request body.
- In this repo, use `filterSignals` when a request has a narrow typed contract and should not carry unrelated page state.
- Keep the request-side filter explicit in the view so the backend signal struct and the frontend payload stay aligned.

## Nesting Signals
- Dot notation: `data-signals:foo.bar="1"`.
- Object syntax: `data-signals="{foo: {bar: 1}}"`.
- Two-way binding: `data-bind:foo.bar`.

## Reading Signals (Backend)
- Decode the `datastar` query param for `GET`.
- Decode the JSON body for other methods.
- Prefer SDK helpers where available.

## SSE Events
- SSE streams can emit multiple events in a single response.
- Use `PatchElements` and `PatchSignals` for DOM and signal updates.
- Prefer SSE for multi-event workflows or real-time updates.
- For per-tab identity in this repo, keep `sseTabId` in page-level `data-signals` so `/events` and follow-up actions carry the same typed tab id.
- Only include extra page-level signals in the `/events` bootstrap when stream setup genuinely depends on them.

## data-indicator
- `data-indicator:fetching` toggles `$fetching` true while the request is in flight.
- Use it to drive loading UI (`data-class`, `data-show`, etc).

## Backend Actions
- Use `@get()`, `@post()`, `@put()`, `@patch()`, `@delete()`.
- Keep responses declarative (HTML/JSON/SSE), let the backend decide next UI state.
- Use Datastar requests for backend-coupled interaction state, not for DOM-only tab or disclosure behavior.
