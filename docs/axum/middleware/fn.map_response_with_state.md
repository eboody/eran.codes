<!-- Generated from rustdoc HTML: middleware/fn.map_response_with_state.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## map_response_with_state

## [axum][1]0.8.8

## map_response_with_state

### Sections

  * Example



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Function map_response_with_state Copy item path

[Source][4]
``` 
pub fn map_response_with_state<F, S, T>(
    state: S,
    f: F,
) -> [MapResponseLayer][5]<F, S, T>
```

Expand description

Create a middleware from an async function that transforms a response, with the given state.

See [`State`][6] for more details about accessing state.

## §Example
``` 
use axum::{
    Router,
    http::StatusCode,
    routing::get,
    response::Response,
    middleware::map_response_with_state,
    extract::State,
};

#[derive(Clone)]
struct AppState { /* ... */ }

async fn my_middleware<B>(
    State(state): State<AppState>,
    // you can add more extractors here but they must
    // all implement `FromRequestParts`
    // `FromRequest` is not allowed
    response: Response<B>,
) -> Response<B> {
    // do something with `state` and `response`...
    response
}

let state = AppState { /* ... */ };

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .route_layer(map_response_with_state(state.clone(), my_middleware))
    .with_state(state);
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/map_response.rs.html#141-147
   [5]: struct.MapResponseLayer.html (struct axum::middleware::MapResponseLayer)
   [6]: ../extract/struct.State.html (struct axum::extract::State)

