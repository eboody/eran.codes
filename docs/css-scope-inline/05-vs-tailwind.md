## 🤔 Why consider this over Tailwind CSS?

Use whatever you'd like, but there's a few advantages with this approach over Tailwind, Twind, UnoCSS:

* No [repeated styles](https://tailwindcss.com/docs/reusing-styles) on child elements (..no [@apply](https://tailwindcss.com/docs/reusing-styles#extracting-classes-with-apply), no `[&>thing]` on each style).
* No repeated prefixes for media queries, hover, focus, etc.
* No visual noise on every `<div>`. Use a local `<style>` per group.
* Share syntax between local and external styles. It's just CSS.
* Regain your "inspect, play with styles, paste" workflow in your web browser!
* No suffering from lost syntax highlighting on properties and units.
* No high risk of eventually requiring a build step.
* No chance of [deprecations](https://windicss.org/posts/sunsetting.html). 16 lines is infinitely maintainable.
* No suffering from FOUC (a flash of unstyled content).
* Zero friction movement of styles between inline and `.css` files. Just replace `me`
* No special tooling or plugins to install.

