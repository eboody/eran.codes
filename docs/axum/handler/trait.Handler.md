<!-- Generated from rustdoc HTML: handler/trait.Handler.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Handler

## [axum][1]0.8.8

## Handler

### Sections

  * Converting `Handler`s into [`Service`]s
    * Debugging handler type errors
  * Handlers that aren’t functions
  * About type parameter `T`



### Required Associated Types

  * Future



### Required Methods

  * call



### Provided Methods

  * layer
  * with_state



### Dyn Compatibility

### Implementors

## [In axum::handler][2]

[axum][3]::[handler][2]

# Trait Handler Copy item path

[Source][4]
``` 
pub trait Handler<T, S>:
    [Clone][5]
    + [Send][6]
    + [Sync][7]
    + [Sized][8]
    + 'static {
    type Future: [Future][9]<Output = [Response][10]> + [Send][6] + 'static;

    // Required method
    fn call(self, req: [Request][11], state: S) -> Self::[Future][12];

    // Provided methods
    fn layer<L>(self, layer: L) -> [Layered][13]<L, Self, T, S>
       where L: Layer<[HandlerService][14]<Self, T, S>> + [Clone][5],
             L::Service: Service<[Request][11]> { ... }
    fn with_state(self, state: S) -> [HandlerService][14]<Self, T, S> { ... }
}
```

Expand description

Trait for async functions that can be used to handle requests.

You shouldn’t need to depend on this trait directly. It is automatically implemented to closures of the right types.

See the [module docs][15] for more details.

## §Converting `Handler`s into [`Service`]s

To convert `Handler`s into [`Service`]s you have to call either [`HandlerWithoutStateExt::into_service`][16] or [`Handler::with_state`][17]:
``` 
use tower::Service;
use axum::{
    extract::{State, Request},
    body::Body,
    handler::{HandlerWithoutStateExt, Handler},
};

// this handler doesn't require any state
async fn one() {}
// so it can be converted to a service with `HandlerWithoutStateExt::into_service`
assert_service(one.into_service());

// this handler requires state
async fn two(_: State<String>) {}
// so we have to provide it
let handler_with_state = two.with_state(String::new());
// which gives us a `Service`
assert_service(handler_with_state);

// helper to check that a value implements `Service`
fn assert_service<S>(service: S)
where
    S: Service<Request>,
{}
```

### §Debugging handler type errors

For a function to be used as a handler it must implement the [`Handler`][18] trait. axum provides blanket implementations for functions that:

  * Are `async fn`s.
  * Take no more than 16 arguments that all implement `Send`. 
    * All except the last argument implement [`FromRequestParts`][19].
    * The last argument implements [`FromRequest`][20].
  * Returns something that implements [`IntoResponse`][21].
  * If a closure is used it must implement `Clone + Send` and be `'static`.
  * Returns a future that is `Send`. The most common way to accidentally make a future `!Send` is to hold a `!Send` type across an await.



Unfortunately Rust gives poor error messages if you try to use a function that doesn’t quite match what’s required by [`Handler`][18].

You might get an error like this:
``` 
error[E0277]: the trait bound `fn(bool) -> impl Future {handler}: Handler<_, _>` is not satisfied
   --> src/main.rs:13:44
    |
13  |     let app = Router::new().route("/", get(handler));
    |                                            ^^^^^^^ the trait `Handler<_, _>` is not implemented for `fn(bool) -> impl Future {handler}`
    |
   ::: axum/src/handler/mod.rs:116:8
    |
116 |     H: Handler<T, B>,
    |        ------------- required by this bound in `axum::routing::get`
```

This error doesn’t tell you _why_ your function doesn’t implement [`Handler`][18]. It’s possible to improve the error with the [`debug_handler`][22] proc-macro from the [axum-macros][23] crate.

## §Handlers that aren’t functions

The `Handler` trait is also implemented for `T: IntoResponse`. That allows easily returning fixed data for routes:
``` 
use axum::{
    Router,
    routing::{get, post},
    Json,
    http::StatusCode,
};
use serde_json::json;

let app = Router::new()
    // respond with a fixed string
    .route("/", get("Hello, World!"))
    // or return some mock data
    .route("/users", post((
        StatusCode::CREATED,
        Json(json!({ "id": 1, "username": "alice" })),
    )));
```

## §About type parameter `T`

**Generally you shouldn’t need to worry about`T`**; when calling methods such as [`post`][24] it will be automatically inferred and this is the intended way for this parameter to be provided in application code.

If you are implementing your own methods that accept implementations of `Handler` as arguments, then the following may be useful:

