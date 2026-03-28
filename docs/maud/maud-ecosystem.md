# The Maud Ecosystem

Maud is small by design, but there are a few tools that round out the authoring
experience. This page highlights community projects that pair well with Maud
templates.

## Template helpers

### maud-extensions

[maud-extensions](https://github.com/eboody/maud-extensions) provides proc
macros for inline CSS, inline JavaScript, and font helpers. It is especially
useful when you keep small, component-scoped styles or scripts close to your
Maud views.

Repo note: `eran_codes` does not currently use `maud-extensions`. The current
repo-local pattern is the custom scoped-style helper in
`crates/http/src/views/scoped.rs` plus a few hand-authored inline scripts where
needed. Treat `maud-extensions` here as an optional migration path, not current
doctrine.

- Crates.io: <https://crates.io/crates/maud-extensions>
- Docs: <https://docs.rs/maud-extensions>

## Formatting

### maudfmt

[maudfmt](https://github.com/Jeosas/maudfmt) is a formatter for Maud templates.
It can help keep large template files consistent and readable across a team.
