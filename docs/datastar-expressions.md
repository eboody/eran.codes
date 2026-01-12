# Datastar Expressions Notes

Use this as a concise reference for Datastar expression behavior and scripting
guidelines in this repo.

## Expressions
- Expressions are strings evaluated by `data-*` attributes.
- `$foo` reads the signal value of `foo`.
- `el` is available and refers to the element the attribute is on.
- Expressions can use JavaScript operators (`?:`, `||`, `&&`) to keep logic terse.
- Multiple statements are allowed, separated by `;`.
- Line breaks do not separate statements; use `;` explicitly.

## Use JavaScript Wisely
- Prefer `data-*` attributes for reactive logic.
- If expressions become complex, extract to external scripts or web components.
- Keep frontend state minimal; send props down and events up.

## External Scripts
- Pass data via arguments; return a result or dispatch an event.
- Async work should dispatch a custom event and update signals from it.

## Web Components
- Pass data via attributes (`data-attr:*`).
- Listen to custom events for results.
- `data-bind` can work if the component dispatches a `change` event.

## Executing Scripts From Backend
- `text/javascript` responses execute directly in the browser.
- SSE can send `datastar-patch-elements` with `<script>` tags.
- Prefer `ExecuteScript` when only script execution is needed.
