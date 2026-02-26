<!-- Generated from rustdoc HTML: extract/derive.FromRequest.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromRequest

## [axum][1]0.8.8

## FromRequest

### Sections

  * Each field individually
    * Extracting via another extractor
    * Optional fields
    * The rejection
    * Concrete state
  * The whole type at once
  * Known limitations



## [In axum::extract][2]

[axum][3]::[extract][2]

# Derive Macro FromRequest Copy item path
```
#[derive(FromRequest)]
{
    // Attributes available to this derive:
    #[from_request]
}

```

Available on **crate feature`macros`** only.

Expand description

Derive an implementation of [`FromRequest`][4].

Supports generating two kinds of implementations:

  1. One that extracts each field individually.
  2. Another that extracts the whole type at once via another extractor.



## §Each field individually

By default `#[derive(FromRequest)]` will call `FromRequest::from_request` for each field:
``` 
use axum_macros::FromRequest;
use axum::{
    extract::Extension,
    body::Bytes,
};
use axum_extra::{
    TypedHeader,
    headers::ContentType,
};

#[derive(FromRequest)]
struct MyExtractor {
    state: Extension<State>,
    content_type: TypedHeader<ContentType>,
    request_body: Bytes,
}

#[derive(Clone)]
struct State {
    // ...
}

async fn handler(extractor: MyExtractor) {}
```

This requires that each field is an extractor (i.e. implements [`FromRequest`][4]).

Note that only the last field can consume the request body. Therefore this doesn’t compile:

ⓘ
```
use axum_macros::FromRequest;
use axum::body::Bytes;

#[derive(FromRequest)]
struct MyExtractor {
    // only the last field can implement `FromRequest`
    // other fields must only implement `FromRequestParts`
    bytes: Bytes,
    string: String,
}
```

### §Extracting via another extractor

You can use `#[from_request(via(...))]` to extract a field via another extractor, meaning the field itself doesn’t need to implement `FromRequest`:
``` 
use axum_macros::FromRequest;
use axum::{
    extract::Extension,
    body::Bytes,
};
use axum_extra::{
    TypedHeader,
    headers::ContentType,
};

#[derive(FromRequest)]
struct MyExtractor {
    // This will extracted via `Extension::<State>::from_request`
    #[from_request(via(Extension))]
    state: State,
    // and this via `TypedHeader::<ContentType>::from_request`
    #[from_request(via(TypedHeader))]
    content_type: ContentType,
    // Can still be combined with other extractors
    request_body: Bytes,
}

#[derive(Clone)]
struct State {
    // ...
}

async fn handler(extractor: MyExtractor) {}
```

Note this requires the via extractor to be a generic newtype struct (a tuple struct with exactly one public field) that implements `FromRequest`:
``` 
pub struct ViaExtractor<T>(pub T);

// impl<T, S> FromRequest<S> for ViaExtractor<T> { ... }
```

More complex via extractors are not supported and require writing a manual implementation.

### §Optional fields

`#[from_request(via(...))]` supports `Option<_>` and `Result<_, _>` to make fields optional:
``` 
use axum_macros::FromRequest;
use axum_extra::{
    TypedHeader,
    headers::{ContentType, UserAgent},
    typed_header::TypedHeaderRejection,
};

#[derive(FromRequest)]
struct MyExtractor {
    // This will extracted via `Option::<TypedHeader<ContentType>>::from_request`
    #[from_request(via(TypedHeader))]
    content_type: Option<ContentType>,
    // This will extracted via
    // `Result::<TypedHeader<UserAgent>, TypedHeaderRejection>::from_request`
    #[from_request(via(TypedHeader))]
    user_agent: Result<UserAgent, TypedHeaderRejection>,
}

async fn handler(extractor: MyExtractor) {}
```

### §The rejection

