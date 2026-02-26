<!-- Generated from rustdoc HTML: index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Crate axum

## [axum][1]0.8.8

  * [All Items][2]



### Sections

  * High-level features
  * Compatibility
  * Example
  * Routing
  * Handlers
  * Extractors
  * Responses
  * Error handling
  * Middleware
  * Sharing state with handlers
    * Using the `State` extractor
    * Using request extensions
    * Using closure captures
    * Using task-local variables
  * Building integrations for axum
  * Required dependencies
  * Examples
  * Feature flags



### Crate Items

  * Re-exports
  * Modules
  * Structs
  * Traits
  * Functions
  * Type Aliases
  * Attribute Macros



# Crate axum Copy item path

[Source][3]

Expand description

axum is an HTTP routing and request-handling library that focuses on ergonomics and modularity.

## §High-level features

  * Route requests to handlers with a macro-free API.
  * Declaratively parse requests using extractors.
  * Simple and predictable error handling model.
  * Generate responses with minimal boilerplate.
  * Take full advantage of the [`tower`][4] and [`tower-http`][5] ecosystem of middleware, services, and utilities.



In particular, the last point is what sets `axum` apart from other libraries / frameworks. `axum` doesn’t have its own middleware system but instead uses [`tower::Service`][6]. This means `axum` gets timeouts, tracing, compression, authorization, and more, for free. It also enables you to share middleware with applications written using [`hyper`][7] or [`tonic`][8].

## §Compatibility

axum is designed to work with [tokio] and [hyper]. Runtime and transport layer independence is not a goal, at least for the time being.

## §Example

The “Hello, World!” of axum is:
``` 
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}
```

Note using `#[tokio::main]` requires you enable tokio’s `macros` and `rt-multi-thread` features or just `full` to enable all features (`cargo add tokio --features macros,rt-multi-thread`).

## §Routing

[`Router`][9] is used to set up which paths go to which services:
``` 
use axum::{Router, routing::get};

// our router
let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));

// which calls one of these handlers
async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}
```

See [`Router`][9] for more details on routing.

## §Handlers

In axum a “handler” is an async function that accepts zero or more [“extractors”][10] as arguments and returns something that can be converted [into a response][11].

Handlers are where your application logic lives and axum applications are built by routing between handlers.

See [`handler`][12] for more details on handlers.

## §Extractors

An extractor is a type that implements [`FromRequest`][13] or [`FromRequestParts`][14]. Extractors are how you pick apart the incoming request to get the parts your handler needs.
``` 
use axum::extract::{Path, Query, Json};
use std::collections::HashMap;

// `Path` gives you the path parameters and deserializes them.
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
async fn json(Json(payload): Json<serde_json::Value>) {}
```

See [`extract`][10] for more details on extractors.

## §Responses

Anything that implements [`IntoResponse`][15] can be returned from handlers.
``` 
use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{Value, json};

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn plain_text() -> &'static str {
    "foo"
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

let app = Router::new()
    .route("/plain_text", get(plain_text))
    .route("/json", get(json));
```

See [`response`][11] for more details on building responses.

## §Error handling

axum aims to have a simple and predictable error handling model. That means it is simple to convert errors into responses and you are guaranteed that all errors are handled.

See [`error_handling`][16] for more details on axum’s error handling model and how to handle errors gracefully.

## §Middleware

There are several different ways to write middleware for axum. See [`middleware`][17] for more details.

## §Sharing state with handlers

It is common to share some state between handlers. For example, a pool of database connections or clients to other services may need to be shared.

The four most common ways of doing that are:

  * Using the [`State`][18] extractor
  * Using request extensions
  * Using closure captures
  * Using task-local variables



### §Using the [`State`][18] extractor
``` 
use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::Arc;

struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route("/", get(handler))
    .with_state(shared_state);

async fn handler(
    State(state): State<Arc<AppState>>,
) {
    // ...
}
```

You should prefer using [`State`][18] if possible since it’s more type safe. The downside is that it’s less dynamic than task-local variables and request extensions.

See [`State`][18] for more details about accessing state.

