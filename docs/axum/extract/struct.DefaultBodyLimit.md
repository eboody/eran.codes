<!-- Generated from rustdoc HTML: extract/struct.DefaultBodyLimit.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## DefaultBodyLimit

## [axum][1]0.8.8

## DefaultBodyLimit

### Sections

  * Difference between `DefaultBodyLimit` and `RequestBodyLimit`
  * Example
  * Different limits for different routes



### Methods

  * apply
  * disable
  * max



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * Layer<S>



### Auto Trait Implementations

  * Freeze
  * RefUnwindSafe
  * Send
  * Sync
  * Unpin
  * UnwindSafe



### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * CloneToUninit
  * From<T>
  * FromRef<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct DefaultBodyLimit Copy item path
```
pub struct DefaultBodyLimit { /* private fields */ }
```

Expand description

Layer for configuring the default request body limit.

For security reasons, [`Bytes`][4] will, by default, not accept bodies larger than 2MB. This also applies to extractors that uses [`Bytes`][4] internally such as `String`, [`Json`][5], and [`Form`][6].

This middleware provides ways to configure that.

Note that if an extractor consumes the body directly with [`Body::poll_frame`][7], or similar, the default limit is _not_ applied.

## §Difference between `DefaultBodyLimit` and [`RequestBodyLimit`][8]

`DefaultBodyLimit` and [`RequestBodyLimit`][8] serve similar functions but in different ways.

`DefaultBodyLimit` is local in that it only applies to [`FromRequest`][9] implementations that explicitly apply it (or call another extractor that does). You can apply the limit with [`RequestExt::with_limited_body`][10] or [`RequestExt::into_limited_body`][11]

[`RequestBodyLimit`][8] is applied globally to all requests, regardless of which extractors are used or how the body is consumed.

## §Example
``` 
use axum::{
    Router,
    routing::post,
    body::Body,
    extract::{Request, DefaultBodyLimit},
};

let app = Router::new()
    .route("/", post(|request: Request| async {}))
    // change the default limit
    .layer(DefaultBodyLimit::max(1024));
```

In general using `DefaultBodyLimit` is recommended but if you need to use third party extractors and want to make sure a limit is also applied there then [`RequestBodyLimit`][8] should be used.

## §Different limits for different routes

`DefaultBodyLimit` can also be selectively applied to have different limits for different routes:
``` 
use axum::{
    Router,
    routing::post,
    body::Body,
    extract::{Request, DefaultBodyLimit},
};

let app = Router::new()
    // this route has a different limit
    .route("/", post(|request: Request| async {}).layer(DefaultBodyLimit::max(1024)))
    // this route still has the default limit
    .route("/foo", post(|request: Request| async {}));
```

## Implementations§

§

### impl [DefaultBodyLimit][12]

#### pub const fn disable() -> [DefaultBodyLimit][12]

Disable the default request body limit.

This must be used to receive bodies larger than the default limit of 2MB using [`Bytes`][4] or an extractor built on it such as `String`, [`Json`][5], [`Form`][6].

Note that if you’re accepting data from untrusted remotes it is recommend to add your own limit such as [`tower_http::limit`].

##### §Example
``` 
use axum::{
    Router,
    routing::get,
    body::{Bytes, Body},
    extract::DefaultBodyLimit,
};
use tower_http::limit::RequestBodyLimitLayer;

let app: Router<()> = Router::new()
    .route("/", get(|body: Bytes| async {}))
    // Disable the default limit
    .layer(DefaultBodyLimit::disable())
    // Set a different limit
    .layer(RequestBodyLimitLayer::new(10 * 1000 * 1000));
```

#### pub const fn max(limit: [usize][13]) -> [DefaultBodyLimit][12]

Set the default request body limit.

By default the limit of request body sizes that [`Bytes::from_request`][4] (and other extractors built on top of it such as `String`, [`Json`][5], and [`Form`][6]) is 2MB. This method can be used to change that limit.

##### §Example
``` 
use axum::{
    Router,
    routing::get,
    body::{Bytes, Body},
    extract::DefaultBodyLimit,
};

let app: Router<()> = Router::new()
    .route("/", get(|body: Bytes| async {}))
    // Replace the default of 2MB with 1024 bytes.
    .layer(DefaultBodyLimit::max(1024));
```

#### pub fn apply<B>(self, req: &mut Request<B>)

Apply a request body limit to the given request.

This can be used, for example, to modify the default body limit inside a specific extractor.

##### §Example

An extractor similar to [`Bytes`][4], but limiting the body to 1 KB.
``` 
use axum::{
    extract::{DefaultBodyLimit, FromRequest, rejection::BytesRejection, Request},
    body::Bytes,
};

struct Bytes1KB(Bytes);

impl<S: Sync> FromRequest<S> for Bytes1KB {
    type Rejection = BytesRejection;

    async fn from_request(mut req: Request, _: &S) -> Result<Self, Self::Rejection> {
        DefaultBodyLimit::max(1024).apply(&mut req);
        Ok(Self(Bytes::from_request(req, &()).await?))
    }
}
```

## Trait Implementations§

§

### impl [Clone][14] for [DefaultBodyLimit][12]

§

#### fn [clone][15](&self) -> [DefaultBodyLimit][12]

Returns a duplicate of the value. [Read more][15]

1.0.0 · [Source][16]§

#### fn [clone_from][17](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][17]

§

### impl [Debug][18] for [DefaultBodyLimit][12]

§

#### fn [fmt][19](&self, f: &mut [Formatter][20]<'_>) -> [Result][21]<[()][22], [Error][23]>

