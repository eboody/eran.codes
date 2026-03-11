# Writing Style and Modeling Standard

This is the canonical style standard for this repository.

It defines how we model concepts in Rust, how we structure code, and how we write docs.
When there is a conflict between convenience and this standard, this standard wins unless a reviewer accepts an explicit exception.

## Core Principle

Encode concepts in types first.

- Prefer compile-time guarantees over runtime checks.
- Prefer explicit domain language over flexible but ambiguous representations.
- Prefer readability over cleverness.

## Decision Order

When multiple implementations are possible, use this order:

1. Model invariants in Rust types (enums/newtypes/typestate).
2. Use typestate builder/state-machine patterns when relevant (prefer `statum` if it fits).
3. Use `bon` builders when typestate is not the right model but staged construction improves clarity.
4. Use plain constructors/functions only when builders add no meaningful safety or readability.

## Type System Rules

### Enums for constrained sets

If a value has a finite meaningful set, use an enum.

- Status, role, action, lifecycle stage, variant, kind, mode, level -> enum.
- Do not represent invariant sets as string literals.
- In component/view structs, semantic fields (`*mode`, `*variant`, `*kind`, `*role`, `*type`, `*scope`, `*authority`) must not be `Text`/`String`.

### No stringly domain logic

- Do not compare string literals for business logic.
- Do not use `String` fields for invariant-bearing struct members.
- Convert boundary strings to typed forms as early as possible.

Boundary exception:
- Raw strings are acceptable only at transport/storage boundaries (HTTP DTOs, DB rows, external APIs), and must be converted immediately.

### Parse, don't validate (Rust)

Treat boundary conversion as parsing into richer types, not as ad-hoc boolean validation.

- Accept raw `String`/wire values only at boundaries.
- Convert boundary values immediately with `FromStr`, `TryFrom`, or smart constructors/newtypes.
- Return typed parse/construction errors instead of carrying raw values with "is valid" flags.
- After successful parsing, pass only typed values through app/domain flows.
- Prefer static enforcement (types/newtypes/enums/typestate) over repeated dynamic checks whenever feasible.

Rust-oriented default:

- `FromStr` for scalar/string parsing (for example, `Email`, `RoomName`).
- `TryFrom<Dto>` / `TryFrom<Row>` for boundary-to-domain conversion.
- Keep `is_valid` helpers private to parsers/constructors; do not make them the primary domain API.

### Nutype-first scalar modeling

Use `nutype` newtypes for meaningful scalar concepts:

- ids, names, labels, keys, slugs, route fragments, tokens, user-entered text with validation needs.
- Put sanitize/validate logic in the type.

### Strum and serde usage

- `strum`: enum <-> string mapping for boundaries only.
- `serde`: serialization/deserialization at boundaries only.
- Internal app/domain logic should operate on typed enums/newtypes, not serialized strings.

## Builder and Construction Rules

### Typestate first

Use typestate/state-machine construction when correctness depends on build order or valid transition sequence.

- If the problem is inherently stateful/transition-based, use `statum`.
- Prefer making invalid states unrepresentable.

### Bon second

Use `bon` when:

- there are many construction steps,
- named setters improve readability,
- typestate modeling is unnecessary overhead.

Optional-member setter rule:
- If you have a concrete value `T`, call the direct setter (`.field(value)`).
- Use `maybe_` setters only when the input is already `Option<T>` or when you intentionally pass `None` to express omission/default semantics.
- Avoid `.maybe_field(Some(value))`; it is noisier than `.field(value)` and weakens readability.

Default for this repo:
- If a component/view needs a builder, use `bon` (typestate or regular as appropriate), not hand-rolled builder code.

### Constructor fallback

Use direct constructors only when:

- construction is simple,
- required invariants are already encoded by types,
- builder syntax would be pure ceremony.

## Nested Domain Expression (Nestum)

Use `nestum` when nested enums express concepts more clearly than flattened variants.

- Good use: hierarchical domain state/events that read better as nested paths.
- Avoid forcing nested structure when it does not improve readability.

## Error Modeling Rules

### Explicit typed error enums

Every module/use case should have explicit error enums.

- No ad-hoc string errors for domain/app logic.
- Keep error meaning semantic, not transport-shaped.

### Derive strategy

Use `derive_more` attributes/derives intentionally:

- `From` for variant conversion paths (`#[from]` where useful).
- `Display` for controlled, readable messages.
- Add other derives only when they improve correctness/readability of error handling.

### Layered mapping

- Keep domain/app/infra errors typed in their layer.
- Map to transport response shapes at HTTP edge only.

## Maud Component Rules

### Variants as enums

Public UI variants must be typed:

- `ButtonVariant::Primary`, `PillTone::Warn`, etc.
- Avoid class-string switches as the primary variant mechanism.

