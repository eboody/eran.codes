# Request And Data Flows

## 1) Auth flow

1. `GET /login` renders login page (`crates/http/src/views/pages/login.rs`).
2. `POST /login` handled in HTTP layer and delegated to auth provider.
3. Infra auth query checks user + credentials (`crates/infra/src/auth.rs`).
4. Session persistence through `tower-sessions` store.
5. Redirect and subsequent `AuthSession`-aware route access.

## 2) Home + demo flow

1. `GET /` handler composes demo data and view model.
2. `Home` page renders:
   - `CapabilityShowcase`
   - principles/audit sections
   - `ProfessionalismInPracticeTabs`
   - feature gallery
   - live placeholders + chat section
3. Component tree is mostly built from `maud::Render` partials.

## 3) Chat message flow

1. Client sends `POST /demo/chat/messages`.
2. HTTP handler validates context and calls app chat service.
3. App layer enforces room membership/moderation/rate policies.
4. Infra persists message + audit rows in Postgres.
5. SSE fanout emits Datastar patch to subscribed clients.

## 4) SSE stream flow

1. Client starts `GET /events?...` with tab/session signals.
2. SSE handler binds stream to session id.
3. Keepalive headers and no-buffering headers keep proxy path stable.
4. Events are pushed from chat and live-log publishers.

## 5) Trace/log flow

1. Router layers add request span fields (method/path/session/request id/etc.).
2. Diagnostic and live targets are classified in trace/log modules.
3. Network/live log partials render streamed entries as structured rows.
