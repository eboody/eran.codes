<!-- Generated from rustdoc HTML: routing/method_routing/struct.MethodRouter.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MethodRouter

## [axum][1]0.8.8

## MethodRouter

### Sections

  * When does `MethodRouter` implement [`Service`]?



### Methods

  * connect
  * connect_service
  * delete
  * delete_service
  * fallback
  * fallback_service
  * get
  * get_service
  * handle_error
  * head
  * head_service
  * into_make_service
  * into_make_service_with_connect_info
  * layer
  * merge
  * method_filter
  * new
  * on
  * on_service
  * options
  * options_service
  * patch
  * patch_service
  * post
  * post_service
  * put
  * put_service
  * route_layer
  * trace
  * trace_service
  * with_state



### Trait Implementations

  * Clone
  * Debug
  * Default
  * Handler<(), S>
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
  * HandlerWithoutStateExt<T>
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



## [In axum::routing::method_routing][2]

[axum][3]::[routing][4]::[method_routing][2]

# Struct MethodRouter Copy item path

[Source][5]
``` 
pub struct MethodRouter<S = [()][6], E = [Infallible][7]> { /* private fields */ }
```

Expand description

A [`Service`] that accepts requests based on a [`MethodFilter`][8] and allows chaining additional handlers and services.

## §When does `MethodRouter` implement [`Service`]?

Whether or not `MethodRouter` implements [`Service`] depends on the state type it requires.
``` 
use tower::Service;
use axum::{routing::get, extract::{State, Request}, body::Body};

// this `MethodRouter` doesn't require any state, i.e. the state is `()`,
let method_router = get(|| async {});
// and thus it implements `Service`
assert_service(method_router);

// this requires a `String` and doesn't implement `Service`
let method_router = get(|_: State<String>| async {});
// until you provide the `String` with `.with_state(...)`
let method_router_with_state = method_router.with_state(String::new());
// and then it implements `Service`
assert_service(method_router_with_state);

// helper to check that a value implements `Service`
fn assert_service<S>(service: S)
where
    S: Service<Request>,
{}
```

## Implementations§

[Source][9]§

### impl<S> [MethodRouter][10]<S, [Infallible][7]>

where S: [Clone][11],

[Source][12]

#### pub fn on<H, T>(self, filter: [MethodFilter][8], handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will accept requests matching the given `MethodFilter`.

##### §Example
``` 
use axum::{
    routing::get,
    Router,
    routing::MethodFilter
};

async fn handler() {}

async fn other_handler() {}

// Requests to `GET /` will go to `handler` and `DELETE /` will go to
// `other_handler`
let app = Router::new().route("/", get(handler).on(MethodFilter::DELETE, other_handler));
```

[Source][16]

#### pub fn connect<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `CONNECT` requests.

See [`MethodFilter::CONNECT`][17] for when you’d want to use this, and [`MethodRouter::get`][18] for an example.

[Source][19]

#### pub fn delete<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `DELETE` requests.

See [`MethodRouter::get`][18] for an example.

[Source][20]

#### pub fn get<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `GET` requests.

##### §Example
``` 
use axum::{routing::post, Router};

async fn handler() {}

async fn other_handler() {}

// Requests to `POST /` will go to `handler` and `GET /` will go to
// `other_handler`.
let app = Router::new().route("/", post(handler).get(other_handler));
```

Note that `get` routes will also be called for `HEAD` requests but will have the response body removed. Make sure to add explicit `HEAD` routes afterwards.

[Source][21]

#### pub fn head<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `HEAD` requests.

See [`MethodRouter::get`][18] for an example.

[Source][22]

#### pub fn options<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `OPTIONS` requests.

See [`MethodRouter::get`][18] for an example.

[Source][23]

#### pub fn patch<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `PATCH` requests.

See [`MethodRouter::get`][18] for an example.

[Source][24]

#### pub fn post<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `POST` requests.

See [`MethodRouter::get`][18] for an example.

[Source][25]

#### pub fn put<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `PUT` requests.

See [`MethodRouter::get`][18] for an example.

[Source][26]

#### pub fn trace<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Chain an additional handler that will only accept `TRACE` requests.

See [`MethodRouter::get`][18] for an example.

