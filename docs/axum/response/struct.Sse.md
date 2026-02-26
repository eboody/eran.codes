<!-- Generated from rustdoc HTML: response/struct.Sse.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Sse

## [axum][1]0.8.8

## Sse

### Methods

  * keep_alive
  * new



### Trait Implementations

  * Clone
  * Debug
  * IntoResponse



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
  * HandlerWithoutStateExt<T>
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



## [In axum::response][2]

[axum][3]::[response][2]

# Struct Sse Copy item path

[Source][4]
``` 
pub struct Sse<S> { /* private fields */ }
```

Expand description

An SSE response

## Implementations§

[Source][5]§

### impl<S> [Sse][6]<S>

[Source][7]

#### pub fn new(stream: S) -> Self

where S: TryStream<Ok = [Event][8]> \+ [Send][9] \+ 'static, S::Error: [Into][10]<[BoxError][11]>,

Create a new [`Sse`][6] response that will respond with the given stream of [`Event`][8]s.

See the [module docs][12] for more details.

[Source][13]

#### pub fn keep_alive(self, keep_alive: [KeepAlive][14]) -> [Sse][6]<[KeepAliveStream][15]<S>>

Available on **crate feature`tokio`** only.

Configure the interval between keep-alive messages.

Defaults to no keep-alive messages.

## Trait Implementations§

[Source][16]§

### impl<S: [Clone][17]> [Clone][17] for [Sse][6]<S>

[Source][16]§

#### fn [clone][18](&self) -> [Sse][6]<S>

Returns a duplicate of the value. [Read more][18]

1.0.0 · [Source][19]§

#### fn [clone_from][20](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][20]

[Source][21]§

### impl<S> [Debug][22] for [Sse][6]<S>

[Source][23]§

#### fn [fmt][24](&self, f: &mut [Formatter][25]<'_>) -> [Result][26]

Formats the value using the given formatter. [Read more][24]

[Source][27]§

### impl<S, E> [IntoResponse][28] for [Sse][6]<S>

where S: Stream<Item = [Result][29]<[Event][8], E>> \+ [Send][9] \+ 'static, E: [Into][10]<[BoxError][11]>,

[Source][30]§

#### fn [into_response][31](self) -> [Response][32]

Create a response.

## Auto Trait Implementations§

§

### impl<S> [Freeze][33] for [Sse][6]<S>

where S: [Freeze][33],

§

### impl<S> [RefUnwindSafe][34] for [Sse][6]<S>

where S: [RefUnwindSafe][34],

§

### impl<S> [Send][9] for [Sse][6]<S>

where S: [Send][9],

§

### impl<S> [Sync][35] for [Sse][6]<S>

where S: [Sync][35],

§

### impl<S> [Unpin][36] for [Sse][6]<S>

where S: [Unpin][36],

§

### impl<S> [UnwindSafe][37] for [Sse][6]<S>

where S: [UnwindSafe][37],

## Blanket Implementations§

[Source][38]§

### impl<T> [Any][39] for T

where T: 'static + ?[Sized][40],

[Source][41]§

#### fn [type_id][42](&self) -> [TypeId][43]

Gets the `TypeId` of `self`. [Read more][42]

[Source][44]§

### impl<T> [Borrow][45]<T> for T

where T: ?[Sized][40],

[Source][46]§

#### fn [borrow][47](&self) -> [&T][48]

Immutably borrows from an owned value. [Read more][47]

[Source][49]§

### impl<T> [BorrowMut][50]<T> for T

where T: ?[Sized][40],

[Source][51]§

#### fn [borrow_mut][52](&mut self) -> [&mut T][48]

Mutably borrows from an owned value. [Read more][52]

[Source][53]§

### impl<T> [CloneToUninit][54] for T

where T: [Clone][17],

[Source][55]§

#### unsafe fn [clone_to_uninit][56](&self, dest: [*mut ][57][u8][58])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][56]

[Source][59]§

### impl<T> [From][60]<T> for T

[Source][61]§

#### fn [from][62](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][63]<T> for T

where T: [Clone][17],

§

#### fn [from_ref][64](input: [&T][48]) -> T

Converts to this type from a reference to the input type.

[Source][65]§

### impl<H, T> [HandlerWithoutStateExt][66]<T> for H

where H: [Handler][67]<T, [()][68]>,

[Source][69]§

#### fn [into_service][70](self) -> [HandlerService][71]<H, T, [()][68]>

Convert the handler into a [`Service`] and no state.

[Source][72]§

#### fn [into_make_service][73](self) -> [IntoMakeService][74]<[HandlerService][71]<H, T, [()][68]>>

Convert the handler into a [`MakeService`][75] and no state. [Read more][73]

[Source][76]§

