<!-- Generated from rustdoc HTML: extract/rejection/enum.RawPathParamsRejection.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RawPathParamsRejection

## [axum][1]0.8.8

## RawPathParamsRejection

### Variants

  * InvalidUtf8InPathParam
  * MissingPathParams



### Methods

  * body_text
  * status



### Trait Implementations

  * Debug
  * Display
  * Error
  * From<InvalidUtf8InPathParam>
  * From<MissingPathParams>
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



## [In axum::extract::rejection][2]

[axum][3]::[extract][4]::[rejection][2]

# Enum RawPathParamsRejection Copy item path

[Source][5]
``` 
#[non_exhaustive]

pub enum RawPathParamsRejection {
    InvalidUtf8InPathParam([InvalidUtf8InPathParam][6]),
    MissingPathParams([MissingPathParams][7]),
}
```

Expand description

Rejection used for [`RawPathParams`][8].

Contains one variant for each way the [`RawPathParams`][8] extractor can fail.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

§

### InvalidUtf8InPathParam([InvalidUtf8InPathParam][6])

§

### MissingPathParams([MissingPathParams][7])

## Implementations§

[Source][5]§

### impl [RawPathParamsRejection][9]

[Source][5]

#### pub fn body_text(&self) -> [String][10]

Get the response body text used for this rejection.

[Source][5]

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

[Source][5]§

### impl [Debug][11] for [RawPathParamsRejection][9]

[Source][5]§

#### fn [fmt][12](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]

Formats the value using the given formatter. [Read more][12]

[Source][5]§

### impl [Display][15] for [RawPathParamsRejection][9]

[Source][5]§

#### fn [fmt][16](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]

Formats the value using the given formatter. [Read more][16]

[Source][5]§

### impl [Error][17] for [RawPathParamsRejection][9]

[Source][5]§

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

[Source][5]§

### impl [From][28]<[InvalidUtf8InPathParam][6]> for [RawPathParamsRejection][9]

[Source][5]§

#### fn [from][29](inner: [InvalidUtf8InPathParam][6]) -> Self

Converts to this type from the input type.

[Source][5]§

### impl [From][28]<[MissingPathParams][7]> for [RawPathParamsRejection][9]

[Source][5]§

#### fn [from][29](inner: [MissingPathParams][7]) -> Self

Converts to this type from the input type.

[Source][5]§

### impl [IntoResponse][30] for [RawPathParamsRejection][9]

[Source][5]§

#### fn [into_response][31](self) -> [Response][32]

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][33] for [RawPathParamsRejection][9]

§

### impl [RefUnwindSafe][34] for [RawPathParamsRejection][9]

§

### impl [Send][35] for [RawPathParamsRejection][9]

§

### impl [Sync][36] for [RawPathParamsRejection][9]

§

### impl [Unpin][37] for [RawPathParamsRejection][9]

§

### impl [UnwindSafe][38] for [RawPathParamsRejection][9]

## Blanket Implementations§

[Source][39]§

### impl<T> [Any][40] for T

where T: 'static + ?[Sized][41],

[Source][42]§

#### fn [type_id][43](&self) -> [TypeId][44]

Gets the `TypeId` of `self`. [Read more][43]

[Source][45]§

### impl<T> [Borrow][46]<T> for T

where T: ?[Sized][41],

[Source][47]§

#### fn [borrow][48](&self) -> [&T][49]

Immutably borrows from an owned value. [Read more][48]

[Source][50]§

### impl<T> [BorrowMut][51]<T> for T

where T: ?[Sized][41],

[Source][52]§

#### fn [borrow_mut][53](&mut self) -> [&mut T][49]

Mutably borrows from an owned value. [Read more][53]

[Source][54]§

### impl<T> [From][28]<T> for T

[Source][55]§

#### fn [from][29](t: T) -> T

Returns the argument unchanged.

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

where U: [From][28]<T>,

[Source][60]§

#### fn [into][61](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][28]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][41],

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

where Self: [Sized][41],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][41],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][66]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][41],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][41],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][41],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][41],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][41],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][41],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][41],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][67] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][41], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][41], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][41],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][41],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][41],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][70]) -> RequestBodyLimit<Self>

where Self: [Sized][41],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][41],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][41],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][71]§

### impl<T> [ToString][72] for T

where T: [Display][15] \+ ?[Sized][41],

[Source][73]§

#### fn [to_string][74](&self) -> [String][10]

Converts the given value to a `String`. [Read more][74]

§

### impl<T> ToStringFallible for T

where T: [Display][15],

§

#### fn try_to_string(&self) -> [Result][75]<[String][10], [TryReserveError][76]>

[`ToString::to_string`][77], but without panic on OOM.

[Source][78]§

### impl<T, U> [TryFrom][79]<U> for T

where U: [Into][59]<T>,

[Source][80]§

#### type [Error][81] = [Infallible][82]

The type returned in the event of a conversion error.

[Source][83]§

#### fn [try_from][84](value: U) -> [Result][75]<T, <T as [TryFrom][79]<U>>::[Error][85]>

Performs the conversion.

[Source][86]§

### impl<T, U> [TryInto][87]<U> for T

where U: [TryFrom][79]<T>,

[Source][88]§

#### type [Error][89] = <U as [TryFrom][79]<T>>::[Error][85]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_into][91](self) -> [Result][75]<U, <U as [TryFrom][79]<T>>::[Error][85]>

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

Attaches the provided [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][93] [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/rejection.rs.html#162-171
   [6]: ../path/struct.InvalidUtf8InPathParam.html (struct axum::extract::path::InvalidUtf8InPathParam)
   [7]: struct.MissingPathParams.html (struct axum::extract::rejection::MissingPathParams)
   [8]: ../struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [9]: enum.RawPathParamsRejection.html (enum axum::extract::rejection::RawPathParamsRejection)
   [10]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [13]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
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
   [28]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [29]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [30]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [31]: ../../response/trait.IntoResponse.html#tymethod.into_response
   [32]: ../../response/type.Response.html (type axum::response::Response)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [34]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [39]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [40]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [42]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [43]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [44]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [45]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [46]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [49]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
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
   [71]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [72]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [73]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [74]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [75]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [76]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [77]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [82]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [92]: super::Subscriber
   [93]: dispatcher#setting-the-default-subscriber