[Source][27]

#### pub fn fallback<H, T>(self, handler: H) -> Self

where H: [Handler][13]<T, S>, T: 'static, S: [Send][14] \+ [Sync][15] \+ 'static,

Add a fallback [`Handler`][13] to the router.

[Source][28]

#### pub fn method_filter(&self) -> [Option][29]<[MethodFilter][8]>

Get a [`MethodFilter`][8] for the methods that this `MethodRouter` has custom code for.

Note that `MethodRouter`’s [`Service`] implementation never fails (it always creates an HTTP response) based on which HTTP method was used. However, the information which methods have the default behavior of returning HTTP 405 is stored, and can be queried with this method.

Returns `None` if the `MethodRouter` was constructed with [`any`][30] or has had a [`fallback`][31] set.

[Source][32]§

### impl [MethodRouter][10]<[()][6], [Infallible][7]>

[Source][33]

#### pub fn into_make_service(self) -> [IntoMakeService][34]<Self>

Convert the router into a [`MakeService`][35].

This allows you to serve a single `MethodRouter` if you don’t need any routing based on the path:
``` 
use axum::{
    handler::Handler,
    http::{Uri, Method},
    response::IntoResponse,
    routing::get,
};
use std::net::SocketAddr;

async fn handler(method: Method, uri: Uri, body: String) -> String {
    format!("received `{method} {uri}` with body `{body:?}`")
}

let router = get(handler).post(handler);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router.into_make_service()).await;
```

[Source][36]

#### pub fn into_make_service_with_connect_info<C>( self, ) -> [IntoMakeServiceWithConnectInfo][37]<Self, C>

Available on **crate feature`tokio`** only.

Convert the router into a [`MakeService`][35] which stores information about the incoming connection.

See [`Router::into_make_service_with_connect_info`][38] for more details.
``` 
use axum::{
    handler::Handler,
    response::IntoResponse,
    extract::ConnectInfo,
    routing::get,
};
use std::net::SocketAddr;

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Hello {addr}")
}

let router = get(handler).post(handler);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router.into_make_service()).await;
```

[Source][39]§

### impl<S, E> [MethodRouter][10]<S, E>

where S: [Clone][11],

[Source][40]

#### pub fn new() -> Self

Create a default `MethodRouter` that will respond with `405 Method Not Allowed` to all requests.

[Source][41]

#### pub fn with_state<S2>(self, state: S) -> [MethodRouter][10]<S2, E>

Provide the state for the router.

[Source][42]

#### pub fn on_service<T>(self, filter: [MethodFilter][8], svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will accept requests matching the given `MethodFilter`.

##### §Example
``` 
use axum::{
    extract::Request,
    Router,
    routing::{MethodFilter, on_service},
    body::Body,
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

// Requests to `DELETE /` will go to `service`
let app = Router::new().route("/", on_service(MethodFilter::DELETE, service));
```

[Source][45]

#### pub fn connect_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `CONNECT` requests.

See [`MethodFilter::CONNECT`][17] for when you’d want to use this, and [`MethodRouter::get_service`][46] for an example.

[Source][47]

#### pub fn delete_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `DELETE` requests.

See [`MethodRouter::get_service`][46] for an example.

[Source][48]

#### pub fn get_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `GET` requests.

##### §Example
``` 
use axum::{
    extract::Request,
    Router,
    routing::post_service,
    body::Body,
};
use http::Response;
use std::convert::Infallible;

let service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

let other_service = tower::service_fn(|request: Request| async {
    Ok::<_, Infallible>(Response::new(Body::empty()))
});

// Requests to `POST /` will go to `service` and `GET /` will go to
// `other_service`.
let app = Router::new().route("/", post_service(service).get_service(other_service));
```

Note that `get` routes will also be called for `HEAD` requests but will have the response body removed. Make sure to add explicit `HEAD` routes afterwards.

[Source][49]

#### pub fn head_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `HEAD` requests.

See [`MethodRouter::get_service`][46] for an example.

[Source][50]

#### pub fn options_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `OPTIONS` requests.

See [`MethodRouter::get_service`][46] for an example.

[Source][51]

#### pub fn patch_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `PATCH` requests.

See [`MethodRouter::get_service`][46] for an example.

