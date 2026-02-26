<!-- Generated from rustdoc HTML: response/struct.ErrorResponse.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ErrorResponse

## [axum][1]0.8.8

## ErrorResponse

### Trait Implementations

  * Debug
  * From<T>



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

# Struct ErrorResponse Copy item path
```
pub struct ErrorResponse(/* private fields */);
```

Expand description

An [`IntoResponse`][4]-based error type

See [`Result`][5] for more details.

## Trait Implementations§

§

### impl [Debug][6] for [ErrorResponse][7]

§

#### fn [fmt][8](&self, f: &mut [Formatter][9]<'_>) -> [Result][10]<[()][11], [Error][12]>

Formats the value using the given formatter. [Read more][8]

§

### impl<T> [From][13]<T> for [ErrorResponse][7]

where T: [IntoResponse][4],

§

#### fn [from][14](value: T) -> [ErrorResponse][7]

Converts to this type from the input type.

## Auto Trait Implementations§

§

### impl [Freeze][15] for [ErrorResponse][7]

§

### impl ![RefUnwindSafe][16] for [ErrorResponse][7]

§

### impl [Send][17] for [ErrorResponse][7]

§

### impl ![Sync][18] for [ErrorResponse][7]

§

### impl [Unpin][19] for [ErrorResponse][7]

§

### impl ![UnwindSafe][20] for [ErrorResponse][7]

## Blanket Implementations§

[Source][21]§

### impl<T> [Any][22] for T

where T: 'static + ?[Sized][23],

[Source][24]§

#### fn [type_id][25](&self) -> [TypeId][26]

Gets the `TypeId` of `self`. [Read more][25]

[Source][27]§

### impl<T> [Borrow][28]<T> for T

where T: ?[Sized][23],

[Source][29]§

#### fn [borrow][30](&self) -> [&T][31]

Immutably borrows from an owned value. [Read more][30]

[Source][32]§

### impl<T> [BorrowMut][33]<T> for T

where T: ?[Sized][23],

[Source][34]§

#### fn [borrow_mut][35](&mut self) -> [&mut T][31]

Mutably borrows from an owned value. [Read more][35]

[Source][36]§

### impl<T> [From][13]<T> for T

[Source][37]§

#### fn [from][14](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][38] [`Span`][39], returning an `Instrumented` wrapper. Read more

[Source][40]§

### impl<T, U> [Into][41]<U> for T

where U: [From][13]<T>,

[Source][42]§

#### fn [into][43](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][13]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][23],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][44]§

### impl<T> [Same][45] for T

[Source][46]§

#### type [Output][47] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][23],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][23],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][48]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][23],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][23],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][23],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][23],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][23],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][23],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][23],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][49] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][50]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][23],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][51] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][50]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][23],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][51] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][50]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][23],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][51] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][23],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][23],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][23],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][23],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][23],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][23],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][23], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][23], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][23],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][23],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][23],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][52]) -> RequestBodyLimit<Self>

where Self: [Sized][23],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][23],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][23],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][53]§

### impl<T, U> [TryFrom][54]<U> for T

where U: [Into][41]<T>,

[Source][55]§

#### type [Error][56] = [Infallible][57]

The type returned in the event of a conversion error.

[Source][58]§

#### fn [try_from][59](value: U) -> [Result][10]<T, <T as [TryFrom][54]<U>>::[Error][60]>

Performs the conversion.

[Source][61]§

### impl<T, U> [TryInto][62]<U> for T

where U: [TryFrom][54]<T>,

[Source][63]§

#### type [Error][64] = <U as [TryFrom][54]<T>>::[Error][60]

The type returned in the event of a conversion error.

[Source][65]§

#### fn [try_into][66](self) -> [Result][10]<U, <U as [TryFrom][54]<T>>::[Error][60]>

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

where S: [Into][41]<Dispatch>,

Attaches the provided [`Subscriber`][67] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][68] [`Subscriber`][67] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [5]: type.Result.html (type axum::response::Result)
   [6]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [7]: struct.ErrorResponse.html (struct axum::response::ErrorResponse)
   [8]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [9]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [10]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [11]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [12]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [13]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [14]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [15]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [16]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [17]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [18]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [19]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [20]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [21]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [22]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [24]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [25]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [26]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [27]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [28]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [29]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [30]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [31]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [32]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [33]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [34]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [35]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [36]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [37]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [38]: super::Span::current()
   [39]: crate::Span
   [40]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [41]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [42]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [43]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [44]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [45]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [46]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [47]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [48]: https://docs.rs/http/latest/http/struct.Extensions.html
   [49]: crate::follow_redirect::policy::Standard
   [50]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [51]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [52]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [53]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [54]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [56]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [57]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [60]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [67]: super::Subscriber
   [68]: dispatcher#setting-the-default-subscriber

