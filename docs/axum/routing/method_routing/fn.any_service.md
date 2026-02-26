<!-- Generated from rustdoc HTML: routing/method_routing/fn.any_service.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## any_service

## [axum][1]0.8.8

## any_service

### Sections

  * Example



## [In axum::routing::method_routing][2]

[axum][3]::[routing][4]::[method_routing][2]

# Function any_service Copy item path

[Source][5]
``` 
pub fn any_service<T, S>(svc: T) -> [MethodRouter][6]<S, T::Error>

where
    T: Service<[Request][7]> + [Clone][8] + [Send][9] + [Sync][10] + 'static,
    T::Response: [IntoResponse][11] + 'static,
    T::Future: [Send][9] + 'static,
    S: [Clone][8],
```

Expand description

Route requests to the given service regardless of its method.

## §Example
``` 
use axum::{
    extract::Request,
    Router,
    routing::any_service,
    body::Body,
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

// All requests to `/` will go to `service`.
let app = Router::new().route("/", any_service(service));
```

Additional methods can still be chained:
``` 
use axum::{
    extract::Request,
    Router,
    routing::any_service,
    body::Body,
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    // ...
});

let other_service = tower::service_fn(|request: Request| async {
    // ...
});

// `POST /` goes to `other_service`. All other requests go to `service`
let app = Router::new().route("/", any_service(service).post_service(other_service));
```

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/routing/method_routing.rs.html#427-437
   [6]: struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [7]: ../../extract/type.Request.html (type axum::extract::Request)
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [9]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [10]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [11]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)

