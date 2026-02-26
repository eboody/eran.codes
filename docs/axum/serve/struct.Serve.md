<!-- Generated from rustdoc HTML: serve/struct.Serve.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Serve

## [axum][1]0.8.8

## Serve

### Methods

  * local_addr
  * with_graceful_shutdown



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

# Struct Serve Copy item path

[Source][4]
``` 
pub struct Serve<L, M, S, B> { /* private fields */ }
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

Future returned by [`serve`][5].

## Implementations§

[Source][6]§

### impl<L, M, S, B> [Serve][7]<L, M, S, B>

where L: [Listener][8],

[Source][9]

#### pub fn with_graceful_shutdown<F>( self, signal: F, ) -> [WithGracefulShutdown][10]<L, M, S, F, B>

where F: [Future][11]<Output = [()][12]> \+ [Send][13] \+ 'static,

Prepares a server to handle graceful shutdown when the provided future completes.

##### §Example
``` 
use axum::{Router, routing::get};

let router = Router::new().route("/", get(|| async { "Hello, World!" }));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .await;

async fn shutdown_signal() {
    // ...
}
```

##### §Return Value

Similarly to [`serve`][5], although this future resolves to `io::Result<()>`, it will never error. It returns `Ok(())` only after the `signal` future completes.

[Source][14]

#### pub fn local_addr(&self) -> [Result][15]<L::[Addr][16]>

Returns the local address this server is bound to.

## Trait Implementations§

[Source][17]§

### impl<L, M, S, B> [Debug][18] for [Serve][7]<L, M, S, B>

where L: [Debug][18] \+ 'static, M: [Debug][18],

[Source][19]§

#### fn [fmt][20](&self, f: &mut [Formatter][21]<'_>) -> [Result][22]

Formats the value using the given formatter. [Read more][20]

[Source][23]§

### impl<L, M, S, B> [IntoFuture][24] for [Serve][7]<L, M, S, B>

where L: [Listener][8], L::[Addr][16]: [Debug][18], M: for<'a> Service<[IncomingStream][25]<'a, L>, Error = [Infallible][26], Response = S> \+ [Send][13] \+ 'static, for<'a> <M as Service<[IncomingStream][25]<'a, L>>>::Future: [Send][13], S: Service<[Request][27], Response = [Response][28]<B>, Error = [Infallible][26]> \+ [Clone][29] \+ [Send][13] \+ 'static, S::Future: [Send][13], B: HttpBody + [Send][13] \+ 'static, B::Data: [Send][13], B::Error: [Into][30]<[Box][31]<dyn [StdError][32] \+ [Send][13] \+ [Sync][33]>>,

[Source][34]§

#### type [Output][35] = [Infallible][26]

The output that the future will produce on completion.

[Source][36]§

#### type [IntoFuture][37] = ServeFuture

Which kind of future are we turning this into?

[Source][38]§

#### fn [into_future][39](self) -> Self::[IntoFuture][40]

Creates a future from a value. [Read more][39]

## Auto Trait Implementations§

§

### impl<L, M, S, B> [Freeze][41] for [Serve][7]<L, M, S, B>

where L: [Freeze][41], M: [Freeze][41],

§

### impl<L, M, S, B> [RefUnwindSafe][42] for [Serve][7]<L, M, S, B>

where L: [RefUnwindSafe][42], M: [RefUnwindSafe][42],

§

### impl<L, M, S, B> [Send][13] for [Serve][7]<L, M, S, B>

where L: [Send][13], M: [Send][13],

§

### impl<L, M, S, B> [Sync][33] for [Serve][7]<L, M, S, B>

where L: [Sync][33], M: [Sync][33],

§

### impl<L, M, S, B> [Unpin][43] for [Serve][7]<L, M, S, B>

where L: [Unpin][43], M: [Unpin][43],

§

### impl<L, M, S, B> [UnwindSafe][44] for [Serve][7]<L, M, S, B>

where L: [UnwindSafe][44], M: [UnwindSafe][44],

## Blanket Implementations§

[Source][45]§

### impl<T> [Any][46] for T

where T: 'static + ?[Sized][47],

[Source][48]§

#### fn [type_id][49](&self) -> [TypeId][50]

Gets the `TypeId` of `self`. [Read more][49]

[Source][51]§

### impl<T> [Borrow][52]<T> for T

where T: ?[Sized][47],

[Source][53]§

#### fn [borrow][54](&self) -> [&T][55]

Immutably borrows from an owned value. [Read more][54]

[Source][56]§

### impl<T> [BorrowMut][57]<T> for T

where T: ?[Sized][47],

[Source][58]§

#### fn [borrow_mut][59](&mut self) -> [&mut T][55]

Mutably borrows from an owned value. [Read more][59]

[Source][60]§

### impl<T> [From][61]<T> for T

[Source][62]§

#### fn [from][63](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][64] [`Span`][65], returning an `Instrumented` wrapper. Read more

[Source][66]§

### impl<T, U> [Into][30]<U> for T

where U: [From][61]<T>,

[Source][67]§

#### fn [into][68](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][61]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][47],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][69]§

### impl<T> [Same][70] for T

[Source][71]§

#### type [Output][72] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][47],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][47],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][73]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][47],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][47],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][47],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][47],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][47],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][47],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][47],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][74] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][47], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][47], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][47],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][47],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][47],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][77]) -> RequestBodyLimit<Self>

where Self: [Sized][47],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][47],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][47],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][78]§

### impl<T, U> [TryFrom][79]<U> for T

where U: [Into][30]<T>,

[Source][80]§

#### type [Error][81] = [Infallible][26]

The type returned in the event of a conversion error.

[Source][82]§

#### fn [try_from][83](value: U) -> [Result][84]<T, <T as [TryFrom][79]<U>>::[Error][85]>

Performs the conversion.

[Source][86]§

### impl<T, U> [TryInto][87]<U> for T

where U: [TryFrom][79]<T>,

[Source][88]§

#### type [Error][89] = <U as [TryFrom][79]<T>>::[Error][85]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_into][91](self) -> [Result][84]<U, <U as [TryFrom][79]<T>>::[Error][85]>

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

where S: [Into][30]<Dispatch>,

Attaches the provided [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][93] [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/mod.rs.html#121-125
   [5]: ../fn.serve.html (fn axum::serve)
   [6]: ../../src/axum/serve/mod.rs.html#128-173
   [7]: struct.Serve.html (struct axum::serve::Serve)
   [8]: trait.Listener.html (trait axum::serve::Listener)
   [9]: ../../src/axum/serve/mod.rs.html#157-167
   [10]: struct.WithGracefulShutdown.html (struct axum::serve::WithGracefulShutdown)
   [11]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [12]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [13]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [14]: ../../src/axum/serve/mod.rs.html#170-172
   [15]: https://doc.rust-lang.org/nightly/std/io/error/type.Result.html (type std::io::error::Result)
   [16]: trait.Listener.html#associatedtype.Addr (type axum::serve::Listener::Addr)
   [17]: ../../src/axum/serve/mod.rs.html#206-224
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [19]: ../../src/axum/serve/mod.rs.html#211-223
   [20]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [21]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [22]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [23]: ../../src/axum/serve/mod.rs.html#227-245
   [24]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html (trait core::future::into_future::IntoFuture)
   [25]: struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [26]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [27]: ../extract/type.Request.html (type axum::extract::Request)
   [28]: ../response/type.Response.html (type axum::response::Response)
   [29]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [30]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [31]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [32]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [34]: ../../src/axum/serve/mod.rs.html#239
   [35]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.Output
   [36]: ../../src/axum/serve/mod.rs.html#240
   [37]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture
   [38]: ../../src/axum/serve/mod.rs.html#242-244
   [39]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future
   [40]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture (type core::future::into_future::IntoFuture::IntoFuture)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [42]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [44]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [45]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [46]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [47]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [48]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [49]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [50]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [55]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [56]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [57]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [58]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [59]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [64]: super::Span::current()
   [65]: crate::Span
   [66]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [68]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [69]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [70]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [71]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [72]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [73]: https://docs.rs/http/latest/http/struct.Extensions.html
   [74]: crate::follow_redirect::policy::Standard
   [75]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [76]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [77]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [84]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [92]: super::Subscriber
   [93]: dispatcher#setting-the-default-subscriber

