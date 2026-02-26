<!-- Generated from rustdoc HTML: extract/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module extract

## [axum][1]0.8.8

## Module extract

### Sections

  * Intro
  * Common extractors
  * Applying multiple extractors
  * The order of extractors
  * Handling extractor rejections
  * Optional extractors
  * Customizing extractor responses
  * Accessing inner errors
  * Defining custom extractors
    * Implementing `FromRequestParts`
    * Implementing `FromRequest`
    * Cannot implement both `FromRequest` and `FromRequestParts`
  * Accessing other extractors in `FromRequest` or `FromRequestParts` implementations
  * Request body limits
  * Wrapping extractors
  * Logging rejections



### Module Items

  * Re-exports
  * Modules
  * Structs
  * Traits
  * Type Aliases
  * Derive Macros



## [In crate axum][2]

[axum][2]

# Module extract Copy item path

[Source][3]

Expand description

Types and traits for extracting data from requests.

## §Intro

A handler function is an async function that takes any number of “extractors” as arguments. An extractor is a type that implements [`FromRequest`][4] or [`FromRequestParts`][5].

For example, [`Json`][6] is an extractor that consumes the request body and deserializes it as JSON into some target type:
``` 
use axum::{
    extract::Json,
    routing::post,
    handler::Handler,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
    // ...
}

let app = Router::new().route("/users", post(create_user));
```

## §Common extractors

Some commonly used extractors are:
``` 
use axum::{
    extract::{Request, Json, Path, Extension, Query},
    routing::post,
    http::header::HeaderMap,
    body::{Bytes, Body},
    Router,
};
use serde_json::Value;
use std::collections::HashMap;

// `Path` gives you the path parameters and deserializes them. See its docs for
// more details
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// `HeaderMap` gives you all the headers
async fn headers(headers: HeaderMap) {}

// `String` consumes the request body and ensures it is valid utf-8
async fn string(body: String) {}

// `Bytes` gives you the raw request body
async fn bytes(body: Bytes) {}

// We've already seen `Json` for parsing the request body as json
async fn json(Json(payload): Json<Value>) {}

// `Request` gives you the whole request for maximum control
async fn request(request: Request) {}

// `Extension` extracts data from "request extensions"
// This is commonly used to share state with handlers
async fn extension(Extension(state): Extension<State>) {}

#[derive(Clone)]
struct State { /* ... */ }

let app = Router::new()
    .route("/path/{user_id}", post(path))
    .route("/query", post(query))
    .route("/string", post(string))
    .route("/bytes", post(bytes))
    .route("/json", post(json))
    .route("/request", post(request))
    .route("/extension", post(extension));
```

## §Applying multiple extractors

You can also apply multiple extractors:
``` 
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use uuid::Uuid;
use serde::Deserialize;

let app = Router::new().route("/users/{id}/things", get(get_user_things));

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

async fn get_user_things(
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) {
    // ...
}
```

## §The order of extractors

Extractors always run in the order of the function parameters that is from left to right.

The request body is an asynchronous stream that can only be consumed once. Therefore you can only have one extractor that consumes the request body. axum enforces this by requiring such extractors to be the _last_ argument your handler takes.

For example
``` 
use axum::{extract::State, http::{Method, HeaderMap}};

async fn handler(
    // `Method` and `HeaderMap` don't consume the request body so they can
    // put anywhere in the argument list (but before `body`)
    method: Method,
    headers: HeaderMap,
    // `State` is also an extractor so it needs to be before `body`
    State(state): State<AppState>,
    // `String` consumes the request body and thus must be the last extractor
    body: String,
) {
    // ...
}
```

We get a compile error if `String` isn’t the last extractor:

ⓘ
```
use axum::http::Method;

async fn handler(
    // this doesn't work since `String` must be the last argument
    body: String,
    method: Method,
) {
    // ...
}
```

This also means you cannot consume the request body twice:

ⓘ
```
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct Payload {}

async fn handler(
    // `String` and `Json` both consume the request body
    // so they cannot both be used
    string_body: String,
    json_body: Json<Payload>,
) {
    // ...
}
```

