# Componentization Follow‑Up (2026-02-03)

## What’s now componentized
- `Pill` + enum variants (`PillVariant`, `MethodKind`, `StatusKind`, `LevelKind`, `BadgeKind`).
- `LogRow` shared by Live backend log, HTTP requests log, and Trace log.
- `LogPanel` used by network/trace/backend log windows.
- `StatusCard` used by auth status, session status, request meta, db check, boundary check.
- `SectionHeader` used for chat section header and fallback chat header.

## Remaining high‑value candidates
These are still copy/paste patterns and should be componentized next.

### 1) CTA row
**Pattern:** `div.cta-row` with buttons/links.
**Where:**
- `crates/http/src/views/partials/demo/home_hero.rs`
- `crates/http/src/views/pages/home.rs` (support systems cards)
- `crates/http/src/views/pages/chat_moderation.rs`

**Component idea:** `CtaRow { items: Vec<Markup> }`.

---

### 2) Support card
**Pattern:** `article` with title, muted body, CTAs, and optional result placeholder.
**Where:** `crates/http/src/views/pages/home.rs` (Support systems grid)

**Component idea:** `SupportCard { title, description, actions, body }`.

---

### 3) Flow map
**Pattern:** `div.flow-map` with steps + arrows.
**Where:** `crates/http/src/views/pages/home.rs`.

**Component idea:** `FlowMap { steps: Vec<String> }`.

---

### 4) Empty state
**Pattern:** repeated “No X yet …” copy in logs/placeholder blocks.
**Where:** `LiveLog`, `NetworkLog`, `TraceLog`, `DemoResultPlaceholder`.

**Component idea:** `EmptyState { message }`.

---

### 5) Chat header room line
**Pattern:** “Room: {name}” block under `SectionHeader` in `ChatDemoSection`.

**Component idea:** Extend `SectionHeader` with a `meta` slot or create `SectionMeta`.

---

## Low‑priority
- `HomeHero` pill list (could reuse `Pill` but not critical).
- `DemoResultPlaceholder` and `StatusCard` share some structural overlap (but different intent).

## Suggested next order
1) `CtaRow`
2) `SupportCard`
3) `FlowMap`
4) `EmptyState`
5) `SectionHeader` meta slot
