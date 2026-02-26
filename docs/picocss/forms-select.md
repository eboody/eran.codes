# Select

- Route: `/docs/forms/select`
- Category: Forms
- Source: `app/routes/docs.forms.select.jsx`

The native `<select>` is styled like the input for consistency.

## Key Sections
- Syntax
- Multiple
- Disabled
- Validation states
- Dropdown

## Example Snippets
```html
<select name="favorite-cuisine" aria-label="Select your favorite cuisine..." required>
  <option selected disabled value="">
    Select your favorite cuisine...
  </option>
  <option>Italian</option>
  <option>Japanese</option>
  <option>Indian</option>
  <option>Thai</option>
  <option>French</option>
</select>
```

```html
<select aria-label="Select your favorite snacks..." multiple size="6">
  <option disabled>
    Select your favorite snacks...
  </option>
  <option>Cheese</option>
  <option selected>Fruits</option>
  <option selected>Nuts</option>
  <option>Chocolate</option>
  <option>Crackers</option>
</select>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
