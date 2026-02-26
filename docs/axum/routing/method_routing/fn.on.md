<!-- Generated from rustdoc HTML: routing/method_routing/fn.on.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## on

## [axum][1]0.8.8

## on

### Sections

  * Example



## [In axum::routing::method_routing][2]

[axum][3]::[routing][4]::[method_routing][2]

# Function on Copy item path

[Source][5]
``` 
pub fn on<H, T, S>(
    filter: [MethodFilter][6],
    handler: H,
) -> [MethodRouter][7]<S, [Infallible][8]>

where
    H: [Handler][9]<T, S>,
    T: 'static,
    S: [Clone][10] + [Send][11] + [Sync][12] + 'static,
```

Expand description

Route requests with the given method to the handler.

## §Example
``` 
use axum::{
    routing::on,
    Router,
    routing::MethodFilter,
};

async fn handler() {}

// Requests to `POST /` will go to `handler`.
let app = Router::new().route("/", on(MethodFilter::POST, handler));
```

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/routing/method_routing.rs.html#466-473
   [6]: ../struct.MethodFilter.html (struct axum::routing::MethodFilter)
   [7]: struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [8]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [9]: ../../handler/trait.Handler.html (trait axum::handler::Handler)
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [12]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)

