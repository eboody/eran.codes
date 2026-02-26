## 👁️ How does it look?

Do surreal things with [Locality of Behavior](https://htmx.org/essays/locality-of-behaviour/) like:
```html
<label for="file-input" >
  <div class="uploader"></div>
  <script>
    me().on("dragover", ev => { halt(ev); me(ev).classAdd('.hover'); console.log("Files in drop zone.") })
    me().on("dragleave", ev => { halt(ev); me(ev).classRemove('.hover'); console.log("Files left drop zone.") })
    me().on("drop", ev => { halt(ev); me(ev).classRemove('.hover').classAdd('.loading'); me('#file-input').attribute('files', ev.dataTransfer.files); me('#form').send('change') })
  </script>
</label>
```

See the [Live Example](https://gnat.github.io/surreal/example.html)! Then [view source](https://github.com/gnat/surreal/blob/main/example.html).

