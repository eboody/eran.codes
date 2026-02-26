## ⚡ Workflow Tips

* Flat, 1 selector per line can be very short like Tailwind. See the examples.
* Use just plain CSS variables in your design system.
* Use the short `@media` queries for responsive design.
  * Mobile First (flow: **above** breakpoint): **🟢 None (xs)** `sm` `md` `lg` `xl` `xx` 🏁
  * Desktop First (flow: **below** breakpoint): 🏁 `xs-` `sm-` `md-` `lg-` `xl-` **🟢 None (xx)**
  * 🟢 = No breakpoint. Default. See the [Live Example](https://gnat.github.io/css-scope-inline/example.html)!
  * Based on [Tailwind](https://tailwindcss.com/docs/responsive-design) breakpoints. We use `xx` not `2xl` to not break CSS highlighters.
  * Unlike Tailwind, you can [nest your @media styles](https://developer.chrome.com/articles/css-nesting/#nesting-media)!
* Positional selectors may be easier using `div[n1]` for `<div n1>` instead of `div:nth-child(1)`
* Try tools like- Auto complete styles: [VSCode](https://code.visualstudio.com/) or [Sublime](https://packagecontrol.io/packages/Emmet)

