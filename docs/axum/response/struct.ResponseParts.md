<!-- Generated from rustdoc HTML: response/struct.ResponseParts.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ResponseParts

## [axum][1]0.8.8

## ResponseParts

### Methods

  * extensions
  * extensions_mut
  * headers
  * headers_mut



### Trait Implementations

  * Debug



### Auto Trait Implementations

  * !RefUnwindSafe
  * !Sync
  * !UnwindSafe
  * Freeze
  * Send
  * Unpin



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



## [In axum::response][2]

[axum][3]::[response][2]

# Struct ResponseParts Copy item path
```
pub struct ResponseParts { /* private fields */ }
```

Expand description

Parts of a response.

Used with [`IntoResponseParts`][4].

## Implementations§

§

### impl [ResponseParts][5]

#### pub fn headers(&self) -> &HeaderMap

Gets a reference to the response headers.

#### pub fn headers_mut(&mut self) -> &mut HeaderMap

Gets a mutable reference to the response headers.

#### pub fn extensions(&self) -> &Extensions

Gets a reference to the response extensions.

#### pub fn extensions_mut(&mut self) -> &mut Extensions

Gets a mutable reference to the response extensions.

## Trait Implementations§

§

### impl [Debug][6] for [ResponseParts][5]

§

#### fn [fmt][7](&self, f: &mut [Formatter][8]<'_>) -> [Result][9]<[()][10], [Error][11]>

Formats the value using the given formatter. [Read more][7]

## Auto Trait Implementations§

§

### impl [Freeze][12] for [ResponseParts][5]

§

### impl ![RefUnwindSafe][13] for [ResponseParts][5]

§

### impl [Send][14] for [ResponseParts][5]

§

### impl ![Sync][15] for [ResponseParts][5]

§

### impl [Unpin][16] for [ResponseParts][5]

§

### impl ![UnwindSafe][17] for [ResponseParts][5]

## Blanket Implementations§

[Source][18]§

### impl<T> [Any][19] for T

where T: 'static + ?[Sized][20],

[Source][21]§

#### fn [type_id][22](&self) -> [TypeId][23]

Gets the `TypeId` of `self`. [Read more][22]

[Source][24]§

### impl<T> [Borrow][25]<T> for T

where T: ?[Sized][20],

[Source][26]§

#### fn [borrow][27](&self) -> [&T][28]

Immutably borrows from an owned value. [Read more][27]

[Source][29]§

### impl<T> [BorrowMut][30]<T> for T

where T: ?[Sized][20],

[Source][31]§

#### fn [borrow_mut][32](&mut self) -> [&mut T][28]

Mutably borrows from an owned value. [Read more][32]

[Source][33]§

### impl<T> [From][34]<T> for T

[Source][35]§

#### fn [from][36](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][37] [`Span`][38], returning an `Instrumented` wrapper. Read more

[Source][39]§

### impl<T, U> [Into][40]<U> for T

where U: [From][34]<T>,

[Source][41]§

#### fn [into][42](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][34]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][20],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][43]§

### impl<T> [Same][44] for T

[Source][45]§

#### type [Output][46] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][20],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][20],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][47]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][20],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][20],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][20],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][20],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][20],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][20],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][20],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][48] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][49]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][20],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][50] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][49]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][20],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][50] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][49]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][20],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][50] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][20],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][20],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][20],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][20],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][20],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][20],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][20], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][20], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][20],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][20],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][20],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][51]) -> RequestBodyLimit<Self>

where Self: [Sized][20],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][20],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][20],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][52]§

### impl<T, U> [TryFrom][53]<U> for T

where U: [Into][40]<T>,

[Source][54]§

#### type [Error][55] = [Infallible][56]

The type returned in the event of a conversion error.

[Source][57]§

#### fn [try_from][58](value: U) -> [Result][9]<T, <T as [TryFrom][53]<U>>::[Error][59]>

Performs the conversion.

[Source][60]§

### impl<T, U> [TryInto][61]<U> for T

where U: [TryFrom][53]<T>,

[Source][62]§

#### type [Error][63] = <U as [TryFrom][53]<T>>::[Error][59]

The type returned in the event of a conversion error.

[Source][64]§

#### fn [try_into][65](self) -> [Result][9]<U, <U as [TryFrom][53]<T>>::[Error][59]>

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

where S: [Into][40]<Dispatch>,

Attaches the provided [`Subscriber`][66] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][67] [`Subscriber`][66] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: trait.IntoResponseParts.html (trait axum::response::IntoResponseParts)
   [5]: struct.ResponseParts.html (struct axum::response::ResponseParts)
   [6]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [7]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [8]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [9]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [10]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [11]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [12]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [13]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [14]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [15]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [16]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [17]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [18]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [19]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [20]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [21]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [22]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [23]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [24]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [25]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [26]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [27]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [28]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [29]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [30]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [31]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [32]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [33]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [34]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [35]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [36]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [37]: super::Span::current()
   [38]: crate::Span
   [39]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [40]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [41]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [42]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [43]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [44]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [45]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [46]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [47]: https://docs.rs/http/latest/http/struct.Extensions.html
   [48]: crate::follow_redirect::policy::Standard
   [49]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [50]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [51]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [55]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [56]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [66]: super::Subscriber
   [67]: dispatcher#setting-the-default-subscriber

