<!-- Generated from rustdoc HTML: serve/struct.TapIo.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## TapIo

## [axum][1]0.8.8

## TapIo

### Trait Implementations

  * Debug
  * Listener



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
  * ListenerExt
  * PolicyExt
  * Same
  * ServiceExt
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::serve][2]

[axum][3]::[serve][2]

# Struct TapIo Copy item path

[Source][4]
``` 
pub struct TapIo<L, F> { /* private fields */ }
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Return type of [`ListenerExt::tap_io`][5].

See that method for details.

## Trait Implementations§

[Source][6]§

### impl<L, F> [Debug][7] for [TapIo][8]<L, F>

where L: [Listener][9] \+ [Debug][7],

[Source][10]§

#### fn [fmt][11](&self, f: &mut [Formatter][12]<'_>) -> [Result][13]

Formats the value using the given formatter. [Read more][11]

[Source][14]§

### impl<L, F> [Listener][9] for [TapIo][8]<L, F>

where L: [Listener][9], F: [FnMut][15](&mut L::[Io][16]) + [Send][17] \+ 'static,

[Source][18]§

#### type [Io][19] = <L as [Listener][9]>::[Io][16]

The listener’s IO type.

[Source][20]§

#### type [Addr][21] = <L as [Listener][9]>::[Addr][22]

The listener’s address type.

[Source][23]§

#### async fn [accept][24](&mut self) -> (Self::[Io][16], Self::[Addr][22])

Accept a new incoming connection to this listener. [Read more][24]

[Source][25]§

#### fn [local_addr][26](&self) -> [Result][27]<Self::[Addr][22]>

Returns the local address that this listener is bound to.

## Auto Trait Implementations§

§

### impl<L, F> [Freeze][28] for [TapIo][8]<L, F>

where L: [Freeze][28], F: [Freeze][28],

§

### impl<L, F> [RefUnwindSafe][29] for [TapIo][8]<L, F>

where L: [RefUnwindSafe][29], F: [RefUnwindSafe][29],

§

### impl<L, F> [Send][17] for [TapIo][8]<L, F>

where L: [Send][17], F: [Send][17],

§

### impl<L, F> [Sync][30] for [TapIo][8]<L, F>

where L: [Sync][30], F: [Sync][30],

§

### impl<L, F> [Unpin][31] for [TapIo][8]<L, F>

where L: [Unpin][31], F: [Unpin][31],

§

### impl<L, F> [UnwindSafe][32] for [TapIo][8]<L, F>

where L: [UnwindSafe][32], F: [UnwindSafe][32],

## Blanket Implementations§

[Source][33]§

### impl<T> [Any][34] for T

where T: 'static + ?[Sized][35],

[Source][36]§

#### fn [type_id][37](&self) -> [TypeId][38]

Gets the `TypeId` of `self`. [Read more][37]

[Source][39]§

### impl<T> [Borrow][40]<T> for T

where T: ?[Sized][35],

[Source][41]§

#### fn [borrow][42](&self) -> [&T][43]

Immutably borrows from an owned value. [Read more][42]

[Source][44]§

### impl<T> [BorrowMut][45]<T> for T

where T: ?[Sized][35],

[Source][46]§

#### fn [borrow_mut][47](&mut self) -> [&mut T][43]

Mutably borrows from an owned value. [Read more][47]

[Source][48]§

### impl<T> [From][49]<T> for T

[Source][50]§

#### fn [from][51](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][52] [`Span`][53], returning an `Instrumented` wrapper. Read more

[Source][54]§

### impl<T, U> [Into][55]<U> for T

where U: [From][49]<T>,

[Source][56]§

#### fn [into][57](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][49]<T> for U` chooses to do.

[Source][58]§

### impl<L> [ListenerExt][59] for L

where L: [Listener][9],

[Source][60]§

#### fn [limit_connections][61](self, limit: [usize][62]) -> [ConnLimiter][63]<Self>

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Limit the number of concurrent connections. Once the limit has been reached, no additional connections will be accepted until an existing connection is closed. Listener implementations will typically continue to queue incoming connections, up to an OS and implementation-specific listener backlog limit. [Read more][61]