### Render trait for public partials

Public partials/components should implement `maud::Render`.

Preferred use at call sites:

- `(Component::Variant(...))`, or
- `(Component { variant: ComponentVariant::Primary, ... })`

### Builder + render terminal-call rule

Do not chain `.build().render()` at call sites.

- Inside `maud::html!` splices, end builder chains with `.build()` and splice the value directly; Maud renders `Render` values.
- Outside `html!` (when a `maud::Markup` value is required immediately), call `.render()` on an already-built value.
- Keep one terminal call at the call site: either `.build()` in splices or `.render()` on a value, not both.

### Lean component heuristics

- Prefer typed composition edges for reusable components (for example `Vec<Panel>` over `Vec<maud::Markup>` for parent `children` props).
- Allow `maud::Markup` props only at explicit rich-content boundaries where typed alternatives would add noise.
- Any intentional `maud::Markup` prop slot in reusable component surfaces must carry a local exemption marker comment:
  - `// ci: markup-slot-exempt <reason>`
- For dense mapping logic (for example event/trace projection), split into module families (`kind`, `builders`, `helpers`) before files become monolithic.

### Inline style/script policy

Where inline CSS/JS is appropriate:

- prefer `crate::inline_css!` and `inline_js!` so style/script code lives outside template bodies and is inserted with `(css())` / `(js())`,
- reserve direct `css!` / `js!` usage in templates for very small one-off blocks,
- scope styles with `css-scope-inline`,
- use `surreal.js` helpers where they reduce DOM boilerplate and improve clarity.

### Component-local styling first

For Maud partials/components:

- Prefer component-scoped style helpers (`crate::inline_css!` + `(css())`) with `me` selectors over global `crates/http/static/app.css` rules.
- Add global `app.css` rules only when styles are intentionally shared across unrelated components/pages.
- Use targeted hooks (`id`, semantic class, or stable `data-*`) for nested styling and keep selectors short.
- Avoid deep selector chains; if a selector grows beyond a couple of combinators, split styling into a smaller subcomponent.
- Use bare `me`/quick structural selectors only for simple local styling, not as a substitute for clear hooks.
- Avoid per-rule magic numbers; define reusable design tokens (space, radius, type, color, motion) at the component root and override tokens responsively.

### Open Props token-first styling

Open Props is the default token system for UI styling in this repo (loaded from `/static/open-props.min.css` in the page shell).

- Prefer Open Props tokens over raw literals for spacing, typography, radii, shadows, color, layout, and motion (`--size-*`, `--font-*`, `--radius-*`, `--shadow-*`, `--gray-*`, `--indigo-*`, `--ease-*`, etc.).
- Define component-local semantic aliases with `--_` variables at the component root, then consume those aliases in nested rules.
- Prefer logical properties (`padding-inline`, `padding-block`, `margin-inline`, `inset-inline`, `inline-size`, `block-size`) over physical-direction properties when both are viable.
- Prefer fluid/content tokens (`--size-fluid-*`, `--size-content-*`, `--font-size-fluid-*`) before fixed pixel values.
- Keep motion accessible: use Open Props easing/animation tokens and guard non-essential animations with reduced-motion checks.
- If custom media aliases are supported in the current styling pipeline, prefer Open Props media aliases (`@media (--motionOK)`, `@media (--OSdark)`); otherwise use native equivalents.
- Prefer importing only the packs needed for a feature (for example `open-props/easings`, `open-props/sizes`) instead of unrelated full bundles.
- Treat hard-coded values as exceptions for intentional brand/art direction or missing-token edge cases; keep them local and briefly document intent.

## Readability Standard

Code should read like a domain explanation.

- Prefer descriptive names over abbreviations.
- Prefer module-scoped naming over compound type names when module context already conveys the role.
- Prefer explicit types over inferred ambiguity when intent is unclear.
- Prefer short, composable functions over long mixed-responsibility functions.
- Keep constructor/wiring call sites self-explanatory.

## Naming Standard

### Prefer hierarchical modules over compounds

When a concept is naturally compound, model it as module hierarchy instead of compound module/type names.

- Prefer `chat::panel::Role` over `ChatPanelRole` or `chat_panel::Role`.
- Prefer `chat::window::State` over `ChatWindowState` or `chat_window::State`.
- Submodules do not need separate files; inline modules are acceptable when that keeps locality and readability.
- Keep type names concise once module context carries meaning (`Role`, `State`, `Mode`, `Window`).

This keeps call sites readable without repeating information in every type name.

### Module exposure for namespacing

Expose modules when the module path is part of the intended API vocabulary.

