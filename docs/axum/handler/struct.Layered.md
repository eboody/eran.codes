<!-- Generated from rustdoc HTML: handler/struct.Layered.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Layered

## [axum][1]0.8.8

## Layered

### Trait Implementations

  * Clone
  * Debug
  * Handler<T, S>



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



## [In axum::handler][2]

[axum][3]::[handler][2]

# Struct Layered Copy item path

[Source][4]
``` 
pub struct Layered<L, H, T, S> { /* private fields */ }
```

Expand description

A [`Service`] created from a [`Handler`][5] by applying a Tower middleware.

Created with [`Handler::layer`][6]. See that method for more details.

## Trait Implementations§

[Source][7]§

### impl<L, H, T, S> [Clone][8] for [Layered][9]<L, H, T, S>

where L: [Clone][8], H: [Clone][8],

[Source][10]§

#### fn [clone][11](&self) -> Self

Returns a duplicate of the value. [Read more][11]

1.0.0 · [Source][12]§

#### fn [clone_from][13](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][13]

[Source][14]§

### impl<L, H, T, S> [Debug][15] for [Layered][9]<L, H, T, S>

where L: [Debug][15],

[Source][16]§

#### fn [fmt][17](&self, f: &mut [Formatter][18]<'_>) -> [Result][19]

Formats the value using the given formatter. [Read more][17]

[Source][20]§

### impl<H, S, T, L> [Handler][5]<T, S> for [Layered][9]<L, H, T, S>

where L: Layer<[HandlerService][21]<H, T, S>> \+ [Clone][8] \+ [Send][22] \+ [Sync][23] \+ 'static, H: [Handler][5]<T, S>, L::Service: Service<[Request][24], Error = [Infallible][25]> \+ [Clone][8] \+ [Send][22] \+ 'static, <L::Service as Service<[Request][24]>>::Response: [IntoResponse][26], <L::Service as Service<[Request][24]>>::Future: [Send][22], T: 'static, S: 'static,

[Source][27]§

#### type [Future][28] = [LayeredFuture][29]<<L as Layer<[HandlerService][21]<H, T, S>>>::Service>

The type of future calling this handler returns.

[Source][30]§

#### fn [call][31](self, req: [Request][24], state: S) -> Self::[Future][32]

Call the handler with the given request.

[Source][33]§

#### fn [layer][34]<L>(self, layer: L) -> [Layered][9]<L, Self, T, S>

where L: Layer<[HandlerService][21]<Self, T, S>> \+ [Clone][8], L::Service: Service<[Request][24]>,

Apply a [`tower::Layer`] to the handler. [Read more][34]

[Source][35]§

#### fn [with_state][36](self, state: S) -> [HandlerService][21]<Self, T, S>

Convert the handler into a [`Service`] by providing the state

## Auto Trait Implementations§

§

### impl<L, H, T, S> [Freeze][37] for [Layered][9]<L, H, T, S>

where L: [Freeze][37], H: [Freeze][37],

§

### impl<L, H, T, S> [RefUnwindSafe][38] for [Layered][9]<L, H, T, S>

where L: [RefUnwindSafe][38], H: [RefUnwindSafe][38],

§

### impl<L, H, T, S> [Send][22] for [Layered][9]<L, H, T, S>

where L: [Send][22], H: [Send][22],

§

### impl<L, H, T, S> [Sync][23] for [Layered][9]<L, H, T, S>

where L: [Sync][23], H: [Sync][23],

§

### impl<L, H, T, S> [Unpin][39] for [Layered][9]<L, H, T, S>

where L: [Unpin][39], H: [Unpin][39],

§

### impl<L, H, T, S> [UnwindSafe][40] for [Layered][9]<L, H, T, S>

where L: [UnwindSafe][40], H: [UnwindSafe][40],

## Blanket Implementations§

[Source][41]§

### impl<T> [Any][42] for T

where T: 'static + ?[Sized][43],

[Source][44]§

#### fn [type_id][45](&self) -> [TypeId][46]

Gets the `TypeId` of `self`. [Read more][45]

[Source][47]§

### impl<T> [Borrow][48]<T> for T

where T: ?[Sized][43],

[Source][49]§

#### fn [borrow][50](&self) -> [&T][51]

Immutably borrows from an owned value. [Read more][50]

[Source][52]§

### impl<T> [BorrowMut][53]<T> for T

where T: ?[Sized][43],

[Source][54]§

#### fn [borrow_mut][55](&mut self) -> [&mut T][51]

Mutably borrows from an owned value. [Read more][55]

[Source][56]§

### impl<T> [CloneToUninit][57] for T

where T: [Clone][8],

[Source][58]§

#### unsafe fn [clone_to_uninit][59](&self, dest: [*mut ][60][u8][61])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][59]

[Source][62]§

### impl<T> [From][63]<T> for T

[Source][64]§

#### fn [from][65](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][66]<T> for T

where T: [Clone][8],

§

#### fn [from_ref][67](input: [&T][51]) -> T

Converts to this type from a reference to the input type.

[Source][68]§

### impl<H, T> [HandlerWithoutStateExt][69]<T> for H

