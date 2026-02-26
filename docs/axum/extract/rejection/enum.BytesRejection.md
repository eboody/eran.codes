<!-- Generated from rustdoc HTML: extract/rejection/enum.BytesRejection.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## BytesRejection

## [axum][1]0.8.8

## BytesRejection

### Variants

  * FailedToBufferBody



### Methods

  * body_text
  * status



### Trait Implementations

  * Debug
  * Display
  * Error
  * From<BytesRejection>
  * From<BytesRejection>
  * From<BytesRejection>
  * From<FailedToBufferBody>
  * IntoResponse



### Auto Trait Implementations

  * !RefUnwindSafe
  * !UnwindSafe
  * Freeze
  * Send
  * Sync
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
  * ToString
  * ToStringFallible
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract::rejection][2]

[axum][3]::[extract][4]::[rejection][2]

# Enum BytesRejection Copy item path
```
#[non_exhaustive]

pub enum BytesRejection {
    FailedToBufferBody([FailedToBufferBody][5]),
}
```

Expand description

Rejection used for [`Bytes`][6].

Contains one variant for each way the [`Bytes`][6] extractor can fail.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

§

### FailedToBufferBody([FailedToBufferBody][5])

## Implementations§

§

### impl [BytesRejection][7]

#### pub fn body_text(&self) -> [String][8]

Get the response body text used for this rejection.

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

§

### impl [Debug][9] for [BytesRejection][7]

§

#### fn [fmt][10](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]<[()][13], [Error][14]>

Formats the value using the given formatter. [Read more][10]

§

### impl [Display][15] for [BytesRejection][7]

§

#### fn [fmt][16](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]<[()][13], [Error][14]>

Formats the value using the given formatter. [Read more][16]

§

### impl [Error][17] for [BytesRejection][7]

§

#### fn [source][18](&self) -> [Option][19]<&(dyn [Error][17] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][18]

1.0.0 · [Source][20]§

#### fn [description][21](&self) -> &[str][22]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][21]

1.0.0 · [Source][23]§

#### fn [cause][24](&self) -> [Option][19]<&dyn [Error][17]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][25]§

#### fn [provide][26]<'a>(&'a self, request: &mut [Request][27]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][26]

[Source][28]§

### impl [From][29]<[BytesRejection][7]> for [FormRejection][30]

[Source][28]§

#### fn [from][31](inner: [BytesRejection][7]) -> Self

Converts to this type from the input type.

[Source][32]§

### impl [From][29]<[BytesRejection][7]> for [JsonRejection][33]

[Source][32]§

#### fn [from][31](inner: [BytesRejection][7]) -> Self

Converts to this type from the input type.

[Source][34]§

### impl [From][29]<[BytesRejection][7]> for [RawFormRejection][35]

[Source][34]§

#### fn [from][31](inner: [BytesRejection][7]) -> Self

Converts to this type from the input type.

§

### impl [From][29]<[FailedToBufferBody][5]> for [BytesRejection][7]

§

#### fn [from][31](inner: [FailedToBufferBody][5]) -> [BytesRejection][7]

Converts to this type from the input type.

§

### impl [IntoResponse][36] for [BytesRejection][7]

§

#### fn [into_response][37](self) -> Response<[Body][38]>

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][39] for [BytesRejection][7]

§

### impl ![RefUnwindSafe][40] for [BytesRejection][7]

§

### impl [Send][41] for [BytesRejection][7]

§

### impl [Sync][42] for [BytesRejection][7]

§

### impl [Unpin][43] for [BytesRejection][7]

§

### impl ![UnwindSafe][44] for [BytesRejection][7]

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

### impl<T> [From][29]<T> for T

[Source][61]§

#### fn [from][31](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][62] [`Span`][63], returning an `Instrumented` wrapper. Read more

[Source][64]§

### impl<T, U> [Into][65]<U> for T

where U: [From][29]<T>,

[Source][66]§

#### fn [into][67](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][29]<T> for U` chooses to do.

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

[Source][68]§

### impl<T> [Same][69] for T

[Source][70]§

#### type [Output][71] = T

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

Add some shareable value to [request extensions][72]. Read more

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

Follow redirect resposes using the [`Standard`][73] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][74]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][75] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][74]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][75] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][74]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][75] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][76]) -> RequestBodyLimit<Self>

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

[Source][77]§

### impl<T> [ToString][78] for T

where T: [Display][15] \+ ?[Sized][47],

[Source][79]§

#### fn [to_string][80](&self) -> [String][8]

Converts the given value to a `String`. [Read more][80]

§

### impl<T> ToStringFallible for T

where T: [Display][15],

§

#### fn try_to_string(&self) -> [Result][12]<[String][8], [TryReserveError][81]>

[`ToString::to_string`][82], but without panic on OOM.

[Source][83]§

### impl<T, U> [TryFrom][84]<U> for T

where U: [Into][65]<T>,

[Source][85]§

#### type [Error][86] = [Infallible][87]

The type returned in the event of a conversion error.

[Source][88]§

#### fn [try_from][89](value: U) -> [Result][12]<T, <T as [TryFrom][84]<U>>::[Error][90]>

Performs the conversion.

[Source][91]§

### impl<T, U> [TryInto][92]<U> for T

where U: [TryFrom][84]<T>,

[Source][93]§

#### type [Error][94] = <U as [TryFrom][84]<T>>::[Error][90]

The type returned in the event of a conversion error.

[Source][95]§

#### fn [try_into][96](self) -> [Result][12]<U, <U as [TryFrom][84]<T>>::[Error][90]>

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

where S: [Into][65]<Dispatch>,

Attaches the provided [`Subscriber`][97] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][98] [`Subscriber`][97] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: enum.FailedToBufferBody.html (enum axum::extract::rejection::FailedToBufferBody)
   [6]: bytes::Bytes
   [7]: enum.BytesRejection.html (enum axum::extract::rejection::BytesRejection)
   [8]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [9]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [11]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [12]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [13]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [18]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [19]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [20]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#137
   [21]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [22]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [23]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [24]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [25]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [26]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [27]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [28]: ../../../src/axum/extract/rejection.rs.html#102-113
   [29]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [30]: enum.FormRejection.html (enum axum::extract::rejection::FormRejection)
   [31]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [32]: ../../../src/axum/extract/rejection.rs.html#127-139
   [33]: enum.JsonRejection.html (enum axum::extract::rejection::JsonRejection)
   [34]: ../../../src/axum/extract/rejection.rs.html#115-124
   [35]: enum.RawFormRejection.html (enum axum::extract::rejection::RawFormRejection)
   [36]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [37]: ../../response/trait.IntoResponse.html#tymethod.into_response
   [38]: ../../body/struct.Body.html (struct axum::body::Body)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
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
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [62]: super::Span::current()
   [63]: crate::Span
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [66]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [67]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [70]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [71]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [72]: https://docs.rs/http/latest/http/struct.Extensions.html
   [73]: crate::follow_redirect::policy::Standard
   [74]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [75]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [76]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [77]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [78]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [79]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [80]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [81]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [82]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [87]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [90]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [95]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [96]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [97]: super::Subscriber
   [98]: dispatcher#setting-the-default-subscriber

