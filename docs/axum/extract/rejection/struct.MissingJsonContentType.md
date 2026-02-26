<!-- Generated from rustdoc HTML: extract/rejection/struct.MissingJsonContentType.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MissingJsonContentType

## [axum][1]0.8.8

## MissingJsonContentType

### Methods

  * body_text
  * status



### Trait Implementations

  * Debug
  * Default
  * Display
  * Error
  * From<MissingJsonContentType>
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

# Struct MissingJsonContentType Copy item path

[Source][5]
``` 
#[non_exhaustive]

pub struct MissingJsonContentType;
```

Available on **crate feature`json`** only.

Expand description

Rejection type for [`Json`][6] used if the `Content-Type` header is missing.

## Implementations§

[Source][5]§

### impl [MissingJsonContentType][7]

[Source][5]

#### pub fn body_text(&self) -> [String][8]

Get the response body text used for this rejection.

[Source][5]

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

[Source][5]§

### impl [Debug][9] for [MissingJsonContentType][7]

[Source][5]§

#### fn [fmt][10](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]

Formats the value using the given formatter. [Read more][10]

[Source][5]§

### impl [Default][13] for [MissingJsonContentType][7]

[Source][5]§

#### fn [default][14]() -> Self

Returns the “default value” for a type. [Read more][14]

[Source][5]§

### impl [Display][15] for [MissingJsonContentType][7]

[Source][5]§

#### fn [fmt][16](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]

Formats the value using the given formatter. [Read more][16]

[Source][5]§

### impl [Error][17] for [MissingJsonContentType][7]

1.30.0 · [Source][18]§

#### fn [source][19](&self) -> [Option][20]<&(dyn [Error][17] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][19]

1.0.0 · [Source][21]§

#### fn [description][22](&self) -> &[str][23]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][22]

1.0.0 · [Source][24]§

#### fn [cause][25](&self) -> [Option][20]<&dyn [Error][17]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][26]§

#### fn [provide][27]<'a>(&'a self, request: &mut [Request][28]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][27]

[Source][29]§

### impl [From][30]<[MissingJsonContentType][7]> for [JsonRejection][31]

[Source][29]§

#### fn [from][32](inner: [MissingJsonContentType][7]) -> Self

Converts to this type from the input type.

[Source][5]§

### impl [IntoResponse][33] for [MissingJsonContentType][7]

[Source][5]§

#### fn [into_response][34](self) -> [Response][35]

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][36] for [MissingJsonContentType][7]

§

### impl [RefUnwindSafe][37] for [MissingJsonContentType][7]

§

### impl [Send][38] for [MissingJsonContentType][7]

§

### impl [Sync][39] for [MissingJsonContentType][7]

§

### impl [Unpin][40] for [MissingJsonContentType][7]

§

### impl [UnwindSafe][41] for [MissingJsonContentType][7]

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

### impl<T> [From][30]<T> for T

[Source][58]§

#### fn [from][32](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][59] [`Span`][60], returning an `Instrumented` wrapper. Read more

[Source][61]§

### impl<T, U> [Into][62]<U> for T

where U: [From][30]<T>,

[Source][63]§

#### fn [into][64](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][30]<T> for U` chooses to do.

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

[Source][65]§

### impl<T> [Same][66] for T

[Source][67]§

#### type [Output][68] = T

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

Add some shareable value to [request extensions][69]. Read more

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

Follow redirect resposes using the [`Standard`][70] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][71]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][72] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][71]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][72] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][71]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][72] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][73]) -> RequestBodyLimit<Self>

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

[Source][74]§

### impl<T> [ToString][75] for T

where T: [Display][15] \+ ?[Sized][44],

[Source][76]§

#### fn [to_string][77](&self) -> [String][8]

Converts the given value to a `String`. [Read more][77]

§

### impl<T> ToStringFallible for T

where T: [Display][15],

§

#### fn try_to_string(&self) -> [Result][78]<[String][8], [TryReserveError][79]>

[`ToString::to_string`][80], but without panic on OOM.

[Source][81]§

### impl<T, U> [TryFrom][82]<U> for T

where U: [Into][62]<T>,

[Source][83]§

#### type [Error][84] = [Infallible][85]

The type returned in the event of a conversion error.

[Source][86]§

#### fn [try_from][87](value: U) -> [Result][78]<T, <T as [TryFrom][82]<U>>::[Error][88]>

Performs the conversion.

[Source][89]§

### impl<T, U> [TryInto][90]<U> for T

where U: [TryFrom][82]<T>,

[Source][91]§

#### type [Error][92] = <U as [TryFrom][82]<T>>::[Error][88]

The type returned in the event of a conversion error.

[Source][93]§

#### fn [try_into][94](self) -> [Result][78]<U, <U as [TryFrom][82]<T>>::[Error][88]>

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

where S: [Into][62]<Dispatch>,

Attaches the provided [`Subscriber`][95] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][96] [`Subscriber`][95] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/rejection.rs.html#33-40
   [6]: ../../struct.Json.html (struct axum::Json)
   [7]: struct.MissingJsonContentType.html (struct axum::extract::rejection::MissingJsonContentType)
   [8]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [9]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [11]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [13]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [14]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [18]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#111
   [19]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [20]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [21]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#137
   [22]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [23]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [24]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [25]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [26]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [27]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [28]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [29]: ../../../src/axum/extract/rejection.rs.html#127-139
   [30]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [31]: enum.JsonRejection.html (enum axum::extract::rejection::JsonRejection)
   [32]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [33]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [34]: ../../response/trait.IntoResponse.html#tymethod.into_response
   [35]: ../../response/type.Response.html (type axum::response::Response)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [37]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
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
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [59]: super::Span::current()
   [60]: crate::Span
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [65]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [66]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [67]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [68]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [69]: https://docs.rs/http/latest/http/struct.Extensions.html
   [70]: crate::follow_redirect::policy::Standard
   [71]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [72]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [73]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [74]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [75]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [76]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [77]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [78]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [79]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [80]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [85]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [90]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [95]: super::Subscriber
   [96]: dispatcher#setting-the-default-subscriber