axum enforces this by requiring the last extractor implements [`FromRequest`][4] and all others implement [`FromRequestParts`][5].

## §Handling extractor rejections

If you want to handle the case of an extractor failing within a specific handler, you can wrap it in `Result`, with the error being the rejection type of the extractor:
``` 
use axum::{
    extract::{Json, rejection::JsonRejection},
    routing::post,
    Router,
};
use serde_json::Value;

async fn create_user(payload: Result<Json<Value>, JsonRejection>) {
    match payload {
        Ok(payload) => {
            // We got a valid JSON payload
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            // Request didn't have `Content-Type: application/json`
            // header
        }
        Err(JsonRejection::JsonDataError(_)) => {
            // Couldn't deserialize the body into the target type
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            // Syntax error in the body
        }
        Err(JsonRejection::BytesRejection(_)) => {
            // Failed to extract the request body
        }
        Err(_) => {
            // `JsonRejection` is marked `#[non_exhaustive]` so match must
            // include a catch-all case.
        }
    }
}

let app = Router::new().route("/users", post(create_user));
```

## §Optional extractors

Some extractors implement [`OptionalFromRequestParts`][7] in addition to [`FromRequestParts`][5], or [`OptionalFromRequest`][8] in addition to [`FromRequest`][4].

These extractors can be used inside of `Option`. It depends on the particular `OptionalFromRequestParts` or `OptionalFromRequest` implementation what this does: For example for `TypedHeader` from axum-extra, you get `None` if the header you’re trying to extract is not part of the request, but if the header is present and fails to parse, the request is rejected.
``` 
use axum::{routing::post, Router};
use axum_extra::{headers::UserAgent, TypedHeader};
use serde_json::Value;

async fn foo(user_agent: Option<TypedHeader<UserAgent>>) {
    if let Some(TypedHeader(user_agent)) = user_agent {
        // The client sent a user agent
    } else {
        // No user agent header
    }
}

let app = Router::new().route("/foo", post(foo));
```

## §Customizing extractor responses

If an extractor fails it will return a response with the error and your handler will not be called. To customize the error response you have two options:

  1. Use `Result<T, T::Rejection>` as your extractor like shown in “Handling extractor rejections”. This works well if you’re only using the extractor in a single handler.
  2. Create your own extractor that in its [`FromRequest`][4] implementation calls one of axum’s built in extractors but returns a different response for rejections. See the [customize-extractor-error][9] example for more details.



## §Accessing inner errors

axum’s built-in extractors don’t directly expose the inner error. This gives us more flexibility and allows us to change internal implementations without breaking the public API.

For example that means while [`Json`][6] is implemented using [`serde_json`][10] it doesn’t directly expose the [`serde_json::Error`][11] that’s contained in [`JsonRejection::JsonDataError`][12]. However it is still possible to access via methods from [`std::error::Error`][13]:
``` 
use std::error::Error;
use axum::{
    extract::{Json, rejection::JsonRejection},
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::{json, Value};

async fn handler(
    result: Result<Json<Value>, JsonRejection>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match result {
        // if the client sent valid JSON then we're good
        Ok(Json(payload)) => Ok(Json(json!({ "payload": payload }))),

        Err(err) => match err {
            JsonRejection::JsonDataError(err) => {
                Err(serde_json_error_response(err))
            }
            JsonRejection::JsonSyntaxError(err) => {
                Err(serde_json_error_response(err))
            }
            // handle other rejections from the `Json` extractor
            JsonRejection::MissingJsonContentType(_) => Err((
                StatusCode::BAD_REQUEST,
                "Missing `Content-Type: application/json` header".to_string(),
            )),
            JsonRejection::BytesRejection(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to buffer request body".to_string(),
            )),
            // we must provide a catch-all case since `JsonRejection` is marked
            // `#[non_exhaustive]`
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            )),
        },
    }
}

