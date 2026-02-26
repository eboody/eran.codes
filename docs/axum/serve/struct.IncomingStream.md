<!-- Generated from rustdoc HTML: serve/struct.IncomingStream.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IncomingStream

## [axum][1]0.8.8

## IncomingStream

### Methods

  * io
  * remote_addr



### Trait Implementations

  * Connected<IncomingStream<'_, L>>
  * Debug
  * Service<IncomingStream<'_, L>>
  * Service<IncomingStream<'_, L>>
  * Service<IncomingStream<'_, L>>



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
  * From<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::serve][2]

[axum][3]::[serve][2]

# Struct IncomingStream Copy item path

[Source][4]
``` 
pub struct IncomingStream<'a, L>

where
    L: [Listener][5],

{ /* private fields */ }
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

An incoming stream.

Used with [`serve`][6] and [`IntoMakeServiceWithConnectInfo`][7].

## Implementations§

[Source][8]§

### impl<L> [IncomingStream][9]<'_, L>

where L: [Listener][5],

[Source][10]

#### pub fn io(&self) -> &L::[Io][11]

Get a reference to the inner IO type.

[Source][12]

#### pub fn remote_addr(&self) -> &L::[Addr][13]

Returns the remote address that this stream is bound to.

## Trait Implementations§

[Source][14]§

### impl<L> [Connected][15]<[IncomingStream][9]<'_, L>> for [SocketAddr][16]

where L: [Listener][5]<Addr = Self>,

[Source][17]§

#### fn [connect_info][18](stream: [IncomingStream][9]<'_, L>) -> Self

Create type holding information about the connection.

[Source][19]§

### impl<'a, L> [Debug][20] for [IncomingStream][9]<'a, L>

where L: [Listener][5] \+ [Debug][20], L::[Io][11]: [Debug][20], L::[Addr][13]: [Debug][20],

[Source][19]§

#### fn [fmt][21](&self, f: &mut [Formatter][22]<'_>) -> [Result][23]

Formats the value using the given formatter. [Read more][21]

[Source][24]§

### impl<H, T, S, L> Service<[IncomingStream][9]<'_, L>> for [HandlerService][25]<H, T, S>

where H: [Clone][26], S: [Clone][26], L: [Listener][5],

[Source][27]§

#### type Response = [HandlerService][25]<H, T, S>

Responses given by the service.

[Source][28]§

#### type Error = [Infallible][29]

Errors produced by the service.

[Source][30]§

#### type Future = [Ready][31]<[Result][32]<<[HandlerService][25]<H, T, S> as Service<[IncomingStream][9]<'_, L>>>::Response, <[HandlerService][25]<H, T, S> as Service<[IncomingStream][9]<'_, L>>>::Error>>

The future response value.

[Source][33]§

#### fn poll_ready(&mut self, _cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][32]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][37]§

#### fn call(&mut self, _req: [IncomingStream][9]<'_, L>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][38]§

### impl<L> Service<[IncomingStream][9]<'_, L>> for [MethodRouter][39]<[()][36]>

where L: [Listener][5],

[Source][40]§

#### type Response = [MethodRouter][39]

Responses given by the service.

[Source][41]§

#### type Error = [Infallible][29]

Errors produced by the service.

[Source][42]§

#### type Future = [Ready][31]<[Result][32]<<[MethodRouter][39] as Service<[IncomingStream][9]<'_, L>>>::Response, <[MethodRouter][39] as Service<[IncomingStream][9]<'_, L>>>::Error>>

The future response value.

[Source][43]§

#### fn poll_ready(&mut self, _cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][32]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][44]§

#### fn call(&mut self, _req: [IncomingStream][9]<'_, L>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][45]§

### impl<L> Service<[IncomingStream][9]<'_, L>> for [Router][46]<[()][36]>

where L: [Listener][5],

[Source][47]§

#### type Response = [Router][46]

Responses given by the service.

[Source][48]§

#### type Error = [Infallible][29]

Errors produced by the service.

[Source][49]§

#### type Future = [Ready][31]<[Result][32]<<[Router][46] as Service<[IncomingStream][9]<'_, L>>>::Response, <[Router][46] as Service<[IncomingStream][9]<'_, L>>>::Error>>

The future response value.

[Source][50]§

#### fn poll_ready(&mut self, _cx: &mut [Context][34]<'_>) -> [Poll][35]<[Result][32]<[()][36], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][51]§

#### fn call(&mut self, _req: [IncomingStream][9]<'_, L>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<'a, L> [Freeze][52] for [IncomingStream][9]<'a, L>

where <L as [Listener][5]>::[Addr][13]: [Freeze][52],

§

### impl<'a, L> [RefUnwindSafe][53] for [IncomingStream][9]<'a, L>

where <L as [Listener][5]>::[Addr][13]: [RefUnwindSafe][53], <L as [Listener][5]>::[Io][11]: [RefUnwindSafe][53],

§

### impl<'a, L> [Send][54] for [IncomingStream][9]<'a, L>

where <L as [Listener][5]>::[Io][11]: [Sync][55],

§

### impl<'a, L> [Sync][55] for [IncomingStream][9]<'a, L>

where <L as [Listener][5]>::[Addr][13]: [Sync][55], <L as [Listener][5]>::[Io][11]: [Sync][55],

§

### impl<'a, L> [Unpin][56] for [IncomingStream][9]<'a, L>

where <L as [Listener][5]>::[Addr][13]: [Unpin][56],

§

### impl<'a, L> [UnwindSafe][57] for [IncomingStream][9]<'a, L>

where <L as [Listener][5]>::[Addr][13]: [UnwindSafe][57], <L as [Listener][5]>::[Io][11]: [RefUnwindSafe][53],

## Blanket Implementations§

[Source][58]§

### impl<T> [Any][59] for T

where T: 'static + ?[Sized][60],

[Source][61]§

#### fn [type_id][62](&self) -> [TypeId][63]

Gets the `TypeId` of `self`. [Read more][62]

[Source][64]§

### impl<T> [Borrow][65]<T> for T

where T: ?[Sized][60],

[Source][66]§

#### fn [borrow][67](&self) -> [&T][68]

Immutably borrows from an owned value. [Read more][67]

[Source][69]§

### impl<T> [BorrowMut][70]<T> for T

where T: ?[Sized][60],

[Source][71]§

#### fn [borrow_mut][72](&mut self) -> [&mut T][68]

Mutably borrows from an owned value. [Read more][72]

[Source][73]§

### impl<T> [From][74]<T> for T

[Source][75]§

#### fn [from][76](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][77] [`Span`][78], returning an `Instrumented` wrapper. Read more

[Source][79]§

### impl<T, U> [Into][80]<U> for T

where U: [From][74]<T>,

[Source][81]§

#### fn [into][82](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][74]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][60],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][83]§

### impl<T> [Same][84] for T

[Source][85]§

#### type [Output][86] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][60],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][60],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][87]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][60],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][60],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][60],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][60],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][60],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][60],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][60],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][88] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][89]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][60],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][90] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][89]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][60],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][90] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][89]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][60],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][90] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][60],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][60],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][60],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][60],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][60],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][60],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][60], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][60], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][60],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][60],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][60],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][91]) -> RequestBodyLimit<Self>

where Self: [Sized][60],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][60],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][60],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][92]§

### impl<T, U> [TryFrom][93]<U> for T

where U: [Into][80]<T>,

[Source][94]§

#### type [Error][95] = [Infallible][29]

The type returned in the event of a conversion error.

[Source][96]§

#### fn [try_from][97](value: U) -> [Result][32]<T, <T as [TryFrom][93]<U>>::[Error][98]>

Performs the conversion.

[Source][99]§

### impl<T, U> [TryInto][100]<U> for T

where U: [TryFrom][93]<T>,

[Source][101]§

#### type [Error][102] = <U as [TryFrom][93]<T>>::[Error][98]

The type returned in the event of a conversion error.

[Source][103]§

#### fn [try_into][104](self) -> [Result][32]<U, <U as [TryFrom][93]<T>>::[Error][98]>

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

where S: [Into][80]<Dispatch>,

Attaches the provided [`Subscriber`][105] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][106] [`Subscriber`][105] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/mod.rs.html#447-453
   [5]: trait.Listener.html (trait axum::serve::Listener)
   [6]: ../fn.serve.html (fn axum::serve)
   [7]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [8]: ../../src/axum/serve/mod.rs.html#455-468
   [9]: struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [10]: ../../src/axum/serve/mod.rs.html#460-462
   [11]: trait.Listener.html#associatedtype.Io (type axum::serve::Listener::Io)
   [12]: ../../src/axum/serve/mod.rs.html#465-467
   [13]: trait.Listener.html#associatedtype.Addr (type axum::serve::Listener::Addr)
   [14]: ../../src/axum/extract/connect_info.rs.html#83-90
   [15]: ../extract/connect_info/trait.Connected.html (trait axum::extract::connect_info::Connected)
   [16]: https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html (enum core::net::socket_addr::SocketAddr)
   [17]: ../../src/axum/extract/connect_info.rs.html#87-89
   [18]: ../extract/connect_info/trait.Connected.html#tymethod.connect_info
   [19]: ../../src/axum/serve/mod.rs.html#446
   [20]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [21]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [22]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [23]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [24]: ../../src/axum/handler/service.rs.html#183-200
   [25]: ../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [26]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [27]: ../../src/axum/handler/service.rs.html#189
   [28]: ../../src/axum/handler/service.rs.html#190
   [29]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [30]: ../../src/axum/handler/service.rs.html#191
   [31]: https://doc.rust-lang.org/nightly/core/future/ready/struct.Ready.html (struct core::future::ready::Ready)
   [32]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [33]: ../../src/axum/handler/service.rs.html#193-195
   [34]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [35]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [36]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [37]: ../../src/axum/handler/service.rs.html#197-199
   [38]: ../../src/axum/routing/method_routing.rs.html#1371-1386
   [39]: ../routing/method_routing/struct.MethodRouter.html (struct axum::routing::method_routing::MethodRouter)
   [40]: ../../src/axum/routing/method_routing.rs.html#1375
   [41]: ../../src/axum/routing/method_routing.rs.html#1376
   [42]: ../../src/axum/routing/method_routing.rs.html#1377
   [43]: ../../src/axum/routing/method_routing.rs.html#1379-1381
   [44]: ../../src/axum/routing/method_routing.rs.html#1383-1385
   [45]: ../../src/axum/routing/mod.rs.html#579-596
   [46]: ../struct.Router.html (struct axum::Router)
   [47]: ../../src/axum/routing/mod.rs.html#583
   [48]: ../../src/axum/routing/mod.rs.html#584
   [49]: ../../src/axum/routing/mod.rs.html#585
   [50]: ../../src/axum/routing/mod.rs.html#587-589
   [51]: ../../src/axum/routing/mod.rs.html#591-595
   [52]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [53]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [54]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [55]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [56]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [57]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [58]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [59]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [60]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [61]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [62]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [63]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [64]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [65]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [66]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [67]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [68]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [69]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [70]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [71]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [72]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [77]: super::Span::current()
   [78]: crate::Span
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [83]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [84]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [85]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [86]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [87]: https://docs.rs/http/latest/http/struct.Extensions.html
   [88]: crate::follow_redirect::policy::Standard
   [89]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [90]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [91]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [98]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [99]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [100]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [101]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [102]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [103]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [105]: super::Subscriber
   [106]: dispatcher#setting-the-default-subscriber

