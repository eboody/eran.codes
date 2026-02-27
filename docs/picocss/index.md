# Pico CSS Docs (Workspace Mirror)

This directory mirrors Pico documentation routes from `/home/eran/code/picocss.com` into local markdown files.

## Category Indexes
- [About](about.md)
- [Components](components.md)
- [Content](content.md)
- [Customization](customization.md)
- [Forms](forms.md)
- [Getting started](getting-started.md)
- [Layout](layout.md)

## Global References
- [Route map](route-map.md)
- [Version picker theme variants](version-picker-themes.md)

## Workspace conventions
- Prefer component-local scoped styling via `inline_css!` + `(css())` with `me` selectors in Maud components.
- Keep `crates/http/static/app.css` for shared Pico variables and broadly reused primitives.
- Avoid extra classes when a structural selector can express the same rule.
- Avoid magic numbers in component CSS; define reusable tokens and override tokens responsively.
- In Maud call sites, avoid `.build().render()` chains; use `.build()` directly in `html!` splices.

## Pages
- [Quick start](quick-start.md) — `/docs`
- [Accordions](accordion.md) — `/docs/accordion`
- [Brand](brand.md) — `/docs/brand`
- [Built With](built-with.md) — `/docs/built-with`
- [Button](button.md) — `/docs/button`
- [Card](card.md) — `/docs/card`
- [Class-less version](classless.md) — `/docs/classless`
- [Color schemes](color-schemes.md) — `/docs/color-schemes`
- [Colors](colors.md) — `/docs/colors`
- [Conditional styling](conditional.md) — `/docs/conditional`
- [Container](container.md) — `/docs/container`
- [CSS variables](css-variables.md) — `/docs/css-variables`
- [Dropdown](dropdown.md) — `/docs/dropdown`
- [Forms overview](forms-overview.md) — `/docs/forms`
- [Checkboxes](forms-checkboxes.md) — `/docs/forms/checkboxes`
- [Input](forms-input.md) — `/docs/forms/input`
- [Radios](forms-radios.md) — `/docs/forms/radios`
- [Range](forms-range.md) — `/docs/forms/range`
- [Select](forms-select.md) — `/docs/forms/select`
- [Switch](forms-switch.md) — `/docs/forms/switch`
- [Textarea](forms-textarea.md) — `/docs/forms/textarea`
- [Grid](grid.md) — `/docs/grid`
- [Group](group.md) — `/docs/group`
- [Landmarks & section](landmarks-section.md) — `/docs/landmarks-section`
- [Link](link.md) — `/docs/link`
- [Loading](loading.md) — `/docs/loading`
- [Mission](mission.md) — `/docs/mission`
- [Modal](modal.md) — `/docs/modal`
- [Nav](nav.md) — `/docs/nav`
- [Overflow auto](overflow-auto.md) — `/docs/overflow-auto`
- [Progress](progress.md) — `/docs/progress`
- [RTL](rtl.md) — `/docs/rtl`
- [Sass](sass.md) — `/docs/sass`
- [Table](table.md) — `/docs/table`
- [Tooltip](tooltip.md) — `/docs/tooltip`
- [Typography](typography.md) — `/docs/typography`
- [Usage scenarios](usage-scenarios.md) — `/docs/usage-scenarios`
- [What’s new in v2?](v2.md) — `/docs/v2`
- [Version picker](version-picker.md) — `/docs/version-picker`
