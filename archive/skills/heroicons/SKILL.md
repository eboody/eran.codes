---
name: heroicons
description: Use this skill for Rust `heroicons` crate (`0.2.0`) icon lookup, variant selection, and code snippets based on the locally mirrored docs.rs reference.
---

# Heroicons

## Overview

Use this skill when the task involves the Rust `heroicons` crate, including:
- Choosing icon names (`icon_name::*`) from the catalog
- Picking icon variants (`Outline`, `Solid`, `Mini`, `Micro`)
- Generating Rust snippets that render SVGs or `hypertext` components
- Verifying API details against the local docs.rs mirror for version `0.2.0`

## References

Primary local references:
- `references/docsrs/docs.rs/heroicons/0.2.0/heroicons/index.html`
- `references/docsrs/docs.rs/heroicons/0.2.0/heroicons/all.html`
- `references/docsrs/docs.rs/heroicons/0.2.0/heroicons/icon_name/index.html`
- `references/docsrs/docs.rs/heroicons/0.2.0/heroicons/icon_variant/index.html`
- `references/docsrs/docs.rs/heroicons/0.2.0/heroicons/trait.ToSvg.html`
- `references/docsrs/docs.rs/heroicons/0.2.0/heroicons/struct.Icon.html`

## Workflow

1. Confirm the request targets `heroicons` crate version `0.2.0`.
2. Resolve candidate icon names from `all.html` or `icon_name/index.html`.
3. Pick variant:
   - `Outline` (24x24, outlined)
   - `Solid` (24x24, filled)
   - `Mini` (20x20, filled)
   - `Micro` (16x16, filled)
4. Produce one of the canonical usage forms below.
5. If icon naming is ambiguous, provide 2-4 likely `icon_name::*` options and ask the user to choose.

## Output Expectations

- Keep responses grounded in local mirrored docs.
- Include `sources_used` with exact local file paths consulted.

## Canonical Usage

Basic icon -> SVG string:

```rust
use heroicons::{Icon, icon_name::*, icon_variant::*};

let home_icon = Icon {
    name: Home,
    variant: Outline,
    ..Default::default()
};

let svg_string = home_icon.to_string();
```

Hypertext `rsx!`:

```rust
use heroicons::{Icon, icon_name::*, icon_variant::*};
use hypertext::prelude::*;

let page = rsx! {
    <div>
        <Icon name=(Home) variant=(Outline) ../>
        <Icon name=(User) variant=(Solid) ../>
    </div>
}.render();
```

Hypertext `maud!`:

```rust
use heroicons::{Icon, icon_name::*, icon_variant::*};
use hypertext::prelude::*;

let page = maud! {
    div {
        Icon name=(Home) variant=(Outline) ..;
        Icon name=(User) variant=(Solid) ..;
    }
}.render();
```

## Lookup Patterns

When searching locally, prefer `rg`:

```bash
# List icon names
rg -o "icon_name::[A-Za-z0-9]+" references/docsrs/docs.rs/heroicons/0.2.0/heroicons/all.html | sort -u

# Search icon names by keyword (example: "chat")
rg -o "icon_name::[A-Za-z0-9]+" references/docsrs/docs.rs/heroicons/0.2.0/heroicons/all.html | sort -u | rg -i chat
```

## Response Rules

- Use imports from crate docs examples unless user asks for a different style.
- Keep icon names in PascalCase (for example `ChatBubbleLeftRight`).
- Mention variant dimensions when helping choose between `Outline`/`Solid`/`Mini`/`Micro`.
- If exact match is uncertain, return best matches instead of guessing a single icon.
- If icon integration changes UI visuals in this repo, baseline update is required:
  - `VISUAL_UPDATE_BASELINE=1 scripts/ci/visual-snapshot.sh`
  - then `scripts/ci/visual-snapshot.sh`
  - report baseline status in the final response.
