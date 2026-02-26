<!-- Generated from rustdoc HTML: response/trait.IntoResponse.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IntoResponse

## [axum][1]0.8.8

## IntoResponse

### Sections

  * Implementing `IntoResponse`



### Required Methods

  * into_response



### Implementations on Foreign Types

  * &'static [u8; N]
  * &'static [u8]
  * &'static str
  * ()
  * (ForceStatusCode, R)
  * (ForceStatusCode, T1, R)
  * (ForceStatusCode, T1, T2, R)
  * (ForceStatusCode, T1, T2, T3, R)
  * (ForceStatusCode, T1, T2, T3, T4, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)
  * (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)
  * (Parts, R)
  * (Parts, T1, R)
  * (Parts, T1, T2, R)
  * (Parts, T1, T2, T3, R)
  * (Parts, T1, T2, T3, T4, R)
  * (Parts, T1, T2, T3, T4, T5, R)
  * (Parts, T1, T2, T3, T4, T5, T6, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)
  * (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)
  * (R,)
  * (Response<()>, R)
  * (Response<()>, T1, R)
  * (Response<()>, T1, T2, R)
  * (Response<()>, T1, T2, T3, R)
  * (Response<()>, T1, T2, T3, T4, R)
  * (Response<()>, T1, T2, T3, T4, T5, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)
  * (Response<()>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)
  * (StatusCode, R)
  * (StatusCode, T1, R)
  * (StatusCode, T1, T2, R)
  * (StatusCode, T1, T2, T3, R)
  * (StatusCode, T1, T2, T3, T4, R)
  * (StatusCode, T1, T2, T3, T4, T5, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)
  * (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)
  * (T1, R)
  * (T1, T2, R)
  * (T1, T2, T3, R)
  * (T1, T2, T3, T4, R)
  * (T1, T2, T3, T4, T5, R)
  * (T1, T2, T3, T4, T5, T6, R)
  * (T1, T2, T3, T4, T5, T6, T7, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)
  * (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)
  * Box<[u8]>
  * Box<str>
  * Bytes
  * BytesMut
  * Chain<T, U>
  * Cow<'static, [u8]>
  * Cow<'static, str>
  * Extensions
  * HeaderMap
  * Infallible
  * Parts
  * Response<B>
  * Result<T, E>
  * Result<T, ErrorResponse>
  * StatusCode
  * String
  * Vec<u8>
  * [(K, V); N]
  * [u8; N]



### Implementors

## [In axum::response][2]

[axum][3]::[response][2]

# Trait IntoResponse Copy item path
```
pub trait IntoResponse {
    // Required method
    fn into_response(self) -> Response<[Body][4]>;
}
```

Expand description

Trait for generating responses.

Types that implement `IntoResponse` can be returned from handlers.

## §Implementing `IntoResponse`

You generally shouldn’t have to implement `IntoResponse` manually, as axum provides implementations for many common types.

However it might be necessary if you have a custom error type that you want to return from handlers:
``` 
use axum::{
    Router,
    body::{self, Bytes},
    routing::get,
    http::StatusCode,
    response::{IntoResponse, Response},
};

enum MyError {
    SomethingWentWrong,
    SomethingElseWentWrong,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::SomethingWentWrong => "something went wrong",
            MyError::SomethingElseWentWrong => "something else went wrong",
        };

        // it's often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

// `Result<impl IntoResponse, MyError>` can now be returned from handlers
let app = Router::new().route("/", get(handler));

async fn handler() -> Result<(), MyError> {
    Err(MyError::SomethingWentWrong)
}
```

Or if you have a custom body type you’ll also need to implement `IntoResponse` for it:
``` 
use axum::{
    body,
    routing::get,
    response::{IntoResponse, Response},
    body::Body,
    Router,
};
use http::HeaderMap;
use bytes::Bytes;
use http_body::Frame;
use std::{
    convert::Infallible,
    task::{Poll, Context},
    pin::Pin,
};

struct MyBody;

// First implement `Body` for `MyBody`. This could for example use
// some custom streaming protocol.
impl http_body::Body for MyBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        // ...
    }
}

// Now we can implement `IntoResponse` directly for `MyBody`
impl IntoResponse for MyBody {
    fn into_response(self) -> Response {
        Response::new(Body::new(self))
    }
}

// `MyBody` can now be returned from handlers.
let app = Router::new().route("/", get(|| async { MyBody }));
```