- Prefer `chat::panel::Role::You` over `ChatPanelRole::You`.
- At call sites, import the most descriptive module namespace and qualify from there.
- Prefer `use crate::views::partials::chat;` then `chat::Window`.
- Avoid leaf imports like `use crate::views::partials::chat_message::Message;` when the parent module can expose a cleaner surface.
- In parent modules, use explicit `pub use` to curate a readable API, but keep generic companion nouns inside namespace roots.

### Import path shaping

Use `use` statements to import descriptive namespaces, not deeply nested leaves, when those leaves are part of a cohesive module API.

- Prefer `use crate::views::partials::chat;` then `chat::Window`, `chat::panel::Role`, `chat::panel::Panel`.
- Avoid importing every leaf directly from parallel modules when they conceptually belong to one surface.
- Avoid unnecessary fully-qualified paths in expressions/call sites (`crate::...::Type::...`) when a `use` can import the top-most descriptive module.
- Keep deep leaf imports for narrow internal helpers, tests, or one-off local usage where module qualification harms readability.
- For namespace-root modules, keep generic companion nouns module-qualified (`user::Id`, `user::Repository`) instead of flattening them into parent APIs.
- For modules explicitly marked as descriptive namespaces, enforce this with `// ci: descriptive-module-import <module_path>` in the exposing module; CI will reject leaf `use` and `pub use` from that module path.
- Within a marked descriptive module tree, consume the parent surface instead of sibling leaf modules (prefer `use super::{Message, Messages};` over `use super::message::{Message, Messages};`).
- For descriptive namespace roots that expose builders, prefer `module::builder()` over `module::RootType::builder()` when the type repeats the module meaning.
- For `tab_set`, import the namespace and qualify from it: `use crate::views::partials::components::tab_set;` then `tab_set::ContentProps::builder()`, `tab_set::Component::from_content(...)`, and `tab_set::content::TabSetContent`.
- Avoid leaf-importing namespace companions like `ContentProps` out of `tab_set` when module qualification keeps the surface clearer.

### Avoid tautological enum/type pairs

Type and variant names should read naturally together.

- Avoid `Interactivity::Interactive`, `Visibility::Visible`, `State::Stated`.
- Prefer one of:
  - rename variants: `Interactivity::Enabled` / `Interactivity::Disabled`
  - rename type: `Mode::Interactive` / `Mode::ReadOnly`
  - rename both for domain clarity: `InputState::Editable` / `InputState::Locked`

### Message-family decomposition heuristic

For descriptive modules, `MessageStatus`-style names are acceptable only when they are the sole `Message*` export in that module surface.

- If a module exposes multiple `Message*` names, break them into a `message` submodule (`message::Status`, `message::List`, etc.).
- Keep the canonical entity at the parent surface (`chat::Message`), and move companion `Message*` concepts under `chat::message::*` (`chat::message::Status`, `chat::message::Time`, etc.).

### Compound-name exceptions

Compound names are acceptable when they improve clarity and cannot be expressed cleanly with module context.

- Public flattened APIs where module context is intentionally hidden.
- External protocol alignment (`OAuthTokenResponse`, etc.).
- Real disambiguation cases where concise names would collide or confuse.

## Documentation Voice Standard

This applies to docs in `docs/` and crate READMEs.

- Tone: crisp, professional, concrete.
- No hype language.
- Claims should be anchored to behavior, code, path, or source.
- Include tradeoffs explicitly.

Default section order for technical docs:

1. What this is
2. When to use it
3. How we use it in this repo
4. Boundaries/ownership
5. Pitfalls/failure modes
6. References

## Good / Bad Examples

### Modeling invariants

Bad:

```rust
pub struct Message {
    pub status: String,
}
```

Good:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString)]
pub enum MessageStatus {
    #[strum(serialize = "visible")]
    Visible,
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "removed")]
    Removed,
}

pub struct Message {
    pub status: MessageStatus,
}
```

### Builder choice

Bad:

```rust
let x = Thing::new(a, b, c, d, e, f);
```

Good:

```rust
let x = Thing::builder()
    .with_a(a)
    .with_b(b)
    .with_c(c)
    .with_d(d)
    .with_e(e)
    .with_f(f)
    .build();
```

### Module-scoped naming

Bad:

```rust
pub enum ChatPanelRole { You, Demo }
```

Good:

```rust
pub mod chat {
    pub mod panel {
        pub enum Role { You, Demo }
    }

    pub use panel::Role;
}

use crate::views::partials::chat;

let role = chat::panel::Role::You;
```

### Import path shaping

Bad:

```rust
let panel = crate::views::partials::chat::panel::Panel::builder().build();

use crate::views::partials::chat_message::Message;
use crate::views::partials::chat_window::Window;
use crate::views::partials::chat_panel::Role;
```

Good:

```rust
use crate::views::partials::chat;