[Source][52]

#### pub fn post_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `POST` requests.

See [`MethodRouter::get_service`][46] for an example.

[Source][53]

#### pub fn put_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `PUT` requests.

See [`MethodRouter::get_service`][46] for an example.

[Source][54]

#### pub fn trace_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Chain an additional service that will only accept `TRACE` requests.

See [`MethodRouter::get_service`][46] for an example.

[Source][55]

#### pub fn fallback_service<T>(self, svc: T) -> Self

where T: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, T::Response: [IntoResponse][44] \+ 'static, T::Future: [Send][14] \+ 'static,

Add a fallback service to the router.

This service will be called if no routes matches the incoming request.
``` 
use axum::{
    Router,
    routing::get,
    handler::Handler,
    response::IntoResponse,
    http::{StatusCode, Method, Uri},
};

let handler = get(|| async {}).fallback(fallback);

let app = Router::new().route("/", handler);

async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("`{method}` not allowed for {uri}"))
}
```

###### §When used with `MethodRouter::merge`

Two routers that both have a fallback cannot be merged. Doing so results in a panic:

ⓘ
```
use axum::{
    routing::{get, post},
    handler::Handler,
    response::IntoResponse,
    http::{StatusCode, Uri},
};

let one = get(|| async {}).fallback(fallback_one);

let two = post(|| async {}).fallback(fallback_two);

let method_route = one.merge(two);

async fn fallback_one() -> impl IntoResponse { /* ... */ }
async fn fallback_two() -> impl IntoResponse { /* ... */ }
```

###### §Setting the `Allow` header

By default `MethodRouter` will set the `Allow` header when returning `405 Method Not Allowed`. This is also done when the fallback returns `405 Method Not Allowed` unless the response generated by the fallback already sets the `Allow` header.

This means if you use `fallback` to accept additional methods, you should make sure you set the `Allow` header correctly.

[Source][56]

#### pub fn layer<L, NewError>(self, layer: L) -> [MethodRouter][10]<S, NewError>

where L: Layer<[Route][57]<E>> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, L::Service: Service<[Request][43]> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, <L::Service as Service<[Request][43]>>::Response: [IntoResponse][44] \+ 'static, <L::Service as Service<[Request][43]>>::Error: [Into][58]<NewError> \+ 'static, <L::Service as Service<[Request][43]>>::Future: [Send][14] \+ 'static, E: 'static, S: 'static, NewError: 'static,

Apply a [`tower::Layer`] to all routes in the router.

This can be used to add additional processing to a request for a group of routes.

Note that the middleware is only applied to existing routes. So you have to first add your routes (and / or fallback) and then call `layer` afterwards. Additional routes added after `layer` is called will not have the middleware added.

Works similarly to [`Router::layer`][59]. See that method for more details.

##### §Example
``` 
use axum::{routing::get, Router};
use tower::limit::ConcurrencyLimitLayer;

async fn handler() {}

let app = Router::new().route(
    "/",
    // All requests to `GET /` will be sent through `ConcurrencyLimitLayer`
    get(handler).layer(ConcurrencyLimitLayer::new(64)),
);
```

[Source][60]

#### pub fn route_layer<L>(self, layer: L) -> Self