// attempt to extract the inner `serde_path_to_error::Error<serde_json::Error>`,
// if that succeeds we can provide a more specific error.
//
// `Json` uses `serde_path_to_error` so the error will be wrapped in `serde_path_to_error::Error`.
fn serde_json_error_response<E>(err: E) -> (StatusCode, String)
where
    E: Error + 'static,
{
    if let Some(err) = find_error_source::<serde_path_to_error::Error<serde_json::Error>>(&err) {
        let serde_json_err = err.inner();
        (
            StatusCode::BAD_REQUEST,
            format!(
                "Invalid JSON at line {} column {}",
                serde_json_err.line(),
                serde_json_err.column()
            ),
        )
    } else {
        (StatusCode::BAD_REQUEST, "Unknown error".to_string())
    }
}

// attempt to downcast `err` into a `T` and if that fails recursively try and
// downcast `err`'s source
fn find_error_source<'a, T>(err: &'a (dyn Error + 'static)) -> Option<&'a T>
where
    T: Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}
```

Note that while this approach works it might break in the future if axum changes its implementation to use a different error type internally. Such changes might happen without major breaking versions.

## §Defining custom extractors

You can also define your own extractors by implementing either [`FromRequestParts`][5] or [`FromRequest`][4].

### §Implementing `FromRequestParts`

Implement `FromRequestParts` if your extractor doesn’t need access to the request body:
``` 
use axum::{
    extract::FromRequestParts,
    routing::get,
    Router,
    http::{
        StatusCode,
        header::{HeaderValue, USER_AGENT},
        request::Parts,
    },
};

struct ExtractUserAgent(HeaderValue);

impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(ExtractUserAgent(user_agent.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}

async fn handler(ExtractUserAgent(user_agent): ExtractUserAgent) {
    // ...
}

let app = Router::new().route("/foo", get(handler));
```

### §Implementing `FromRequest`

If your extractor needs to consume the request body you must implement [`FromRequest`][4]
``` 
use axum::{
    extract::{Request, FromRequest},
    response::{Response, IntoResponse},
    body::{Bytes, Body},
    routing::get,
    Router,
    http::{
        StatusCode,
        header::{HeaderValue, USER_AGENT},
    },
};

struct ValidatedBody(Bytes);

impl<S> FromRequest<S> for ValidatedBody
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        // do validation...

        Ok(Self(body))
    }
}

async fn handler(ValidatedBody(body): ValidatedBody) {
    // ...
}

let app = Router::new().route("/foo", get(handler));
```

### §Cannot implement both `FromRequest` and `FromRequestParts`

Note that you will make your extractor unusable by implementing both `FromRequest` and `FromRequestParts` directly for the same type, unless it is wrapping another extractor:

ⓘ
```
use axum::{
    Router,
    routing::get,
    extract::{FromRequest, Request, FromRequestParts},
    http::request::Parts,
    body::Body,
};
use std::convert::Infallible;

// Some extractor that doesn't wrap another extractor
struct MyExtractor;

// `MyExtractor` implements both `FromRequest`
impl<S> FromRequest<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // ...
    }
}

// and `FromRequestParts`
impl<S> FromRequestParts<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // ...
    }
}

let app = Router::new().route(
    "/",
    // This fails when we go to actually use `MyExtractor` in a handler function.
    // This is due to a limit in Rust's type system.
    //
    // The workaround is to implement either `FromRequest` or `FromRequestParts`
    // but not both, if your extractor doesn't wrap another extractor.
    //
    // See "Wrapping extractors" for how to wrap other extractors.
    get(|_: MyExtractor| async {}),
);
```

## §Accessing other extractors in `FromRequest` or `FromRequestParts` implementations

When defining custom extractors you often need to access another extractor in your implementation.
``` 
use axum::{
    extract::{Extension, FromRequestParts},
    http::{StatusCode, HeaderMap, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[derive(Clone)]
struct State {
    // ...
}

struct AuthenticatedUser {
    // ...
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // You can either call them directly...
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|err| match err {})?;

        // ... or use `extract` / `extract_with_state` from `RequestExt` / `RequestPartsExt`
        use axum::RequestPartsExt;
        let Extension(state) = parts.extract::<Extension<State>>()
            .await
            .map_err(|err| err.into_response())?;

        unimplemented!("actually perform the authorization")
    }
}

