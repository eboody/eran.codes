## <a name="plugins"></a>🔌 Your own plugin

Feel free to edit Surreal directly- but if you prefer, you can use plugins to effortlessly merge with new versions.

```javascript
function pluginHello(e) {
  function hello(e, name="World") {
    console.log(`Hello ${name} from ${e}`)
    return e // Make chainable.
  }
  // Add sugar
  e.hello = (name) => { return hello(e, name) }
}

surreal.plugins.push(pluginHello)
```

Now use your function like: `me().hello("Internet")`

* See the included `pluginEffects` for a more comprehensive example.
* Your functions are added globally by `globalsAdd()` If you do not want this, add it to the `restricted` list.
* Refer to an existing function to see how to make yours work with 1 or many elements.

Make an [issue](https://github.com/gnat/surreal/issues) or [pull request](https://github.com/gnat/surreal/pulls) if you think people would like to use it! If it's useful enough we'll want it in core.

