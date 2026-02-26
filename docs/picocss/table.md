# Table

- Route: `/docs/table`
- Category: Content
- Source: `app/routes/docs.table.jsx`

Clean and minimal styles for `<table>`, providing consistent spacings and a minimal unbordered look.

## Key Sections
- Syntax
- Color schemes
- Striped

## Example Snippets
```html
<table>
  <thead>
    <tr>
      <th scope="col">Planet</th>
      <th scope="col">Diameter (km)</th>
      <th scope="col">Distance to Sun (AU)</th>
      <th scope="col">Orbit (days)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th scope="row">Mercury</th>
      <td>4,880</td>
      <td>0.39</td>
      <td>88</td>
    </tr>
    <tr>
      <th scope="row">Venus</th>
      <td>12,104</td>
      <td>0.72</td>
      <td>225</td>
    </tr>
    <tr>
      <th scope="row">Earth</th>
      <td>12,742</td>
      <td>1.00</td>
      <td>365</td>
    </tr>
    <tr>
      <th scope="row">Mars</th>
      <td>6,779</td>
      <td>1.52</td>
      <td>687</td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <th scope="row">Average</th>
      <td>9,126</td>
      <td>0.91</td>
      <td>341</td>
    </tr>
  </tfoot>
</table>
```

```html
<table class="striped">
  ...
</table>
```

## Notes
- Generated from the Pico docs route source above.
- For exact examples, inspect the source file listed at the top.