#### fn [into_make_service_with_connect_info][77]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][78]<[HandlerService][71]<H, T, [()][68]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][75] which stores information about the incoming connection and has no state. [Read more][77]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][79] [`Span`][80], returning an `Instrumented` wrapper. Read more

[Source][81]§

### impl<T, U> [Into][10]<U> for T

where U: [From][60]<T>,

[Source][82]§

#### fn [into][83](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][60]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][40],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][84]§

### impl<T> [Same][85] for T

[Source][86]§

#### type [Output][87] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][40],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][40],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][88]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][40],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][40],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][40],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][40],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][40],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][40],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][40],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][89] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][90]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][40],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][91] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][90]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][40],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][91] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][90]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][40],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][91] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][40], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][40], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][40],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][40],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][40],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][92]) -> RequestBodyLimit<Self>

where Self: [Sized][40],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][40],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][40],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][93]§

### impl<T> [ToOwned][94] for T

where T: [Clone][17],

[Source][95]§

#### type [Owned][96] = T

The resulting type after obtaining ownership.

[Source][97]§

#### fn [to_owned][98](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][98]

[Source][99]§

#### fn [clone_into][100](&self, target: [&mut T][48])

Uses borrowed data to replace owned data, usually by cloning. [Read more][100]

[Source][101]§

### impl<T, U> [TryFrom][102]<U> for T

where U: [Into][10]<T>,

[Source][103]§

#### type [Error][104] = [Infallible][105]

The type returned in the event of a conversion error.

[Source][106]§

#### fn [try_from][107](value: U) -> [Result][29]<T, <T as [TryFrom][102]<U>>::[Error][108]>

Performs the conversion.

[Source][109]§

### impl<T, U> [TryInto][110]<U> for T

where U: [TryFrom][102]<T>,

[Source][111]§

#### type [Error][112] = <U as [TryFrom][102]<T>>::[Error][108]

The type returned in the event of a conversion error.

[Source][113]§

#### fn [try_into][114](self) -> [Result][29]<U, <U as [TryFrom][102]<T>>::[Error][108]>

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

where S: [Into][10]<Dispatch>,

Attaches the provided [`Subscriber`][115] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][116] [`Subscriber`][115] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/response/sse.rs.html#54-56
   [5]: ../../src/axum/response/sse.rs.html#58-80
   [6]: struct.Sse.html (struct axum::response::Sse)
   [7]: ../../src/axum/response/sse.rs.html#63-69
   [8]: sse/struct.Event.html (struct axum::response::sse::Event)
   [9]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [10]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [11]: ../type.BoxError.html (type axum::BoxError)
   [12]: sse/index.html (mod axum::response::sse)
   [13]: ../../src/axum/response/sse.rs.html#75-79
   [14]: sse/struct.KeepAlive.html (struct axum::response::sse::KeepAlive)
   [15]: sse/struct.KeepAliveStream.html (struct axum::response::sse::KeepAliveStream)
   [16]: ../../src/axum/response/sse.rs.html#52
   [17]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [18]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [19]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [20]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [21]: ../../src/axum/response/sse.rs.html#82-88
   [22]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [23]: ../../src/axum/response/sse.rs.html#83-87
   [24]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [25]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [26]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [27]: ../../src/axum/response/sse.rs.html#90-107
   [28]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [29]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [30]: ../../src/axum/response/sse.rs.html#95-106
   [31]: trait.IntoResponse.html#tymethod.into_response
   [32]: type.Response.html (type axum::response::Response)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [34]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [37]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [38]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [39]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [41]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [42]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [43]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [48]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [53]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [54]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [55]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [56]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [57]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [58]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [59]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [60]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [63]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [64]: ../extract/trait.FromRef.html#tymethod.from_ref
   [65]: ../../src/axum/handler/mod.rs.html#380-398
   [66]: ../handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [67]: ../handler/trait.Handler.html (trait axum::handler::Handler)
   [68]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [69]: ../../src/axum/handler/mod.rs.html#384-386
   [70]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [71]: ../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [72]: ../../src/axum/handler/mod.rs.html#388-390
   [73]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [74]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [75]: tower::make::MakeService
   [76]: ../../src/axum/handler/mod.rs.html#393-397
   [77]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [78]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [79]: super::Span::current()
   [80]: crate::Span
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [84]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [85]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [86]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [87]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [88]: https://docs.rs/http/latest/http/struct.Extensions.html
   [89]: crate::follow_redirect::policy::Standard
   [90]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [91]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [92]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [93]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [94]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [95]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [96]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [97]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [98]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [99]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [100]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [101]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [102]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [103]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [105]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [106]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [107]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [108]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [109]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [110]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [111]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [112]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [113]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [114]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [115]: super::Subscriber
   [116]: dispatcher#setting-the-default-subscriber

