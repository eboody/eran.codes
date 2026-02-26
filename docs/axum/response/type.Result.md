<!-- Generated from rustdoc HTML: response/type.Result.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Result

## [axum][1]0.8.8

## Result

### Sections

  * Example
  * As a replacement for `std::result::Result`



### Aliased Type

### Variants

  * Err
  * Ok



## [In axum::response][2]

[axum][3]::[response][2]

# Type Alias Result Copy item path
```
pub type Result<T, E = [ErrorResponse][4]> = [Result][5]<T, E>;
```

Expand description

An [`IntoResponse`][6]-based result type that uses [`ErrorResponse`][4] as the error type.

All types which implement [`IntoResponse`][6] can be converted to an [`ErrorResponse`][4]. This makes it useful as a general purpose error type for functions which combine multiple distinct error types that all implement [`IntoResponse`][6].

## §Example
``` 
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

// two fallible functions with different error types
fn try_something() -> Result<(), ErrorA> {
    // ...
}

fn try_something_else() -> Result<(), ErrorB> {
    // ...
}

// each error type implements `IntoResponse`
struct ErrorA;

impl IntoResponse for ErrorA {
    fn into_response(self) -> Response {
        // ...
    }
}

enum ErrorB {
    SomethingWentWrong,
}

impl IntoResponse for ErrorB {
    fn into_response(self) -> Response {
        // ...
    }
}

// we can combine them using `axum::response::Result` and still use `?`
async fn handler() -> axum::response::Result<&'static str> {
    // the errors are automatically converted to `ErrorResponse`
    try_something()?;
    try_something_else()?;

    Ok("it worked!")
}
```

## §As a replacement for `std::result::Result`

Since `axum::response::Result` has a default error type you only have to specify the `Ok` type:
``` 
use axum::{
    response::{IntoResponse, Response, Result},
    http::StatusCode,
};

// `Result<T>` automatically uses `ErrorResponse` as the error type.
async fn handler() -> Result<&'static str> {
    try_something()?;

    Ok("it worked!")
}

// You can still specify the error even if you've imported `axum::response::Result`
fn try_something() -> Result<(), StatusCode> {
    // ...
}
```

## Aliased Type§
```
pub enum Result<T, E = [ErrorResponse][4]> {
    Ok(T),
    Err(E),
}
```

## Variants§

§1.0.0

### Ok(T)

Contains the success value

§1.0.0

### Err(E)

Contains the error value

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: struct.ErrorResponse.html (struct axum::response::ErrorResponse)
   [5]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [6]: trait.IntoResponse.html (trait axum::response::IntoResponse)