async fn handler(user: AuthenticatedUser) {
    // ...
}

let state = State { /* ... */ };

let app = Router::new().route("/", get(handler)).layer(Extension(state));
```

## §Request body limits

For security reasons, [`Bytes`][14] will, by default, not accept bodies larger than 2MB. This also applies to extractors that uses [`Bytes`][14] internally such as `String`, [`Json`][6], and [`Form`][15].

For more details, including how to disable this limit, see [`DefaultBodyLimit`][16].

## §Wrapping extractors

If you want to write an extractor that generically wraps another extractor (that may or may not consume the request body) you should implement both [`FromRequest`][4] and [`FromRequestParts`][5]:
``` 
use axum::{
    Router,
    body::Body,
    routing::get,
    extract::{Request, FromRequest, FromRequestParts},
    http::{HeaderMap, request::Parts},
};
use std::time::{Instant, Duration};

// an extractor that wraps another and measures how long time it takes to run
struct Timing<E> {
    extractor: E,
    duration: Duration,
}

// we must implement both `FromRequestParts`
impl<S, T> FromRequestParts<S> for Timing<T>
where
    S: Send + Sync,
    T: FromRequestParts<S>,
{
    type Rejection = T::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let start = Instant::now();
        let extractor = T::from_request_parts(parts, state).await?;
        let duration = start.elapsed();
        Ok(Timing {
            extractor,
            duration,
        })
    }
}

// and `FromRequest`
impl<S, T> FromRequest<S> for Timing<T>
where
    S: Send + Sync,
    T: FromRequest<S>,
{
    type Rejection = T::Rejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let start = Instant::now();
        let extractor = T::from_request(req, state).await?;
        let duration = start.elapsed();
        Ok(Timing {
            extractor,
            duration,
        })
    }
}

