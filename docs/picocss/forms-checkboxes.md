# Checkboxes

- Route: `/docs/forms/checkboxes`
- Category: Forms
- Source: `app/routes/docs.forms.checkboxes.jsx`

The native `<input>` with a custom and responsive style.

## Key Sections
- Syntax
- Horizontal stacking
- Indeterminate
- Validation states

## Example Snippets
```html
<fieldset>
  <legend>Language preferences:</legend>
  <label>
    <input type="checkbox" name="english" checked />
    English
  </label>
  <label>
    <input type="checkbox" name="french" checked />
    French
  </label>
  <label>
    <input type="checkbox" name="mandarin" />
    Mandarin
  </label>
  <label>
    <input type="checkbox" name="thai" />
    Thai
  </label>
  <label aria-disabled="true">
    <input type="checkbox" name="dothraki" disabled />
    Dothraki
  </label>
</fieldset>
```

```html
<fieldset>
  <legend>Language preferences:</legend>
  <input type="checkbox" id="hindi" name="hindi" checked />
  <label htmlFor="hindi">Hindi</label>
  <input type="checkbox" id="swahili" name="swahili" />
  <label htmlFor="swahili">Swahili</label>
  <input type="checkbox" id="navi" name="navi" disabled />
  <label htmlFor="navi" aria-disabled="true">Na'vi</label>
</fieldset>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
