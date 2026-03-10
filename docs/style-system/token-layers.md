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

## Layer 3: Component-Local Aliases

Create local aliases only when they improve readability inside one component and should not yet be promoted.

Name them with a local-only prefix such as `--_component-*`.