[Source][64]§

#### fn [tap_io][65]<F>(self, tap_fn: F) -> [TapIo][8]<Self, F>

where F: [FnMut][15](&mut Self::[Io][16]) + [Send][17] \+ 'static,

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Run a mutable closure on every accepted `Io`. [Read more][65]

§

### impl<T> PolicyExt for T

where T: ?[Sized][35],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][66]§

### impl<T> [Same][67] for T

[Source][68]§

#### type [Output][69] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][35],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][35],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][70]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][35],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][35],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][35],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][35],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][35],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][35],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][35],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][71] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][35],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][35],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][35],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][35], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][35], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][35],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][35],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][35],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][62]) -> RequestBodyLimit<Self>

where Self: [Sized][35],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][35],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][35],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][74]§

### impl<T, U> [TryFrom][75]<U> for T

where U: [Into][55]<T>,

[Source][76]§

#### type [Error][77] = [Infallible][78]

The type returned in the event of a conversion error.

[Source][79]§

#### fn [try_from][80](value: U) -> [Result][81]<T, <T as [TryFrom][75]<U>>::[Error][82]>

Performs the conversion.

[Source][83]§

### impl<T, U> [TryInto][84]<U> for T

where U: [TryFrom][75]<T>,

[Source][85]§

#### type [Error][86] = <U as [TryFrom][75]<T>>::[Error][82]

The type returned in the event of a conversion error.

[Source][87]§

#### fn [try_into][88](self) -> [Result][81]<U, <U as [TryFrom][75]<T>>::[Error][82]>

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

where S: [Into][55]<Dispatch>,

Attaches the provided [`Subscriber`][89] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][90] [`Subscriber`][89] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/listener.rs.html#210-213
   [5]: trait.ListenerExt.html#method.tap_io (method axum::serve::ListenerExt::tap_io)
   [6]: ../../src/axum/serve/listener.rs.html#215-224
   [7]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [8]: struct.TapIo.html (struct axum::serve::TapIo)
   [9]: trait.Listener.html (trait axum::serve::Listener)
   [10]: ../../src/axum/serve/listener.rs.html#219-223
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [12]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [14]: ../../src/axum/serve/listener.rs.html#226-243
   [15]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [16]: trait.Listener.html#associatedtype.Io (type axum::serve::Listener::Io)
   [17]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [18]: ../../src/axum/serve/listener.rs.html#231
   [19]: trait.Listener.html#associatedtype.Io
   [20]: ../../src/axum/serve/listener.rs.html#232
   [21]: trait.Listener.html#associatedtype.Addr
   [22]: trait.Listener.html#associatedtype.Addr (type axum::serve::Listener::Addr)
   [23]: ../../src/axum/serve/listener.rs.html#234-238
   [24]: trait.Listener.html#tymethod.accept
   [25]: ../../src/axum/serve/listener.rs.html#240-242
   [26]: trait.Listener.html#tymethod.local_addr
   [27]: https://doc.rust-lang.org/nightly/std/io/error/type.Result.html (type std::io::error::Result)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [29]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [32]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [33]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [34]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [36]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [37]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [38]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [39]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [40]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [41]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [42]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [43]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [48]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [49]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [50]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [51]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [52]: super::Span::current()
   [53]: crate::Span
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [55]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [57]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [58]: ../../src/axum/serve/listener.rs.html#127
   [59]: trait.ListenerExt.html (trait axum::serve::ListenerExt)
   [60]: ../../src/axum/serve/listener.rs.html#87-92
   [61]: trait.ListenerExt.html#method.limit_connections
   [62]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [63]: struct.ConnLimiter.html (struct axum::serve::ConnLimiter)
   [64]: ../../src/axum/serve/listener.rs.html#116-124
   [65]: trait.ListenerExt.html#method.tap_io
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [70]: https://docs.rs/http/latest/http/struct.Extensions.html
   [71]: crate::follow_redirect::policy::Standard
   [72]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [73]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [74]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [75]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [76]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [77]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [78]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [81]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [89]: super::Subscriber
   [90]: dispatcher#setting-the-default-subscriber

