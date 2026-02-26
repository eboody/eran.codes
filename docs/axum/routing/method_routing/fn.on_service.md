<!-- Generated from rustdoc HTML: routing/method_routing/fn.on_service.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## on_service

## [axum][1]0.8.8

## on_service

### Sections

  * Example



## [In axum::routing::method_routing][2]

[axum][3]::[routing][4]::[method_routing][2]

# Function on_service Copy item path

[Source][5]
``` 
pub fn on_service<T, S>(
    filter: [MethodFilter][6],
    svc: T,
) -> [MethodRouter][7]<S, T::Error>

where
    T: Service<[Request][8]> + [Clone][9] + [Send][10] + [Sync][11] + 'static,
    T::Response: [IntoResponse][12] + 'static,
    T::Future: [Send][10] + 'static,
    S: [Clone][9],
```

Expand description

Route requests with the given method to the service.

## §Example
``` 
use axum::{
    extract::Request,
    routing::on,
    Router,
    body::Body,
    routing::{MethodFilter, on_service},
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

// Requests to `POST /` will go to `service`.
let app = Router::new().route("/", on_service(MethodFilter::POST, service));
```

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/routing/method_routing.rs.html#368-376
   [6]: ../struct.MethodFilter.html (struct axum::routing::MethodFilter)
   [7]: struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [8]: ../../extract/type.Request.html (type axum::extract::Request)
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [10]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [12]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)

