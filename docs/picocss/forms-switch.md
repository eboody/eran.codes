# Switch

- Route: `/docs/forms/switch`
- Category: Forms
- Source: `app/routes/docs.forms.switch.jsx`

A switch component in pure CSS, using the checkbox syntax.

## Key Sections
- Syntax
- Disabled
- Validation states

## Example Snippets
```html
<fieldset>
  <label>
    <input name="terms" type="checkbox" role="switch" />
    I agree to the Terms
  </label>
  <label>
    <input name="opt-in" type="checkbox" role="switch" checked />
    Receive news and offers
  </label>
</fieldset>
```

```html
<fieldset>
  <label>
    <input name="publish" type="checkbox" role="switch" disabled />
    Publish on my profile
  </label>
  <label>
    <input name="change-password" type="checkbox" role="switch" checked disabled />
    Change my password at next login
  </label>
</fieldset>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