async fn handler(
    // this uses the `FromRequestParts` impl
    _: Timing<HeaderMap>,
    // this uses the `FromRequest` impl
    _: Timing<String>,
) {}
```

## §Logging rejections

All built-in extractors will log rejections for easier debugging. To see the logs, enable the `tracing` feature for axum (enabled by default) and the `axum::rejection=trace` tracing target, for example with `RUST_LOG=info,axum::rejection=trace cargo run`.

## Re-exports§

`pub use crate::[Json][6];``json`
`pub use crate::[Extension][17];`
`pub use crate::form::[Form][15];``form`

## Modules§

[connect_info][18]`tokio`
    Extractor for getting connection information from a client.
[multipart][19]`multipart`
    Extractor that parses `multipart/form-data` requests commonly used with file uploads.
[path][20]
    Extractor that will get captures from the URL and parse them using [`serde`][21].
[rejection][22]
    Rejection response types.
[ws][23]`ws`
    Handle WebSocket connections.

## Structs§

[ConnectInfo][24]`tokio`
    Extractor for getting connection information produced by a [`Connected`][25].
[DefaultBodyLimit][16]
    Layer for configuring the default request body limit.
[MatchedPath][26]`matched-path`
    Access the path in the router that matches the request.
[Multipart][27]`multipart`
    Extractor that parses `multipart/form-data` requests (commonly used with file uploads).
[NestedPath][28]
    Access the path the matched the route is nested at.
[OriginalUri][29]`original-uri`
    Extractor that gets the original request URI regardless of nesting.
[Path][30]
    Extractor that will get captures from the URL and parse them using [`serde`][31].
[Query][32]`query`
    Extractor that deserializes query strings into some type.
[RawForm][33]
    Extractor that extracts raw form requests.
[RawPathParams][34]
    Extractor that will get captures from the URL without deserializing them.
[RawQuery][35]
    Extractor that extracts the raw query string, without parsing it.
[State][36]
    Extractor for state.
[WebSocketUpgrade][37]`ws`
    Extractor for establishing WebSocket connections.

## Traits§

[FromRef][38]
    Used to do reference-to-value conversions thus not consuming the input value.
[FromRequest][4]
    Types that can be created from requests.
[FromRequestParts][5]
    Types that can be created from request parts.
[OptionalFromRequest][8]
    Customize the behavior of `Option<Self>` as a [`FromRequest`][4] extractor.
[OptionalFromRequestParts][7]
    Customize the behavior of `Option<Self>` as a [`FromRequestParts`][5] extractor.

## Type Aliases§

[Request][39]
    Type alias for [`http::Request`] whose body type defaults to [`Body`][40], the most common body type used with axum.

## Derive Macros§

[FromRef][41]`macros`
    Derive an implementation of [`FromRef`][42] for each field in a struct.
[FromRequest][43]`macros`
    Derive an implementation of [`FromRequest`][44].
[FromRequestParts][45]`macros`
    Derive an implementation of [`FromRequestParts`][46].

   [1]: ../../axum/index.html
   [2]: ../index.html
   [3]: ../../src/axum/extract/mod.rs.html#1-109
   [4]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [5]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [6]: ../struct.Json.html (struct axum::Json)
   [7]: trait.OptionalFromRequestParts.html (trait axum::extract::OptionalFromRequestParts)
   [8]: trait.OptionalFromRequest.html (trait axum::extract::OptionalFromRequest)
   [9]: https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/main.rs
   [10]: https://docs.rs/serde_json/1.0.149/serde_json/index.html (mod serde_json)
   [11]: https://docs.rs/serde_json/1.0.149/serde_json/error/struct.Error.html (struct serde_json::error::Error)
   [12]: rejection/enum.JsonRejection.html#variant.JsonDataError (variant axum::extract::rejection::JsonRejection::JsonDataError)
   [13]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [14]: crate::body::Bytes
   [15]: ../struct.Form.html (struct axum::Form)
   [16]: struct.DefaultBodyLimit.html (struct axum::extract::DefaultBodyLimit)
   [17]: ../struct.Extension.html (struct axum::Extension)
   [18]: connect_info/index.html (mod axum::extract::connect_info)
   [19]: multipart/index.html (mod axum::extract::multipart)
   [20]: path/index.html (mod axum::extract::path)
   [21]: https://docs.rs/serde/1.0.228/serde/index.html (mod serde)
   [22]: rejection/index.html (mod axum::extract::rejection)
   [23]: ws/index.html (mod axum::extract::ws)
   [24]: struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [25]: connect_info/trait.Connected.html (trait axum::extract::connect_info::Connected)
   [26]: struct.MatchedPath.html (struct axum::extract::MatchedPath)
   [27]: struct.Multipart.html (struct axum::extract::Multipart)
   [28]: struct.NestedPath.html (struct axum::extract::NestedPath)
   [29]: struct.OriginalUri.html (struct axum::extract::OriginalUri)
   [30]: struct.Path.html (struct axum::extract::Path)
   [31]: https://crates.io/crates/serde
   [32]: struct.Query.html (struct axum::extract::Query)
   [33]: struct.RawForm.html (struct axum::extract::RawForm)
   [34]: struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [35]: struct.RawQuery.html (struct axum::extract::RawQuery)
   [36]: struct.State.html (struct axum::extract::State)
   [37]: struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [38]: trait.FromRef.html (trait axum::extract::FromRef)
   [39]: type.Request.html (type axum::extract::Request)
   [40]: ../body/struct.Body.html (struct axum::body::Body)
   [41]: derive.FromRef.html (derive axum::extract::FromRef)
   [42]: https://docs.rs/axum/0.8/axum/extract/trait.FromRef.html
   [43]: derive.FromRequest.html (derive axum::extract::FromRequest)
   [44]: https://docs.rs/axum/0.8/axum/extract/trait.FromRequest.html
   [45]: derive.FromRequestParts.html (derive axum::extract::FromRequestParts)
   [46]: https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html

