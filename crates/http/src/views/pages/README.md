# http::views::pages

Full-page Maud documents live here. Read this directory to separate the proof surfaces from the support pages.

## What this directory proves
- pages are integration surfaces, not the place where reusable UI contracts are invented
- larger demos should compose from `page.rs` and `partials/` instead of growing bespoke page-local markup systems
- proof surfaces such as `lab` and `work*` stay readable because the reusable pieces are pushed downward

## Proof surfaces
- [home.rs](./home.rs) for the portfolio landing page
- [lab.rs](./lab.rs) for the operational demo surface
- [work.rs](./work.rs) and [work_case.rs](./work_case.rs) for focused system writeups

## Support pages
- [login.rs](./login.rs), [register.rs](./register.rs), [protected.rs](./protected.rs), and [chat_moderation.rs](./chat_moderation.rs) for auth and moderation pages

## Rule of thumb
Pages assemble larger surfaces from `page.rs` and `partials/`; they should not become the place where reusable UI contracts are invented.
