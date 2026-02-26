<!-- Generated from rustdoc HTML: trait.RequestExt.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RequestExt

## [axum][1]0.8.8

## RequestExt

### Required Methods

  * extract
  * extract_parts
  * extract_parts_with_state
  * extract_with_state
  * into_limited_body
  * with_limited_body



### Implementations on Foreign Types

  * Request<Body>



### Dyn Compatibility

### Implementors

## [In crate axum][2]

[axum][2]

# Trait RequestExt Copy item path
```
pub trait RequestExt: [Sized][3] + Sealed {
    // Required methods
    fn extract<E, M>(
        self,
    ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequest][6]<[()][7], M>>::[Rejection][8]>> + [Send][9]
       where E: [FromRequest][6]<[()][7], M> + 'static,
             M: 'static;
    fn extract_with_state<E, S, M>(
        self,
        state: [&S][10],
    ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequest][6]<S, M>>::[Rejection][8]>> + [Send][9]
       where E: [FromRequest][6]<S, M> + 'static,
             S: [Send][9] + [Sync][11];
    fn extract_parts<E>(
        &mut self,
    ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][12]<[()][7]>>::[Rejection][13]>> + [Send][9]
       where E: [FromRequestParts][12]<[()][7]> + 'static;
    fn extract_parts_with_state<'a, E, S>(
        &'a mut self,
        state: [&'a S][10],
    ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][12]<S>>::[Rejection][13]>> + [Send][9] + 'a
       where E: [FromRequestParts][12]<S> + 'static,
             S: [Send][9] + [Sync][11];
    fn with_limited_body(self) -> Request<[Body][14]>;
    fn into_limited_body(self) -> [Body][14];
}
```

Expand description

Extension trait that adds additional methods to [`Request`][15].

## Required Methods§

#### fn extract<E, M>( self, ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequest][6]<[()][7], M>>::[Rejection][8]>> \+ [Send][9]

where E: [FromRequest][6]<[()][7], M> \+ 'static, M: 'static,

Apply an extractor to this `Request`.

This is just a convenience for `E::from_request(req, &())`.

Note this consumes the request. Use [`RequestExt::extract_parts`][16] if you’re not extracting the body and don’t want to consume the request.

##### §Example
``` 
use axum::{
    extract::{Request, FromRequest},
    body::Body,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    Form, Json, RequestExt,
};

struct FormOrJson<T>(T);

impl<S, T> FromRequest<S> for FormOrJson<T>
where
    Json<T>: FromRequest<()>,
    Form<T>: FromRequest<()>,
    T: 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| StatusCode::BAD_REQUEST.into_response())?;

        if content_type.starts_with("application/json") {
            let Json(payload) = req
                .extract::<Json<T>, _>()
                .await
                .map_err(|err| err.into_response())?;

            Ok(Self(payload))
        } else if content_type.starts_with("application/x-www-form-urlencoded") {
            let Form(payload) = req
                .extract::<Form<T>, _>()
                .await
                .map_err(|err| err.into_response())?;

            Ok(Self(payload))
        } else {
            Err(StatusCode::BAD_REQUEST.into_response())
        }
    }
}
```

#### fn extract_with_state<E, S, M>( self, state: [&S][10], ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequest][6]<S, M>>::[Rejection][8]>> \+ [Send][9]

where E: [FromRequest][6]<S, M> \+ 'static, S: [Send][9] \+ [Sync][11],

Apply an extractor that requires some state to this `Request`.

This is just a convenience for `E::from_request(req, state)`.

Note this consumes the request. Use [`RequestExt::extract_parts_with_state`][17] if you’re not extracting the body and don’t want to consume the request.

##### §Example
``` 
use axum::{
    body::Body,
    extract::{Request, FromRef, FromRequest},
    RequestExt,
};

struct MyExtractor {
    requires_state: RequiresState,
}

impl<S> FromRequest<S> for MyExtractor
where
    String: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let requires_state = req.extract_with_state::<RequiresState, _, _>(state).await?;

        Ok(Self { requires_state })
    }
}

// some extractor that consumes the request body and requires state
struct RequiresState { /* ... */ }

impl<S> FromRequest<S> for RequiresState
where
    String: FromRef<S>,
    S: Send + Sync,
{
    // ...
}
```

#### fn extract_parts<E>( &mut self, ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][12]<[()][7]>>::[Rejection][13]>> \+ [Send][9]

where E: [FromRequestParts][12]<[()][7]> \+ 'static,

Apply a parts extractor to this `Request`.

This is just a convenience for `E::from_request_parts(parts, state)`.

