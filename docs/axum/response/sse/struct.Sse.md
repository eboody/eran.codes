<!-- Generated from rustdoc HTML: response/sse/struct.Sse.html -->
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



## [In axum::response::sse][2]

[axum][3]::[response][4]::[sse][2]

# Struct Sse Copy item path

[Source][5]
``` 
pub struct Sse<S> { /* private fields */ }
```

Expand description

An SSE response

## Implementations§

[Source][6]§

### impl<S> [Sse][7]<S>

[Source][8]

#### pub fn new(stream: S) -> Self

where S: TryStream<Ok = [Event][9]> \+ [Send][10] \+ 'static, S::Error: [Into][11]<[BoxError][12]>,

Create a new [`Sse`][7] response that will respond with the given stream of [`Event`][9]s.

See the [module docs][13] for more details.

[Source][14]

#### pub fn keep_alive(self, keep_alive: [KeepAlive][15]) -> [Sse][7]<[KeepAliveStream][16]<S>>

Available on **crate feature`tokio`** only.

Configure the interval between keep-alive messages.

Defaults to no keep-alive messages.

## Trait Implementations§

[Source][17]§

### impl<S: [Clone][18]> [Clone][18] for [Sse][7]<S>

[Source][17]§

#### fn [clone][19](&self) -> [Sse][7]<S>

Returns a duplicate of the value. [Read more][19]

1.0.0 · [Source][20]§

#### fn [clone_from][21](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][21]

[Source][22]§

### impl<S> [Debug][23] for [Sse][7]<S>

[Source][24]§

#### fn [fmt][25](&self, f: &mut [Formatter][26]<'_>) -> [Result][27]

Formats the value using the given formatter. [Read more][25]

[Source][28]§

### impl<S, E> [IntoResponse][29] for [Sse][7]<S>

where S: Stream<Item = [Result][30]<[Event][9], E>> \+ [Send][10] \+ 'static, E: [Into][11]<[BoxError][12]>,

[Source][31]§

#### fn [into_response][32](self) -> [Response][33]

Create a response.

## Auto Trait Implementations§

§

### impl<S> [Freeze][34] for [Sse][7]<S>

where S: [Freeze][34],

§

### impl<S> [RefUnwindSafe][35] for [Sse][7]<S>

where S: [RefUnwindSafe][35],

§

### impl<S> [Send][10] for [Sse][7]<S>

where S: [Send][10],

§

### impl<S> [Sync][36] for [Sse][7]<S>

where S: [Sync][36],

§

### impl<S> [Unpin][37] for [Sse][7]<S>

where S: [Unpin][37],

§

### impl<S> [UnwindSafe][38] for [Sse][7]<S>

where S: [UnwindSafe][38],

## Blanket Implementations§

[Source][39]§

### impl<T> [Any][40] for T

where T: 'static + ?[Sized][41],

[Source][42]§

#### fn [type_id][43](&self) -> [TypeId][44]

Gets the `TypeId` of `self`. [Read more][43]

[Source][45]§

### impl<T> [Borrow][46]<T> for T

where T: ?[Sized][41],

[Source][47]§

#### fn [borrow][48](&self) -> [&T][49]

Immutably borrows from an owned value. [Read more][48]

[Source][50]§

### impl<T> [BorrowMut][51]<T> for T

where T: ?[Sized][41],

[Source][52]§

#### fn [borrow_mut][53](&mut self) -> [&mut T][49]

Mutably borrows from an owned value. [Read more][53]

[Source][54]§

### impl<T> [CloneToUninit][55] for T

where T: [Clone][18],

[Source][56]§

#### unsafe fn [clone_to_uninit][57](&self, dest: [*mut ][58][u8][59])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][57]

[Source][60]§

### impl<T> [From][61]<T> for T

[Source][62]§

#### fn [from][63](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][64]<T> for T

where T: [Clone][18],

§

#### fn [from_ref][65](input: [&T][49]) -> T

Converts to this type from a reference to the input type.

[Source][66]§

### impl<H, T> [HandlerWithoutStateExt][67]<T> for H

where H: [Handler][68]<T, [()][69]>,

[Source][70]§

#### fn [into_service][71](self) -> [HandlerService][72]<H, T, [()][69]>

Convert the handler into a [`Service`] and no state.

[Source][73]§

#### fn [into_make_service][74](self) -> [IntoMakeService][75]<[HandlerService][72]<H, T, [()][69]>>

Convert the handler into a [`MakeService`][76] and no state. [Read more][74]

[Source][77]§

#### fn [into_make_service_with_connect_info][78]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][79]<[HandlerService][72]<H, T, [()][69]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][76] which stores information about the incoming connection and has no state. [Read more][78]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][80] [`Span`][81], returning an `Instrumented` wrapper. Read more

