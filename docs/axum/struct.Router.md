<!-- Generated from rustdoc HTML: struct.Router.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Router

## [axum][1]0.8.8

## Router

### Methods

  * as_service
  * fallback
  * fallback_service
  * has_routes
  * into_make_service
  * into_make_service_with_connect_info
  * into_service
  * layer
  * merge
  * method_not_allowed_fallback
  * nest
  * nest_service
  * new
  * reset_fallback
  * route
  * route_layer
  * route_service
  * with_state
  * without_v07_checks



### Trait Implementations

  * Clone
  * Debug
  * Default
  * Service<IncomingStream<'_, L>>
  * Service<Request<B>>



### Auto Trait Implementations

  * !RefUnwindSafe
  * !UnwindSafe
  * Freeze
  * Send
  * Sync
  * Unpin



### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * CloneToUninit
  * From<T>
  * FromRef<T>
  * Instrument
  * Into<U>
  * MakeService<Target, Request>
  * PolicyExt
  * Same
  * ServiceExt
  * ServiceExt<R>
  * ServiceExt<Request>
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In crate axum][2]

[axum][2]

# Struct Router Copy item path

[Source][3]
``` 
pub struct Router<S = [()][4]> { /* private fields */ }
```

Expand description

The router type for composing handlers and services.

`Router<S>` means a router that is _missing_ a state of type `S` to be able to handle requests. Thus, only `Router<()>` (i.e. without missing state) can be passed to [`serve`][5]. See [`Router::with_state`][6] for more details.

## Implementations§

[Source][7]§

### impl<S> [Router][8]<S>

