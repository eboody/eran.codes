<!-- Generated from rustdoc HTML: response/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module response

## [axum][1]0.8.8

## Module response

### Sections

  * Building responses
  * Returning different response types
  * Regarding `impl IntoResponse`



### Module Items

  * Re-exports
  * Modules
  * Structs
  * Traits
  * Type Aliases



## [In crate axum][2]

[axum][2]

# Module response Copy item path

[Source][3]

Expand description

Types and traits for generating responses.

## §Building responses

Anything that implements [`IntoResponse`][4] can be returned from a handler. axum provides implementations for common types:
``` 
use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};

// `()` gives an empty response
async fn empty() {}

// String will get a `text/plain; charset=utf-8` content-type
async fn plain_text(uri: Uri) -> String {
    format!("Hi from {}", uri.path())
}

// Bytes will get a `application/octet-stream` content-type
async fn bytes() -> Vec<u8> {
    vec![1, 2, 3, 4]
}

// `Json` will get a `application/json` content-type and work with anything that
// implements `serde::Serialize`
async fn json() -> Json<Vec<String>> {
    Json(vec!["foo".to_owned(), "bar".to_owned()])
}

// `Html` will get a `text/html` content-type
async fn html() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}

// `StatusCode` gives an empty response with that status code
async fn status() -> StatusCode {
    StatusCode::NOT_FOUND
}

// `HeaderMap` gives an empty response with some headers
async fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::SERVER, "axum".parse().unwrap());
    headers
}

// An array of tuples also gives headers
async fn array_headers() -> [(HeaderName, &'static str); 2] {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain")
    ]
}

// Use `impl IntoResponse` to avoid writing the whole type
async fn impl_trait() -> impl IntoResponse {
    [
        (header::SERVER, "axum"),
        (header::CONTENT_TYPE, "text/plain")
    ]
}
```

Additionally you can return tuples to build more complex responses from individual parts.
``` 
use axum::{
    Json,
    response::IntoResponse,
    http::{StatusCode, HeaderMap, Uri, header},
    extract::Extension,
};

// `(StatusCode, impl IntoResponse)` will override the status code of the response
async fn with_status(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path()))
}

// Use `impl IntoResponse` to avoid having to type the whole type
async fn impl_trait(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path()))
}

// `(HeaderMap, impl IntoResponse)` to add additional headers
async fn with_headers() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (headers, "foo")
}

// Or an array of tuples to more easily build the headers
async fn with_array_headers() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain")], "foo")
}

// Use string keys for custom headers
async fn with_array_headers_custom() -> impl IntoResponse {
    ([("x-custom", "custom")], "foo")
}

// `(StatusCode, headers, impl IntoResponse)` to set status and add headers
// `headers` can be either a `HeaderMap` or an array of tuples
async fn with_status_and_array_headers() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/plain")],
        "foo",
    )
}

// `(Extension<_>, impl IntoResponse)` to set response extensions
async fn with_status_extensions() -> impl IntoResponse {
    (
        Extension(Foo("foo")),
        "foo",
    )
}

#[derive(Clone)]
struct Foo(&'static str);

// Or mix and match all the things
async fn all_the_things(uri: Uri) -> impl IntoResponse {
    let mut header_map = HeaderMap::new();
    if uri.path() == "/" {
        header_map.insert(header::SERVER, "axum".parse().unwrap());
    }

    (
        // set status code
        StatusCode::NOT_FOUND,
        // headers with an array
        [("x-custom", "custom")],
        // some extensions
        Extension(Foo("foo")),
        Extension(Foo("bar")),
        // more headers, built dynamically
        header_map,
        // and finally the body
        "foo",
    )
}
```

In general you can return tuples like:

  * `(StatusCode, impl IntoResponse)`
  * `(Parts, impl IntoResponse)`
  * `(Response<()>, impl IntoResponse)`
  * `(T1, .., Tn, impl IntoResponse)` where `T1` to `Tn` all implement [`IntoResponseParts`][5].
  * `(StatusCode, T1, .., Tn, impl IntoResponse)` where `T1` to `Tn` all implement [`IntoResponseParts`][5].
  * `(Parts, T1, .., Tn, impl IntoResponse)` where `T1` to `Tn` all implement [`IntoResponseParts`][5].
  * `(Response<()>, T1, .., Tn, impl IntoResponse)` where `T1` to `Tn` all implement [`IntoResponseParts`][5].



This means you cannot accidentally override the status or body as [`IntoResponseParts`][5] only allows setting headers and extensions.

Use [`Response`][6] for more low level control:
``` 
use axum::{
    Json,
    response::{IntoResponse, Response},
    body::Body,
    http::StatusCode,
};

async fn response() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("x-foo", "custom header")
        .body(Body::from("not found"))
        .unwrap()
}
```

## §Returning different response types

