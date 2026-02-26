<!-- Generated from rustdoc HTML: extract/path/struct.FailedToDeserializePathParams.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FailedToDeserializePathParams

## [axum][1]0.8.8

## FailedToDeserializePathParams

### Methods

  * body_text
  * into_kind
  * kind
  * status



### Trait Implementations

  * Debug
  * Display
  * Error
  * From<FailedToDeserializePathParams>
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
  * From<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ToString
  * ToStringFallible
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract::path][2]

[axum][3]::[extract][4]::[path][2]

# Struct FailedToDeserializePathParams Copy item path

[Source][5]
``` 
pub struct FailedToDeserializePathParams(/* private fields */);
```

Expand description

Rejection type for [`Path`][6] if the captured routes params couldn’t be deserialized into the expected type.

## Implementations§

[Source][7]§

### impl [FailedToDeserializePathParams][8]

[Source][9]

#### pub fn kind(&self) -> &[ErrorKind][10]

Get a reference to the underlying error kind.

[Source][11]

#### pub fn into_kind(self) -> [ErrorKind][10]

Convert this error into the underlying error kind.

[Source][12]

#### pub fn body_text(&self) -> [String][13]

Get the response body text used for this rejection.

[Source][14]

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

[Source][15]§

### impl [Debug][16] for [FailedToDeserializePathParams][8]

[Source][15]§

#### fn [fmt][17](&self, f: &mut [Formatter][18]<'_>) -> [Result][19]

Formats the value using the given formatter. [Read more][17]

[Source][20]§

### impl [Display][21] for [FailedToDeserializePathParams][8]

[Source][22]§

#### fn [fmt][23](&self, f: &mut [Formatter][18]<'_>) -> [Result][19]

Formats the value using the given formatter. [Read more][23]

[Source][24]§

### impl [Error][25] for [FailedToDeserializePathParams][8]

1.30.0 · [Source][26]§

#### fn [source][27](&self) -> [Option][28]<&(dyn [Error][25] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][27]

1.0.0 · [Source][29]§

#### fn [description][30](&self) -> &[str][31]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][30]

1.0.0 · [Source][32]§

#### fn [cause][33](&self) -> [Option][28]<&dyn [Error][25]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][34]§

#### fn [provide][35]<'a>(&'a self, request: &mut [Request][36]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][35]

[Source][37]§

### impl [From][38]<[FailedToDeserializePathParams][8]> for [PathRejection][39]

[Source][37]§

#### fn [from][40](inner: [FailedToDeserializePathParams][8]) -> Self

Converts to this type from the input type.

[Source][41]§

### impl [IntoResponse][42] for [FailedToDeserializePathParams][8]

[Source][43]§

#### fn [into_response][44](self) -> [Response][45]

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][46] for [FailedToDeserializePathParams][8]

§

### impl [RefUnwindSafe][47] for [FailedToDeserializePathParams][8]

§

### impl [Send][48] for [FailedToDeserializePathParams][8]

§

### impl [Sync][49] for [FailedToDeserializePathParams][8]

§

### impl [Unpin][50] for [FailedToDeserializePathParams][8]

§

### impl [UnwindSafe][51] for [FailedToDeserializePathParams][8]

## Blanket Implementations§

[Source][52]§

### impl<T> [Any][53] for T

where T: 'static + ?[Sized][54],

[Source][55]§

#### fn [type_id][56](&self) -> [TypeId][57]

Gets the `TypeId` of `self`. [Read more][56]

[Source][58]§

### impl<T> [Borrow][59]<T> for T

where T: ?[Sized][54],

[Source][60]§

#### fn [borrow][61](&self) -> [&T][62]

Immutably borrows from an owned value. [Read more][61]

[Source][63]§

### impl<T> [BorrowMut][64]<T> for T

where T: ?[Sized][54],

[Source][65]§

#### fn [borrow_mut][66](&mut self) -> [&mut T][62]

Mutably borrows from an owned value. [Read more][66]

[Source][67]§

### impl<T> [From][38]<T> for T

[Source][68]§

#### fn [from][40](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][69] [`Span`][70], returning an `Instrumented` wrapper. Read more

[Source][71]§

### impl<T, U> [Into][72]<U> for T