where S: [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static,

[Source][12]

#### pub fn new() -> Self

Create a new `Router`.

Unless you add additional routes this will respond with `404 Not Found` to all requests.

[Source][13]

#### pub fn without_v07_checks(self) -> Self

Turn off checks for compatibility with route matching syntax from 0.7.

This allows usage of paths starting with a colon `:` or an asterisk `*` which are otherwise prohibited.

##### §Example
``` 
use axum::{
    routing::get,
    Router,
};

let app = Router::<()>::new()
    .without_v07_checks()
    .route("/:colon", get(|| async {}))
    .route("/*asterisk", get(|| async {}));

// Our app now accepts
// - GET /:colon
// - GET /*asterisk
```

Adding such routes without calling this method first will panic.

ⓘ
```
use axum::{
    routing::get,
    Router,
};

// This panics...
let app = Router::<()>::new()
    .route("/:colon", get(|| async {}));
```

##### §Merging

When two routers are merged, v0.7 checks are disabled for route registrations on the resulting router if both of the two routers had them also disabled.

##### §Nesting

Each router needs to have the checks explicitly disabled. Nesting a router with the checks either enabled or disabled has no effect on the outer router.

[Source][14]

#### pub fn route(self, path: &[str][15], method_router: [MethodRouter][16]<S>) -> Self

Add another route to the router.

`path` is a string of path segments separated by `/`. Each segment can be either static, a capture, or a wildcard.

`method_router` is the [`MethodRouter`][16] that should receive the request if the path matches `path`. Usually, `method_router` will be a handler wrapped in a method router like [`get`][17]. See [`handler`][18] for more details on handlers.

##### §Static paths

Examples:

  * `/`
  * `/foo`
  * `/users/123`



If the incoming request matches the path exactly the corresponding service will be called.

##### §Captures

Paths can contain segments like `/{key}` which matches any single segment and will store the value captured at `key`. The value captured can be zero-length except for in the invalid path `//`.

Examples:

  * `/{key}`
  * `/users/{id}`
  * `/users/{id}/tweets`



Captures can be extracted using [`Path`][19]. See its documentation for more details.

It is not possible to create segments that only match some types like numbers or regular expression. You must handle that manually in your handlers.

[`MatchedPath`][20] can be used to extract the matched path rather than the actual path.

##### §Wildcards

Paths can end in `/{*key}` which matches all segments and will store the segments captured at `key`.

Examples:

  * `/{*key}`
  * `/assets/{*path}`
  * `/{id}/{repo}/{*tree}`



Note that `/{*key}` doesn’t match empty segments. Thus:

  * `/{*key}` doesn’t match `/` but does match `/a`, `/a/`, etc.
  * `/x/{*key}` doesn’t match `/x` or `/x/` but does match `/x/a`, `/x/a/`, etc.



Wildcard captures can also be extracted using [`Path`][19]:
``` 
use axum::{
    Router,
    routing::get,
    extract::Path,
};

let app: Router = Router::new().route("/{*key}", get(handler));

async fn handler(Path(path): Path<String>) -> String {
    path
}
```

Note that the leading slash is not included, i.e. for the route `/foo/{*rest}` and the path `/foo/bar/baz` the value of `rest` will be `bar/baz`.

##### §Accepting multiple methods

To accept multiple methods for the same route you can add all handlers at the same time:
``` 
use axum::{Router, routing::{get, delete}, extract::Path};

let app = Router::new().route(
    "/",
    get(get_root).post(post_root).delete(delete_root),
);

async fn get_root() {}

async fn post_root() {}

async fn delete_root() {}
```

Or you can add them one by one:
``` 
let app = Router::new()
    .route("/", get(get_root))
    .route("/", post(post_root))
    .route("/", delete(delete_root));
```

##### §More examples
``` 
use axum::{Router, routing::{get, delete}, extract::Path};

let app = Router::new()
    .route("/", get(root))
    .route("/users", get(list_users).post(create_user))
    .route("/users/{id}", get(show_user))
    .route("/api/{version}/users/{id}/action", delete(do_users_action))
    .route("/assets/{*path}", get(serve_asset));

async fn root() {}

async fn list_users() {}

async fn create_user() {}

async fn show_user(Path(id): Path<u64>) {}

async fn do_users_action(Path((version, id)): Path<(String, u64)>) {}

async fn serve_asset(Path(path): Path<String>) {}
```

##### §Panics

Panics if the route overlaps with another route:

ⓘ
```
use axum::{routing::get, Router};

let app = Router::new()
    .route("/", get(|| async {}))
    .route("/", get(|| async {}));
```

The static route `/foo` and the dynamic route `/{key}` are not considered to overlap and `/foo` will take precedence.

Also panics if `path` is empty.

[Source][21]

#### pub fn route_service<T>(self, path: &[str][15], service: T) -> Self

where T: Service<[Request][22], Error = [Infallible][23]> \+ [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static, T::Response: [IntoResponse][24], T::Future: [Send][10] \+ 'static,

Add another route to the router that calls a [`Service`].

##### §Example
``` 
use axum::{
    Router,
    body::Body,
    routing::{any_service, get_service},
    extract::Request,
    http::StatusCode,
    error_handling::HandleErrorLayer,
};
use tower_http::services::ServeFile;
use http::Response;
use std::{convert::Infallible, io};
use tower::service_fn;

let app = Router::new()
    .route(
        // Any request to `/` goes to a service
        "/",
        // Services whose response body is not `axum::body::BoxBody`
        // can be wrapped in `axum::routing::any_service` (or one of the other routing filters)
        // to have the response body mapped
        any_service(service_fn(|_: Request| async {
            let res = Response::new(Body::from("Hi from `GET /`"));
            Ok::<_, Infallible>(res)
        }))
    )
    .route_service(
        "/foo",
        // This service's response body is `axum::body::BoxBody` so
        // it can be routed to directly.
        service_fn(|req: Request| async move {
            let body = Body::from(format!("Hi from `{} /foo`", req.method()));
            let res = Response::new(body);
            Ok::<_, Infallible>(res)
        })
    )
    .route_service(
        // GET `/static/Cargo.toml` goes to a service from tower-http
        "/static/Cargo.toml",
        ServeFile::new("Cargo.toml"),
    );
```

Routing to arbitrary services in this way has complications for backpressure ([`Service::poll_ready`]). See the [Routing to services and backpressure][25] module for more details.

##### §Panics

Panics for the same reasons as [`Router::route`][26] or if you attempt to route to a `Router`:

ⓘ
```
use axum::{routing::get, Router};

let app = Router::new().route_service(
    "/",
    Router::new().route("/foo", get(|| async {})),
);
```

Use [`Router::nest`][27] instead.

[Source][28]

#### pub fn nest(self, path: &[str][15], router: Self) -> Self

Nest a [`Router`][8] at some path.

This allows you to break your application into smaller pieces and compose them together.

##### §Example
``` 
use axum::{
    routing::{get, post},
    Router,
};

let user_routes = Router::new().route("/{id}", get(|| async {}));

let team_routes = Router::new().route("/", post(|| async {}));

let api_routes = Router::new()
    .nest("/users", user_routes)
    .nest("/teams", team_routes);

let app = Router::new().nest("/api", api_routes);

// Our app now accepts
// - GET /api/users/{id}
// - POST /api/teams
```

##### §How the URI changes

Note that nested routes will not see the original request URI but instead have the matched prefix stripped. This is necessary for services like static file serving to work. Use [`OriginalUri`][29] if you need the original request URI.

##### §Captures from outer routes

Take care when using `nest` together with dynamic routes as nesting also captures from the outer routes:
``` 
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use std::collections::HashMap;

async fn users_get(Path(params): Path<HashMap<String, String>>) {
    // Both `version` and `id` were captured even though `users_api` only
    // explicitly captures `id`.
    let version = params.get("version");
    let id = params.get("id");
}

let users_api = Router::new().route("/users/{id}", get(users_get));

let app = Router::new().nest("/{version}/api", users_api);
```

##### §Differences from wildcard routes

Nested routes are similar to wildcard routes. The difference is that wildcard routes still see the whole URI whereas nested routes will have the prefix stripped:
``` 
use axum::{routing::get, http::Uri, Router};

let nested_router = Router::new()
    .route("/", get(|uri: Uri| async {
        // `uri` will _not_ contain `/bar`
    }));

let app = Router::new()
    .route("/foo/{*rest}", get(|uri: Uri| async {
        // `uri` will contain `/foo`
    }))
    .nest("/bar", nested_router);
```

Additionally, while the wildcard route `/foo/*rest` will not match the paths `/foo` or `/foo/`, a nested router at `/foo` will match the path `/foo` (but not `/foo/`), and a nested router at `/foo/` will match the path `/foo/` (but not `/foo`).

##### §Fallbacks

If a nested router doesn’t have its own fallback then it will inherit the fallback from the outer router:
``` 
use axum::{routing::get, http::StatusCode, handler::Handler, Router};

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

let api_routes = Router::new().route("/users", get(|| async {}));

let app = Router::new()
    .nest("/api", api_routes)
    .fallback(fallback);
```

Here requests like `GET /api/not-found` will go into `api_routes` but because it doesn’t have a matching route and doesn’t have its own fallback it will call the fallback from the outer router, i.e. the `fallback` function.

If the nested router has its own fallback then the outer fallback will not be inherited:
``` 
use axum::{
    routing::get,
    http::StatusCode,
    handler::Handler,
    Json,
    Router,
};

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}

let api_routes = Router::new()
    .route("/users", get(|| async {}))
    .fallback(api_fallback);

let app = Router::new()
    .nest("/api", api_routes)
    .fallback(fallback);
```

Here requests like `GET /api/not-found` will go to `api_fallback`.

##### §Nesting routers with state

When combining [`Router`][8]s with this method, each [`Router`][8] must have the same type of state. If your routers have different types you can use [`Router::with_state`][6] to provide the state and make the types match:
``` 
use axum::{
    Router,
    routing::get,
    extract::State,
};

#[derive(Clone)]
struct InnerState {}

#[derive(Clone)]
struct OuterState {}

async fn inner_handler(state: State<InnerState>) {}

let inner_router = Router::new()
    .route("/bar", get(inner_handler))
    .with_state(InnerState {});

async fn outer_handler(state: State<OuterState>) {}

let app = Router::new()
    .route("/", get(outer_handler))
    .nest("/foo", inner_router)
    .with_state(OuterState {});
```

Note that the inner router will still inherit the fallback from the outer router.

##### §Panics

  * If the route overlaps with another route. See [`Router::route`][26] for more details.
  * If the route contains a wildcard (`*`).
  * If `path` is empty.



[Source][30]

#### pub fn nest_service<T>(self, path: &[str][15], service: T) -> Self

where T: Service<[Request][22], Error = [Infallible][23]> \+ [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static, T::Response: [IntoResponse][24], T::Future: [Send][10] \+ 'static,

Like [`nest`][27], but accepts an arbitrary `Service`.

[Source][31]

#### pub fn merge<R>(self, other: R) -> Self

where R: [Into][32]<Self>,

Merge the paths and fallbacks of two routers into a single [`Router`][8].

This is useful for breaking apps into smaller pieces and combining them into one.
``` 
use axum::{
    routing::get,
    Router,
};

// define some routes separately
let user_routes = Router::new()
    .route("/users", get(users_list))
    .route("/users/{id}", get(users_show));

let team_routes = Router::new()
    .route("/teams", get(teams_list));

// combine them into one
let app = Router::new()
    .merge(user_routes)
    .merge(team_routes);

// could also do `user_routes.merge(team_routes)`

// Our app now accepts
// - GET /users
// - GET /users/{id}
// - GET /teams
```

##### §Merging routers with state

When combining [`Router`][8]s with this method, each [`Router`][8] must have the same type of state. If your routers have different types you can use [`Router::with_state`][6] to provide the state and make the types match:
``` 
use axum::{
    Router,
    routing::get,
    extract::State,
};

#[derive(Clone)]
struct InnerState {}

#[derive(Clone)]
struct OuterState {}

async fn inner_handler(state: State<InnerState>) {}

let inner_router = Router::new()
    .route("/bar", get(inner_handler))
    .with_state(InnerState {});

async fn outer_handler(state: State<OuterState>) {}

let app = Router::new()
    .route("/", get(outer_handler))
    .merge(inner_router)
    .with_state(OuterState {});
```

##### §Merging routers with fallbacks

When combining [`Router`][8]s with this method, the [fallback][33] is also merged. However only one of the routers can have a fallback.

##### §Panics

  * If two routers that each have a [fallback][33] are merged. This is because `Router` only allows a single fallback.



[Source][34]

#### pub fn layer<L>(self, layer: L) -> Self

where L: Layer<[Route][35]> \+ [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static, L::Service: Service<[Request][22]> \+ [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static, <L::Service as Service<[Request][22]>>::Response: [IntoResponse][24] \+ 'static, <L::Service as Service<[Request][22]>>::Error: [Into][32]<[Infallible][23]> \+ 'static, <L::Service as Service<[Request][22]>>::Future: [Send][10] \+ 'static,

Apply a [`tower::Layer`] to all routes in the router.

This can be used to add additional processing to a request for a group of routes.

Note that the middleware is only applied to existing routes. So you have to first add your routes (and / or fallback) and then call `layer` afterwards. Additional routes added after `layer` is called will not have the middleware added.

If you want to add middleware to a single handler you can either use [`MethodRouter::layer`][36] or [`Handler::layer`][37].

##### §Example

Adding the [`tower_http::trace::TraceLayer`]:
``` 
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/foo", get(|| async {}))
    .route("/bar", get(|| async {}))
    .layer(TraceLayer::new_for_http());
```

If you need to write your own middleware see [“Writing middleware”][38] for the different options.

If you only want middleware on some routes you can use [`Router::merge`][39]:
``` 
use axum::{routing::get, Router};
use tower_http::{trace::TraceLayer, compression::CompressionLayer};

let with_tracing = Router::new()
    .route("/foo", get(|| async {}))
    .layer(TraceLayer::new_for_http());

let with_compression = Router::new()
    .route("/bar", get(|| async {}))
    .layer(CompressionLayer::new());

// Merge everything into one `Router`
let app = Router::new()
    .merge(with_tracing)
    .merge(with_compression);
```

##### §Multiple middleware

It’s recommended to use [`tower::ServiceBuilder`] when applying multiple middleware. See [`middleware`][40] for more details.

##### §Runs after routing

Middleware added with this method will run _after_ routing and thus cannot be used to rewrite the request URI. See [“Rewriting request URI in middleware”][41] for more details and a workaround.

##### §Error handling

See [`middleware`][40] for details on how error handling impacts middleware.

[Source][42]

#### pub fn route_layer<L>(self, layer: L) -> Self

where L: Layer<[Route][35]> \+ [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static, L::Service: Service<[Request][22]> \+ [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static, <L::Service as Service<[Request][22]>>::Response: [IntoResponse][24] \+ 'static, <L::Service as Service<[Request][22]>>::Error: [Into][32]<[Infallible][23]> \+ 'static, <L::Service as Service<[Request][22]>>::Future: [Send][10] \+ 'static,

Apply a [`tower::Layer`] to the router that will only run if the request matches a route.

Note that the middleware is only applied to existing routes. So you have to first add your routes (and / or fallback) and then call `route_layer` afterwards. Additional routes added after `route_layer` is called will not have the middleware added.

This works similarly to [`Router::layer`][43] except the middleware will only run if the request matches a route. This is useful for middleware that return early (such as authorization) which might otherwise convert a `404 Not Found` into a `401 Unauthorized`.

This function will panic if no routes have been declared yet on the router, since the new layer will have no effect, and this is typically a bug. In generic code, you can test if that is the case first, by calling [`Router::has_routes`][44].

##### §Example
``` 
use axum::{
    routing::get,
    Router,
};
use tower_http::validate_request::ValidateRequestHeaderLayer;

let app = Router::new()
    .route("/foo", get(|| async {}))
    .route_layer(ValidateRequestHeaderLayer::bearer("password"));

// `GET /foo` with a valid token will receive `200 OK`
// `GET /foo` with a invalid token will receive `401 Unauthorized`
// `GET /not-found` with a invalid token will receive `404 Not Found`
```

[Source][45]

#### pub fn has_routes(&self) -> [bool][46]

True if the router currently has at least one route added.

[Source][47]

#### pub fn fallback<H, T>(self, handler: H) -> Self

where H: [Handler][48]<T, S>, T: 'static,

Add a fallback [`Handler`][48] to the router.

This service will be called if no routes matches the incoming request.
``` 
use axum::{
    Router,
    routing::get,
    handler::Handler,
    response::IntoResponse,
    http::{StatusCode, Uri},
};

let app = Router::new()
    .route("/foo", get(|| async { /* ... */ }))
    .fallback(fallback);

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
```

Fallbacks only apply to routes that aren’t matched by anything in the router. If a handler is matched by a request but returns 404 the fallback is not called. Note that this applies to [`MethodRouter`][16]s too: if the request hits a valid path but the [`MethodRouter`][16] does not have an appropriate method handler installed, the fallback is not called (use [`MethodRouter::fallback`][49] for this purpose instead).

##### §Handling all requests without other routes

Using `Router::new().fallback(...)` to accept all request regardless of path or method, if you don’t have other routes, isn’t optimal:
``` 
use axum::Router;

async fn handler() {}

let app = Router::new().fallback(handler);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await;
```

Running the handler directly is faster since it avoids the overhead of routing:
``` 
use axum::handler::HandlerWithoutStateExt;

async fn handler() {}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, handler.into_make_service()).await;
```

[Source][50]

#### pub fn fallback_service<T>(self, service: T) -> Self

where T: Service<[Request][22], Error = [Infallible][23]> \+ [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static, T::Response: [IntoResponse][24], T::Future: [Send][10] \+ 'static,

Add a fallback [`Service`] to the router.

See [`Router::fallback`][33] for more details.

[Source][51]

#### pub fn method_not_allowed_fallback<H, T>(self, handler: H) -> Self

where H: [Handler][48]<T, S>, T: 'static,

Add a fallback [`Handler`][48] for the case where a route exists, but the method of the request is not supported.

Sets a fallback on all previously registered [`MethodRouter`][16]s, to be called when no matching method handler is set.
``` 
use axum::{response::IntoResponse, routing::get, Router};

async fn hello_world() -> impl IntoResponse {
    "Hello, world!\n"
}

async fn default_fallback() -> impl IntoResponse {
    "Default fallback\n"
}

async fn handle_405() -> impl IntoResponse {
    "Method not allowed fallback"
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(hello_world))
        .fallback(default_fallback)
        .method_not_allowed_fallback(handle_405);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await;
}
```

The fallback only applies if there is a `MethodRouter` registered for a given path, but the method used in the request is not specified. In the example, a `GET` on `http://localhost:3000` causes the `hello_world` handler to react, while issuing a `POST` triggers `handle_405`. Calling an entirely different route, like `http://localhost:3000/hello` causes `default_fallback` to run.

[Source][52]

#### pub fn reset_fallback(self) -> Self

Reset the fallback to its default.

Useful to merge two routers with fallbacks, as [`merge`][39] doesn’t allow both routers to have an explicit fallback. Use this method to remove the one you want to discard before merging.

[Source][53]

#### pub fn with_state<S2>(self, state: S) -> [Router][8]<S2>

Provide the state for the router. State passed to this method is global and will be used for all requests this router receives. That means it is not suitable for holding state derived from a request, such as authorization data extracted in a middleware. Use [`Extension`][54] instead for such data.
``` 
use axum::{Router, routing::get, extract::State};

#[derive(Clone)]
struct AppState {}

let routes = Router::new()
    .route("/", get(|State(state): State<AppState>| async {
        // use state
    }))
    .with_state(AppState {});

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, routes).await;
```

##### §Returning routers with states from functions

When returning `Router`s from functions, it is generally recommended not to set the state directly:
``` 
use axum::{Router, routing::get, extract::State};

#[derive(Clone)]
struct AppState {}

// Don't call `Router::with_state` here
fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|_: State<AppState>| async {}))
}

// Instead do it before you run the server
let routes = routes().with_state(AppState {});

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, routes).await;
```

If you do need to provide the state, and you’re _not_ nesting/merging the router into another router, then return `Router` without any type parameters:
``` 
// Don't return `Router<AppState>`
fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(|_: State<AppState>| async {}))
        .with_state(state)
}

let routes = routes(AppState {});

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, routes).await;
```

This is because we can only call `Router::into_make_service` on `Router<()>`, not `Router<AppState>`. See below for more details about why that is.

Note that the state defaults to `()` so `Router` and `Router<()>` is the same.

If you are nesting/merging the router it is recommended to use a generic state type on the resulting router:
``` 
fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(|_: State<AppState>| async {}))
        .with_state(state)
}

let routes = Router::new().nest("/api", routes(AppState {}));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, routes).await;
```

##### §What `S` in `Router<S>` means

`Router<S>` means a router that is _missing_ a state of type `S` to be able to handle requests. It does _not_ mean a `Router` that _has_ a state of type `S`.

For example:
``` 
// A router that _needs_ an `AppState` to handle requests
let router: Router<AppState> = Router::new()
    .route("/", get(|_: State<AppState>| async {}));

// Once we call `Router::with_state` the router isn't missing
// the state anymore, because we just provided it
//
// Therefore the router type becomes `Router<()>`, i.e a router
// that is not missing any state
let router: Router<()> = router.with_state(AppState {});

// Only `Router<()>` has the `into_make_service` method.
//
// You cannot call `into_make_service` on a `Router<AppState>`
// because it is still missing an `AppState`.
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router).await;
```

Perhaps a little counter intuitively, `Router::with_state` doesn’t always return a `Router<()>`. Instead you get to pick what the new missing state type is:
``` 
let router: Router<AppState> = Router::new()
    .route("/", get(|_: State<AppState>| async {}));

// When we call `with_state` we're able to pick what the next missing state type is.
// Here we pick `String`.
let string_router: Router<String> = router.with_state(AppState {});

// That allows us to add new routes that uses `String` as the state type
let string_router = string_router
    .route("/needs-string", get(|_: State<String>| async {}));

// Provide the `String` and choose `()` as the new missing state.
let final_router: Router<()> = string_router.with_state("foo".to_owned());

// Since we have a `Router<()>` we can run it.
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, final_router).await;
```

This is why returning `Router<AppState>` after calling `with_state` doesn’t work:

ⓘ
```
// This won't work because we're returning a `Router<AppState>`
// i.e. we're saying we're still missing an `AppState`
fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(|_: State<AppState>| async {}))
        .with_state(state)
}

let app = routes(AppState {});

// We can only call `Router::into_make_service` on a `Router<()>`
// but `app` is a `Router<AppState>`
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await;
```

Instead return `Router<()>` since we have provided all the state needed:
``` 
// We've provided all the state necessary so return `Router<()>`
fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/", get(|_: State<AppState>| async {}))
        .with_state(state)
}

let app = routes(AppState {});

// We can now call `Router::into_make_service`
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await;
```

##### §A note about performance

If you need a `Router` that implements `Service` but you don’t need any state (perhaps you’re making a library that uses axum internally) then it is recommended to call this method before you start serving requests:
``` 
use axum::{Router, routing::get};

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    // even though we don't need any state, call `with_state(())` anyway
    .with_state(());
```

This is not required but it gives axum a chance to update some internals in the router which may impact performance and reduce allocations.

Note that [`Router::into_make_service`][55] and [`Router::into_make_service_with_connect_info`][56] do this automatically.

[Source][57]

#### pub fn as_service<B>(&mut self) -> [RouterAsService][58]<'_, B, S>

Convert the router into a borrowed [`Service`] with a fixed request body type, to aid type inference.

In some cases when calling methods from [`tower::ServiceExt`] on a [`Router`][8] you might get type inference errors along the lines of
``` 
let response = router.ready().await?.call(request).await?;
                      ^^^^^ cannot infer type for type parameter `B`
```

This happens because `Router` implements [`Service`] with `impl<B> Service<Request<B>> for Router<()>`.

For example:

ⓘ
```
use axum::{
    Router,
    routing::get,
    http::Request,
    body::Body,
};
use tower::{Service, ServiceExt};

let mut router = Router::new().route("/", get(|| async {}));
let request = Request::new(Body::empty());
let response = router.ready().await?.call(request).await?;
```

Calling `Router::as_service` fixes that:
``` 
use axum::{
    Router,
    routing::get,
    http::Request,
    body::Body,
};
use tower::{Service, ServiceExt};

let mut router = Router::new().route("/", get(|| async {}));
let request = Request::new(Body::empty());
let response = router.as_service().ready().await?.call(request).await?;
```

This is mainly used when calling `Router` in tests. It shouldn’t be necessary when running the `Router` normally via [`Router::into_make_service`][55].

[Source][59]

#### pub fn into_service<B>(self) -> [RouterIntoService][60]<B, S>

Convert the router into an owned [`Service`] with a fixed request body type, to aid type inference.

This is the same as [`Router::as_service`][61] instead it returns an owned [`Service`]. See that method for more details.

[Source][62]§

### impl [Router][8]

[Source][63]

#### pub fn into_make_service(self) -> [IntoMakeService][64]<Self>

Convert this router into a [`MakeService`][65], that is a [`Service`] whose response is another service.
``` 
use axum::{
    routing::get,
    Router,
};

let app = Router::new().route("/", get(|| async { "Hi!" }));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await;
```

[Source][66]

#### pub fn into_make_service_with_connect_info<C>( self, ) -> [IntoMakeServiceWithConnectInfo][67]<Self, C>

Available on **crate feature`tokio`** only.

Convert this router into a [`MakeService`][65], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][68] can extract it.

This enables extracting things like the client’s remote address.

Extracting [`std::net::SocketAddr`][69] is supported out of the box:
``` 
use axum::{
    extract::ConnectInfo,
    routing::get,
    Router,
};
use std::net::SocketAddr;

let app = Router::new().route("/", get(handler));

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello {addr}")
}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await;
```

You can implement custom a [`Connected`][70] like so:
``` 
use axum::{
    extract::connect_info::{ConnectInfo, Connected},
    routing::get,
    serve::IncomingStream,
    Router,
};
use tokio::net::TcpListener;

let app = Router::new().route("/", get(handler));

async fn handler(
    ConnectInfo(my_connect_info): ConnectInfo<MyConnectInfo>,
) -> String {
    format!("Hello {my_connect_info:?}")
}

#[derive(Clone, Debug)]
struct MyConnectInfo {
    // ...
}

impl Connected<IncomingStream<'_, TcpListener>> for MyConnectInfo {
    fn connect_info(target: IncomingStream<'_, TcpListener>) -> Self {
        MyConnectInfo {
            // ...
        }
    }
}

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app.into_make_service_with_connect_info::<MyConnectInfo>()).await;
```

See the [unix domain socket example][71] for an example of how to use this to collect UDS connection info.

## Trait Implementations§

[Source][72]§

### impl<S> [Clone][9] for [Router][8]<S>

[Source][73]§

#### fn [clone][74](&self) -> Self

Returns a duplicate of the value. [Read more][74]

1.0.0 · [Source][75]§

#### fn [clone_from][76](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][76]

[Source][77]§

### impl<S> [Debug][78] for [Router][8]<S>

[Source][79]§

#### fn [fmt][80](&self, f: &mut [Formatter][81]<'_>) -> [Result][82]

Formats the value using the given formatter. [Read more][80]

[Source][83]§

### impl<S> [Default][84] for [Router][8]<S>

where S: [Clone][9] \+ [Send][10] \+ [Sync][11] \+ 'static,

[Source][85]§

#### fn [default][86]() -> Self

Returns the “default value” for a type. [Read more][86]

[Source][87]§

### impl<L> Service<[IncomingStream][88]<'_, L>> for [Router][8]<[()][4]>

where L: [Listener][89],

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

[Source][90]§

#### type Response = [Router][8]

Responses given by the service.

[Source][91]§

#### type Error = [Infallible][23]

Errors produced by the service.

[Source][92]§

#### type Future = [Ready][93]<[Result][94]<<[Router][8] as Service<[IncomingStream][88]<'_, L>>>::Response, <[Router][8] as Service<[IncomingStream][88]<'_, L>>>::Error>>

The future response value.

[Source][95]§

#### fn poll_ready(&mut self, _cx: &mut [Context][96]<'_>) -> [Poll][97]<[Result][94]<[()][4], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][98]§

#### fn call(&mut self, _req: [IncomingStream][88]<'_, L>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][99]§

### impl<B> Service<Request<B>> for [Router][8]<[()][4]>

where B: HttpBody<Data = Bytes> \+ [Send][10] \+ 'static, B::Error: [Into][32]<[BoxError][100]>,

[Source][101]§

#### type Response = Response<[Body][102]>

Responses given by the service.

[Source][103]§

#### type Error = [Infallible][23]

Errors produced by the service.

[Source][104]§

#### type Future = [RouteFuture][105]<[Infallible][23]>

The future response value.

[Source][106]§

#### fn poll_ready(&mut self, _: &mut [Context][96]<'_>) -> [Poll][97]<[Result][94]<[()][4], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][107]§

#### fn call(&mut self, req: [Request][22]<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<S> [Freeze][108] for [Router][8]<S>

§

### impl<S = [()][4]> ![RefUnwindSafe][109] for [Router][8]<S>

§

### impl<S> [Send][10] for [Router][8]<S>

§

### impl<S> [Sync][11] for [Router][8]<S>

§

### impl<S> [Unpin][110] for [Router][8]<S>

§

### impl<S = [()][4]> ![UnwindSafe][111] for [Router][8]<S>

## Blanket Implementations§

[Source][112]§

### impl<T> [Any][113] for T

where T: 'static + ?[Sized][114],

[Source][115]§

#### fn [type_id][116](&self) -> [TypeId][117]

Gets the `TypeId` of `self`. [Read more][116]

[Source][118]§

### impl<T> [Borrow][119]<T> for T

where T: ?[Sized][114],

[Source][120]§

#### fn [borrow][121](&self) -> [&T][122]

Immutably borrows from an owned value. [Read more][121]

[Source][123]§

### impl<T> [BorrowMut][124]<T> for T

where T: ?[Sized][114],

[Source][125]§

#### fn [borrow_mut][126](&mut self) -> [&mut T][122]

Mutably borrows from an owned value. [Read more][126]

[Source][127]§

### impl<T> [CloneToUninit][128] for T

where T: [Clone][9],

[Source][129]§

#### unsafe fn [clone_to_uninit][130](&self, dest: [*mut ][131][u8][132])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][130]

[Source][133]§

### impl<T> [From][134]<T> for T

[Source][135]§

#### fn [from][136](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][137]<T> for T

where T: [Clone][9],

§

#### fn [from_ref][138](input: [&T][122]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][139] [`Span`][140], returning an `Instrumented` wrapper. Read more

[Source][141]§

### impl<T, U> [Into][32]<U> for T

where U: [From][134]<T>,

[Source][142]§

#### fn [into][143](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][134]<T> for U` chooses to do.

§

### impl<M, S, Target, Request> MakeService<Target, Request> for M

where M: Service<Target, Response = S>, S: Service<Request>,

§

#### type Response = <S as Service<Request>>::Response

Responses given by the service

§

#### type Error = <S as Service<Request>>::Error

Errors produced by the service

§

#### type Service = S

The [`Service`] value created by this factory

§

#### type MakeError = <M as Service<Target>>::Error

Errors produced while building a service.

§

#### type Future = <M as Service<Target>>::Future

The future of the [`Service`] instance.

§

#### fn poll_ready( &mut self, cx: &mut [Context][96]<'_>, ) -> [Poll][97]<[Result][94]<[()][4], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][144] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][114],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][114],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][114],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][145]§

### impl<T> [Same][146] for T

[Source][147]§

#### type [Output][148] = T

Should always be `Self`

[Source][149]§

### impl<S, R> [ServiceExt][150]<R> for S

where S: Service<R>,

[Source][151]§

#### fn [into_make_service][152](self) -> [IntoMakeService][64]<S>

Convert this service into a [`MakeService`][65], that is a [`Service`] whose response is another service. [Read more][152]

[Source][153]§

#### fn [into_make_service_with_connect_info][154]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][67]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][65], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][68] can extract it. [Read more][154]

[Source][155]§

#### fn [handle_error][156]<F, T>(self, f: F) -> [HandleError][157]<Self, F, T>

Convert this service into a [`HandleError`][157], that will handle errors by converting them into responses. [Read more][156]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][114],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][114],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][114],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][114],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][114], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][158], and produce a [`Stream`][158] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][114], F: [Clone][9],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][159] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][114], F: [FnOnce][160](Self::Response) -> Response + [Clone][9],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][159] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][114], F: [FnOnce][160](Self::Error) -> Error + [Clone][9],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][159] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][114], Error: [From][134]<Self::Error>, F: [FnOnce][160]([Result][94]<Self::Response, Self::Error>) -> [Result][94]<Response, Error> \+ [Clone][9],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][114], F: [FnMut][161](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][114], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][162] that conditionally accepts or rejects requests based on a [predicate][163]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][114], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][164] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][114], Error: [From][134]<Self::Error>, F: [FnOnce][160]([Result][94]<Self::Response, Self::Error>) -> Fut + [Clone][9], Fut: [Future][165]<Output = [Result][94]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][114], F: [FnMut][161](Self::Future) -> Fut, Error: [From][134]<Self::Error>, Fut: [Future][165]<Output = [Result][94]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][114] \+ [Send][10] \+ 'static, Self::Future: [Send][10] \+ 'static,