If you need to return multiple response types, and `Result<T, E>` isn’t appropriate, you can call `.into_response()` to turn things into `axum::response::Response`:
``` 
use axum::{
    response::{IntoResponse, Redirect, Response},
    http::StatusCode,
};

async fn handle() -> Response {
    if something() {
        "All good!".into_response()
    } else if something_else() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong...",
        ).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

fn something() -> bool {
    // ...
}

fn something_else() -> bool {
    // ...
}
```

## §Regarding `impl IntoResponse`

You can use `impl IntoResponse` as the return type from handlers to avoid typing large types. For example
``` 
use axum::http::StatusCode;

async fn handler() -> (StatusCode, [(&'static str, &'static str); 1], &'static str) {
    (StatusCode::OK, [("x-foo", "bar")], "Hello, World!")
}
```

Becomes easier using `impl IntoResponse`:
``` 
use axum::{http::StatusCode, response::IntoResponse};

async fn impl_into_response() -> impl IntoResponse {
    (StatusCode::OK, [("x-foo", "bar")], "Hello, World!")
}
```

However `impl IntoResponse` has a few limitations. Firstly it can only be used to return a single type:

ⓘ
```
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> impl IntoResponse {
    if check_something() {
        StatusCode::NOT_FOUND
    } else {
        "Hello, World!"
    }
}

fn check_something() -> bool {
    // ...
}
```

This function returns either a `StatusCode` or a `&'static str` which `impl Trait` doesn’t allow.

Secondly `impl IntoResponse` can lead to type inference issues when used with `Result` and `?`:

ⓘ
```
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> impl IntoResponse {
    create_thing()?;
    Ok(StatusCode::CREATED)
}

fn create_thing() -> Result<(), StatusCode> {
    // ...
}
```

This is because `?` supports using the [`From`][7] trait to convert to a different error type but it doesn’t know which type to convert to, because we only specified `impl IntoResponse` as the return type.

`Result<impl IntoResponse, impl IntoResponse>` doesn’t always work either:

ⓘ
```
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> Result<impl IntoResponse, impl IntoResponse> {
    create_thing()?;
    Ok(StatusCode::CREATED)
}

fn create_thing() -> Result<(), StatusCode> {
    // ...
}
```

The solution is to use a concrete error type, such as `Result<impl IntoResponse, StatusCode>`:
``` 
use axum::{http::StatusCode, response::IntoResponse};

async fn handler() -> Result<impl IntoResponse, StatusCode> {
    create_thing()?;
    Ok(StatusCode::CREATED)
}

fn create_thing() -> Result<(), StatusCode> {
    // ...
}
```

Because of this it is generally not recommended to use `impl IntoResponse` unless you’re familiar with the details of how `impl Trait` works.

## Re-exports§

`pub use crate::[Json][8];``json`
`pub use crate::form::[Form][9];``form`
`pub use crate::[Extension][10];`

## Modules§

[sse][11]
    Server-Sent Events (SSE) responses.

## Structs§

[AppendHeaders][12]
    Append headers to a response.
[ErrorResponse][13]
    An [`IntoResponse`][4]-based error type
[Html][14]
    An HTML response.
[IntoResponseFailed][15]
    Response part that stops status code overrides.
[NoContent][16]
    An empty response with 204 No Content status.
[Redirect][17]
    Response that redirects the request to another location.
[ResponseParts][18]
    Parts of a response.
[Sse][19]
    An SSE response

## Traits§

[IntoResponse][4]
    Trait for generating responses.
[IntoResponseParts][5]
    Trait for adding headers and extensions to a response.

## Type Aliases§

[Response][6]
    Type alias for [`http::Response`] whose body type defaults to [`Body`][20], the most common body type used with axum.
[Result][21]
    An [`IntoResponse`][4]-based result type that uses [`ErrorResponse`][13] as the error type.

   [1]: ../../axum/index.html
   [2]: ../index.html
   [3]: ../../src/axum/response/mod.rs.html#1-545
   [4]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [5]: trait.IntoResponseParts.html (trait axum::response::IntoResponseParts)
   [6]: type.Response.html (type axum::response::Response)
   [7]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [8]: ../struct.Json.html (struct axum::Json)
   [9]: ../struct.Form.html (struct axum::Form)
   [10]: ../struct.Extension.html (struct axum::Extension)
   [11]: sse/index.html (mod axum::response::sse)
   [12]: struct.AppendHeaders.html (struct axum::response::AppendHeaders)
   [13]: struct.ErrorResponse.html (struct axum::response::ErrorResponse)
   [14]: struct.Html.html (struct axum::response::Html)
   [15]: struct.IntoResponseFailed.html (struct axum::response::IntoResponseFailed)
   [16]: struct.NoContent.html (struct axum::response::NoContent)
   [17]: struct.Redirect.html (struct axum::response::Redirect)
   [18]: struct.ResponseParts.html (struct axum::response::ResponseParts)
   [19]: struct.Sse.html (struct axum::response::Sse)
   [20]: ../body/struct.Body.html (struct axum::body::Body)
   [21]: type.Result.html (type axum::response::Result)

