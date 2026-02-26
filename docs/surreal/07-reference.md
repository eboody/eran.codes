## <a name="reference"></a>👁️ Functions
Looking for [DOM Selectors](#selectors)?
Looking for stuff [we recommend doing in vanilla JS](#no-surreal)?
### 🧭 Legend
* 🔗 Chainable off `me()` and `any()`
* 🌐 Global shortcut.
* 🔥 Runnable example.
* 🔌 Built-in Plugin
### 👁️ At a glance

* 🔗 `run`
  * It's `forEach` but less wordy and works on single elements, too!
  * 🔥 `me().run(e => { alert(e) })`
  * 🔥 `any('button').run(e => { alert(e) })`
* 🔗 `remove`
  * 🔥 `me().remove()`
  * 🔥 `any('button').remove()`
* 🔗 `classAdd` 🌗 `class_add` 🌗 `addClass` 🌗 `add_class`
  * 🔥 `me().classAdd('active')`
  * Leading `.` is **optional**
    * Same thing: `me().classAdd('active')` 🌗 `me().classAdd('.active')`
* 🔗 `classRemove` 🌗 `class_remove` 🌗 `removeClass` 🌗 `remove_class`
  * 🔥 `me().classRemove('active')`
* 🔗 `classToggle` 🌗 `class_toggle` 🌗 `toggleClass` 🌗 `toggle_class`
  * 🔥 `me().classToggle('active')`
* 🔗 `styles`
  * 🔥 `me().styles('color: red')` Add style.
  * 🔥 `me().styles({ 'color':'red', 'background':'blue' })` Add multiple styles.
  * 🔥 `me().styles({ 'background':null })` Remove style.
* 🔗 `attribute` 🌗 `attributes` 🌗 `attr`
  * Get: 🔥 `me().attribute('data-x')`
    * For single elements.
    * For many elements, wrap it in: `any(...).run(...)` or `any(...).forEach(...)`
  * Set: 🔥`me().attribute('data-x', true)`
  * Set multiple: 🔥 `me().attribute({ 'data-x':'yes', 'data-y':'no' })`
  * Remove: 🔥 `me().attribute('data-x', null)`
  * Remove multiple: 🔥 `me().attribute({ 'data-x': null, 'data-y':null })`
* 🔗 `send` 🌗 `trigger`
  * 🔥 `me().send('change')`
  * 🔥 `me().send('change', {'data':'thing'})`
  * Wraps `dispatchEvent`
* 🔗 `on`
  * 🔥 `me().on('click', ev => { me(ev).styles('background', 'red') })`
  * Wraps `addEventListener`
* 🔗 `off`
  * 🔥 `me().off('click', fn)`
  * Wraps `removeEventListener`
* 🔗 `offAll`
  * 🔥 `me().offAll()`
* 🔗 `disable`
  * 🔥 `me().disable()`
  * Easy alternative to `off()`. Disables click, key, submit events.
* 🔗 `enable`
  * 🔥 `me().enable()`
  * Opposite of `disable()`
* 🌐 `createElement` 🌗 `create_element`
  * 🔥 `e_new = createElement("div"); me().prepend(e_new)`
  * Alias of [document.createElement](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
* 🌐 `sleep`
  * 🔥 `await sleep(1000, ev => { alert(ev) })`
  * `async` version of `setTimeout`
  * Wonderful for animation timelines.
* 🌐 `halt`
  * 🔥 `halt(event)`
  * When recieving an event, stop propagation, and prevent default actions (such as form submit).
  * Wrapper for [stopPropagation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation) and [preventDefault](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)
* 🌐 `tick`
  * 🔥 `await tick()`
  * `await` version of `rAF` / `requestAnimationFrame`.
  * Waits for 1 frame (browser paint).
  * Useful to guarantee CSS properties are applied, and events have propagated.
* 🌐 `rAF`
  * 🔥 `rAF(e => { return e })`
  * Calls after 1 frame (browser paint). Alias of [requestAnimationFrame](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)
  * Useful to guarantee CSS properties are applied, and events have propagated.
* 🌐 `rIC`
  * 🔥 `rIC(e => { return e })`
  * Calls when Javascript is idle. Alias of [requestIdleCallback](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)
* 🌐 `onloadAdd` 🌗 `onload_add` 🌗 `addOnload` 🌗 `add_onload`
  * 🔥 `onloadAdd(_ => { alert("loaded!"); })`
  * 🔥 `<script>let e = me(); onloadAdd(_ => { me(e).on("click", ev => { alert("clicked") }) })</script>`
  * Execute after the DOM is ready. Similar to jquery `ready()`
  * Add to `window.onload` while preventing overwrites of `window.onload` and predictable loading!
  * Alternatives:
    * Skip missing elements using `?.` example: `me("video")?.requestFullscreen()`
    * Place `<script>` after the loaded element.
      * See `me('-')` / `me('prev')`
* 🔌 `fadeOut`
  * See below
* 🔌 `fadeIn`
  * See below

### <a name="plugin-included"></a>🔌 Built-in Plugins

### Effects
Build effects with `me().styles({...})` with timelines using [CSS transitioned `await` or callbacks](#timelines).

Common effects included:

* 🔗 `fadeOut` 🌗 `fade_out`
  * Fade out and remove element.
  * Keep element with `remove=false`.
  * 🔥 `me().fadeOut()`
  * 🔥 `me().fadeOut(ev => { alert("Faded out!") }, 3000)` Over 3 seconds then call function.

* 🔗 `fadeIn` 🌗 `fade_in`
  * Fade in existing element which has `opacity: 0`
  * 🔥 `me().fadeIn()`
  * 🔥 `me().fadeIn(ev => { alert("Faded in!") }, 3000)` Over 3 seconds then call function.