where U: [From][38]<T>,

[Source][73]§

#### fn [into][74](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][38]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][54],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][75]§

### impl<T> [Same][76] for T

[Source][77]§

#### type [Output][78] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][54],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][54],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][79]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][54],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][54],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][54],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][54],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][54],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][54],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][54],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][80] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][81]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][54],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][82] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][81]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][54],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][82] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][81]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][54],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][82] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][54], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][54], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][54],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][54],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][54],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][83]) -> RequestBodyLimit<Self>

where Self: [Sized][54],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][54],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][54],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][84]§

### impl<T> [ToString][85] for T

where T: [Display][21] \+ ?[Sized][54],

[Source][86]§

#### fn [to_string][87](&self) -> [String][13]

Converts the given value to a `String`. [Read more][87]

§

### impl<T> ToStringFallible for T

where T: [Display][21],

§

#### fn try_to_string(&self) -> [Result][88]<[String][13], [TryReserveError][89]>

[`ToString::to_string`][90], but without panic on OOM.

[Source][91]§

### impl<T, U> [TryFrom][92]<U> for T

where U: [Into][72]<T>,

[Source][93]§

#### type [Error][94] = [Infallible][95]

The type returned in the event of a conversion error.

[Source][96]§

#### fn [try_from][97](value: U) -> [Result][88]<T, <T as [TryFrom][92]<U>>::[Error][98]>

Performs the conversion.

[Source][99]§

### impl<T, U> [TryInto][100]<U> for T

where U: [TryFrom][92]<T>,

[Source][101]§

#### type [Error][102] = <U as [TryFrom][92]<T>>::[Error][98]

The type returned in the event of a conversion error.

[Source][103]§

#### fn [try_into][104](self) -> [Result][88]<U, <U as [TryFrom][92]<T>>::[Error][98]>

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

where S: [Into][72]<Dispatch>,

Attaches the provided [`Subscriber`][105] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][106] [`Subscriber`][105] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/path/mod.rs.html#407
   [6]: ../struct.Path.html (struct axum::extract::Path)
   [7]: ../../../src/axum/extract/path/mod.rs.html#409-451
   [8]: struct.FailedToDeserializePathParams.html (struct axum::extract::path::FailedToDeserializePathParams)
   [9]: ../../../src/axum/extract/path/mod.rs.html#411-413
   [10]: enum.ErrorKind.html (enum axum::extract::path::ErrorKind)
   [11]: ../../../src/axum/extract/path/mod.rs.html#416-418
   [12]: ../../../src/axum/extract/path/mod.rs.html#422-434
   [13]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [14]: ../../../src/axum/extract/path/mod.rs.html#438-450
   [15]: ../../../src/axum/extract/path/mod.rs.html#406
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [18]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [20]: ../../../src/axum/extract/path/mod.rs.html#465-469
   [21]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [22]: ../../../src/axum/extract/path/mod.rs.html#466-468
   [23]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [24]: ../../../src/axum/extract/path/mod.rs.html#471
   [25]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [26]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#111
   [27]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [28]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [29]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#137
   [30]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [31]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [32]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [33]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [34]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [35]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [36]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [37]: ../../../src/axum/extract/rejection.rs.html#151-160
   [38]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [39]: ../rejection/enum.PathRejection.html (enum axum::extract::rejection::PathRejection)
   [40]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [41]: ../../../src/axum/extract/path/mod.rs.html#453-463
   [42]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [43]: ../../../src/axum/extract/path/mod.rs.html#454-462
   [44]: ../../response/trait.IntoResponse.html#tymethod.into_response
   [45]: ../../response/type.Response.html (type axum::response::Response)
   [46]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [47]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [48]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [49]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [50]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [51]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [52]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [53]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [54]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [55]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [56]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [57]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [58]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [59]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [60]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [61]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [62]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [63]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [64]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [65]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [66]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [68]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [69]: super::Span::current()
   [70]: crate::Span
   [71]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [72]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [75]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [76]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [77]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [78]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [79]: https://docs.rs/http/latest/http/struct.Extensions.html
   [80]: crate::follow_redirect::policy::Standard
   [81]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [82]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [83]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [84]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [85]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [86]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [87]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [88]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [89]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [90]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [95]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
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