where L: Layer<[Route][57]<E>> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, L::Service: Service<[Request][43], Error = E> \+ [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, <L::Service as Service<[Request][43]>>::Response: [IntoResponse][44] \+ 'static, <L::Service as Service<[Request][43]>>::Future: [Send][14] \+ 'static, E: 'static, S: 'static,

Apply a [`tower::Layer`] to the router that will only run if the request matches a route.

Note that the middleware is only applied to existing routes. First add your routes and then call `route_layer` afterwards. Additional routes added after `route_layer` is called will not have the middleware added.

This works similarly to [`MethodRouter::layer`][61] except the middleware will only run if the request matches a route. This is useful for middleware that return early (such as authorization) which might otherwise convert a `405 Method Not Allowed` into a `401 Unauthorized`.

##### §Example
``` 
use axum::{
    routing::get,
    Router,
};
use tower_http::validate_request::ValidateRequestHeaderLayer;

let app = Router::new().route(
    "/foo",
    get(|| async {})
        .route_layer(ValidateRequestHeaderLayer::bearer("password"))
);

// `GET /foo` with a valid token will receive `200 OK`
// `GET /foo` with a invalid token will receive `401 Unauthorized`
// `POST /FOO` with a invalid token will receive `405 Method Not Allowed`
```

[Source][62]

#### pub fn merge(self, other: Self) -> Self

Merge two routers into one.

This is useful for breaking routers into smaller pieces and combining them into one.
``` 
use axum::{
    routing::{get, post},
    Router,
};

let get = get(|| async {});
let post = post(|| async {});

let merged = get.merge(post);

let app = Router::new().route("/", merged);

// Our app now accepts
// - GET /
// - POST /
```

[Source][63]

#### pub fn handle_error<F, T>(self, f: F) -> [MethodRouter][10]<S, [Infallible][7]>

where F: [Clone][11] \+ [Send][14] \+ [Sync][15] \+ 'static, [HandleError][64]<[Route][57]<E>, F, T>: Service<[Request][43], Error = [Infallible][7]>, <[HandleError][64]<[Route][57]<E>, F, T> as Service<[Request][43]>>::Future: [Send][14], <[HandleError][64]<[Route][57]<E>, F, T> as Service<[Request][43]>>::Response: [IntoResponse][44] \+ [Send][14], T: 'static, E: 'static, S: 'static,

Apply a [`HandleErrorLayer`][65].

This is a convenience method for doing `self.layer(HandleErrorLayer::new(f))`.

## Trait Implementations§

[Source][66]§

### impl<S, E> [Clone][11] for [MethodRouter][10]<S, E>

[Source][67]§

#### fn [clone][68](&self) -> Self

Returns a duplicate of the value. [Read more][68]

1.0.0 · [Source][69]§

#### fn [clone_from][70](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][70]

[Source][71]§

### impl<S, E> [Debug][72] for [MethodRouter][10]<S, E>

[Source][73]§

#### fn [fmt][74](&self, f: &mut [Formatter][75]<'_>) -> [Result][76]

Formats the value using the given formatter. [Read more][74]

[Source][77]§

### impl<S, E> [Default][78] for [MethodRouter][10]<S, E>

where S: [Clone][11],

[Source][79]§

#### fn [default][80]() -> Self

Returns the “default value” for a type. [Read more][80]

[Source][81]§

### impl<S> [Handler][13]<[()][6], S> for [MethodRouter][10]<S>

where S: [Clone][11] \+ 'static,

[Source][82]§

#### type [Future][83] = [InfallibleRouteFuture][84]

The type of future calling this handler returns.

[Source][85]§

#### fn [call][86](self, req: [Request][43], state: S) -> Self::[Future][87]

Call the handler with the given request.

[Source][88]§

#### fn [layer][89]<L>(self, layer: L) -> [Layered][90]<L, Self, T, S>

where L: Layer<[HandlerService][91]<Self, T, S>> \+ [Clone][11], L::Service: Service<[Request][43]>,

Apply a [`tower::Layer`] to the handler. [Read more][89]

[Source][92]§

#### fn [with_state][93](self, state: S) -> [HandlerService][91]<Self, T, S>

Convert the handler into a [`Service`] by providing the state

[Source][94]§

### impl<L> Service<[IncomingStream][95]<'_, L>> for [MethodRouter][10]<[()][6]>

where L: [Listener][96],

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

[Source][97]§

#### type Response = [MethodRouter][10]

Responses given by the service.

[Source][98]§

#### type Error = [Infallible][7]

Errors produced by the service.

[Source][99]§

#### type Future = [Ready][100]<[Result][101]<<[MethodRouter][10] as Service<[IncomingStream][95]<'_, L>>>::Response, <[MethodRouter][10] as Service<[IncomingStream][95]<'_, L>>>::Error>>

The future response value.

[Source][102]§

#### fn poll_ready(&mut self, _cx: &mut [Context][103]<'_>) -> [Poll][104]<[Result][101]<[()][6], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][105]§

#### fn call(&mut self, _req: [IncomingStream][95]<'_, L>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][106]§

### impl<B, E> Service<Request<B>> for [MethodRouter][10]<[()][6], E>

where B: HttpBody<Data = Bytes> \+ [Send][14] \+ 'static, B::Error: [Into][58]<[BoxError][107]>,

[Source][108]§

#### type Response = Response<[Body][109]>

Responses given by the service.

[Source][110]§

#### type Error = E

Errors produced by the service.

[Source][111]§

#### type Future = [RouteFuture][112]<E>

The future response value.

[Source][113]§

#### fn poll_ready(&mut self, _cx: &mut [Context][103]<'_>) -> [Poll][104]<[Result][101]<[()][6], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][114]§

#### fn call(&mut self, req: [Request][43]<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<S, E> [Freeze][115] for [MethodRouter][10]<S, E>

§

### impl<S = [()][6], E = [Infallible][7]> ![RefUnwindSafe][116] for [MethodRouter][10]<S, E>

§

### impl<S, E> [Send][14] for [MethodRouter][10]<S, E>

§

### impl<S, E> [Sync][15] for [MethodRouter][10]<S, E>

§

### impl<S, E> [Unpin][117] for [MethodRouter][10]<S, E>

§

### impl<S = [()][6], E = [Infallible][7]> ![UnwindSafe][118] for [MethodRouter][10]<S, E>

## Blanket Implementations§

[Source][119]§

### impl<T> [Any][120] for T

where T: 'static + ?[Sized][121],

[Source][122]§

#### fn [type_id][123](&self) -> [TypeId][124]

Gets the `TypeId` of `self`. [Read more][123]

[Source][125]§

### impl<T> [Borrow][126]<T> for T

where T: ?[Sized][121],

[Source][127]§

#### fn [borrow][128](&self) -> [&T][129]

Immutably borrows from an owned value. [Read more][128]

[Source][130]§

### impl<T> [BorrowMut][131]<T> for T

where T: ?[Sized][121],

[Source][132]§

#### fn [borrow_mut][133](&mut self) -> [&mut T][129]

Mutably borrows from an owned value. [Read more][133]

[Source][134]§

### impl<T> [CloneToUninit][135] for T

where T: [Clone][11],

[Source][136]§

#### unsafe fn [clone_to_uninit][137](&self, dest: [*mut ][138][u8][139])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][137]

[Source][140]§

### impl<T> [From][141]<T> for T

[Source][142]§

#### fn [from][143](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][144]<T> for T

where T: [Clone][11],

§

#### fn [from_ref][145](input: [&T][129]) -> T

Converts to this type from a reference to the input type.

[Source][146]§

### impl<H, T> [HandlerWithoutStateExt][147]<T> for H

where H: [Handler][13]<T, [()][6]>,

[Source][148]§

#### fn [into_service][149](self) -> [HandlerService][91]<H, T, [()][6]>

Convert the handler into a [`Service`] and no state.

[Source][150]§

#### fn [into_make_service][151](self) -> [IntoMakeService][34]<[HandlerService][91]<H, T, [()][6]>>

Convert the handler into a [`MakeService`][35] and no state. [Read more][151]

[Source][152]§

#### fn [into_make_service_with_connect_info][153]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][37]<[HandlerService][91]<H, T, [()][6]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][35] which stores information about the incoming connection and has no state. [Read more][153]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][154] [`Span`][155], returning an `Instrumented` wrapper. Read more

[Source][156]§

### impl<T, U> [Into][58]<U> for T

where U: [From][141]<T>,

[Source][157]§

#### fn [into][158](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][141]<T> for U` chooses to do.

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

#### fn poll_ready( &mut self, cx: &mut [Context][103]<'_>, ) -> [Poll][104]<[Result][101]<[()][6], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][159] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][121],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][121],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][121],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][160]§

### impl<T> [Same][161] for T

[Source][162]§

#### type [Output][163] = T

Should always be `Self`

[Source][164]§

### impl<S, R> [ServiceExt][165]<R> for S

where S: Service<R>,

[Source][166]§

#### fn [into_make_service][167](self) -> [IntoMakeService][34]<S>

Convert this service into a [`MakeService`][35], that is a [`Service`] whose response is another service. [Read more][167]

[Source][168]§

#### fn [into_make_service_with_connect_info][169]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][37]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][35], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][170] can extract it. [Read more][169]

[Source][171]§

#### fn [handle_error][172]<F, T>(self, f: F) -> [HandleError][64]<Self, F, T>

Convert this service into a [`HandleError`][64], that will handle errors by converting them into responses. [Read more][172]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][121],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][121],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][121],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][121],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][121], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][173], and produce a [`Stream`][173] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][121], F: [Clone][11],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][174] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][121], F: [FnOnce][175](Self::Response) -> Response + [Clone][11],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][174] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][121], F: [FnOnce][175](Self::Error) -> Error + [Clone][11],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][174] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][121], Error: [From][141]<Self::Error>, F: [FnOnce][175]([Result][101]<Self::Response, Self::Error>) -> [Result][101]<Response, Error> \+ [Clone][11],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][121], F: [FnMut][176](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][121], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][177] that conditionally accepts or rejects requests based on a [predicate][178]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][121], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][179] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][121], Error: [From][141]<Self::Error>, F: [FnOnce][175]([Result][101]<Self::Response, Self::Error>) -> Fut + [Clone][11], Fut: [Future][180]<Output = [Result][101]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][121], F: [FnMut][176](Self::Future) -> Fut, Error: [From][141]<Self::Error>, Fut: [Future][180]<Output = [Result][101]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][121] \+ [Send][14] \+ 'static, Self::Future: [Send][14] \+ 'static,