##### §Example
``` 
use axum::{
    extract::{Path, Request, FromRequest},
    response::{IntoResponse, Response},
    body::Body,
    Json, RequestExt,
};
use axum_extra::{
    TypedHeader,
    headers::{authorization::Bearer, Authorization},
};
use std::collections::HashMap;

struct MyExtractor<T> {
    path_params: HashMap<String, String>,
    payload: T,
}

impl<S, T> FromRequest<S> for MyExtractor<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    T: 'static,
{
    type Rejection = Response;

    async fn from_request(mut req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let path_params = req
            .extract_parts::<Path<_>>()
            .await
            .map(|Path(path_params)| path_params)
            .map_err(|err| err.into_response())?;

        let Json(payload) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|err| err.into_response())?;

        Ok(Self { path_params, payload })
    }
}
```

#### fn extract_parts_with_state<'a, E, S>( &'a mut self, state: [&'a S][10], ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][12]<S>>::[Rejection][13]>> \+ [Send][9] \+ 'a

where E: [FromRequestParts][12]<S> \+ 'static, S: [Send][9] \+ [Sync][11],

Apply a parts extractor that requires some state to this `Request`.

This is just a convenience for `E::from_request_parts(parts, state)`.

##### §Example
``` 
use axum::{
    extract::{Request, FromRef, FromRequest, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
    body::Body,
    Json, RequestExt,
};

struct MyExtractor<T> {
    requires_state: RequiresState,
    payload: T,
}

impl<S, T> FromRequest<S> for MyExtractor<T>
where
    String: FromRef<S>,
    Json<T>: FromRequest<()>,
    T: 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(mut req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let requires_state = req
            .extract_parts_with_state::<RequiresState, _>(state)
            .await
            .map_err(|err| err.into_response())?;

        let Json(payload) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|err| err.into_response())?;

        Ok(Self {
            requires_state,
            payload,
        })
    }
}

struct RequiresState {}

impl<S> FromRequestParts<S> for RequiresState
where
    String: FromRef<S>,
    S: Send + Sync,
{
    // ...
}
```

#### fn with_limited_body(self) -> Request<[Body][14]>

Apply the [default body limit][18].

If it is disabled, the request is returned as-is.

#### fn into_limited_body(self) -> [Body][14]

Consumes the request, returning the body wrapped in [`http_body_util::Limited`] if a [default limit][18] is in place, or not wrapped if the default limit is disabled.

## Dyn Compatibility§

This trait is **not** [dyn compatible][19].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementations on Foreign Types§

§

### impl [RequestExt][20] for Request<[Body][14]>

§

#### fn extract<E, M>( self, ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequest][6]<[()][7], M>>::[Rejection][8]>> \+ [Send][9]

where E: [FromRequest][6]<[()][7], M> \+ 'static, M: 'static,

§

#### fn extract_with_state<E, S, M>( self, state: [&S][10], ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequest][6]<S, M>>::[Rejection][8]>> \+ [Send][9]

where E: [FromRequest][6]<S, M> \+ 'static, S: [Send][9] \+ [Sync][11],

§

#### fn extract_parts<E>( &mut self, ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][12]<[()][7]>>::[Rejection][13]>> \+ [Send][9]

where E: [FromRequestParts][12]<[()][7]> \+ 'static,

§

#### async fn extract_parts_with_state<'a, E, S>( &'a mut self, state: [&'a S][10], ) -> [Result][5]<E, <E as [FromRequestParts][12]<S>>::[Rejection][13]>

where E: [FromRequestParts][12]<S> \+ 'static, S: [Send][9] \+ [Sync][11],

§

#### fn with_limited_body(self) -> Request<[Body][14]>

§

#### fn into_limited_body(self) -> [Body][14]

## Implementors§

   [1]: ../axum/index.html
   [2]: index.html
   [3]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [4]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [5]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [6]: extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [7]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [8]: extract/trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [9]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [10]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [12]: extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [13]: extract/trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [14]: body/struct.Body.html (struct axum::body::Body)
   [15]: extract/type.Request.html (type axum::extract::Request)
   [16]: trait.RequestExt.html#tymethod.extract_parts (method axum::RequestExt::extract_parts)
   [17]: trait.RequestExt.html#tymethod.extract_parts_with_state (method axum::RequestExt::extract_parts_with_state)
   [18]: extract/struct.DefaultBodyLimit.html (struct axum::extract::DefaultBodyLimit)
   [19]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [20]: trait.RequestExt.html (trait axum::RequestExt)

