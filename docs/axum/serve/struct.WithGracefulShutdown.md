<!-- Generated from rustdoc HTML: serve/struct.WithGracefulShutdown.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## WithGracefulShutdown

## [axum][1]0.8.8

## WithGracefulShutdown

### Methods

  * local_addr



### Trait Implementations

  * Debug
  * IntoFuture



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

# Struct WithGracefulShutdown Copy item path

[Source][4]
``` 
pub struct WithGracefulShutdown<L, M, S, F, B> { /* private fields */ }
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Serve future with graceful shutdown enabled.

## Implementations§

[Source][5]§

### impl<L, M, S, F, B> [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [Listener][7],

[Source][8]

#### pub fn local_addr(&self) -> [Result][9]<L::[Addr][10]>

Returns the local address this server is bound to.

## Trait Implementations§

[Source][11]§

### impl<L, M, S, F, B> [Debug][12] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [Debug][12] \+ 'static, M: [Debug][12], S: [Debug][12], F: [Debug][12],

[Source][13]§

#### fn [fmt][14](&self, f: &mut [Formatter][15]<'_>) -> [Result][16]

Formats the value using the given formatter. [Read more][14]

[Source][17]§

### impl<L, M, S, F, B> [IntoFuture][18] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [Listener][7], L::[Addr][10]: [Debug][12], M: for<'a> Service<[IncomingStream][19]<'a, L>, Error = [Infallible][20], Response = S> \+ [Send][21] \+ 'static, for<'a> <M as Service<[IncomingStream][19]<'a, L>>>::Future: [Send][21], S: Service<[Request][22], Response = [Response][23]<B>, Error = [Infallible][20]> \+ [Clone][24] \+ [Send][21] \+ 'static, S::Future: [Send][21], F: [Future][25]<Output = [()][26]> \+ [Send][21] \+ 'static, B: HttpBody + [Send][21] \+ 'static, B::Data: [Send][21], B::Error: [Into][27]<[Box][28]<dyn [StdError][29] \+ [Send][21] \+ [Sync][30]>>,

[Source][31]§

#### type [Output][32] = [()][26]

The output that the future will produce on completion.

[Source][33]§

#### type [IntoFuture][34] = ServeFuture<[()][26]>

Which kind of future are we turning this into?

[Source][35]§

#### fn [into_future][36](self) -> Self::[IntoFuture][37]

Creates a future from a value. [Read more][36]

## Auto Trait Implementations§

§

### impl<L, M, S, F, B> [Freeze][38] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [Freeze][38], M: [Freeze][38], F: [Freeze][38],

§

### impl<L, M, S, F, B> [RefUnwindSafe][39] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [RefUnwindSafe][39], M: [RefUnwindSafe][39], F: [RefUnwindSafe][39],

§

### impl<L, M, S, F, B> [Send][21] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [Send][21], M: [Send][21], F: [Send][21],

§

### impl<L, M, S, F, B> [Sync][30] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [Sync][30], M: [Sync][30], F: [Sync][30],

§

### impl<L, M, S, F, B> [Unpin][40] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [Unpin][40], M: [Unpin][40], F: [Unpin][40],

§

### impl<L, M, S, F, B> [UnwindSafe][41] for [WithGracefulShutdown][6]<L, M, S, F, B>

where L: [UnwindSafe][41], M: [UnwindSafe][41], F: [UnwindSafe][41],

## Blanket Implementations§

[Source][42]§

### impl<T> [Any][43] for T

where T: 'static + ?[Sized][44],

[Source][45]§

#### fn [type_id][46](&self) -> [TypeId][47]

Gets the `TypeId` of `self`. [Read more][46]

[Source][48]§

### impl<T> [Borrow][49]<T> for T

where T: ?[Sized][44],

[Source][50]§

#### fn [borrow][51](&self) -> [&T][52]

Immutably borrows from an owned value. [Read more][51]

[Source][53]§

### impl<T> [BorrowMut][54]<T> for T

where T: ?[Sized][44],

[Source][55]§

#### fn [borrow_mut][56](&mut self) -> [&mut T][52]

Mutably borrows from an owned value. [Read more][56]

[Source][57]§

### impl<T> [From][58]<T> for T

[Source][59]§

#### fn [from][60](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][61] [`Span`][62], returning an `Instrumented` wrapper. Read more

[Source][63]§

### impl<T, U> [Into][27]<U> for T

where U: [From][58]<T>,

[Source][64]§

#### fn [into][65](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][58]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][44],

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

where Self: [Sized][44],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][44],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][70]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][44],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][44],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][44],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][71] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][44],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][74]) -> RequestBodyLimit<Self>

where Self: [Sized][44],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][75]§

### impl<T, U> [TryFrom][76]<U> for T

where U: [Into][27]<T>,

[Source][77]§

#### type [Error][78] = [Infallible][20]

The type returned in the event of a conversion error.

[Source][79]§

#### fn [try_from][80](value: U) -> [Result][81]<T, <T as [TryFrom][76]<U>>::[Error][82]>

Performs the conversion.

[Source][83]§

### impl<T, U> [TryInto][84]<U> for T

where U: [TryFrom][76]<T>,

[Source][85]§

#### type [Error][86] = <U as [TryFrom][76]<T>>::[Error][82]

The type returned in the event of a conversion error.

[Source][87]§

#### fn [try_into][88](self) -> [Result][81]<U, <U as [TryFrom][76]<T>>::[Error][82]>

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

where S: [Into][27]<Dispatch>,

Attaches the provided [`Subscriber`][89] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][90] [`Subscriber`][89] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/mod.rs.html#250-255
   [5]: ../../src/axum/serve/mod.rs.html#258-266
   [6]: struct.WithGracefulShutdown.html (struct axum::serve::WithGracefulShutdown)
   [7]: trait.Listener.html (trait axum::serve::Listener)
   [8]: ../../src/axum/serve/mod.rs.html#263-265
   [9]: https://doc.rust-lang.org/nightly/std/io/error/type.Result.html (type std::io::error::Result)
   [10]: trait.Listener.html#associatedtype.Addr (type axum::serve::Listener::Addr)
   [11]: ../../src/axum/serve/mod.rs.html#323-344
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [13]: ../../src/axum/serve/mod.rs.html#330-343
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [15]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [17]: ../../src/axum/serve/mod.rs.html#347-366
   [18]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html (trait core::future::into_future::IntoFuture)
   [19]: struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [20]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [21]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [22]: ../extract/type.Request.html (type axum::extract::Request)
   [23]: ../response/type.Response.html (type axum::response::Response)
   [24]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [25]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [26]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [27]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [28]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [29]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [31]: ../../src/axum/serve/mod.rs.html#360
   [32]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.Output
   [33]: ../../src/axum/serve/mod.rs.html#361
   [34]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture
   [35]: ../../src/axum/serve/mod.rs.html#363-365
   [36]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future
   [37]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture (type core::future::into_future::IntoFuture::IntoFuture)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [39]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [41]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [42]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [43]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [45]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [46]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [47]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [52]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [59]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [60]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [61]: super::Span::current()
   [62]: crate::Span
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [70]: https://docs.rs/http/latest/http/struct.Extensions.html
   [71]: crate::follow_redirect::policy::Standard
   [72]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [73]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [74]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
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

