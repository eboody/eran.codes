<!-- Generated from rustdoc HTML: extract/rejection/enum.FailedToBufferBody.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FailedToBufferBody

## [axum][1]0.8.8

## FailedToBufferBody

### Variants

  * LengthLimitError
  * UnknownBodyError



### Methods

  * body_text
  * status



### Trait Implementations

  * Debug
  * Display
  * Error
  * From<FailedToBufferBody>
  * From<FailedToBufferBody>
  * From<LengthLimitError>
  * From<UnknownBodyError>
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

# Enum FailedToBufferBody Copy item path
```
#[non_exhaustive]

pub enum FailedToBufferBody {
    LengthLimitError([LengthLimitError][5]),
    UnknownBodyError([UnknownBodyError][6]),
}
```

Expand description

Rejection type for extractors that buffer the request body. Used if the request body cannot be buffered due to an error.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

§

### LengthLimitError([LengthLimitError][5])

§

### UnknownBodyError([UnknownBodyError][6])

## Implementations§

§

### impl [FailedToBufferBody][7]

#### pub fn body_text(&self) -> [String][8]

Get the response body text used for this rejection.

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

§

### impl [Debug][9] for [FailedToBufferBody][7]

§

#### fn [fmt][10](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]<[()][13], [Error][14]>

Formats the value using the given formatter. [Read more][10]

§

### impl [Display][15] for [FailedToBufferBody][7]

§

#### fn [fmt][16](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]<[()][13], [Error][14]>

Formats the value using the given formatter. [Read more][16]

§

### impl [Error][17] for [FailedToBufferBody][7]

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

§

### impl [From][28]<[FailedToBufferBody][7]> for [BytesRejection][29]

§

#### fn [from][30](inner: [FailedToBufferBody][7]) -> [BytesRejection][29]

Converts to this type from the input type.

§

### impl [From][28]<[FailedToBufferBody][7]> for [StringRejection][31]

§

#### fn [from][30](inner: [FailedToBufferBody][7]) -> [StringRejection][31]

Converts to this type from the input type.

§

### impl [From][28]<[LengthLimitError][5]> for [FailedToBufferBody][7]

§

#### fn [from][30](inner: [LengthLimitError][5]) -> [FailedToBufferBody][7]

Converts to this type from the input type.

§

### impl [From][28]<[UnknownBodyError][6]> for [FailedToBufferBody][7]

§

#### fn [from][30](inner: [UnknownBodyError][6]) -> [FailedToBufferBody][7]

Converts to this type from the input type.

§

### impl [IntoResponse][32] for [FailedToBufferBody][7]

§

#### fn [into_response][33](self) -> Response<[Body][34]>

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][35] for [FailedToBufferBody][7]

§

### impl ![RefUnwindSafe][36] for [FailedToBufferBody][7]

§

### impl [Send][37] for [FailedToBufferBody][7]

§

### impl [Sync][38] for [FailedToBufferBody][7]

§

### impl [Unpin][39] for [FailedToBufferBody][7]

§

### impl ![UnwindSafe][40] for [FailedToBufferBody][7]

## Blanket Implementations§

[Source][41]§

### impl<T> [Any][42] for T

where T: 'static + ?[Sized][43],

[Source][44]§

#### fn [type_id][45](&self) -> [TypeId][46]

Gets the `TypeId` of `self`. [Read more][45]

[Source][47]§

### impl<T> [Borrow][48]<T> for T

where T: ?[Sized][43],

[Source][49]§

#### fn [borrow][50](&self) -> [&T][51]

Immutably borrows from an owned value. [Read more][50]

[Source][52]§

### impl<T> [BorrowMut][53]<T> for T

where T: ?[Sized][43],

[Source][54]§

#### fn [borrow_mut][55](&mut self) -> [&mut T][51]

Mutably borrows from an owned value. [Read more][55]

[Source][56]§

### impl<T> [From][28]<T> for T

[Source][57]§

#### fn [from][30](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][58] [`Span`][59], returning an `Instrumented` wrapper. Read more

[Source][60]§

### impl<T, U> [Into][61]<U> for T

where U: [From][28]<T>,

[Source][62]§

#### fn [into][63](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][28]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][43],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][64]§

### impl<T> [Same][65] for T

[Source][66]§

#### type [Output][67] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][43],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][43],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][68]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][43],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][43],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][43],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][69] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][43],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][72]) -> RequestBodyLimit<Self>

where Self: [Sized][43],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][73]§

### impl<T> [ToString][74] for T

where T: [Display][15] \+ ?[Sized][43],

[Source][75]§

#### fn [to_string][76](&self) -> [String][8]

Converts the given value to a `String`. [Read more][76]

§

### impl<T> ToStringFallible for T

where T: [Display][15],

§

#### fn try_to_string(&self) -> [Result][12]<[String][8], [TryReserveError][77]>

[`ToString::to_string`][78], but without panic on OOM.

[Source][79]§

### impl<T, U> [TryFrom][80]<U> for T

where U: [Into][61]<T>,

[Source][81]§

#### type [Error][82] = [Infallible][83]

The type returned in the event of a conversion error.

[Source][84]§

#### fn [try_from][85](value: U) -> [Result][12]<T, <T as [TryFrom][80]<U>>::[Error][86]>

Performs the conversion.

[Source][87]§

### impl<T, U> [TryInto][88]<U> for T

where U: [TryFrom][80]<T>,

[Source][89]§

#### type [Error][90] = <U as [TryFrom][80]<T>>::[Error][86]

The type returned in the event of a conversion error.

[Source][91]§

#### fn [try_into][92](self) -> [Result][12]<U, <U as [TryFrom][80]<T>>::[Error][86]>

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

where S: [Into][61]<Dispatch>,

Attaches the provided [`Subscriber`][93] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][94] [`Subscriber`][93] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: struct.LengthLimitError.html (struct axum::extract::rejection::LengthLimitError)
   [6]: struct.UnknownBodyError.html (struct axum::extract::rejection::UnknownBodyError)
   [7]: enum.FailedToBufferBody.html (enum axum::extract::rejection::FailedToBufferBody)
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
   [28]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [29]: enum.BytesRejection.html (enum axum::extract::rejection::BytesRejection)
   [30]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [31]: enum.StringRejection.html (enum axum::extract::rejection::StringRejection)
   [32]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [33]: ../../response/trait.IntoResponse.html#tymethod.into_response
   [34]: ../../body/struct.Body.html (struct axum::body::Body)
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [36]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [41]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [42]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [44]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [45]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [46]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [51]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [58]: super::Span::current()
   [59]: crate::Span
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [64]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [65]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [68]: https://docs.rs/http/latest/http/struct.Extensions.html
   [69]: crate::follow_redirect::policy::Standard
   [70]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [71]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [72]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [73]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [74]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [75]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [76]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [77]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [78]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [83]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [84]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [90]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [93]: super::Subscriber
   [94]: dispatcher#setting-the-default-subscriber

