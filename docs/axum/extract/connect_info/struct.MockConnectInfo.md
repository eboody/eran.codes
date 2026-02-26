<!-- Generated from rustdoc HTML: extract/connect_info/struct.MockConnectInfo.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MockConnectInfo

## [axum][1]0.8.8

## MockConnectInfo

### Sections

  * Example



### Tuple Fields

  * 0



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



## [In axum::extract::connect_info][2]

[axum][3]::[extract][4]::[connect_info][2]

# Struct MockConnectInfo Copy item path

[Source][5]
``` 
pub struct MockConnectInfo<T>(pub T);
```

Available on **crate feature`tokio`** only.

Expand description

Middleware used to mock [`ConnectInfo`][6] during tests.

If you’re accidentally using [`MockConnectInfo`][7] and [`Router::into_make_service_with_connect_info`][8] at the same time then [`Router::into_make_service_with_connect_info`][8] takes precedence.

## §Example
``` 
use axum::{
    Router,
    extract::connect_info::{MockConnectInfo, ConnectInfo},
    body::Body,
    routing::get,
    http::{Request, StatusCode},
};
use std::net::SocketAddr;
use tower::ServiceExt;

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) {}

// this router you can run with `app.into_make_service_with_connect_info::<SocketAddr>()`
fn app() -> Router {
    Router::new().route("/", get(handler))
}

// use this router for tests
fn test_app() -> Router {
    app().layer(MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 1337))))
}

// #[tokio::test]
async fn some_test() {
    let app = test_app();

    let request = Request::new(Body::empty());
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][9]§

### impl<T: [Clone][10]> [Clone][10] for [MockConnectInfo][7]<T>

[Source][9]§

#### fn [clone][11](&self) -> [MockConnectInfo][7]<T>

Returns a duplicate of the value. [Read more][11]

1.0.0 · [Source][12]§

#### fn [clone_from][13](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][13]

[Source][9]§

### impl<T: [Debug][14]> [Debug][14] for [MockConnectInfo][7]<T>

[Source][9]§

#### fn [fmt][15](&self, f: &mut [Formatter][16]<'_>) -> [Result][17]

Formats the value using the given formatter. [Read more][15]

[Source][18]§

### impl<S, T> Layer<S> for [MockConnectInfo][7]<T>

where T: [Clone][10] \+ [Send][19] \+ [Sync][20] \+ 'static,

[Source][21]§

#### type Service = <[Extension][22]<[MockConnectInfo][7]<T>> as Layer<S>>::Service

The wrapped service

[Source][23]§

#### fn layer(&self, inner: S) -> Self::Service

Wrap the given service with the middleware, returning a new service that has been decorated with the middleware.

[Source][9]§

### impl<T: [Copy][24]> [Copy][24] for [MockConnectInfo][7]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][25] for [MockConnectInfo][7]<T>

where T: [Freeze][25],

§

### impl<T> [RefUnwindSafe][26] for [MockConnectInfo][7]<T>

where T: [RefUnwindSafe][26],

§

### impl<T> [Send][19] for [MockConnectInfo][7]<T>

where T: [Send][19],

§

### impl<T> [Sync][20] for [MockConnectInfo][7]<T>

where T: [Sync][20],

§

### impl<T> [Unpin][27] for [MockConnectInfo][7]<T>

where T: [Unpin][27],

§

### impl<T> [UnwindSafe][28] for [MockConnectInfo][7]<T>

where T: [UnwindSafe][28],

## Blanket Implementations§

[Source][29]§

### impl<T> [Any][30] for T

where T: 'static + ?[Sized][31],

[Source][32]§

#### fn [type_id][33](&self) -> [TypeId][34]

Gets the `TypeId` of `self`. [Read more][33]

[Source][35]§

### impl<T> [Borrow][36]<T> for T

where T: ?[Sized][31],

[Source][37]§

#### fn [borrow][38](&self) -> [&T][39]

Immutably borrows from an owned value. [Read more][38]

[Source][40]§

### impl<T> [BorrowMut][41]<T> for T

where T: ?[Sized][31],

[Source][42]§

#### fn [borrow_mut][43](&mut self) -> [&mut T][39]

Mutably borrows from an owned value. [Read more][43]

[Source][44]§

### impl<T> [CloneToUninit][45] for T

where T: [Clone][10],

[Source][46]§

#### unsafe fn [clone_to_uninit][47](&self, dest: [*mut ][48][u8][49])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][47]

[Source][50]§

### impl<T> [From][51]<T> for T

[Source][52]§

#### fn [from][53](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][54]<T> for T

where T: [Clone][10],

§

#### fn [from_ref][55](input: [&T][39]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][56] [`Span`][57], returning an `Instrumented` wrapper. Read more

[Source][58]§

### impl<T, U> [Into][59]<U> for T

where U: [From][51]<T>,

[Source][60]§

#### fn [into][61](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][51]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][31],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][62]§

### impl<T> [Same][63] for T

[Source][64]§

#### type [Output][65] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][31],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][31],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][66]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][31],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][31],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][31],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][31],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][31],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][31],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][31],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][67] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][31], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][31], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][31],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][31],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][31],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][70]) -> RequestBodyLimit<Self>

where Self: [Sized][31],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][31],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][31],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][71]§

### impl<T> [ToOwned][72] for T

where T: [Clone][10],

[Source][73]§

#### type [Owned][74] = T

The resulting type after obtaining ownership.

[Source][75]§

#### fn [to_owned][76](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][76]

[Source][77]§

#### fn [clone_into][78](&self, target: [&mut T][39])

Uses borrowed data to replace owned data, usually by cloning. [Read more][78]

[Source][79]§

### impl<T, U> [TryFrom][80]<U> for T

where U: [Into][59]<T>,

[Source][81]§

#### type [Error][82] = [Infallible][83]

The type returned in the event of a conversion error.

[Source][84]§

#### fn [try_from][85](value: U) -> [Result][86]<T, <T as [TryFrom][80]<U>>::[Error][87]>

Performs the conversion.

[Source][88]§

### impl<T, U> [TryInto][89]<U> for T

where U: [TryFrom][80]<T>,

[Source][90]§

#### type [Error][91] = <U as [TryFrom][80]<T>>::[Error][87]

The type returned in the event of a conversion error.

[Source][92]§

#### fn [try_into][93](self) -> [Result][86]<U, <U as [TryFrom][80]<T>>::[Error][87]>

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

where S: [Into][59]<Dispatch>,

Attaches the provided [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][95] [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/connect_info.rs.html#206
   [6]: ../struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [7]: struct.MockConnectInfo.html (struct axum::extract::connect_info::MockConnectInfo)
   [8]: ../../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [9]: ../../../src/axum/extract/connect_info.rs.html#205
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [12]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [13]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [16]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [18]: ../../../src/axum/extract/connect_info.rs.html#208-217
   [19]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [20]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [21]: ../../../src/axum/extract/connect_info.rs.html#212
   [22]: ../../struct.Extension.html (struct axum::Extension)
   [23]: ../../../src/axum/extract/connect_info.rs.html#214-216
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [26]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [28]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [29]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [30]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [32]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [33]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [34]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [35]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [36]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [37]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [38]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [39]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [44]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [45]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [46]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [47]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [48]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [49]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [50]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [51]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [54]: ../trait.FromRef.html (trait axum::extract::FromRef)
   [55]: ../trait.FromRef.html#tymethod.from_ref
   [56]: super::Span::current()
   [57]: crate::Span
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [62]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [63]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [64]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [65]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [66]: https://docs.rs/http/latest/http/struct.Extensions.html
   [67]: crate::follow_redirect::policy::Standard
   [68]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [69]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [70]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [71]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [72]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [73]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [74]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [75]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [76]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [77]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [78]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [83]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [84]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [86]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [94]: super::Subscriber
   [95]: dispatcher#setting-the-default-subscriber

