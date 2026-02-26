<!-- Generated from rustdoc HTML: routing/method_routing/fn.trace.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## trace

## [axum][1]0.8.8

## [In axum::routing::method_routing][2]

[axum][3]::[routing][4]::[method_routing][2]

# Function trace Copy item path

[Source][5]
``` 
pub fn trace<H, T, S>(handler: H) -> [MethodRouter][6]<S, [Infallible][7]>

where
    H: [Handler][8]<T, S>,
    T: 'static,
    S: [Clone][9] + [Send][10] + [Sync][11] + 'static,
```

Expand description

Route `TRACE` requests to the given handler.

See [`get`][12] for an example.

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/routing/method_routing.rs.html#447
   [6]: struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [7]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [8]: ../../handler/trait.Handler.html (trait axum::handler::Handler)
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [10]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [12]: fn.get.html (fn axum::routing::method_routing::get)

