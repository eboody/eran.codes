# http::sse

Server-Sent Events registry and session handling.

## Responsibilities
- Maintain SSE streams keyed by typed stream identity:
- `session_id` from signed cookie.
- `sse_tab_id` from Datastar signals (when present).
- Support session-scoped fanout (`send_by_id`) across all tabs in the same session.
- Dispatch Datastar patches to the correct stream or session group.