where H: [Handler][5]<T, [()][70]>,

[Source][71]§

#### fn [into_service][72](self) -> [HandlerService][21]<H, T, [()][70]>

Convert the handler into a [`Service`] and no state.

[Source][73]§

#### fn [into_make_service][74](self) -> [IntoMakeService][75]<[HandlerService][21]<H, T, [()][70]>>

Convert the handler into a [`MakeService`][76] and no state. [Read more][74]

[Source][77]§

#### fn [into_make_service_with_connect_info][78]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][79]<[HandlerService][21]<H, T, [()][70]>, C>

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

### impl<T, U> [Into][83]<U> for T

where U: [From][63]<T>,

[Source][84]§

#### fn [into][85](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][63]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][43],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][86]§

### impl<T> [Same][87] for T

[Source][88]§

#### type [Output][89] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][43],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][43],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][90]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][43],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][43],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][43],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][91] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][92]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][93] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][92]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][93] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][92]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][93] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][43],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][94]) -> RequestBodyLimit<Self>

where Self: [Sized][43],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][95]§

### impl<T> [ToOwned][96] for T

where T: [Clone][8],

[Source][97]§

#### type [Owned][98] = T

The resulting type after obtaining ownership.

[Source][99]§

#### fn [to_owned][100](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][100]

[Source][101]§

#### fn [clone_into][102](&self, target: [&mut T][51])

Uses borrowed data to replace owned data, usually by cloning. [Read more][102]

[Source][103]§

### impl<T, U> [TryFrom][104]<U> for T

where U: [Into][83]<T>,

[Source][105]§

#### type [Error][106] = [Infallible][25]

The type returned in the event of a conversion error.

[Source][107]§

#### fn [try_from][108](value: U) -> [Result][109]<T, <T as [TryFrom][104]<U>>::[Error][110]>

Performs the conversion.

[Source][111]§

### impl<T, U> [TryInto][112]<U> for T

where U: [TryFrom][104]<T>,

[Source][113]§

#### type [Error][114] = <U as [TryFrom][104]<T>>::[Error][110]

The type returned in the event of a conversion error.

[Source][115]§

#### fn [try_into][116](self) -> [Result][109]<U, <U as [TryFrom][104]<T>>::[Error][110]>

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

where S: [Into][83]<Dispatch>,

Attaches the provided [`Subscriber`][117] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][118] [`Subscriber`][117] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/handler/mod.rs.html#285-289
   [5]: trait.Handler.html (trait axum::handler::Handler)
   [6]: trait.Handler.html#method.layer (method axum::handler::Handler::layer)
   [7]: ../../src/axum/handler/mod.rs.html#302-314
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [9]: struct.Layered.html (struct axum::handler::Layered)
   [10]: ../../src/axum/handler/mod.rs.html#307-313
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [12]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [13]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [14]: ../../src/axum/handler/mod.rs.html#291-300
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [16]: ../../src/axum/handler/mod.rs.html#295-299
   [17]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [18]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [20]: ../../src/axum/handler/mod.rs.html#317-350
   [21]: struct.HandlerService.html (struct axum::handler::HandlerService)
   [22]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [24]: ../extract/type.Request.html (type axum::extract::Request)
   [25]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [26]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [27]: ../../src/axum/handler/mod.rs.html#327
   [28]: trait.Handler.html#associatedtype.Future
   [29]: future/struct.LayeredFuture.html (struct axum::handler::future::LayeredFuture)
   [30]: ../../src/axum/handler/mod.rs.html#329-349
   [31]: trait.Handler.html#tymethod.call
   [32]: trait.Handler.html#associatedtype.Future (type axum::handler::Handler::Future)
   [33]: ../../src/axum/handler/mod.rs.html#189-199
   [34]: trait.Handler.html#method.layer
   [35]: ../../src/axum/handler/mod.rs.html#202-204
   [36]: trait.Handler.html#method.with_state
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [41]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [42]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [44]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [45]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [46]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [51]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [56]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [57]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [58]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [59]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [60]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [61]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [66]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [67]: ../extract/trait.FromRef.html#tymethod.from_ref
   [68]: ../../src/axum/handler/mod.rs.html#380-398
   [69]: trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [70]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [71]: ../../src/axum/handler/mod.rs.html#384-386
   [72]: trait.HandlerWithoutStateExt.html#tymethod.into_service
   [73]: ../../src/axum/handler/mod.rs.html#388-390
   [74]: trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [75]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [76]: tower::make::MakeService
   [77]: ../../src/axum/handler/mod.rs.html#393-397
   [78]: trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [79]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [80]: super::Span::current()
   [81]: crate::Span
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [84]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [86]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [87]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [88]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [89]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [90]: https://docs.rs/http/latest/http/struct.Extensions.html
   [91]: crate::follow_redirect::policy::Standard
   [92]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [93]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [94]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [95]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [96]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [97]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [98]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [99]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [100]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [101]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [102]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [103]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [105]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [106]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [107]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [108]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [109]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [110]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [111]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [112]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [113]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [114]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [115]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [116]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [117]: super::Subscriber
   [118]: dispatcher#setting-the-default-subscriber

