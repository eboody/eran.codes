## ✨ What does it add to Javascript?

* ⚡️ [Locality of Behavior (LoB)](https://htmx.org/essays/locality-of-behaviour/) Use `me()` inside `<script>`
  * No **.class** or **#id** needed! Get an element without creating a unique name.
  * `this` but much more flexible!
  * Want `me` in your CSS `<style>` tags, too? See our [companion script](https://github.com/gnat/css-scope-inline)
* 🔗 Call chaining, jQuery style.
* ♻️ Functions work seamlessly on 1 element or arrays of elements!
  * All functions can use: `me()`, `any()`, `NodeList`, `HTMLElement` (..or arrays of these!)
  * Get 1 element: `me()`
  * ..or many elements: `any()`
  * `me()` or `any()` can chain with any Surreal function.
    * `me()` can be used directly as a single element (like `querySelector()` or `$()`)
    * `any()` can use: `for` / `forEach` / `filter` / `map` (like `querySelectorAll()` or `$()`)
* 🌗 No forced style. Use: `classAdd` or `class_add` or `addClass` or `add_class`
  * Use `camelCase` (Javascript) or `snake_case` (Python, Rust, PHP, Ruby, SQL, CSS).

### 🤔 Why use `me()` / `any()` instead of `$()`
* 💡 Solves the classic jQuery bloat problem: Am I getting 1 element or an array of elements?
  * `me()` is guaranteed to return 1 element (or first found, or null).
  * `any()` is guaranteed to return an array (or empty array).
  * No more checks = write less code. Bonus: Reads more like self-documenting english.