By default [`axum::response::Response`][5] will be used as the rejection. You can also use your own rejection type with `#[from_request(rejection(YourType))]`:
``` 
use axum::{
    extract::{
        rejection::{ExtensionRejection, StringRejection},
        FromRequest,
    },
    Extension,
    response::{Response, IntoResponse},
};

#[derive(FromRequest)]
#[from_request(rejection(MyRejection))]
struct MyExtractor {
    state: Extension<String>,
    body: String,
}

struct MyRejection(Response);

// This tells axum how to convert `Extension`'s rejections into `MyRejection`
impl From<ExtensionRejection> for MyRejection {
    fn from(rejection: ExtensionRejection) -> Self {
        // ...
    }
}

// This tells axum how to convert `String`'s rejections into `MyRejection`
impl From<StringRejection> for MyRejection {
    fn from(rejection: StringRejection) -> Self {
        // ...
    }
}

// All rejections must implement `IntoResponse`
impl IntoResponse for MyRejection {
    fn into_response(self) -> Response {
        self.0
    }
}
```

### §Concrete state

If the extraction can be done only for a concrete state, that type can be specified with `#[from_request(state(YourState))]`:
``` 
use axum::extract::{FromRequest, FromRequestParts};

#[derive(Clone)]
struct CustomState;

struct MyInnerType;

impl FromRequestParts<CustomState> for MyInnerType {
    // ...

}

#[derive(FromRequest)]
#[from_request(state(CustomState))]
struct MyExtractor {
    custom: MyInnerType,
    body: String,
}
```

This is not needed for a `State<T>` as the type is inferred in that case.
``` 
use axum::extract::{FromRequest, FromRequestParts, State};

#[derive(Clone)]
struct CustomState;

#[derive(FromRequest)]
struct MyExtractor {
    custom: State<CustomState>,
    body: String,
}
```

## §The whole type at once

By using `#[from_request(via(...))]` on the container you can extract the whole type at once, instead of each field individually:
``` 
use axum_macros::FromRequest;
use axum::extract::Extension;

// This will extracted via `Extension::<State>::from_request`
#[derive(Clone, FromRequest)]
#[from_request(via(Extension))]
struct State {
    // ...
}

async fn handler(state: State) {}
```

The rejection will be the “via extractors”’s rejection. For the previous example that would be [`axum::extract::rejection::ExtensionRejection`][6].

You can use a different rejection type with `#[from_request(rejection(YourType))]`:
``` 
use axum_macros::FromRequest;
use axum::{
    extract::{Extension, rejection::ExtensionRejection},
    response::{IntoResponse, Response},
    Json,
    http::StatusCode,
};
use serde_json::json;

// This will extracted via `Extension::<State>::from_request`
#[derive(Clone, FromRequest)]
#[from_request(
    via(Extension),
    // Use your own rejection type
    rejection(MyRejection),
)]
struct State {
    // ...
}

struct MyRejection(Response);

// This tells axum how to convert `Extension`'s rejections into `MyRejection`
impl From<ExtensionRejection> for MyRejection {
    fn from(rejection: ExtensionRejection) -> Self {
        let response = (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Something went wrong..." })),
        ).into_response();

        MyRejection(response)
    }
}

// All rejections must implement `IntoResponse`
impl IntoResponse for MyRejection {
    fn into_response(self) -> Response {
        self.0
    }
}

async fn handler(state: State) {}
```

This allows you to wrap other extractors and easily customize the rejection:
``` 
use axum_macros::FromRequest;
use axum::{
    extract::{Extension, rejection::JsonRejection},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde_json::json;
use serde::Deserialize;

// create an extractor that internally uses `axum::Json` but has a custom rejection
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(MyRejection))]
struct MyJson<T>(T);

struct MyRejection(Response);

impl From<JsonRejection> for MyRejection {
    fn from(rejection: JsonRejection) -> Self {
        let response = (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(json!({ "error": rejection.to_string() })),
        ).into_response();

        MyRejection(response)
    }
}

impl IntoResponse for MyRejection {
    fn into_response(self) -> Response {
        self.0
    }
}

#[derive(Deserialize)]
struct Payload {}

async fn handler(
    // make sure to use `MyJson` and not `axum::Json`
    MyJson(payload): MyJson<Payload>,
) {}
```

## §Known limitations

Generics are only supported on tuple structs with exactly one field. Thus this doesn’t work

ⓘ
```
#[derive(axum_macros::FromRequest)]
struct MyExtractor<T> {
    thing: Option<T>,
}
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://docs.rs/axum/0.8/axum/extract/trait.FromRequest.html
   [5]: https://docs.rs/axum/0.8/axum/response/type.Response.html
   [6]: https://docs.rs/axum/0.8/axum/extract/rejection/enum.ExtensionRejection.html

