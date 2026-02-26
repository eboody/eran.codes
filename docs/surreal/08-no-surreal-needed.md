## <a name="no-surreal"></a>⚪ No Surreal Needed

More often than not, Vanilla JS is the easiest way!

Logging
* 🔥 `console.log()` `console.warn()` `console.error()`
* Event logging: 🔥 `monitorEvents(me())` See: [Chrome Blog](https://developer.chrome.com/blog/quickly-monitor-events-from-the-console-panel-2/)

Benchmarking / Time It!
* 🔥 `console.time('name')`
* 🔥 `console.timeEnd('name')`

Text / HTML Content
* 🔥 `me().textContent = "hello world"`
  * XSS Safe! See: [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
* 🔥 `me().innerHTML = "<p>hello world</p>"`
* 🔥 `me().innerText = "hello world"`

Children
* 🔥 `me().children`
* 🔥 `me().children.hidden = true`

Append / Prepend elements.
* 🔥 `me().prepend(new_element)`
* 🔥 `me().appendChild(new_element)`
* 🔥 `me().insertBefore(element, other_element.firstChild)`
* 🔥 `me().insertAdjacentHTML("beforebegin", new_element)`

AJAX (replace jQuery `ajax()`)
* Use [htmx](https://htmx.org/) or [htmz](https://leanrada.com/htmz/) or [fetch()](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) or [XMLHttpRequest()](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest)
* Example using `fetch()`
```js
me().on("click", async event => {
  let e = me(event)
  // EXAMPLE 1: Hit an endpoint.
  if((await fetch("/webhook")).ok) console.log("Did the thing.")
  // EXAMPLE 2: Get content and replace me()
  try {
    let response = await fetch('/endpoint')
    if (response.ok) e.innerHTML = await response.text()
    else console.warn('fetch(): Bad response')
  }
  catch (error) { console.warn(`fetch(): ${error}`) }
})
```
* Example using `XMLHttpRequest()`
```js
me().on("click", async event => {
  let e = me(event)
  // EXAMPLE 1: Hit an endpoint.
  var xhr = new XMLHttpRequest()
  xhr.open("GET", "/webhook")
  xhr.send()
  // EXAMPLE 2: Get content and replace me()
  var xhr = new XMLHttpRequest()
  xhr.open("GET", "/endpoint")
  xhr.onreadystatechange = () => {
    if (xhr.readyState == 4 && xhr.status >= 200 && xhr.status < 300) e.innerHTML = xhr.responseText
  }
  xhr.send()
})
```

 ## 💎 Conventions & Tips

* Many ideas can be done in HTML / CSS (ex: dropdowns)
* `_` = for temporary or unused variables. Keep it short and sweet!
* `e`, `el`, `elt` = element
* `e`, `ev`, `evt` = event
* `f`, `fn` = function

#### Scope functions and variables inside `<script>`
  * ⭐ Use a block `{ let note = "hi"; function hey(text) { alert(text) }; me().on('click', ev => { hey(note) }) }`
    * `let` and `function` is scoped within `{ }`
  * ⭐ Use `me()`
    *  `me().hey = (text) => { alert(text) }`
    *  `me().on('click', (ev) => { me(ev).hey("hi") })`
  * ⭐ Use an event `me().on('click', ev => { /* add and call function here */ })`
  * Use an inline module: `<script type="module">`
    * Note: `me()` in modules will not see `parentElement`, explicit selectors are required: `me(".mybutton")`

#### Select a void element like `<input type="text" />`
* Use: `me('-')` or `me('prev')` or `me('previous')`
  * 🔥 `<input type="text" /> <script>me('-').value = "hello"</script>`
  * Inspired by the CSS "next sibling" combinator `+` but in reverse `-`
* Or, use a relative start.
  * 🔥 `<form> <input type="text" n1 /> <script>me('[n1]', me()).value = "hello"</script> </form>`

#### Ignore call chain when element is missing.
* 🔥 `me("#i_dont_exist")?.classAdd('active')`
* No warnings: 🔥 `me("#i_dont_exist", document, false)?.classAdd('active')`