### §Using request extensions

Another way to share state with handlers is using [`Extension`][19] as layer and extractor:
``` 
use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use std::sync::Arc;

struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route("/", get(handler))
    .layer(Extension(shared_state));

async fn handler(
    Extension(state): Extension<Arc<AppState>>,
) {
    // ...
}
```

The downside to this approach is that you’ll get runtime errors (specifically a `500 Internal Server Error` response) if you try and extract an extension that doesn’t exist, perhaps because you forgot to add the middleware or because you’re extracting the wrong type.

### §Using closure captures

State can also be passed directly to handlers using closure captures:
``` 
use axum::{
    Json,
    extract::{Extension, Path},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::Deserialize;

struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route(
        "/users",
        post({
            let shared_state = Arc::clone(&shared_state);
            move |body| create_user(body, shared_state)
        }),
    )
    .route(
        "/users/{id}",
        get({
            let shared_state = Arc::clone(&shared_state);
            move |path| get_user(path, shared_state)
        }),
    );

async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) {
    // ...
}

async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<AppState>) {
    // ...
}

#[derive(Deserialize)]
struct CreateUserPayload {
    // ...
}
```

The downside to this approach is that it’s the most verbose approach.

### §Using task-local variables

This also allows to share state with `IntoResponse` implementations:
``` 
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::task_local;

#[derive(Clone)]
struct CurrentUser {
    name: String,
}
task_local! {
    pub static USER: CurrentUser;
}

async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if let Some(current_user) = authorize_current_user(auth_header).await {
        // State is setup here in the middleware
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    Some(CurrentUser {
        name: auth_token.to_string(),
    })
}

struct UserResponse;

impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        // State is accessed here in the IntoResponse implementation
        let current_user = USER.with(|u| u.clone());
        (StatusCode::OK, current_user.name).into_response()
    }
}

async fn handler() -> UserResponse {
    UserResponse
}

let app: Router = Router::new()
    .route("/", get(handler))
    .route_layer(middleware::from_fn(auth));
```

The main downside to this approach is that it only works when the async executor being used has the concept of task-local variables. The example above uses [tokio’s `task_local` macro][20]. smol does not yet offer equivalent functionality at the time of writing (see [this GitHub issue][21]).

## §Building integrations for axum

Libraries authors that want to provide [`FromRequest`][13], [`FromRequestParts`][14], or [`IntoResponse`][15] implementations should depend on the [`axum-core`][22] crate, instead of `axum` if possible. [`axum-core`][22] contains core types and traits and is less likely to receive breaking changes.

## §Required dependencies

To use axum there are a few dependencies you have to pull in as well:
``` 
[dependencies]
axum = "<latest-version>"
tokio = { version = "<latest-version>", features = ["full"] }
tower = "<latest-version>"
```

The `"full"` feature for tokio isn’t necessary but it’s the easiest way to get started.

Tower isn’t strictly necessary either but helpful for testing. See the testing example in the repo to learn more about testing axum apps.

## §Examples

The axum repo contains [a number of examples][23] that show how to put all the pieces together.

## §Feature flags

axum uses a set of [feature flags][24] to reduce the amount of compiled and optional dependencies.

The following optional features are available:

Name| Description| Default?  
---|---|---  
`http1`| Enables hyper’s `http1` feature| ✔  
`http2`| Enables hyper’s `http2` feature|   
`json`| Enables the [`Json`][25] type and some similar convenience functionality| ✔  
`macros`| Enables optional utility macros|   
`matched-path`| Enables capturing of every request’s router path and the [`MatchedPath`][26] extractor| ✔  
`multipart`| Enables parsing `multipart/form-data` requests with [`Multipart`][27]|   
`original-uri`| Enables capturing of every request’s original URI and the [`OriginalUri`][28] extractor| ✔  
`tokio`| Enables `tokio` as a dependency and `axum::serve`, `SSE` and `extract::connect_info` types.| ✔  
`tower-log`| Enables `tower`’s `log` feature| ✔  
`tracing`| Log rejections from built-in extractors| ✔  
`ws`| Enables WebSockets support via [`extract::ws`][29]|   
`form`| Enables the `Form` extractor| ✔  
`query`| Enables the `Query` extractor| ✔  
  