The type parameter `T` is a workaround for trait coherence rules, allowing us to write blanket implementations of `Handler` over many types of handler functions with different numbers of arguments, without the compiler forbidding us from doing so because one type `F` can in theory implement both `Fn(A) -> X` and `Fn(A, B) -> Y`. `T` is a placeholder taking on a representation of the parameters of the handler function, as well as other similar ‘coherence rule workaround’ discriminators, allowing us to select one function signature to use as a `Handler`.

## Required Associated Types§

[Source][25]

#### type Future: [Future][9]<Output = [Response][10]> \+ [Send][6] \+ 'static

The type of future calling this handler returns.

## Required Methods§

[Source][26]

#### fn call(self, req: [Request][11], state: S) -> Self::[Future][12]

Call the handler with the given request.

## Provided Methods§

[Source][27]

#### fn layer<L>(self, layer: L) -> [Layered][13]<L, Self, T, S>

where L: Layer<[HandlerService][14]<Self, T, S>> \+ [Clone][5], L::Service: Service<[Request][11]>,

Apply a [`tower::Layer`] to the handler.

All requests to the handler will be processed by the layer’s corresponding middleware.

This can be used to add additional processing to a request for a single handler.

Note this differs from [`routing::Router::layer`][28] which adds a middleware to a group of routes.

If you’re applying middleware that produces errors you have to handle the errors so they’re converted into responses. You can learn more about doing that [here][29].

##### §Example

Adding the [`tower::limit::ConcurrencyLimit`] middleware to a handler can be done like so:
``` 
use axum::{
    routing::get,
    handler::Handler,
    Router,
};
use tower::limit::{ConcurrencyLimitLayer, ConcurrencyLimit};

async fn handler() { /* ... */ }

let layered_handler = handler.layer(ConcurrencyLimitLayer::new(64));
let app = Router::new().route("/", get(layered_handler));
```

[Source][30]

#### fn with_state(self, state: S) -> [HandlerService][14]<Self, T, S>

Convert the handler into a [`Service`] by providing the state

## Dyn Compatibility§

This trait is **not** [dyn compatible][31].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source][32]§

### impl<F, Fut, Res, S> [Handler][18]<([()][33],), S> for F

where F: [FnOnce][34]() -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], Res: [IntoResponse][21],

[Source][35]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1> [Handler][18]<[(M, T1)][40], S> for F

where F: [FnOnce][34](T1) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2> [Handler][18]<[(M, T1, T2)][40], S> for F

where F: [FnOnce][34](T1, T2) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3> [Handler][18]<[(M, T1, T2, T3)][40], S> for F

where F: [FnOnce][34](T1, T2, T3) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4> [Handler][18]<[(M, T1, T2, T3, T4)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5> [Handler][18]<[(M, T1, T2, T3, T4, T5)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequestParts][19]<S> \+ [Send][6], T10: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequestParts][19]<S> \+ [Send][6], T10: [FromRequestParts][19]<S> \+ [Send][6], T11: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequestParts][19]<S> \+ [Send][6], T10: [FromRequestParts][19]<S> \+ [Send][6], T11: [FromRequestParts][19]<S> \+ [Send][6], T12: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequestParts][19]<S> \+ [Send][6], T10: [FromRequestParts][19]<S> \+ [Send][6], T11: [FromRequestParts][19]<S> \+ [Send][6], T12: [FromRequestParts][19]<S> \+ [Send][6], T13: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequestParts][19]<S> \+ [Send][6], T10: [FromRequestParts][19]<S> \+ [Send][6], T11: [FromRequestParts][19]<S> \+ [Send][6], T12: [FromRequestParts][19]<S> \+ [Send][6], T13: [FromRequestParts][19]<S> \+ [Send][6], T14: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequestParts][19]<S> \+ [Send][6], T10: [FromRequestParts][19]<S> \+ [Send][6], T11: [FromRequestParts][19]<S> \+ [Send][6], T12: [FromRequestParts][19]<S> \+ [Send][6], T13: [FromRequestParts][19]<S> \+ [Send][6], T14: [FromRequestParts][19]<S> \+ [Send][6], T15: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][39]§

### impl<F, Fut, S, Res, M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [Handler][18]<[(M, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)][40], S> for F

