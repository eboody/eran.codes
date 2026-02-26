<!-- Generated from rustdoc HTML: middleware/fn.from_fn.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## from_fn

## [axum][1]0.8.8

## from_fn

### Sections

  * Example
  * Running extractors



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Function from_fn Copy item path

[Source][4]
``` 
pub fn from_fn<F, T>(f: F) -> [FromFnLayer][5]<F, [()][6], T>
```

Expand description

Create a middleware from an async function.

`from_fn` requires the function given to

  1. Be an `async fn`.
  2. Take zero or more [`FromRequestParts`][7] extractors.
  3. Take exactly one [`FromRequest`][8] extractor as the second to last argument.
  4. Take [`Next`][9] as the last argument.
  5. Return something that implements [`IntoResponse`][10].



Note that this function doesn’t support extracting [`State`][11]. For that, use [`from_fn_with_state`][12].

## §Example
``` 
use axum::{
    Router,
    http,
    routing::get,
    response::Response,
    middleware::{self, Next},
    extract::Request,
};

async fn my_middleware(
    request: Request,
    next: Next,
) -> Response {
    // do something with `request`...

    let response = next.run(request).await;

    // do something with `response`...

    response
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .layer(middleware::from_fn(my_middleware));
```

## §Running extractors
``` 
use axum::{
    Router,
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::{self, Next},
    response::Response,
    routing::get,
};

async fn auth(
    // run the `HeaderMap` extractor
    headers: HeaderMap,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Some(token) if token_is_valid(token) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    // ...
}

fn token_is_valid(token: &str) -> bool {
    // ...
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .route_layer(middleware::from_fn(auth));
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/from_fn.rs.html#114-116
   [5]: struct.FromFnLayer.html (struct axum::middleware::FromFnLayer)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [7]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [8]: ../extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [9]: struct.Next.html (struct axum::middleware::Next)
   [10]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [11]: ../extract/struct.State.html (struct axum::extract::State)
   [12]: fn.from_fn_with_state.html (fn axum::middleware::from_fn_with_state)

