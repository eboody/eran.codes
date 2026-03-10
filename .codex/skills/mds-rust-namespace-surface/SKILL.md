---
name: mds-rust-namespace-surface
version: 0.1.0
description: Module-first Rust naming and export rules for this repo, including descriptive namespace roots, anti-flattening imports, and companion type-family shaping.
scope: project
---

# mds-rust-namespace-surface

## Purpose
Keep Rust APIs module-first: the module carries the namespace, generic companion nouns stay inside it, and call sites keep that namespace visible.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic Rust surface.

## Use
- Use before changing Rust module layout, `mod.rs`, `use`, `pub use`, or public type surfaces.
- Required for `mds-codegen` and any manual Rust edit that reshapes namespace roots.
- Pair with `docs/writing-style.md` and `scripts/ci/descriptive-module-imports.sh`.

## Core Rules
- If a module path is part of the intended API vocabulary, expose the module and qualify from it.
- Prefer `use domain::user;` then `user::Id`, `user::Repository`, `user::Username`.
- Avoid `use domain::user::Id;` and `use domain::user::Repository;` when `user` is the surface.
- Avoid parent-level flattening like `pub use crate::domain::user::Id;` or `pub use crate::domain::user::Repository;`.
- Keep concise local nouns inside the namespace; avoid standalone prefixed companions when the namespace will stay visible.
- Use standalone descriptive names only when the module context is intentionally hidden or the noun is already uniquely descriptive.

## Namespace Root Markers
- In an exposing `mod.rs`, mark intentional namespace roots with `// ci: descriptive-module-import <full_module_path>`.
- Once marked, CI rejects leaf `use` and `pub use` against that module path.
- Inside a marked module tree, consume the parent namespace instead of sibling leaf modules.

## Good
```rust
use crate::domain::user;

fn lookup(repo: &dyn user::Repository, id: user::Id) {}
```

```rust
pub use crate::views::partials::components::tab_set;
```

## Bad
```rust
use crate::domain::user::Repository;
use crate::domain::user::Id;
```

```rust
pub use crate::domain::user::Repository;
pub use crate::domain::user::Id;
```

## Exceptions
- Flat `Error` / `Result` crate surfaces are allowed when they are the canonical crate boundary.
- Unique nouns like `PasswordHash`, `RegisterUser`, and `RequestMeta` can stand alone when the name already carries enough meaning.