## Required Methods§

#### fn into_response(self) -> Response<[Body][4]>

Create a response.

## Implementations on Foreign Types§

§

### impl [IntoResponse][5] for &'static [str][6]

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for &'static [[u8][7]]

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [Cow][8]<'static, [str][6]>

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [Cow][8]<'static, [[u8][7]]>

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [Infallible][9]

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [()][10]

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [Box][11]<[str][6]>

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [Box][11]<[[u8][7]]>

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [String][12]

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for [Vec][13]<[u8][7]>

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for Bytes

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for BytesMut

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for Extensions

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for HeaderMap

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for Parts

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl [IntoResponse][5] for StatusCode

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<B> [IntoResponse][5] for Response<B>

where B: Body<Data = Bytes> \+ [Send][14] \+ 'static, <B as Body>::Error: [Into][15]<[Box][11]<dyn [Error][16] \+ [Send][14] \+ [Sync][17]>>,

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<K, V, const N: [usize][18]> [IntoResponse][5] for [[(K, V)][19]; [N][20]]

where K: [TryInto][21]<HeaderName>, <K as [TryInto][21]<HeaderName>>::[Error][22]: [Display][23], V: [TryInto][21]<HeaderValue>, <V as [TryInto][21]<HeaderValue>>::[Error][22]: [Display][23],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R> [IntoResponse][5] for (ForceStatusCode, R)

where R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R> [IntoResponse][5] for (Parts, R)

where R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R> [IntoResponse][5] for (Response<[()][10]>, R)

where R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R> [IntoResponse][5] for (StatusCode, R)

where R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R> [IntoResponse][5] for [(R,)][19]

where R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1> [IntoResponse][5] for (ForceStatusCode, T1, R)

where T1: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1> [IntoResponse][5] for (Parts, T1, R)

where T1: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1> [IntoResponse][5] for (Response<[()][10]>, T1, R)

where T1: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1> [IntoResponse][5] for (StatusCode, T1, R)

where T1: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1> [IntoResponse][5] for [(T1, R)][19]

