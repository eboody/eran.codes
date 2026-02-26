# Forms overview

- Route: `/docs/forms`
- Category: Forms
- Source: `app/routes/docs.forms._index.jsx`

All form elements are fully responsive with pure semantic HTML, enabling forms to scale gracefully across devices and viewports.

## Key Sections
- Introduction
- Helper text
- Usage with grid
- Usage with group

## Example Snippets
```html
<form>
  <fieldset>
    <label>
      First name
      <input
        name="first_name"
        placeholder="First name"
        autocomplete="given-name"
      />
    </label>
    <label>
      Email
      <input
        type="email"
        name="email"
        placeholder="Email"
        autocomplete="email"
      />
    </label>
  </fieldset>

  <input
    type="submit"
    value="Subscribe"
  />
</form>
```

```html
<form>
  
  <!-- Input inside label -->
  <label>
    First name
    <input
      name="first_name"
      placeholder="First name"
      autocomplete="given-name"
    />
  </label>

  <!-- Input outside label -->
  <label for="email">Email</label>
  <input
    type="email"
    id="email"
    placeholder="Email"
    autocomplete="email"
  />

</form>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
