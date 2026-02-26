<!-- Generated from rustdoc HTML: routing/method_routing/fn.patch_service.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## patch_service

## [axum][1]0.8.8

## [In axum::routing::method_routing][2]

[axum][3]::[routing][4]::[method_routing][2]

# Function patch_service Copy item path

[Source][5]
``` 
pub fn patch_service<T, S>(svc: T) -> [MethodRouter][6]<S, T::Error>

where
    T: Service<[Request][7]> + [Clone][8] + [Send][9] + [Sync][10] + 'static,
    T::Response: [IntoResponse][11] + 'static,
    T::Future: [Send][9] + 'static,
    S: [Clone][8],
```

Expand description

Route `PATCH` requests to the given service.

See [`get_service`][12] for an example.

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/routing/method_routing.rs.html#340
   [6]: struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [7]: ../../extract/type.Request.html (type axum::extract::Request)
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [9]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [10]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [11]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [12]: fn.get_service.html (fn axum::routing::method_routing::get_service)

