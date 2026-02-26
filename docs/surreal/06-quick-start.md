## <a name="quick-start"></a>⚡ Quick Start

* Add a class
  * `me().classAdd('red')`
  * `any("button").classAdd('red')`
* Events
  * `me().on("click", ev => me(ev).fadeOut() )`
  * `any('button').on('click', ev => { me(ev).styles('color: red') })`
* Run functions over elements.
  * `any('button').run(_ => { alert(_) })`
* Styles / CSS
  * `me().styles('color: red')`
  * `me().styles({ 'color':'red', 'background':'blue' })`
* Attributes
  * `me().attribute('active', true)`

<a name="timelines"></a>
#### Timeline animations without any libraries.
```html
<div>I change color every second.
  <script>
    // On click, animate something new every second.
    me().on("click", async ev => {
      let el = me(ev) // Save target because async will lose it.
      me(el).styles({ "transition": "background 1s" })
      await sleep(1000)
      me(el).styles({ "background": "red" })
      await sleep(1000)
      me(el).styles({ "background": "green" })
      await sleep(1000)
      me(el).styles({ "background": "blue" })
      await sleep(1000)
      me(el).styles({ "background": "none" })
      await sleep(1000)
      me(el).remove()
    })
  </script>
</div>
```
```html
<div>I fade out and remove myself.
  <script>me().on("click", ev => { me(ev).fadeOut() })</script>
</div>
```
```html
<div>Change color every second.
  <script>
    // Run immediately.
    (async (e = me()) => {
      me(e).styles({ "transition": "background 1s" })
      await sleep(1000)
      me(e).styles({ "background": "red" })
      await sleep(1000)
      me(e).styles({ "background": "green" })
      await sleep(1000)
      me(e).styles({ "background": "blue" })
      await sleep(1000)
      me(e).styles({ "background": "none" })
      await sleep(1000)
      me(e).remove()
    })()
  </script>
</div>
```
```html
<script>
  // Run immediately, for every <button> globally!
  (async () => {
    any("button").fadeOut()
  })()
</script>
```
#### Array methods
```js
any('button')?.forEach(...)
any('button')?.map(...)
```

