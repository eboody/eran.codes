# Landmarks & section

- Route: `/docs/landmarks-section`
- Category: Layout
- Source: `app/routes/docs.landmarks-section.jsx`

Structure your pages with semantic landmarks and sections for better accessibility and graceful spacings.

## Key Sections
- Landmarks
- Custom root container
- Section

## Example Snippets
```html
<body>
  <header>...</header>
  <main>...</main>
  <footer>...</footer>
</body>
```

```scss
/* Custom Class-less version for React */
@use "pico" with (
  
  // Define the root element used to target <header>, <main>, <footer>
  // with $enable-semantic-container and $enable-responsive-spacings
  $semantic-root-element: "#root",
  
  // Enable <header>, <main>, <footer> inside $semantic-root-element as containers
  $enable-semantic-container: true,

  // Enable .classes
  $enable-classes: false
)
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