## Re-exports§

`pub use http;`

## Modules§

[body][30]
    HTTP body utilities.
[error_handling][16]
    Error handling model and utilities
[extract][10]
    Types and traits for extracting data from requests.
[handler][12]
    Async functions that can be used to handle requests.
[middleware][17]
    Utilities for writing middleware
[response][11]
    Types and traits for generating responses.
[routing][31]
    Routing between [`Service`]s and handlers.
[serve][32]`tokio` and (`http1` or `http2`)
    Serve services.

## Structs§

[Error][33]
    Errors that can happen when using axum.
[Extension][19]
    Extractor and response for extensions.
[Form][34]`form`
    URL encoded extractor and response.
[Json][25]`json`
    JSON Extractor / Response.
[Router][9]
    The router type for composing handlers and services.

## Traits§

[RequestExt][35]
    Extension trait that adds additional methods to [`Request`][36].
[RequestPartsExt][37]
    Extension trait that adds additional methods to [`Parts`].
[ServiceExt][38]
    Extension trait that adds additional methods to any [`Service`].

## Functions§

[serve][39]`tokio` and (`http1` or `http2`)
    Serve the service with the supplied listener.

## Type Aliases§

[BoxError][40]
    Alias for a type-erased error type.

## Attribute Macros§

[debug_handler][41]`macros`
    Generates better error messages when applied to handler functions.
[debug_middleware][42]`macros`
    Generates better error messages when applied to middleware functions.

   [1]: ../axum/index.html
   [2]: all.html
   [3]: ../src/axum/lib.rs.html#1-489
   [4]: https://crates.io/crates/tower
   [5]: https://crates.io/crates/tower-http
   [6]: tower::Service
   [7]: http://crates.io/crates/hyper
   [8]: http://crates.io/crates/tonic
   [9]: struct.Router.html (struct axum::Router)
   [10]: extract/index.html (mod axum::extract)
   [11]: response/index.html (mod axum::response)
   [12]: handler/index.html (mod axum::handler)
   [13]: extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [14]: extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [15]: response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [16]: error_handling/index.html (mod axum::error_handling)
   [17]: middleware/index.html (mod axum::middleware)
   [18]: extract/struct.State.html (struct axum::extract::State)
   [19]: struct.Extension.html (struct axum::Extension)
   [20]: https://docs.rs/tokio/1/tokio/macro.task_local.html
   [21]: https://github.com/smol-rs/async-executor/issues/139
   [22]: http://crates.io/crates/axum-core
   [23]: https://github.com/tokio-rs/axum/tree/main/examples
   [24]: https://doc.rust-lang.org/cargo/reference/features.html#the-features-section
   [25]: struct.Json.html (struct axum::Json)
   [26]: extract/struct.MatchedPath.html (struct axum::extract::MatchedPath)
   [27]: extract/struct.Multipart.html (struct axum::extract::Multipart)
   [28]: extract/struct.OriginalUri.html (struct axum::extract::OriginalUri)
   [29]: extract/ws/index.html (mod axum::extract::ws)
   [30]: body/index.html (mod axum::body)
   [31]: routing/index.html (mod axum::routing)
   [32]: serve/index.html (mod axum::serve)
   [33]: struct.Error.html (struct axum::Error)
   [34]: struct.Form.html (struct axum::Form)
   [35]: trait.RequestExt.html (trait axum::RequestExt)
   [36]: extract/type.Request.html (type axum::extract::Request)
   [37]: trait.RequestPartsExt.html (trait axum::RequestPartsExt)
   [38]: trait.ServiceExt.html (trait axum::ServiceExt)
   [39]: fn.serve.html (fn axum::serve)
   [40]: type.BoxError.html (type axum::BoxError)
   [41]: attr.debug_handler.html (attr axum::debug_handler)
   [42]: attr.debug_middleware.html (attr axum::debug_middleware)

