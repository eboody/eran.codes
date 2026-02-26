# Grid

- Route: `/docs/grid`
- Category: Layout
- Source: `app/routes/docs.grid.jsx`

Create minimal responsive layouts with .grid to enable auto-layout columns.

## Key Sections
- Syntax
- About CSS Grids

## Example Snippets
```html
<div class="grid">
  ${Array.from({ length: columns }, (_, i) => {
    const index = i + 1;
    return `${i > 0 ? "  " : ""}<div>${index}</div>`;
  }).join("\n")}
</div>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
