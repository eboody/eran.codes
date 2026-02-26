<!-- Generated from rustdoc HTML: middleware/fn.map_response.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## map_response

## [axum][1]0.8.8

## map_response

### Sections

  * Example
  * Running extractors
  * Returning any `impl IntoResponse`



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Function map_response Copy item path

[Source][4]
``` 
pub fn map_response<F, T>(f: F) -> [MapResponseLayer][5]<F, [()][6], T>
```

Expand description

Create a middleware from an async function that transforms a response.

This differs from [`tower::util::MapResponse`] in that it allows you to easily run axum-specific extractors.

## §Example
``` 
use axum::{
    Router,
    routing::get,
    middleware::map_response,
    response::Response,
};

async fn set_header<B>(mut response: Response<B>) -> Response<B> {
    response.headers_mut().insert("x-foo", "foo".parse().unwrap());
    response
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .layer(map_response(set_header));
```

## §Running extractors

It is also possible to run extractors that implement [`FromRequestParts`][7]. These will be run before calling the handler.
``` 
use axum::{
    Router,
    routing::get,
    middleware::map_response,
    extract::Path,
    response::Response,
};
use std::collections::HashMap;

async fn log_path_params<B>(
    Path(path_params): Path<HashMap<String, String>>,
    response: Response<B>,
) -> Response<B> {
    tracing::debug!(?path_params);
    response
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .layer(map_response(log_path_params));
```

Note that to access state you must use either [`map_response_with_state`][8].

## §Returning any `impl IntoResponse`

It is also possible to return anything that implements [`IntoResponse`][9]
``` 
use axum::{
    Router,
    routing::get,
    middleware::map_response,
    response::{Response, IntoResponse},
};
use std::collections::HashMap;

async fn set_header(response: Response) -> impl IntoResponse {
    (
        [("x-foo", "foo")],
        response,
    )
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .layer(map_response(set_header));
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/map_response.rs.html#99-101
   [5]: struct.MapResponseLayer.html (struct axum::middleware::MapResponseLayer)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [7]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [8]: fn.map_response_with_state.html (fn axum::middleware::map_response_with_state)
   [9]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)

