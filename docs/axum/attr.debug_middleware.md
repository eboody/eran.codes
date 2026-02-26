<!-- Generated from rustdoc HTML: attr.debug_middleware.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## debug_middleware

## [axum][1]0.8.8

## debug_middleware

### Sections

  * Example
  * Performance



## [In crate axum][2]

[axum][2]

# Attribute Macro debug_middleware Copy item path
```
#[debug_middleware]
```

Available on **crate feature`macros`** only.

Expand description

Generates better error messages when applied to middleware functions.

This works similarly to [`#[debug_handler]`][3] except for middleware using [`axum::middleware::from_fn`][4].

## §Example
``` 
use axum::{
    routing::get,
    extract::Request,
    response::Response,
    Router,
    middleware::{self, Next},
    debug_middleware,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {}))
        .layer(middleware::from_fn(my_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}

// if this wasn't a valid middleware function #[debug_middleware] would
// improve compile error
#[debug_middleware]
async fn my_middleware(
    request: Request,
    next: Next,
) -> Response {
    next.run(request).await
}
```

## §Performance

This macro has no effect when compiled with the release profile. (eg. `cargo build --release`)

   [1]: ../axum/index.html
   [2]: index.html
   [3]: attr.debug_handler.html (attr axum::debug_handler)
   [4]: https://docs.rs/axum/0.8/axum/middleware/fn.from_fn.html