[Source][82]§

### impl<T, U> [Into][11]<U> for T

where U: [From][61]<T>,

[Source][83]§

#### fn [into][84](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][61]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][41],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][85]§

### impl<T> [Same][86] for T

[Source][87]§

#### type [Output][88] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][41],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][41],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][89]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][41],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][41],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][41],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][41],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][41],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][41],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][41],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][90] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][91]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][92] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][91]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][92] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][91]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][92] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][41], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][41], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][41],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][41],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][41],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][93]) -> RequestBodyLimit<Self>

where Self: [Sized][41],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][41],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][41],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][94]§

### impl<T> [ToOwned][95] for T

where T: [Clone][18],

[Source][96]§

#### type [Owned][97] = T

The resulting type after obtaining ownership.

[Source][98]§

#### fn [to_owned][99](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][99]

[Source][100]§

#### fn [clone_into][101](&self, target: [&mut T][49])

Uses borrowed data to replace owned data, usually by cloning. [Read more][101]

[Source][102]§

### impl<T, U> [TryFrom][103]<U> for T

where U: [Into][11]<T>,

[Source][104]§

#### type [Error][105] = [Infallible][106]

The type returned in the event of a conversion error.

[Source][107]§

#### fn [try_from][108](value: U) -> [Result][30]<T, <T as [TryFrom][103]<U>>::[Error][109]>

Performs the conversion.

[Source][110]§

### impl<T, U> [TryInto][111]<U> for T

where U: [TryFrom][103]<T>,

[Source][112]§

#### type [Error][113] = <U as [TryFrom][103]<T>>::[Error][109]

The type returned in the event of a conversion error.

[Source][114]§

#### fn [try_into][115](self) -> [Result][30]<U, <U as [TryFrom][103]<T>>::[Error][109]>

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

where S: [Into][11]<Dispatch>,

Attaches the provided [`Subscriber`][116] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][117] [`Subscriber`][116] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/response/sse.rs.html#54-56
   [6]: ../../../src/axum/response/sse.rs.html#58-80
   [7]: ../struct.Sse.html (struct axum::response::Sse)
   [8]: ../../../src/axum/response/sse.rs.html#63-69
   [9]: struct.Event.html (struct axum::response::sse::Event)
   [10]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [11]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [12]: ../../type.BoxError.html (type axum::BoxError)
   [13]: index.html (mod axum::response::sse)
   [14]: ../../../src/axum/response/sse.rs.html#75-79
   [15]: struct.KeepAlive.html (struct axum::response::sse::KeepAlive)
   [16]: struct.KeepAliveStream.html (struct axum::response::sse::KeepAliveStream)
   [17]: ../../../src/axum/response/sse.rs.html#52
   [18]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [19]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [20]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [21]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [22]: ../../../src/axum/response/sse.rs.html#82-88
   [23]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [24]: ../../../src/axum/response/sse.rs.html#83-87
   [25]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [26]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [27]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [28]: ../../../src/axum/response/sse.rs.html#90-107
   [29]: ../trait.IntoResponse.html (trait axum::response::IntoResponse)
   [30]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [31]: ../../../src/axum/response/sse.rs.html#95-106
   [32]: ../trait.IntoResponse.html#tymethod.into_response
   [33]: ../type.Response.html (type axum::response::Response)
   [34]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [35]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [39]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [40]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [42]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [43]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [44]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [45]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [46]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [49]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [54]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [55]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [56]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [57]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [58]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [59]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [64]: ../../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [65]: ../../extract/trait.FromRef.html#tymethod.from_ref
   [66]: ../../../src/axum/handler/mod.rs.html#380-398
   [67]: ../../handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [68]: ../../handler/trait.Handler.html (trait axum::handler::Handler)
   [69]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [70]: ../../../src/axum/handler/mod.rs.html#384-386
   [71]: ../../handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [72]: ../../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [73]: ../../../src/axum/handler/mod.rs.html#388-390
   [74]: ../../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [75]: ../../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [76]: tower::make::MakeService
   [77]: ../../../src/axum/handler/mod.rs.html#393-397
   [78]: ../../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [79]: ../../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [80]: super::Span::current()
   [81]: crate::Span
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [85]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [86]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [87]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [88]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [89]: https://docs.rs/http/latest/http/struct.Extensions.html
   [90]: crate::follow_redirect::policy::Standard
   [91]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [92]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [93]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [94]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [95]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [96]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [97]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [98]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [99]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [100]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [101]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [102]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [103]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [104]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [105]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [106]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [107]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [108]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [109]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [110]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [111]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [112]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [113]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [114]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [115]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [116]: super::Subscriber
   [117]: dispatcher#setting-the-default-subscriber