Convert the service into a [`Service`][181] \+ [`Send`][14] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][121] \+ [Clone][11] \+ [Send][14] \+ 'static, Self::Future: [Send][14] \+ 'static,

Convert the service into a [`Service`][181] \+ [`Clone`][11] \+ [`Send`][14] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][121],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][121],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][182]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][121],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][121],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][121],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][121],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][121],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][121],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][121],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][183] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][184]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][121],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][185] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][184]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][121],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][185] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][184]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][121],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][185] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][121],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][121],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][121],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][121],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][121],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][121],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][121], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][121], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][121],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][121],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][121],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][186]) -> RequestBodyLimit<Self>

where Self: [Sized][121],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][121],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][121],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][187]§

### impl<T> [ToOwned][188] for T

where T: [Clone][11],

[Source][189]§

#### type [Owned][190] = T

The resulting type after obtaining ownership.

[Source][191]§

#### fn [to_owned][192](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][192]

[Source][193]§

#### fn [clone_into][194](&self, target: [&mut T][129])

Uses borrowed data to replace owned data, usually by cloning. [Read more][194]

[Source][195]§

### impl<T, U> [TryFrom][196]<U> for T

where U: [Into][58]<T>,

[Source][197]§

#### type [Error][198] = [Infallible][7]

The type returned in the event of a conversion error.

