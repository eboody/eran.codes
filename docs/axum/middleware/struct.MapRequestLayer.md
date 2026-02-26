<!-- Generated from rustdoc HTML: middleware/struct.MapRequestLayer.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MapRequestLayer

## [axum][1]0.8.8

## MapRequestLayer

### Trait Implementations

  * Clone
  * Debug
  * Layer<I>



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



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Struct MapRequestLayer Copy item path

[Source][4]
``` 
pub struct MapRequestLayer<F, S, T> { /* private fields */ }
```

Expand description

A [`tower::Layer`] from an async function that transforms a request.

Created with [`map_request`][5]. See that function for more details.

## Trait Implementations§

[Source][6]§

### impl<F, S, T> [Clone][7] for [MapRequestLayer][8]<F, S, T>

where F: [Clone][7], S: [Clone][7],

[Source][9]§

#### fn [clone][10](&self) -> Self

Returns a duplicate of the value. [Read more][10]

1.0.0 · [Source][11]§

#### fn [clone_from][12](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][12]

[Source][13]§

### impl<F, S, T> [Debug][14] for [MapRequestLayer][8]<F, S, T>

where S: [Debug][14],

[Source][15]§

#### fn [fmt][16](&self, f: &mut [Formatter][17]<'_>) -> [Result][18]

Formats the value using the given formatter. [Read more][16]

[Source][19]§

### impl<S, I, F, T> Layer<I> for [MapRequestLayer][8]<F, S, T>

where F: [Clone][7], S: [Clone][7],

[Source][20]§

#### type Service = [MapRequest][21]<F, S, I, T>

The wrapped service

[Source][22]§

#### fn layer(&self, inner: I) -> Self::Service

Wrap the given service with the middleware, returning a new service that has been decorated with the middleware.

## Auto Trait Implementations§

§

### impl<F, S, T> [Freeze][23] for [MapRequestLayer][8]<F, S, T>

where F: [Freeze][23], S: [Freeze][23],

§

### impl<F, S, T> [RefUnwindSafe][24] for [MapRequestLayer][8]<F, S, T>

where F: [RefUnwindSafe][24], S: [RefUnwindSafe][24],

§

### impl<F, S, T> [Send][25] for [MapRequestLayer][8]<F, S, T>

where F: [Send][25], S: [Send][25],

§

### impl<F, S, T> [Sync][26] for [MapRequestLayer][8]<F, S, T>

where F: [Sync][26], S: [Sync][26],

§

### impl<F, S, T> [Unpin][27] for [MapRequestLayer][8]<F, S, T>

where F: [Unpin][27], S: [Unpin][27],

§

### impl<F, S, T> [UnwindSafe][28] for [MapRequestLayer][8]<F, S, T>

where F: [UnwindSafe][28], S: [UnwindSafe][28],

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

where T: [Clone][7],

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

where T: [Clone][7],

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

where T: [Clone][7],

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

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/map_request.rs.html#171-175
   [5]: fn.map_request.html (fn axum::middleware::map_request)
   [6]: ../../src/axum/middleware/map_request.rs.html#177-189
   [7]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [8]: struct.MapRequestLayer.html (struct axum::middleware::MapRequestLayer)
   [9]: ../../src/axum/middleware/map_request.rs.html#182-188
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [11]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [13]: ../../src/axum/middleware/map_request.rs.html#208-219
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: ../../src/axum/middleware/map_request.rs.html#212-218
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [19]: ../../src/axum/middleware/map_request.rs.html#191-206
   [20]: ../../src/axum/middleware/map_request.rs.html#196
   [21]: struct.MapRequest.html (struct axum::middleware::MapRequest)
   [22]: ../../src/axum/middleware/map_request.rs.html#198-205
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [24]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
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
   [54]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [55]: ../extract/trait.FromRef.html#tymethod.from_ref
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