Convert the service into a [`Service`][166] \+ [`Send`][10] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][114] \+ [Clone][9] \+ [Send][10] \+ 'static, Self::Future: [Send][10] \+ 'static,

Convert the service into a [`Service`][166] \+ [`Clone`][9] \+ [`Send`][10] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][114],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][114],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][167]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][114],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][114],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][114],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][114],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][114],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][114],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][114],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][168] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][169]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][114],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][170] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][169]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][114],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][170] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][169]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][114],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][170] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][114],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][114],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][114],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][114],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][114],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][114],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][114], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][114], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][114],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][114],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][114],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][171]) -> RequestBodyLimit<Self>

where Self: [Sized][114],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][114],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][114],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][172]§

### impl<T> [ToOwned][173] for T

where T: [Clone][9],

[Source][174]§

#### type [Owned][175] = T

The resulting type after obtaining ownership.

[Source][176]§

#### fn [to_owned][177](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][177]

[Source][178]§

#### fn [clone_into][179](&self, target: [&mut T][122])

Uses borrowed data to replace owned data, usually by cloning. [Read more][179]

[Source][180]§

### impl<T, U> [TryFrom][181]<U> for T

where U: [Into][32]<T>,

[Source][182]§

#### type [Error][183] = [Infallible][23]

The type returned in the event of a conversion error.

