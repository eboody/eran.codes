<!-- Generated from rustdoc HTML: middleware/fn.from_fn_with_state.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## from_fn_with_state

## [axum][1]0.8.8

## from_fn_with_state

### Sections

  * Example



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Function from_fn_with_state Copy item path

[Source][4]
``` 
pub fn from_fn_with_state<F, S, T>(state: S, f: F) -> [FromFnLayer][5]<F, S, T>
```

Expand description

Create a middleware from an async function with the given state.

For the requirements for the function supplied see [`from_fn`][6].

See [`State`][7] for more details about accessing state.

## §Example
``` 
use axum::{
    Router,
    http::StatusCode,
    routing::get,
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::{Request, State},
};

#[derive(Clone)]
struct AppState { /* ... */ }

async fn my_middleware(
    State(state): State<AppState>,
    // you can add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Response {
    // do something with `request`...

    let response = next.run(request).await;

    // do something with `response`...

    response
}

let state = AppState { /* ... */ };

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .route_layer(middleware::from_fn_with_state(state.clone(), my_middleware))
    .with_state(state);
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/from_fn.rs.html#164-170
   [5]: struct.FromFnLayer.html (struct axum::middleware::FromFnLayer)
   [6]: fn.from_fn.html (fn axum::middleware::from_fn)
   [7]: ../extract/struct.State.html (struct axum::extract::State)