where F: [FnOnce][34](T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Fut + [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, Fut: [Future][9]<Output = Res> \+ [Send][6], S: [Send][6] \+ [Sync][7] \+ 'static, Res: [IntoResponse][21], T1: [FromRequestParts][19]<S> \+ [Send][6], T2: [FromRequestParts][19]<S> \+ [Send][6], T3: [FromRequestParts][19]<S> \+ [Send][6], T4: [FromRequestParts][19]<S> \+ [Send][6], T5: [FromRequestParts][19]<S> \+ [Send][6], T6: [FromRequestParts][19]<S> \+ [Send][6], T7: [FromRequestParts][19]<S> \+ [Send][6], T8: [FromRequestParts][19]<S> \+ [Send][6], T9: [FromRequestParts][19]<S> \+ [Send][6], T10: [FromRequestParts][19]<S> \+ [Send][6], T11: [FromRequestParts][19]<S> \+ [Send][6], T12: [FromRequestParts][19]<S> \+ [Send][6], T13: [FromRequestParts][19]<S> \+ [Send][6], T14: [FromRequestParts][19]<S> \+ [Send][6], T15: [FromRequestParts][19]<S> \+ [Send][6], T16: [FromRequest][20]<S, M> \+ [Send][6],

[Source][39]§

#### type Future = [Pin][36]<[Box][37]<dyn [Future][9]<Output = Response<[Body][38]>> \+ [Send][6]>>

[Source][41]§

### impl<H, S, T, L> [Handler][18]<T, S> for [Layered][13]<L, H, T, S>

where L: Layer<[HandlerService][14]<H, T, S>> \+ [Clone][5] \+ [Send][6] \+ [Sync][7] \+ 'static, H: [Handler][18]<T, S>, L::Service: Service<[Request][11], Error = [Infallible][42]> \+ [Clone][5] \+ [Send][6] \+ 'static, <L::Service as Service<[Request][11]>>::Response: [IntoResponse][21], <L::Service as Service<[Request][11]>>::Future: [Send][6], T: 'static, S: 'static,

[Source][43]§

#### type Future = [LayeredFuture][44]<<L as Layer<[HandlerService][14]<H, T, S>>>::Service>

[Source][45]§

### impl<S> [Handler][18]<[()][33], S> for [MethodRouter][46]<S>

where S: [Clone][5] \+ 'static,

[Source][47]§

#### type Future = [InfallibleRouteFuture][48]

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/handler/mod.rs.html#148-205
   [5]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [6]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [7]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [8]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [9]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [10]: ../response/type.Response.html (type axum::response::Response)
   [11]: ../extract/type.Request.html (type axum::extract::Request)
   [12]: trait.Handler.html#associatedtype.Future (type axum::handler::Handler::Future)
   [13]: struct.Layered.html (struct axum::handler::Layered)
   [14]: struct.HandlerService.html (struct axum::handler::HandlerService)
   [15]: index.html (mod axum::handler)
   [16]: trait.HandlerWithoutStateExt.html#tymethod.into_service (method axum::handler::HandlerWithoutStateExt::into_service)
   [17]: trait.Handler.html#method.with_state (method axum::handler::Handler::with_state)
   [18]: trait.Handler.html (trait axum::handler::Handler)
   [19]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [20]: ../extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [21]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [22]: https://docs.rs/axum-macros/latest/axum_macros/attr.debug_handler.html
   [23]: https://docs.rs/axum-macros
   [24]: ../routing/method_routing/fn.post.html (fn axum::routing::method_routing::post)
   [25]: ../../src/axum/handler/mod.rs.html#150
   [26]: ../../src/axum/handler/mod.rs.html#153
   [27]: ../../src/axum/handler/mod.rs.html#189-199
   [28]: ../struct.Router.html#method.layer (method axum::Router::layer)
   [29]: ../error_handling/index.html (mod axum::error_handling)
   [30]: ../../src/axum/handler/mod.rs.html#202-204
   [31]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [32]: ../../src/axum/handler/mod.rs.html#208-219
   [33]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [34]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [35]: ../../src/axum/handler/mod.rs.html#214
   [36]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [37]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [38]: ../body/struct.Body.html (struct axum::body::Body)
   [39]: ../../src/axum/handler/mod.rs.html#262
   [40]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [41]: ../../src/axum/handler/mod.rs.html#317-350
   [42]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [43]: ../../src/axum/handler/mod.rs.html#327
   [44]: future/struct.LayeredFuture.html (struct axum::handler::future::LayeredFuture)
   [45]: ../../src/axum/routing/method_routing.rs.html#1355-1364
   [46]: ../routing/method_routing/struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [47]: ../../src/axum/routing/method_routing.rs.html#1359
   [48]: ../routing/future/struct.InfallibleRouteFuture.html (struct axum::routing::future::InfallibleRouteFuture)

