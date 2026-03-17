# Token Layers

Use three layers of tokens:

1. Open Props primitives
2. workspace semantic aliases
3. component-local aliases

## Layer 1: Open Props

Use Open Props as the primitive source for sizing, radii, typography, shadows, easings, and other low-level values.

Do not treat raw Open Props names as the normal authored API for shared components.

When modern CSS features such as relative color syntax or `color-mix()` improve token derivation, prefer them over duplicating hand-tuned literal variants.

## Layer 2: Workspace Semantic Aliases

Define or consume semantic aliases in `crates/http/static/app.css`, usually in `:root` and dark-mode overrides.

Preferred families:
- `--ui-*`
- `--surface-*`
- `--text-*`
- `--border-*`
- `--accent-*`
- `--space-*`
- `--radius-*`
- `--shadow-*`
- `--motion-*`

Typography should follow the same rule as spacing and surfaces:
- do not leave authored component CSS full of ad hoc `font-size`, `line-height`, or `letter-spacing` literals when the value can be expressed as a semantic alias
- promote repeated text-scale meanings into the shared token layer in `app.css`
- use local aliases only when the typography choice is truly component-specific and not yet shared
- do not leave raw rhythm and spacing numbers directly in declarations when a shared token or small component-owned alias should carry that value instead
- raw primitives such as `--size-*` are fine as the source layer when they are being mapped into the repo's actual token vocabulary or a real component metric; the problem is declaration-site magic numbers, not the primitive name by itself
- do not create a local alias that only renames one selector's single semantic token use; keep the semantic token inline unless the alias earns its keep across multiple declarations or breakpoint overrides

## Layer 3: Component-Local Aliases

Create local aliases only when they improve readability inside one component and should not yet be promoted.

Name them with a local-only prefix such as `--_component-*`.

For typography, local aliases are the temporary escape hatch, not the default end state. If the same alias pattern shows up in multiple components, promote it.

Avoid local alias farms. If a component root is mostly pass-through variables that mirror shared tokens one-to-one, collapse them back to the shared tokens and keep only the aliases that own a real component metric, repeated relationship, or breakpoint/state override.
