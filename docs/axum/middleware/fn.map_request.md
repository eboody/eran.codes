<!-- Generated from rustdoc HTML: middleware/fn.map_request.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## map_request

## [axum][1]0.8.8

## map_request

### Sections

  * Example
  * Rejecting the request
  * Running extractors



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Function map_request Copy item path

[Source][4]
``` 
pub fn map_request<F, T>(f: F) -> [MapRequestLayer][5]<F, [()][6], T>
```

Expand description

Create a middleware from an async function that transforms a request.

This differs from [`tower::util::MapRequest`] in that it allows you to easily run axum-specific extractors.

## §Example
``` 
use axum::{
    Router,
    routing::get,
    middleware::map_request,
    http::Request,
};

async fn set_header<B>(mut request: Request<B>) -> Request<B> {
    request.headers_mut().insert("x-foo", "foo".parse().unwrap());
    request
}

async fn handler<B>(request: Request<B>) {
    // `request` will have an `x-foo` header
}

let app = Router::new()
    .route("/", get(handler))
    .layer(map_request(set_header));
```

## §Rejecting the request

The function given to `map_request` is allowed to also return a `Result` which can be used to reject the request and return a response immediately, without calling the remaining middleware.

Specifically the valid return types are:

  * `Request<B>`
  * `Result<Request<B>, E> where E: IntoResponse`


``` 
use axum::{
    Router,
    http::{Request, StatusCode},
    routing::get,
    middleware::map_request,
};

async fn auth<B>(request: Request<B>) -> Result<Request<B>, StatusCode> {
    let auth_header = request.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) if token_is_valid(auth_header) => Ok(request),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn token_is_valid(token: &str) -> bool {
    // ...
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .route_layer(map_request(auth));
```

## §Running extractors
``` 
use axum::{
    Router,
    routing::get,
    middleware::map_request,
    extract::Path,
    http::Request,
};
use std::collections::HashMap;

async fn log_path_params<B>(
    Path(path_params): Path<HashMap<String, String>>,
    request: Request<B>,
) -> Request<B> {
    tracing::debug!(?path_params);
    request
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .layer(map_request(log_path_params));
```

Note that to access state you must use either [`map_request_with_state`][7].

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/map_request.rs.html#117-119
   [5]: struct.MapRequestLayer.html (struct axum::middleware::MapRequestLayer)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [7]: fn.map_request_with_state.html (fn axum::middleware::map_request_with_state)

