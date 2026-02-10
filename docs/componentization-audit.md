# Componentization Audit (2026-02-03)

## Scope
Reviewed `crates/http/src/views` for repeated markup and UI patterns that could be lifted into reusable Maud partials. This focuses on view-layer componentization (not Rust domain/app types).

## High-confidence candidates
These are repeated patterns across pages/partials and would benefit from shared partials.

### 1) Section headers with title + subtitle + action
**Where:**
- `crates/http/src/views/partials/demo/chat_demo_section.rs`
- `crates/http/src/views/pages/home.rs` (fallback chat section)

**Why:** same layout pattern: title + muted text + right-aligned action.

**Component idea:** `views::partials::SectionHeader { title, subtitle, action: Option<Markup> }`.

---

### 2) CTA row + action groups
**Where:**
- `crates/http/src/views/partials/demo/home_hero.rs`
- `crates/http/src/views/pages/home.rs` (Support systems section)
- `crates/http/src/views/pages/chat_moderation.rs`

**Why:** repeated `div.cta-row` grouping with buttons/links.

**Component idea:** `views::partials::CtaRow { items: Vec<Markup> }`.

---

### 3) Demo result wrappers / placeholders
**Where:**
- `crates/http/src/views/partials/demo/demo_result.rs`
- `crates/http/src/views/partials/demo/auth_status.rs`
- `crates/http/src/views/partials/demo/session_status.rs`
- `crates/http/src/views/partials/demo/request_meta.rs`
- `crates/http/src/views/partials/demo/db_check.rs`
- `crates/http/src/views/partials/demo/boundary_check.rs`
- `crates/http/src/views/partials/demo/trace_log.rs`

**Why:** same `div.demo-result` framing, varying content.

**Component idea:** `DemoResultShell { target_id, content }` to wrap interior markup.

---

### 4) Flow map chips
**Where:**
- `crates/http/src/views/pages/home.rs` (support systems)

**Why:** `div.flow-map` + `span.step` + `span.arrow` is already a distinct layout.

**Component idea:** `FlowMap { steps: Vec<String> }` which renders steps with arrows.

---

### 5) “No data yet” empty states
**Where:**
- `NetworkLog`, `LiveLog`, `TraceLog`, `DemoResultPlaceholder`

**Why:** consistent pattern for empty views. Current copy differs but could be a shared component with custom message.

**Component idea:** `EmptyState { message, muted: bool }`.

---

### 6) Panels of consistent height + scroll behavior
**Where:**
- `LogPanel` used for network + backend log
- chat windows (`ChatWindow`) have similar structure

**Why:** both are “pane with header + scrollable content.”

**Component idea:** general `ScrollablePanel { title, body, class }` which `LogPanel` and `ChatWindow` can delegate to.

---

### 7) “Status badge/pill” specialization
**Where:**
- `Pill` exists
- `Badge` exists only as CSS classes in markup

**Why:** badges/pills are currently split. `Pill` could accept a `variant` enum for consistent usage across sender badges, status pills, and target pills.

**Component idea:** `Pill { text, variant: PillVariant }` (enum → class mapping).

---

## Medium-confidence candidates
Useful, but may not be worth it unless we expect more usage.

### 8) Chat layout shell
**Where:**
- `ChatDemoSection` + `ChatWindow` + `ChatPanel`

**Why:** Chat layout is now stable. If future demos reuse a chat-like layout, wrap the column layout + shared script logic into a `ChatLayout` component.

---

### 9) Info cards in “Support systems inside the chat demo”
**Where:**
- `crates/http/src/views/pages/home.rs`

**Why:** Each `article` block uses the same structure: title, muted text, CTA row, and a result placeholder. Could be a `SupportCard` component.

---

## Low-confidence candidates
Probably not worth it unless the site grows.

### 10) Hero pills list
**Where:**
- `HomeHero`

**Why:** Already small; reusing `Pill` could be nice but might not be worth the churn.

---

## Suggested next order (smallest diff to highest value)
1. `SectionHeader` (reduces duplication immediately in chat sections).
2. `DemoResultShell` (unifies result wrappers).
3. `FlowMap` (clarity + reduces layout repetition).
4. `EmptyState` (standardize “no data yet” UX).
5. `ScrollablePanel` (unifies chat/log panels; can refactor `LogPanel`).
6. `PillVariant` enum (cleaner pill usage + class mapping).

## Notes
- `LogRow` and `Pill` are already shared across the HTTP and backend logs. Keep extending these to SSE/chat tables if we want them consistent.
- If we do `ScrollablePanel`, ensure we keep the per-panel auto-scroll hooks isolated so we don’t reintroduce global query selectors.