where T1: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2> [IntoResponse][5] for (ForceStatusCode, T1, T2, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2> [IntoResponse][5] for (Parts, T1, T2, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2> [IntoResponse][5] for (Response<[()][10]>, T1, T2, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2> [IntoResponse][5] for (StatusCode, T1, T2, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2> [IntoResponse][5] for [(T1, T2, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3> [IntoResponse][5] for (Parts, T1, T2, T3, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3> [IntoResponse][5] for (StatusCode, T1, T2, T3, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3> [IntoResponse][5] for [(T1, T2, T3, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4> [IntoResponse][5] for (Parts, T1, T2, T3, T4, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4> [IntoResponse][5] for [(T1, T2, T3, T4, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5> [IntoResponse][5] for [(T1, T2, T3, T4, T5, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [IntoResponse][5] for (ForceStatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], T16: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [IntoResponse][5] for (Parts, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], T16: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [IntoResponse][5] for (Response<[()][10]>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], T16: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [IntoResponse][5] for (StatusCode, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], T16: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> [IntoResponse][5] for [(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R)][19]

where T1: [IntoResponseParts][24], T2: [IntoResponseParts][24], T3: [IntoResponseParts][24], T4: [IntoResponseParts][24], T5: [IntoResponseParts][24], T6: [IntoResponseParts][24], T7: [IntoResponseParts][24], T8: [IntoResponseParts][24], T9: [IntoResponseParts][24], T10: [IntoResponseParts][24], T11: [IntoResponseParts][24], T12: [IntoResponseParts][24], T13: [IntoResponseParts][24], T14: [IntoResponseParts][24], T15: [IntoResponseParts][24], T16: [IntoResponseParts][24], R: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<T> [IntoResponse][5] for [Result][25]<T, [ErrorResponse][26]>

where T: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<T, E> [IntoResponse][5] for [Result][25]<T, E>

where T: [IntoResponse][5], E: [IntoResponse][5],

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<T, U> [IntoResponse][5] for Chain<T, U>

where T: Buf + [Unpin][27] \+ [Send][14] \+ 'static, U: Buf + [Unpin][27] \+ [Send][14] \+ 'static,

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<const N: [usize][18]> [IntoResponse][5] for &'static [[u8][7]; [N][20]]

§

#### fn into_response(self) -> Response<[Body][4]>

§

### impl<const N: [usize][18]> [IntoResponse][5] for [[u8][7]; [N][20]]

§

#### fn into_response(self) -> Response<[Body][4]>

## Implementors§

[Source][28]§

### impl [IntoResponse][5] for [MultipartRejection][29]

Available on **crate feature`multipart`** only.

§

### impl [IntoResponse][5] for [BytesRejection][30]

[Source][31]§

### impl [IntoResponse][5] for [ExtensionRejection][32]

§

### impl [IntoResponse][5] for [FailedToBufferBody][33]

[Source][34]§

### impl [IntoResponse][5] for [FormRejection][35]

[Source][36]§

### impl [IntoResponse][5] for [JsonRejection][37]

[Source][38]§

### impl [IntoResponse][5] for [MatchedPathRejection][39]

[Source][40]§

### impl [IntoResponse][5] for [PathRejection][41]

[Source][42]§

### impl [IntoResponse][5] for [QueryRejection][43]

[Source][44]§

### impl [IntoResponse][5] for [RawFormRejection][45]

[Source][46]§

### impl [IntoResponse][5] for [RawPathParamsRejection][47]

§

### impl [IntoResponse][5] for [StringRejection][48]

[Source][49]§

### impl [IntoResponse][5] for [WebSocketUpgradeRejection][50]

Available on **crate feature`ws`** only.

§

### impl [IntoResponse][5] for [Body][4]

[Source][51]§

### impl [IntoResponse][5] for [InvalidBoundary][52]

Available on **crate feature`multipart`** only.

[Source][53]§

### impl [IntoResponse][5] for [MultipartError][54]

Available on **crate feature`multipart`** only.

[Source][55]§

### impl [IntoResponse][5] for [FailedToDeserializePathParams][56]

[Source][57]§

### impl [IntoResponse][5] for [InvalidUtf8InPathParam][58]

[Source][59]§

### impl [IntoResponse][5] for [FailedToDeserializeForm][60]

[Source][61]§

### impl [IntoResponse][5] for [FailedToDeserializeFormBody][62]

[Source][63]§

### impl [IntoResponse][5] for [FailedToDeserializeQueryString][64]

[Source][65]§

### impl [IntoResponse][5] for [InvalidFormContentType][66]

§

### impl [IntoResponse][5] for [InvalidUtf8][67]

[Source][68]§

### impl [IntoResponse][5] for [JsonDataError][69]

[Source][70]§

### impl [IntoResponse][5] for [JsonSyntaxError][71]

§

### impl [IntoResponse][5] for [LengthLimitError][72]

[Source][73]§

### impl [IntoResponse][5] for [MatchedPathMissing][74]

[Source][75]§

### impl [IntoResponse][5] for [MissingExtension][76]

[Source][77]§

### impl [IntoResponse][5] for [MissingJsonContentType][78]

[Source][79]§

### impl [IntoResponse][5] for [MissingPathParams][80]

[Source][81]§

### impl [IntoResponse][5] for [NestedPathRejection][82]

§

### impl [IntoResponse][5] for [UnknownBodyError][83]

[Source][84]§

### impl [IntoResponse][5] for [ConnectionNotUpgradable][85]

Available on **crate feature`ws`** only.

[Source][86]§

### impl [IntoResponse][5] for [InvalidConnectionHeader][87]

Available on **crate feature`ws`** only.

[Source][88]§

### impl [IntoResponse][5] for [InvalidProtocolPseudoheader][89]

Available on **crate feature`ws`** only.

[Source][90]§

### impl [IntoResponse][5] for [InvalidUpgradeHeader][91]

Available on **crate feature`ws`** only.

[Source][92]§

### impl [IntoResponse][5] for [InvalidWebSocketVersionHeader][93]

Available on **crate feature`ws`** only.

[Source][94]§

### impl [IntoResponse][5] for [MethodNotConnect][95]

Available on **crate feature`ws`** only.

[Source][96]§

### impl [IntoResponse][5] for [MethodNotGet][97]

Available on **crate feature`ws`** only.

[Source][98]§

### impl [IntoResponse][5] for [WebSocketKeyHeaderMissing][99]

Available on **crate feature`ws`** only.

[Source][100]§

### impl [IntoResponse][5] for [NoContent][101]

[Source][102]§

### impl [IntoResponse][5] for [Redirect][103]

§

### impl [IntoResponse][5] for ForceStatusCode

§

### impl<I, K, V> [IntoResponse][5] for [AppendHeaders][104]<I>

where I: [IntoIterator][105]<Item = [(K, V)][19]>, K: [TryInto][21]<HeaderName>, <K as [TryInto][21]<HeaderName>>::[Error][22]: [Display][23], V: [TryInto][21]<HeaderValue>, <V as [TryInto][21]<HeaderValue>>::[Error][22]: [Display][23],

§

### impl<K, V> [IntoResponse][5] for TryIntoHeaderError<K, V>

where K: [Display][23], V: [Display][23],

[Source][106]§

### impl<S, E> [IntoResponse][5] for [Sse][107]<S>

where S: Stream<Item = [Result][25]<[Event][108], E>> \+ [Send][14] \+ 'static, E: [Into][15]<[BoxError][109]>,

[Source][110]§

### impl<T> [IntoResponse][5] for [Extension][111]<T>

where T: [Clone][112] \+ [Send][14] \+ [Sync][17] \+ 'static,

[Source][113]§

### impl<T> [IntoResponse][5] for [Form][114]<T>

where T: [Serialize][115],

Available on **crate feature`form`** only.

[Source][116]§

### impl<T> [IntoResponse][5] for [Json][117]<T>

where T: [Serialize][115],

Available on **crate feature`json`** only.

[Source][118]§

### impl<T> [IntoResponse][5] for [Html][119]<T>

where T: [IntoResponse][5],

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../body/struct.Body.html (struct axum::body::Body)
   [5]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [7]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [8]: https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html (enum alloc::borrow::Cow)
   [9]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [10]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [11]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [12]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [13]: https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html (struct alloc::vec::Vec)
   [14]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [15]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [16]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [17]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [18]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [19]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [20]: https://doc.rust-lang.org/nightly/std/primitive.array.html
   [21]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [22]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error (type core::convert::TryInto::Error)
   [23]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [24]: trait.IntoResponseParts.html (trait axum::response::IntoResponseParts)
   [25]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [26]: struct.ErrorResponse.html (struct axum::response::ErrorResponse)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [28]: ../../src/axum/extract/multipart.rs.html#340-347
   [29]: ../extract/multipart/enum.MultipartRejection.html (enum axum::extract::multipart::MultipartRejection)
   [30]: ../extract/rejection/enum.BytesRejection.html (enum axum::extract::rejection::BytesRejection)
   [31]: ../../src/axum/extract/rejection.rs.html#141-149
   [32]: ../extract/rejection/enum.ExtensionRejection.html (enum axum::extract::rejection::ExtensionRejection)
   [33]: ../extract/rejection/enum.FailedToBufferBody.html (enum axum::extract::rejection::FailedToBufferBody)
   [34]: ../../src/axum/extract/rejection.rs.html#102-113
   [35]: ../extract/rejection/enum.FormRejection.html (enum axum::extract::rejection::FormRejection)
   [36]: ../../src/axum/extract/rejection.rs.html#127-139
   [37]: ../extract/rejection/enum.JsonRejection.html (enum axum::extract::rejection::JsonRejection)
   [38]: ../../src/axum/extract/rejection.rs.html#185-191
   [39]: ../extract/rejection/enum.MatchedPathRejection.html (enum axum::extract::rejection::MatchedPathRejection)
   [40]: ../../src/axum/extract/rejection.rs.html#151-160
   [41]: ../extract/rejection/enum.PathRejection.html (enum axum::extract::rejection::PathRejection)
   [42]: ../../src/axum/extract/rejection.rs.html#92-100
   [43]: ../extract/rejection/enum.QueryRejection.html (enum axum::extract::rejection::QueryRejection)
   [44]: ../../src/axum/extract/rejection.rs.html#115-124
   [45]: ../extract/rejection/enum.RawFormRejection.html (enum axum::extract::rejection::RawFormRejection)
   [46]: ../../src/axum/extract/rejection.rs.html#162-171
   [47]: ../extract/rejection/enum.RawPathParamsRejection.html (enum axum::extract::rejection::RawPathParamsRejection)
   [48]: ../extract/rejection/enum.StringRejection.html (enum axum::extract::rejection::StringRejection)
   [49]: ../../src/axum/extract/ws.rs.html#1010-1025
   [50]: ../extract/ws/rejection/enum.WebSocketUpgradeRejection.html (enum axum::extract::ws::rejection::WebSocketUpgradeRejection)
   [51]: ../../src/axum/extract/multipart.rs.html#349-355
   [52]: ../extract/multipart/struct.InvalidBoundary.html (struct axum::extract::multipart::InvalidBoundary)
   [53]: ../../src/axum/extract/multipart.rs.html#324-334
   [54]: ../extract/multipart/struct.MultipartError.html (struct axum::extract::multipart::MultipartError)
   [55]: ../../src/axum/extract/path/mod.rs.html#453-463
   [56]: ../extract/path/struct.FailedToDeserializePathParams.html (struct axum::extract::path::FailedToDeserializePathParams)
   [57]: ../../src/axum/extract/path/mod.rs.html#588-598
   [58]: ../extract/path/struct.InvalidUtf8InPathParam.html (struct axum::extract::path::InvalidUtf8InPathParam)
   [59]: ../../src/axum/extract/rejection.rs.html#68-74
   [60]: ../extract/rejection/struct.FailedToDeserializeForm.html (struct axum::extract::rejection::FailedToDeserializeForm)
   [61]: ../../src/axum/extract/rejection.rs.html#76-82
   [62]: ../extract/rejection/struct.FailedToDeserializeFormBody.html (struct axum::extract::rejection::FailedToDeserializeFormBody)
   [63]: ../../src/axum/extract/rejection.rs.html#84-90
   [64]: ../extract/rejection/struct.FailedToDeserializeQueryString.html (struct axum::extract::rejection::FailedToDeserializeQueryString)
   [65]: ../../src/axum/extract/rejection.rs.html#59-66
   [66]: ../extract/rejection/struct.InvalidFormContentType.html (struct axum::extract::rejection::InvalidFormContentType)
   [67]: ../extract/rejection/struct.InvalidUtf8.html (struct axum::extract::rejection::InvalidUtf8)
   [68]: ../../src/axum/extract/rejection.rs.html#10-19
   [69]: ../extract/rejection/struct.JsonDataError.html (struct axum::extract::rejection::JsonDataError)
   [70]: ../../src/axum/extract/rejection.rs.html#22-30
   [71]: ../extract/rejection/struct.JsonSyntaxError.html (struct axum::extract::rejection::JsonSyntaxError)
   [72]: ../extract/rejection/struct.LengthLimitError.html (struct axum::extract::rejection::LengthLimitError)
   [73]: ../../src/axum/extract/rejection.rs.html#174-182
   [74]: ../extract/rejection/struct.MatchedPathMissing.html (struct axum::extract::rejection::MatchedPathMissing)
   [75]: ../../src/axum/extract/rejection.rs.html#42-48
   [76]: ../extract/rejection/struct.MissingExtension.html (struct axum::extract::rejection::MissingExtension)
   [77]: ../../src/axum/extract/rejection.rs.html#33-40
   [78]: ../extract/rejection/struct.MissingJsonContentType.html (struct axum::extract::rejection::MissingJsonContentType)
   [79]: ../../src/axum/extract/rejection.rs.html#50-57
   [80]: ../extract/rejection/struct.MissingPathParams.html (struct axum::extract::rejection::MissingPathParams)
   [81]: ../../src/axum/extract/rejection.rs.html#193-200
   [82]: ../extract/rejection/struct.NestedPathRejection.html (struct axum::extract::rejection::NestedPathRejection)
   [83]: ../extract/rejection/struct.UnknownBodyError.html (struct axum::extract::rejection::UnknownBodyError)
   [84]: ../../src/axum/extract/ws.rs.html#996-1008
   [85]: ../extract/ws/rejection/struct.ConnectionNotUpgradable.html (struct axum::extract::ws::rejection::ConnectionNotUpgradable)
   [86]: ../../src/axum/extract/ws.rs.html#961-966
   [87]: ../extract/ws/rejection/struct.InvalidConnectionHeader.html (struct axum::extract::ws::rejection::InvalidConnectionHeader)
   [88]: ../../src/axum/extract/ws.rs.html#975-980
   [89]: ../extract/ws/rejection/struct.InvalidProtocolPseudoheader.html (struct axum::extract::ws::rejection::InvalidProtocolPseudoheader)
   [90]: ../../src/axum/extract/ws.rs.html#968-973
   [91]: ../extract/ws/rejection/struct.InvalidUpgradeHeader.html (struct axum::extract::ws::rejection::InvalidUpgradeHeader)
   [92]: ../../src/axum/extract/ws.rs.html#982-987
   [93]: ../extract/ws/rejection/struct.InvalidWebSocketVersionHeader.html (struct axum::extract::ws::rejection::InvalidWebSocketVersionHeader)
   [94]: ../../src/axum/extract/ws.rs.html#954-959
   [95]: ../extract/ws/rejection/struct.MethodNotConnect.html (struct axum::extract::ws::rejection::MethodNotConnect)
   [96]: ../../src/axum/extract/ws.rs.html#947-952
   [97]: ../extract/ws/rejection/struct.MethodNotGet.html (struct axum::extract::ws::rejection::MethodNotGet)
   [98]: ../../src/axum/extract/ws.rs.html#989-994
   [99]: ../extract/ws/rejection/struct.WebSocketKeyHeaderMissing.html (struct axum::extract::ws::rejection::WebSocketKeyHeaderMissing)
   [100]: ../../src/axum/response/mod.rs.html#80-84
   [101]: struct.NoContent.html (struct axum::response::NoContent)
   [102]: ../../src/axum/response/redirect.rs.html#87-94
   [103]: struct.Redirect.html (struct axum::response::Redirect)
   [104]: struct.AppendHeaders.html (struct axum::response::AppendHeaders)
   [105]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [106]: ../../src/axum/response/sse.rs.html#90-107
   [107]: struct.Sse.html (struct axum::response::Sse)
   [108]: sse/struct.Event.html (struct axum::response::sse::Event)
   [109]: ../type.BoxError.html (type axum::BoxError)
   [110]: ../../src/axum/extension.rs.html#129-138
   [111]: ../struct.Extension.html (struct axum::Extension)
   [112]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [113]: ../../src/axum/form.rs.html#107-131
   [114]: ../struct.Form.html (struct axum::Form)
   [115]: https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html (trait serde_core::ser::Serialize)
   [116]: ../../src/axum/json.rs.html#197-232
   [117]: ../struct.Json.html (struct axum::Json)
   [118]: ../../src/axum/response/mod.rs.html#39-53
   [119]: struct.Html.html (struct axum::response::Html)

