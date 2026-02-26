# Radios

- Route: `/docs/forms/radios`
- Category: Forms
- Source: `app/routes/docs.forms.radios.jsx`

The native `<input>` with a custom and responsive style.

## Key Sections
- Syntax
- Horizontal stacking
- Validation states

## Example Snippets
```html
<fieldset>
  <legend>Language preference:</legend>
  <label>
    <input type="radio" name="language" checked />
    English
  </label>
  <label>
    <input type="radio" name="language" />
    French
  </label>
  <label>
    <input type="radio" name="language" />
    Mandarin
  </label>
  <label>
    <input type="radio" name="language" />
    Thai
  </label>
  <label aria-disabled="true">
    <input type="radio" name="language" disabled />
    Dothraki
  </label>
</fieldset>
```

```html
<fieldset>
  <legend>Second language:</legend>
  <input type="radio" id="hindi" name="second-language" checked />
  <label htmlFor="hindi">Hindi</label>
  <input type="radio" id="swahili" name="second-language" />
  <label htmlFor="swahili">Swahili</label>
  <input type="radio" id="navi" name="second-language" disabled />
  <label htmlFor="navi" aria-disabled="true">Na'vi</label>
</fieldset>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
