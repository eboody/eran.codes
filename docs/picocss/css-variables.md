# CSS variables

- Route: `/docs/css-variables`
- Category: Customization
- Source: `app/routes/docs.css-variables.jsx`

Customize Pico's design system with over 130 CSS variables to create a unique look and feel.

## Key Sections
- Introduction
- Example
- Color schemes
- All CSS variables

## Example Snippets
```html
<style>
  :root {
    --pico-border-radius: 2rem;
    --pico-typography-spacing-vertical: 1.5rem;
    --pico-form-element-spacing-vertical: 1rem;
    --pico-form-element-spacing-horizontal: 1.25rem;
  }
  h1 {
    --pico-font-family: Pacifico, cursive;
    --pico-font-weight: 400;
    --pico-typography-spacing-vertical: 0.5rem;
  }
  button {
    --pico-font-weight: 700;
  }
</style>

<h1>Music fest mania</h1>
<p>
  Get ready to dance and sing your heart out at 
  our Music Fest Mania. Join the crowd, jam to
  your favorite band, and discover new artists.
</p>
<button>Let's rock out!</button>
```

```css
/* Light color scheme (Default) */
/* Can be forced with data-theme="light" */
[data-theme="light"],
:root:not([data-theme="dark"]) {
 ...
}
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