[Source][199]§

#### fn [try_from][200](value: U) -> [Result][101]<T, <T as [TryFrom][196]<U>>::[Error][201]>

Performs the conversion.

[Source][202]§

### impl<T, U> [TryInto][203]<U> for T

where U: [TryFrom][196]<T>,

[Source][204]§

#### type [Error][205] = <U as [TryFrom][196]<T>>::[Error][201]

The type returned in the event of a conversion error.

[Source][206]§

#### fn [try_into][207](self) -> [Result][101]<U, <U as [TryFrom][196]<T>>::[Error][201]>

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

where S: [Into][58]<Dispatch>,

Attaches the provided [`Subscriber`][208] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][209] [`Subscriber`][208] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/routing/method_routing.rs.html#547-559
   [6]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [7]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [8]: ../struct.MethodFilter.html (struct axum::routing::MethodFilter)
   [9]: ../../../src/axum/routing/method_routing.rs.html#604-723
   [10]: struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [12]: ../../../src/axum/routing/method_routing.rs.html#630-640
   [13]: ../../handler/trait.Handler.html (trait axum::handler::Handler)
   [14]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [15]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [16]: ../../../src/axum/routing/method_routing.rs.html#642
   [17]: ../struct.MethodFilter.html#associatedconstant.CONNECT (associated constant axum::routing::MethodFilter::CONNECT)
   [18]: struct.MethodRouter.html#method.get (method axum::routing::method_routing::MethodRouter::get)
   [19]: ../../../src/axum/routing/method_routing.rs.html#643
   [20]: ../../../src/axum/routing/method_routing.rs.html#644
   [21]: ../../../src/axum/routing/method_routing.rs.html#645
   [22]: ../../../src/axum/routing/method_routing.rs.html#646
   [23]: ../../../src/axum/routing/method_routing.rs.html#647
   [24]: ../../../src/axum/routing/method_routing.rs.html#648
   [25]: ../../../src/axum/routing/method_routing.rs.html#649
   [26]: ../../../src/axum/routing/method_routing.rs.html#650
   [27]: ../../../src/axum/routing/method_routing.rs.html#653-661
   [28]: ../../../src/axum/routing/method_routing.rs.html#673-709
   [29]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [30]: fn.any.html (fn axum::routing::method_routing::any)
   [31]: struct.MethodRouter.html#method.fallback (method axum::routing::method_routing::MethodRouter::fallback)
   [32]: ../../../src/axum/routing/method_routing.rs.html#725-791
   [33]: ../../../src/axum/routing/method_routing.rs.html#754-756
   [34]: ../struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [35]: tower::make::MakeService
   [36]: ../../../src/axum/routing/method_routing.rs.html#788-790
   [37]: ../../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [38]: ../../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [39]: ../../../src/axum/routing/method_routing.rs.html#793-1223
   [40]: ../../../src/axum/routing/method_routing.rs.html#799-817
   [41]: ../../../src/axum/routing/method_routing.rs.html#820-834
   [42]: ../../../src/axum/routing/method_routing.rs.html#860-867
   [43]: ../../extract/type.Request.html (type axum::extract::Request)
   [44]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [45]: ../../../src/axum/routing/method_routing.rs.html#992
   [46]: struct.MethodRouter.html#method.get_service (method axum::routing::method_routing::MethodRouter::get_service)
   [47]: ../../../src/axum/routing/method_routing.rs.html#993
   [48]: ../../../src/axum/routing/method_routing.rs.html#994
   [49]: ../../../src/axum/routing/method_routing.rs.html#995
   [50]: ../../../src/axum/routing/method_routing.rs.html#996
   [51]: ../../../src/axum/routing/method_routing.rs.html#997
   [52]: ../../../src/axum/routing/method_routing.rs.html#998
   [53]: ../../../src/axum/routing/method_routing.rs.html#999
   [54]: ../../../src/axum/routing/method_routing.rs.html#1000
   [55]: ../../../src/axum/routing/method_routing.rs.html#1003-1011
   [56]: ../../../src/axum/routing/method_routing.rs.html#1014-1040
   [57]: ../struct.Route.html (struct axum::routing::Route)
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [59]: ../../struct.Router.html#method.layer (method axum::Router::layer)
   [60]: ../../../src/axum/routing/method_routing.rs.html#1044-1082
   [61]: struct.MethodRouter.html#method.layer (method axum::routing::method_routing::MethodRouter::layer)
   [62]: ../../../src/axum/routing/method_routing.rs.html#1138-1144
   [63]: ../../../src/axum/routing/method_routing.rs.html#1149-1160
   [64]: ../../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [65]: ../../error_handling/struct.HandleErrorLayer.html (struct axum::error_handling::HandleErrorLayer)
   [66]: ../../../src/axum/routing/method_routing.rs.html#1245-1261
   [67]: ../../../src/axum/routing/method_routing.rs.html#1246-1260
   [68]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [69]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [70]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [71]: ../../../src/axum/routing/method_routing.rs.html#586-602
   [72]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [73]: ../../../src/axum/routing/method_routing.rs.html#587-601
   [74]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [75]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [76]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [77]: ../../../src/axum/routing/method_routing.rs.html#1263-1270
   [78]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [79]: ../../../src/axum/routing/method_routing.rs.html#1267-1269
   [80]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [81]: ../../../src/axum/routing/method_routing.rs.html#1355-1364
   [82]: ../../../src/axum/routing/method_routing.rs.html#1359
   [83]: ../../handler/trait.Handler.html#associatedtype.Future
   [84]: ../future/struct.InfallibleRouteFuture.html (struct axum::routing::future::InfallibleRouteFuture)
   [85]: ../../../src/axum/routing/method_routing.rs.html#1361-1363
   [86]: ../../handler/trait.Handler.html#tymethod.call
   [87]: ../../handler/trait.Handler.html#associatedtype.Future (type axum::handler::Handler::Future)
   [88]: ../../../src/axum/handler/mod.rs.html#189-199
   [89]: ../../handler/trait.Handler.html#method.layer
   [90]: ../../handler/struct.Layered.html (struct axum::handler::Layered)
   [91]: ../../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [92]: ../../../src/axum/handler/mod.rs.html#202-204
   [93]: ../../handler/trait.Handler.html#method.with_state
   [94]: ../../../src/axum/routing/method_routing.rs.html#1371-1386
   [95]: ../../serve/struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [96]: ../../serve/trait.Listener.html (trait axum::serve::Listener)
   [97]: ../../../src/axum/routing/method_routing.rs.html#1375
   [98]: ../../../src/axum/routing/method_routing.rs.html#1376
   [99]: ../../../src/axum/routing/method_routing.rs.html#1377
   [100]: https://doc.rust-lang.org/nightly/core/future/ready/struct.Ready.html (struct core::future::ready::Ready)
   [101]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [102]: ../../../src/axum/routing/method_routing.rs.html#1379-1381
   [103]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [104]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [105]: ../../../src/axum/routing/method_routing.rs.html#1383-1385
   [106]: ../../../src/axum/routing/method_routing.rs.html#1333-1352
   [107]: ../../type.BoxError.html (type axum::BoxError)
   [108]: ../../../src/axum/routing/method_routing.rs.html#1338
   [109]: ../../body/struct.Body.html (struct axum::body::Body)
   [110]: ../../../src/axum/routing/method_routing.rs.html#1339
   [111]: ../../../src/axum/routing/method_routing.rs.html#1340
   [112]: ../future/struct.RouteFuture.html (struct axum::routing::future::RouteFuture)
   [113]: ../../../src/axum/routing/method_routing.rs.html#1343-1345
   [114]: ../../../src/axum/routing/method_routing.rs.html#1348-1351
   [115]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [116]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [117]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [118]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [119]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [120]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [121]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [122]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [123]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [124]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [125]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [126]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [127]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [128]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [129]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [130]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [131]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [132]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [133]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [134]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [135]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [136]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [137]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [138]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [139]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [140]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [141]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [142]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [143]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [144]: ../../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [145]: ../../extract/trait.FromRef.html#tymethod.from_ref
   [146]: ../../../src/axum/handler/mod.rs.html#380-398
   [147]: ../../handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [148]: ../../../src/axum/handler/mod.rs.html#384-386
   [149]: ../../handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [150]: ../../../src/axum/handler/mod.rs.html#388-390
   [151]: ../../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [152]: ../../../src/axum/handler/mod.rs.html#393-397
   [153]: ../../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [154]: super::Span::current()
   [155]: crate::Span
   [156]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [157]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [158]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [159]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [160]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [161]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [162]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [163]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [164]: ../../../src/axum/service_ext.rs.html#47-59
   [165]: ../../trait.ServiceExt.html (trait axum::ServiceExt)
   [166]: ../../../src/axum/service_ext.rs.html#51-53
   [167]: ../../trait.ServiceExt.html#tymethod.into_make_service
   [168]: ../../../src/axum/service_ext.rs.html#56-58
   [169]: ../../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [170]: ../../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [171]: ../../../src/axum/service_ext.rs.html#42-44
   [172]: ../../trait.ServiceExt.html#method.handle_error
   [173]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [174]: crate::Service::poll_ready
   [175]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [176]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [177]: crate::filter::Filter
   [178]: crate::filter::Predicate
   [179]: crate::filter::AsyncFilter
   [180]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [181]: crate::Service
   [182]: https://docs.rs/http/latest/http/struct.Extensions.html
   [183]: crate::follow_redirect::policy::Standard
   [184]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [185]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [186]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [187]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [188]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [189]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [190]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [191]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [192]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [193]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [194]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [195]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [196]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [197]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [198]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [199]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [200]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [201]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [202]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [203]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [204]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [205]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [206]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [207]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [208]: super::Subscriber
   [209]: dispatcher#setting-the-default-subscriber

