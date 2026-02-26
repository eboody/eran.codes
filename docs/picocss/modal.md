# Modal

- Route: `/docs/modal`
- Category: Components
- Source: `app/routes/docs.modal.jsx`

The classic modal component with graceful spacings across devices and viewports, using the semantic HTML tag `<dialog>`.

## Key Sections
- Syntax
- Demo
- Utilities

## Example Snippets
```html
<dialog open>
  <article>
    <header>
      <button aria-label="Close" rel="prev"></button>
      <p>
        <strong>🗓️ Thank You for Registering!</strong>
      </p>
    </header>
    <p>
      We're excited to have you join us for our
      upcoming event. Please arrive at the museum 
      on time to check in and get started.
    </p>
    <ul>
      <li>Date: Saturday, April 15</li>
      <li>Time: 10:00am - 12:00pm</li>
    </ul>
  </article>
</dialog>
```

```html
<dialog open>
  <article>
    <h2>Confirm Your Membership</h2>
    <p>
      Thank you for signing up for a membership!
      Please review the membership details below:
    </p>
    <ul>
      <li>Membership: Individual</li>
      <li>Price: $10</li>
    </ul>
    <footer>
      <button class="secondary">
        Cancel
      </button>
      <button>Confirm</button>
    </footer>
  </article>
</dialog>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
