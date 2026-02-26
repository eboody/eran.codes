<!-- Generated from rustdoc HTML: handler/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module handler

## [axum][1]0.8.8

## Module handler

### Sections

  * Debugging handler type errors



### Module Items

  * Modules
  * Structs
  * Traits



## [In crate axum][2]

[axum][2]

# Module handler Copy item path

[Source][3]

Expand description

Async functions that can be used to handle requests.

In axum a “handler” is an async function that accepts zero or more [“extractors”][4] as arguments and returns something that can be converted [into a response][5].

Handlers are where your application logic lives and axum applications are built by routing between handlers.

Some examples of handlers:
``` 
use axum::{body::Bytes, http::StatusCode};

// Handler that immediately returns an empty `200 OK` response.
async fn unit_handler() {}

// Handler that immediately returns a `200 OK` response with a plain text
// body.
async fn string_handler() -> String {
    "Hello, World!".to_string()
}

// Handler that buffers the request body and returns it.
//
// This works because `Bytes` implements `FromRequest`
// and therefore can be used as an extractor.
//
// `String` and `StatusCode` both implement `IntoResponse` and
// therefore `Result<String, StatusCode>` also implements `IntoResponse`
async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
```

Instead of a direct `StatusCode`, it makes sense to use intermediate error type that can ultimately be converted to `Response`. This allows using `?` operator in handlers. See those examples:

  * [`anyhow-error-response`][6] for generic boxed errors
  * [`error-handling`][7] for application-specific detailed errors



### §Debugging handler type errors

For a function to be used as a handler it must implement the [`Handler`][8] trait. axum provides blanket implementations for functions that:

  * Are `async fn`s.
  * Take no more than 16 arguments that all implement `Send`. 
    * All except the last argument implement [`FromRequestParts`][9].
    * The last argument implements [`FromRequest`][10].
  * Returns something that implements [`IntoResponse`][11].
  * If a closure is used it must implement `Clone + Send` and be `'static`.
  * Returns a future that is `Send`. The most common way to accidentally make a future `!Send` is to hold a `!Send` type across an await.



Unfortunately Rust gives poor error messages if you try to use a function that doesn’t quite match what’s required by [`Handler`][8].

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

This error doesn’t tell you _why_ your function doesn’t implement [`Handler`][8]. It’s possible to improve the error with the [`debug_handler`][12] proc-macro from the [axum-macros][13] crate.

## Modules§

[future][14]
    Handler future types.

## Structs§

[HandlerService][15]
    An adapter that makes a [`Handler`][8] into a [`Service`].
[Layered][16]
    A [`Service`] created from a [`Handler`][8] by applying a Tower middleware.

## Traits§

[Handler][8]
    Trait for async functions that can be used to handle requests.
[HandlerWithoutStateExt][17]
    Extension trait for [`Handler`][8]s that don’t have state.

   [1]: ../../axum/index.html
   [2]: ../index.html
   [3]: ../../src/axum/handler/mod.rs.html#1-447
   [4]: ../extract/index.html (mod axum::extract)
   [5]: ../response/index.html (mod axum::response)
   [6]: https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
   [7]: https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs
   [8]: trait.Handler.html (trait axum::handler::Handler)
   [9]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [10]: ../extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [11]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [12]: https://docs.rs/axum-macros/latest/axum_macros/attr.debug_handler.html
   [13]: https://docs.rs/axum-macros
   [14]: future/index.html (mod axum::handler::future)
   [15]: struct.HandlerService.html (struct axum::handler::HandlerService)
   [16]: struct.Layered.html (struct axum::handler::Layered)
   [17]: trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)

