<!-- Generated from rustdoc HTML: extract/ws/struct.DefaultOnFailedUpgrade.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## DefaultOnFailedUpgrade

## [axum][1]0.8.8

## DefaultOnFailedUpgrade

### Trait Implementations

  * Debug
  * OnFailedUpgrade



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



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Struct DefaultOnFailedUpgrade Copy item path

[Source][5]
``` 
#[non_exhaustive]

pub struct DefaultOnFailedUpgrade;
```

Available on **crate feature`ws`** only.

Expand description

The default `OnFailedUpgrade` used by `WebSocketUpgrade`.

It simply ignores the error.

## Trait Implementations§

[Source][6]§

### impl [Debug][7] for [DefaultOnFailedUpgrade][8]

[Source][6]§

#### fn [fmt][9](&self, f: &mut [Formatter][10]<'_>) -> [Result][11]

Formats the value using the given formatter. [Read more][9]

[Source][12]§

### impl [OnFailedUpgrade][13] for [DefaultOnFailedUpgrade][8]

[Source][14]§

#### fn [call][15](self, _error: [Error][16])

Call the callback.

## Auto Trait Implementations§

§

### impl [Freeze][17] for [DefaultOnFailedUpgrade][8]

§

### impl [RefUnwindSafe][18] for [DefaultOnFailedUpgrade][8]

§

### impl [Send][19] for [DefaultOnFailedUpgrade][8]

§

### impl [Sync][20] for [DefaultOnFailedUpgrade][8]

§

### impl [Unpin][21] for [DefaultOnFailedUpgrade][8]

§

### impl [UnwindSafe][22] for [DefaultOnFailedUpgrade][8]

## Blanket Implementations§

[Source][23]§

### impl<T> [Any][24] for T

where T: 'static + ?[Sized][25],

[Source][26]§

#### fn [type_id][27](&self) -> [TypeId][28]

Gets the `TypeId` of `self`. [Read more][27]

[Source][29]§

### impl<T> [Borrow][30]<T> for T

where T: ?[Sized][25],

[Source][31]§

#### fn [borrow][32](&self) -> [&T][33]

Immutably borrows from an owned value. [Read more][32]

[Source][34]§

### impl<T> [BorrowMut][35]<T> for T

where T: ?[Sized][25],

[Source][36]§

#### fn [borrow_mut][37](&mut self) -> [&mut T][33]

Mutably borrows from an owned value. [Read more][37]

[Source][38]§

### impl<T> [From][39]<T> for T

[Source][40]§

#### fn [from][41](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][42] [`Span`][43], returning an `Instrumented` wrapper. Read more

[Source][44]§

### impl<T, U> [Into][45]<U> for T

where U: [From][39]<T>,

[Source][46]§

#### fn [into][47](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][39]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][25],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][48]§

### impl<T> [Same][49] for T

[Source][50]§

#### type [Output][51] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][25],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][25],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][52]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][25],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][25],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][25],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][25],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][25],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][25],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][25],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][53] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][54]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][25],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][55] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][54]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][25],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][55] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][54]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][25],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][55] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][25],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][25],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][25],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][25],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][25],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][25],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][25], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][25], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][25],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][25],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][25],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][56]) -> RequestBodyLimit<Self>

where Self: [Sized][25],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][25],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][25],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][57]§

### impl<T, U> [TryFrom][58]<U> for T

where U: [Into][45]<T>,

[Source][59]§

#### type [Error][60] = [Infallible][61]

The type returned in the event of a conversion error.

[Source][62]§

#### fn [try_from][63](value: U) -> [Result][64]<T, <T as [TryFrom][58]<U>>::[Error][65]>

Performs the conversion.

[Source][66]§

### impl<T, U> [TryInto][67]<U> for T

where U: [TryFrom][58]<T>,

[Source][68]§

#### type [Error][69] = <U as [TryFrom][58]<T>>::[Error][65]

The type returned in the event of a conversion error.

[Source][70]§

#### fn [try_into][71](self) -> [Result][64]<U, <U as [TryFrom][58]<T>>::[Error][65]>

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

where S: [Into][45]<Dispatch>,

Attaches the provided [`Subscriber`][72] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][73] [`Subscriber`][72] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/ws.rs.html#435
   [6]: ../../../src/axum/extract/ws.rs.html#434
   [7]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [8]: struct.DefaultOnFailedUpgrade.html (struct axum::extract::ws::DefaultOnFailedUpgrade)
   [9]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [10]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [11]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [12]: ../../../src/axum/extract/ws.rs.html#437-440
   [13]: trait.OnFailedUpgrade.html (trait axum::extract::ws::OnFailedUpgrade)
   [14]: ../../../src/axum/extract/ws.rs.html#439
   [15]: trait.OnFailedUpgrade.html#tymethod.call
   [16]: ../../struct.Error.html (struct axum::Error)
   [17]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [18]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [19]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [20]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [21]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [22]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [23]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [24]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [26]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [27]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [28]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [29]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [30]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [31]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [32]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [33]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [34]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [35]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [36]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [37]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [38]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [39]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [40]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [41]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [42]: super::Span::current()
   [43]: crate::Span
   [44]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [45]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [46]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [47]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [48]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [49]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [50]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [51]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [52]: https://docs.rs/http/latest/http/struct.Extensions.html
   [53]: crate::follow_redirect::policy::Standard
   [54]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [55]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [56]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [59]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [60]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [61]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [64]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [66]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [67]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [68]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [69]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [70]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [71]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [72]: super::Subscriber
   [73]: dispatcher#setting-the-default-subscriber

