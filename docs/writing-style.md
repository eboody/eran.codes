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

### No stringly domain logic

- Do not compare string literals for business logic.
- Do not use `String` fields for invariant-bearing struct members.
- Convert boundary strings to typed forms as early as possible.

Boundary exception:
- Raw strings are acceptable only at transport/storage boundaries (HTTP DTOs, DB rows, external APIs), and must be converted immediately.

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

### Inline style/script policy

Where inline CSS/JS is appropriate:

- use `css!` and `js!` macros,
- scope styles with `css-scope-inline`,
- use `surreal.js` helpers where they reduce DOM boilerplate and improve clarity.

## Readability Standard

Code should read like a domain explanation.

- Prefer descriptive names over abbreviations.
- Prefer explicit types over inferred ambiguity when intent is unclear.
- Prefer short, composable functions over long mixed-responsibility functions.
- Keep constructor/wiring call sites self-explanatory.

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
- Typestate/statum considered before bon/constructors?
- Builder choice justified by readability and correctness?
- Errors explicit, typed, and properly converted with derive strategy?
- Boundaries respected (domain/app/http/infra)?
- Public Maud components typed and `Render`-based?
- Docs tone concrete and non-hype?

## Relationship to Tool-Specific Style Files

Tool-specific files such as `docs/<tool>/patterns-style.md` should extend this standard.
They may add stricter rules for a tool, but should not weaken this baseline.