let role = chat::panel::Role::You;
let window = chat::Window::builder().build();
let panel = chat::panel::Panel::builder().build();
```

### Namespace root builders

Bad:

```rust
use crate::views::partials::components::tab_set::{self, ContentProps};

let content: tab_set::content::TabSetContent = load_content();
let view = tab_set::Component::from_content(
    ContentProps::builder()
        .id("tab-set-showcase")
        .class("u-surface-card tab-set-showcase")
        .aria_label(Text::from("Solutions"))
        .content(&content)
        .build(),
);
```

Good:

```rust
use crate::views::partials::components::tab_set;

let content: tab_set::content::TabSetContent = load_content();
let view = tab_set::Component::from_content(
    tab_set::ContentProps::builder()
        .id("tab-set-showcase")
        .class("u-surface-card tab-set-showcase")
        .aria_label(Text::from("Solutions"))
        .content(&content)
        .build(),
);
```

### Parent module API curation

Bad:

```rust
pub mod chat_message;
pub mod chat_panel;
pub mod chat_window;
```

Good:

```rust
pub mod chat {
    pub mod message;
    pub mod panel;
    pub mod window;

    pub use message::Message;
    pub use panel::Role;
    pub use window::Window;
}
```

### Enum readability

Bad:

```rust
pub enum Interactivity {
    Interactive,
    NonInteractive,
}
```

Good:

```rust
pub enum Mode {
    Interactive,
    ReadOnly,
}
```

### Maud variants

Bad:

```rust
html! {
    button class=(if is_primary { "btn-primary" } else { "btn-secondary" }) { "Save" }
}
```

Good:

```rust
enum ButtonVariant {
    Primary,
    Secondary,
}

struct Button {
    variant: ButtonVariant,
    label: Text,
}

impl maud::Render for Button {
    fn render(&self) -> maud::Markup {
        let class = match self.variant {
            ButtonVariant::Primary => "btn-primary",
            ButtonVariant::Secondary => "btn-secondary",
        };
        maud::html! { button class=(class) { (self.label.as_str()) } }
    }
}
```

### Open Props styling

Bad:

```css
.card {
  border-radius: 14px;
  padding: 24px;
  box-shadow: 0 8px 24px hsl(220 40% 2% / 0.25);
  color: #0f172a;
}
```

Good:

```css
.card {
  --_surface: var(--surface-2);
  --_ink: var(--text-1);

  color: var(--_ink);
  background: var(--_surface);
  border-radius: var(--radius-3);
  padding: var(--size-fluid-3);
  box-shadow: var(--shadow-2);

  @media (prefers-reduced-motion: no-preference) {
    transition: transform 180ms var(--ease-3);
  }
}
```

## Exception Policy

Defaults are strong, but exceptions are allowed when justified.

Exception requirements:

- State the exception briefly in code comments or PR description.
- Explain why the default reduces clarity or adds non-trivial overhead.
- Keep exception scope minimal and local.

## Review Checklist

Use this checklist for new features and substantial refactors:

- Invariants encoded with enums/newtypes?
- Any stringly checks left in domain/app logic?
- Boundary inputs parsed once into typed values (instead of re-validating raw strings downstream)?
- Typestate/statum considered before bon/constructors?
- Builder choice justified by readability and correctness?
- Module-scoped naming preferred over compound names where practical?
- Compound concepts represented with hierarchical modules where useful (e.g., `chat::panel`)?
- Call sites import descriptive modules first, then qualify (`use ...::chat; chat::panel::Role`)?
- Enum variants read cleanly with their type names (no tautology like `Interactivity::Interactive`)?
- Errors explicit, typed, and properly converted with derive strategy?
- Boundaries respected (domain/app/http/infra)?
- Public Maud components typed and `Render`-based?
- Open Props tokens used before ad-hoc literals for layout/typography/color/motion?
- Component-local semantic aliases (`--_...`) used when mapping tokens to feature-specific meaning?
- Logical properties and fluid/content tokens preferred where applicable?
- Non-essential motion guarded by reduced-motion checks and tokenized easings/durations?
- Docs tone concrete and non-hype?

## Relationship to Tool-Specific Style Files

Tool-specific files such as `docs/<tool>/patterns-style.md` should extend this standard.
They may add stricter rules for a tool, but should not weaken this baseline.

## References for "Parse, don't validate"

- Alexis King, "Parse, don't validate": https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
- Rust `std::str::FromStr`: https://doc.rust-lang.org/std/str/trait.FromStr.html
- Rust `std::convert::TryFrom`: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
- Rust Book, Newtype Pattern: https://doc.rust-lang.org/book/ch20-03-advanced-types.html
- Rust API Guidelines (`C-VALIDATE`): https://rust-lang.github.io/api-guidelines/dependability.html
