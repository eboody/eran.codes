<!-- Generated from rustdoc HTML: middleware/fn.from_extractor.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## from_extractor

## [axum][1]0.8.8

## from_extractor

### Sections

  * Example



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Function from_extractor Copy item path

[Source][4]
``` 
pub fn from_extractor<E>() -> [FromExtractorLayer][5]<E, [()][6]>
```

Expand description

Create a middleware from an extractor.

If the extractor succeeds the value will be discarded and the inner service will be called. If the extractor fails the rejection will be returned and the inner service will _not_ be called.

This can be used to perform validation of requests if the validation doesn’t produce any useful output, and run the extractor for several handlers without repeating it in the function signature.

Note that if the extractor consumes the request body, as `String` or [`Bytes`][7] does, an empty body will be left in its place. Thus won’t be accessible to subsequent extractors or handlers.

## §Example
``` 
use axum::{
    extract::FromRequestParts,
    middleware::from_extractor,
    routing::{get, post},
    Router,
    http::{header, StatusCode, request::Parts},
};

// An extractor that performs authorization.
struct RequireAuth;

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) if token_is_valid(auth_header) => {
                Ok(Self)
            }
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

fn token_is_valid(token: &str) -> bool {
    // ...
}

async fn handler() {
    // If we get here the request has been authorized
}

async fn other_handler() {
    // If we get here the request has been authorized
}

let app = Router::new()
    .route("/", get(handler))
    .route("/foo", post(other_handler))
    // The extractor will run before all routes
    .route_layer(from_extractor::<RequireAuth>());
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/from_extractor.rs.html#89-91
   [5]: struct.FromExtractorLayer.html (struct axum::middleware::FromExtractorLayer)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [7]: bytes::Bytes

