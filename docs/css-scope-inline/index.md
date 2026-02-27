# CSS Scope Inline Docs (Workspace Mirror)

This directory mirrors CSS Scope Inline documentation from the upstream repository.

- Upstream: https://github.com/gnat/css-scope-inline
- Snapshot source: `/tmp/css-scope-inline-src/README.md`

## Contents
- [00-intro.md](00-intro.md)
- [01-why.md](01-why.md)
- [02-how-it-looks.md](02-how-it-looks.md)
- [03-how-it-works.md](03-how-it-works.md)
- [04-install.md](04-install.md)
- [05-vs-tailwind.md](05-vs-tailwind.md)
- [06-workflow-tips.md](06-workflow-tips.md)
- [07-showdowns.md](07-showdowns.md)
- [08-technical-faq.md](08-technical-faq.md)
- [upstream-readme.md](upstream-readme.md)
- [script-source.md](script-source.md)
- [example-source.md](example-source.md)

## Workspace conventions
- Default to component-scoped `inline_css!` helpers with `me` selectors, inserted with `(css())` inside the component root.
- Prefer short, targeted hooks (`id`, semantic class, stable `data-*`) over long selector chains.
- Use quick `me` selectors for simple local rules; when nested styling gets deep, split into a subcomponent.
- Avoid magic-number-heavy rules; define component design tokens and update tokens for responsive breakpoints.
- Treat global `app.css` additions as a shared-style exception, not the default.
