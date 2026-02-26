<!-- Generated from rustdoc HTML: extract/ws/trait.OnFailedUpgrade.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## OnFailedUpgrade

## [axum][1]0.8.8

## OnFailedUpgrade

### Required Methods

  * call



### Implementors

## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Trait OnFailedUpgrade Copy item path

[Source][5]
``` 
pub trait OnFailedUpgrade: [Send][6] + 'static {
    // Required method
    fn call(self, error: [Error][7]);
}
```

Available on **crate feature`ws`** only.

Expand description

What to do when a connection upgrade fails.

See [`WebSocketUpgrade::on_failed_upgrade`][8] for more details.

## Required Methods§

[Source][9]

#### fn call(self, error: [Error][7])

Call the callback.

## Implementors§

[Source][10]§

### impl [OnFailedUpgrade][11] for [DefaultOnFailedUpgrade][12]

[Source][13]§

### impl<F> [OnFailedUpgrade][11] for F

where F: [FnOnce][14]([Error][7]) + [Send][6] \+ 'static,

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/ws.rs.html#416-419
   [6]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [7]: ../../struct.Error.html (struct axum::Error)
   [8]: ../struct.WebSocketUpgrade.html#method.on_failed_upgrade (method axum::extract::WebSocketUpgrade::on_failed_upgrade)
   [9]: ../../../src/axum/extract/ws.rs.html#418
   [10]: ../../../src/axum/extract/ws.rs.html#437-440
   [11]: trait.OnFailedUpgrade.html (trait axum::extract::ws::OnFailedUpgrade)
   [12]: struct.DefaultOnFailedUpgrade.html (struct axum::extract::ws::DefaultOnFailedUpgrade)
   [13]: ../../../src/axum/extract/ws.rs.html#421-428
   [14]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)