Formats the value using the given formatter. [Read more][19]

§

### impl<S> Layer<S> for [DefaultBodyLimit][12]

§

#### type Service = DefaultBodyLimitService<S>

The wrapped service

§

#### fn layer(&self, inner: S) -> <[DefaultBodyLimit][12] as Layer<S>>::Service

Wrap the given service with the middleware, returning a new service that has been decorated with the middleware.

§

### impl [Copy][24] for [DefaultBodyLimit][12]

## Auto Trait Implementations§

§

### impl [Freeze][25] for [DefaultBodyLimit][12]

§

### impl [RefUnwindSafe][26] for [DefaultBodyLimit][12]

§

### impl [Send][27] for [DefaultBodyLimit][12]

§

### impl [Sync][28] for [DefaultBodyLimit][12]

§

### impl [Unpin][29] for [DefaultBodyLimit][12]

§

### impl [UnwindSafe][30] for [DefaultBodyLimit][12]

## Blanket Implementations§

[Source][31]§

### impl<T> [Any][32] for T

where T: 'static + ?[Sized][33],

[Source][34]§

#### fn [type_id][35](&self) -> [TypeId][36]

Gets the `TypeId` of `self`. [Read more][35]

[Source][37]§

### impl<T> [Borrow][38]<T> for T

where T: ?[Sized][33],

[Source][39]§

#### fn [borrow][40](&self) -> [&T][41]

Immutably borrows from an owned value. [Read more][40]

[Source][42]§

### impl<T> [BorrowMut][43]<T> for T

where T: ?[Sized][33],

[Source][44]§

#### fn [borrow_mut][45](&mut self) -> [&mut T][41]

Mutably borrows from an owned value. [Read more][45]

[Source][46]§

### impl<T> [CloneToUninit][47] for T

where T: [Clone][14],

[Source][48]§

#### unsafe fn [clone_to_uninit][49](&self, dest: [*mut ][50][u8][51])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][49]

[Source][52]§

### impl<T> [From][53]<T> for T

[Source][54]§

#### fn [from][55](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][56]<T> for T

where T: [Clone][14],

§

#### fn [from_ref][57](input: [&T][41]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][58] [`Span`][59], returning an `Instrumented` wrapper. Read more

[Source][60]§

### impl<T, U> [Into][61]<U> for T

where U: [From][53]<T>,

[Source][62]§

#### fn [into][63](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][53]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][33],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][64]§

### impl<T> [Same][65] for T

[Source][66]§

#### type [Output][67] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][33],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][33],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][68]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][33],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][33],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][33],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][33],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][33],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][33],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][33],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][69] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][33], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][33], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][33],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][33],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][33],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][13]) -> RequestBodyLimit<Self>

where Self: [Sized][33],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][33],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][33],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][72]§

### impl<T> [ToOwned][73] for T

where T: [Clone][14],

[Source][74]§

#### type [Owned][75] = T

The resulting type after obtaining ownership.

[Source][76]§

#### fn [to_owned][77](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][77]

[Source][78]§

#### fn [clone_into][79](&self, target: [&mut T][41])

Uses borrowed data to replace owned data, usually by cloning. [Read more][79]

[Source][80]§

### impl<T, U> [TryFrom][81]<U> for T

where U: [Into][61]<T>,

[Source][82]§

#### type [Error][83] = [Infallible][84]

The type returned in the event of a conversion error.

[Source][85]§

#### fn [try_from][86](value: U) -> [Result][21]<T, <T as [TryFrom][81]<U>>::[Error][87]>

Performs the conversion.

[Source][88]§

### impl<T, U> [TryInto][89]<U> for T

where U: [TryFrom][81]<T>,

[Source][90]§

#### type [Error][91] = <U as [TryFrom][81]<T>>::[Error][87]

The type returned in the event of a conversion error.

[Source][92]§

#### fn [try_into][93](self) -> [Result][21]<U, <U as [TryFrom][81]<T>>::[Error][87]>

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

where S: [Into][61]<Dispatch>,

Attaches the provided [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][95] [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: bytes::Bytes
   [5]: https://docs.rs/axum/0.8/axum/struct.Json.html
   [6]: https://docs.rs/axum/0.8/axum/struct.Form.html
   [7]: http_body::Body::poll_frame
   [8]: tower_http::limit::RequestBodyLimit
   [9]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [10]: ../trait.RequestExt.html#tymethod.with_limited_body (method axum::RequestExt::with_limited_body)
   [11]: ../trait.RequestExt.html#tymethod.into_limited_body (method axum::RequestExt::into_limited_body)
   [12]: struct.DefaultBodyLimit.html (struct axum::extract::DefaultBodyLimit)
   [13]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [15]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [16]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [17]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [20]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [21]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [22]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [23]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [26]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [30]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [31]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [32]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [34]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [35]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [36]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [37]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [38]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [39]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [40]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [41]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [46]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [47]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [48]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [49]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [50]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [51]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [55]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [56]: trait.FromRef.html (trait axum::extract::FromRef)
   [57]: trait.FromRef.html#tymethod.from_ref
   [58]: super::Span::current()
   [59]: crate::Span
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [64]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [65]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [68]: https://docs.rs/http/latest/http/struct.Extensions.html
   [69]: crate::follow_redirect::policy::Standard
   [70]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [71]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [72]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [73]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [74]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [75]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [76]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [77]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [78]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [79]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [84]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [94]: super::Subscriber
   [95]: dispatcher#setting-the-default-subscriber

