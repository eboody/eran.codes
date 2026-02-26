## ⚡ Usage

### <a name="selectors"></a>🔍️ DOM Selection

* Select **one** element: `me(...)`
  * Can be any of:
    * CSS selector: `".button"`, `"#header"`, `"h1"`, `"body > .block"`
    * Variables: `body`, `e`, `some_element`
    * Events: `event.currentTarget` will be used.
    * Surreal selectors: `me()`,`any()`
    * Choose the start location in the DOM with the 2nd arg. (Default: `document`)
      * 🔥 `any('button', me('#header')).classAdd('red')`
        * Add `.red` to any `<button>` inside of `#header`
  * `me()` ⭐ Get parent element of `<script>` without a **.class** or **#id** !
  * `me("body")` Gets `<body>`
  * `me(".button")` Gets the first `<div class="button">...</div>`. To get all of them use `any()`
* Select **one or more** elements as an array: `any(...)`
  * Like `me()` but guaranteed to return an array (or empty array). 
  * `any(".foo")` ⭐ Get all matching elements.
  * Convert between arrays of elements and single elements: `any(me())`, `me(any(".something"))`
 
### 🔥 DOM Functions

* ♻️ All functions work on single elements or arrays of elements.
* 🔗 Start a chain using `me()` and `any()`
  * 🟢 Style A `me().classAdd('red')` ⭐ Chain style. Recommended!
  * 🟠 Style B: `classAdd(me(), 'red')`
* 🌐 Global conveniences help you write less code.
  * `globalsAdd()` will automatically warn you of any clobbering issues!
  * 💀🩸 If you want no conveniences, or are a masochist, delete `globalsAdd()`
    * 🟢 `me().classAdd('red')` becomes `surreal.me().classAdd('red')`
    * 🟠 `classAdd(me(), 'red')` becomes `surreal.classAdd(surreal.me(), 'red')`

See: [Quick Start](#quick-start) and [Reference](#reference) and [No Surreal Needed](#no-surreal)