[Source][184]§

#### fn [try_from][185](value: U) -> [Result][94]<T, <T as [TryFrom][181]<U>>::[Error][186]>

Performs the conversion.

[Source][187]§

### impl<T, U> [TryInto][188]<U> for T

where U: [TryFrom][181]<T>,

[Source][189]§

#### type [Error][190] = <U as [TryFrom][181]<T>>::[Error][186]

The type returned in the event of a conversion error.

[Source][191]§

#### fn [try_into][192](self) -> [Result][94]<U, <U as [TryFrom][181]<T>>::[Error][186]>

Performs the conversion.

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][32]<Dispatch>,

Attaches the provided [`Subscriber`][193] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][194] [`Subscriber`][193] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../axum/index.html
   [2]: index.html
   [3]: ../src/axum/routing/mod.rs.html#86-88
   [4]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [5]: fn.serve.html (fn axum::serve)
   [6]: struct.Router.html#method.with_state (method axum::Router::with_state)
   [7]: ../src/axum/routing/mod.rs.html#154-536
   [8]: struct.Router.html (struct axum::Router)
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [10]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [12]: ../src/axum/routing/mod.rs.html#162-170
   [13]: ../src/axum/routing/mod.rs.html#184-188
   [14]: ../src/axum/routing/mod.rs.html#192-196
   [15]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [16]: routing/method_routing/struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [17]: routing/method_routing/fn.get.html (fn axum::routing::method_routing::get)
   [18]: handler/index.html (mod axum::handler)
   [19]: extract/struct.Path.html (struct axum::extract::Path)
   [20]: extract/struct.MatchedPath.html (struct axum::extract::MatchedPath)
   [21]: ../src/axum/routing/mod.rs.html#199-215
   [22]: extract/type.Request.html (type axum::extract::Request)
   [23]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [24]: response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [25]: middleware/index.html#routing-to-servicesmiddleware-and-backpressure
   [26]: struct.Router.html#method.route (method axum::Router::route)
   [27]: struct.Router.html#method.nest (method axum::Router::nest)
   [28]: ../src/axum/routing/mod.rs.html#220-237
   [29]: extract/struct.OriginalUri.html (struct axum::extract::OriginalUri)
   [30]: ../src/axum/routing/mod.rs.html#241-254
   [31]: ../src/axum/routing/mod.rs.html#258-293
   [32]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [33]: struct.Router.html#method.fallback (method axum::Router::fallback)
   [34]: ../src/axum/routing/mod.rs.html#296-309
   [35]: routing/struct.Route.html (struct axum::routing::Route)
   [36]: routing/method_routing/struct.MethodRouter.html#method.layer (method axum::routing::method_routing::MethodRouter::layer)
   [37]: handler/trait.Handler.html#method.layer (method axum::handler::Handler::layer)
   [38]: middleware/index.html#writing-middleware (mod axum::middleware)
   [39]: struct.Router.html#method.merge (method axum::Router::merge)
   [40]: middleware/index.html (mod axum::middleware)
   [41]: middleware/index.html#rewriting-request-uri-in-middleware (mod axum::middleware)
   [42]: ../src/axum/routing/mod.rs.html#313-326
   [43]: struct.Router.html#method.layer (method axum::Router::layer)
   [44]: struct.Router.html#method.has_routes (method axum::Router::has_routes)
   [45]: ../src/axum/routing/mod.rs.html#330-332
   [46]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [47]: ../src/axum/routing/mod.rs.html#336-346
   [48]: handler/trait.Handler.html (trait axum::handler::Handler)
   [49]: routing/method_routing/struct.MethodRouter.html#method.fallback (method axum::routing::method_routing::MethodRouter::fallback)
   [50]: ../src/axum/routing/mod.rs.html#351-362
   [51]: ../src/axum/routing/mod.rs.html#366-375
   [52]: ../src/axum/routing/mod.rs.html#384-389
   [53]: ../src/axum/routing/mod.rs.html#444-450
   [54]: struct.Extension.html (struct axum::Extension)
   [55]: struct.Router.html#method.into_make_service (method axum::Router::into_make_service)
   [56]: struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [57]: ../src/axum/routing/mod.rs.html#517-522
   [58]: routing/struct.RouterAsService.html (struct axum::routing::RouterAsService)
   [59]: ../src/axum/routing/mod.rs.html#530-535
   [60]: routing/struct.RouterIntoService.html (struct axum::routing::RouterIntoService)
   [61]: struct.Router.html#method.as_service (method axum::Router::as_service)
   [62]: ../src/axum/routing/mod.rs.html#538-572
   [63]: ../src/axum/routing/mod.rs.html#558-562
   [64]: routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [65]: tower::make::MakeService
   [66]: ../src/axum/routing/mod.rs.html#567-571
   [67]: extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [68]: extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [69]: https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html (enum core::net::socket_addr::SocketAddr)
   [70]: extract/connect_info/trait.Connected.html (trait axum::extract::connect_info::Connected)
   [71]: https://github.com/tokio-rs/axum/blob/main/examples/unix-domain-socket/src/main.rs
   [72]: ../src/axum/routing/mod.rs.html#90-96
   [73]: ../src/axum/routing/mod.rs.html#91-95
   [74]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [75]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [76]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [77]: ../src/axum/routing/mod.rs.html#113-121
   [78]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [79]: ../src/axum/routing/mod.rs.html#114-120
   [80]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [81]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [82]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [83]: ../src/axum/routing/mod.rs.html#104-111
   [84]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [85]: ../src/axum/routing/mod.rs.html#108-110
   [86]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [87]: ../src/axum/routing/mod.rs.html#579-596
   [88]: serve/struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [89]: serve/trait.Listener.html (trait axum::serve::Listener)
   [90]: ../src/axum/routing/mod.rs.html#583
   [91]: ../src/axum/routing/mod.rs.html#584
   [92]: ../src/axum/routing/mod.rs.html#585
   [93]: https://doc.rust-lang.org/nightly/core/future/ready/struct.Ready.html (struct core::future::ready::Ready)
   [94]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [95]: ../src/axum/routing/mod.rs.html#587-589
   [96]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [97]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [98]: ../src/axum/routing/mod.rs.html#591-595
   [99]: ../src/axum/routing/mod.rs.html#599-618
   [100]: type.BoxError.html (type axum::BoxError)
   [101]: ../src/axum/routing/mod.rs.html#604
   [102]: body/struct.Body.html (struct axum::body::Body)
   [103]: ../src/axum/routing/mod.rs.html#605
   [104]: ../src/axum/routing/mod.rs.html#606
   [105]: routing/future/struct.RouteFuture.html (struct axum::routing::future::RouteFuture)
   [106]: ../src/axum/routing/mod.rs.html#609-611
   [107]: ../src/axum/routing/mod.rs.html#614-617
   [108]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [109]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [110]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [111]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [112]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [113]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [114]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [115]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [116]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [117]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [118]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [119]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [120]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [121]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [122]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [123]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [124]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [125]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [126]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [127]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [128]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [129]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [130]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [131]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [132]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [133]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [134]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [135]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [136]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [137]: extract/trait.FromRef.html (trait axum::extract::FromRef)
   [138]: extract/trait.FromRef.html#tymethod.from_ref
   [139]: super::Span::current()
   [140]: crate::Span
   [141]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [142]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [143]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [144]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [145]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [146]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [147]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [148]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [149]: ../src/axum/service_ext.rs.html#47-59
   [150]: trait.ServiceExt.html (trait axum::ServiceExt)
   [151]: ../src/axum/service_ext.rs.html#51-53
   [152]: trait.ServiceExt.html#tymethod.into_make_service
   [153]: ../src/axum/service_ext.rs.html#56-58
   [154]: trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [155]: ../src/axum/service_ext.rs.html#42-44
   [156]: trait.ServiceExt.html#method.handle_error
   [157]: error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [158]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [159]: crate::Service::poll_ready
   [160]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [161]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [162]: crate::filter::Filter
   [163]: crate::filter::Predicate
   [164]: crate::filter::AsyncFilter
   [165]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [166]: crate::Service
   [167]: https://docs.rs/http/latest/http/struct.Extensions.html
   [168]: crate::follow_redirect::policy::Standard
   [169]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [170]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [171]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [172]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [173]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [174]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [175]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [176]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [177]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [178]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [179]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [180]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [181]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [182]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [183]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [184]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [185]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [186]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [187]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [188]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [189]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [190]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [191]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [192]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [193]: super::Subscriber
   [194]: dispatcher#setting-the-default-subscriber

