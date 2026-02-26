<!-- Generated from rustdoc HTML: extract/path/struct.InvalidUtf8InPathParam.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## InvalidUtf8InPathParam

## [axum][1]0.8.8

## InvalidUtf8InPathParam

### Methods

  * body_text
  * status



### Trait Implementations

  * Debug
  * Display
  * Error
  * From<InvalidUtf8InPathParam>
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

# Struct InvalidUtf8InPathParam Copy item path

[Source][5]
``` 
pub struct InvalidUtf8InPathParam { /* private fields */ }
```

Expand description

Rejection used by [`RawPathParams`][6] if a parameter contained text that, once percent decoded, wasn’t valid UTF-8.

## Implementations§

[Source][7]§

### impl [InvalidUtf8InPathParam][8]

[Source][9]

#### pub fn body_text(&self) -> [String][10]

Get the response body text used for this rejection.

[Source][11]

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

[Source][12]§

### impl [Debug][13] for [InvalidUtf8InPathParam][8]

[Source][12]§

#### fn [fmt][14](&self, f: &mut [Formatter][15]<'_>) -> [Result][16]

Formats the value using the given formatter. [Read more][14]

[Source][17]§

### impl [Display][18] for [InvalidUtf8InPathParam][8]

[Source][19]§

#### fn [fmt][20](&self, f: &mut [Formatter][15]<'_>) -> [Result][16]

Formats the value using the given formatter. [Read more][20]

[Source][21]§

### impl [Error][22] for [InvalidUtf8InPathParam][8]

1.30.0 · [Source][23]§

#### fn [source][24](&self) -> [Option][25]<&(dyn [Error][22] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][24]

1.0.0 · [Source][26]§

#### fn [description][27](&self) -> &[str][28]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][27]

1.0.0 · [Source][29]§

#### fn [cause][30](&self) -> [Option][25]<&dyn [Error][22]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][31]§

#### fn [provide][32]<'a>(&'a self, request: &mut [Request][33]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][32]

[Source][34]§

### impl [From][35]<[InvalidUtf8InPathParam][8]> for [RawPathParamsRejection][36]

[Source][34]§

#### fn [from][37](inner: [InvalidUtf8InPathParam][8]) -> Self

Converts to this type from the input type.

[Source][38]§

### impl [IntoResponse][39] for [InvalidUtf8InPathParam][8]

[Source][40]§

#### fn [into_response][41](self) -> [Response][42]

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][43] for [InvalidUtf8InPathParam][8]

§

### impl [RefUnwindSafe][44] for [InvalidUtf8InPathParam][8]

§

### impl [Send][45] for [InvalidUtf8InPathParam][8]

§

### impl [Sync][46] for [InvalidUtf8InPathParam][8]

§

### impl [Unpin][47] for [InvalidUtf8InPathParam][8]

§

### impl [UnwindSafe][48] for [InvalidUtf8InPathParam][8]

## Blanket Implementations§

[Source][49]§

### impl<T> [Any][50] for T

where T: 'static + ?[Sized][51],

[Source][52]§

#### fn [type_id][53](&self) -> [TypeId][54]

Gets the `TypeId` of `self`. [Read more][53]

[Source][55]§

### impl<T> [Borrow][56]<T> for T

where T: ?[Sized][51],

[Source][57]§

#### fn [borrow][58](&self) -> [&T][59]

Immutably borrows from an owned value. [Read more][58]

[Source][60]§

### impl<T> [BorrowMut][61]<T> for T

where T: ?[Sized][51],

[Source][62]§

#### fn [borrow_mut][63](&mut self) -> [&mut T][59]

Mutably borrows from an owned value. [Read more][63]

[Source][64]§

### impl<T> [From][35]<T> for T

[Source][65]§

#### fn [from][37](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][66] [`Span`][67], returning an `Instrumented` wrapper. Read more

[Source][68]§

### impl<T, U> [Into][69]<U> for T

where U: [From][35]<T>,

[Source][70]§

#### fn [into][71](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][35]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][51],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][72]§

### impl<T> [Same][73] for T

[Source][74]§

#### type [Output][75] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][51],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][51],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][76]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][51],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][51],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][51],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][51],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][51],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][51],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][51],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][77] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][51],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][51],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][51],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][51], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][51], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][51],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][51],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][51],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][80]) -> RequestBodyLimit<Self>

where Self: [Sized][51],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][51],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][51],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][81]§

### impl<T> [ToString][82] for T

where T: [Display][18] \+ ?[Sized][51],

[Source][83]§

#### fn [to_string][84](&self) -> [String][10]

Converts the given value to a `String`. [Read more][84]

§

### impl<T> ToStringFallible for T

where T: [Display][18],

§

#### fn try_to_string(&self) -> [Result][85]<[String][10], [TryReserveError][86]>

[`ToString::to_string`][87], but without panic on OOM.

[Source][88]§

### impl<T, U> [TryFrom][89]<U> for T

where U: [Into][69]<T>,

[Source][90]§

#### type [Error][91] = [Infallible][92]

The type returned in the event of a conversion error.

[Source][93]§

#### fn [try_from][94](value: U) -> [Result][85]<T, <T as [TryFrom][89]<U>>::[Error][95]>

Performs the conversion.

[Source][96]§

### impl<T, U> [TryInto][97]<U> for T

where U: [TryFrom][89]<T>,

[Source][98]§

#### type [Error][99] = <U as [TryFrom][89]<T>>::[Error][95]

The type returned in the event of a conversion error.

[Source][100]§

#### fn [try_into][101](self) -> [Result][85]<U, <U as [TryFrom][89]<T>>::[Error][95]>

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

where S: [Into][69]<Dispatch>,

Attaches the provided [`Subscriber`][102] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][103] [`Subscriber`][102] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/path/mod.rs.html#562-564
   [6]: ../struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [7]: ../../../src/axum/extract/path/mod.rs.html#566-578
   [8]: struct.InvalidUtf8InPathParam.html (struct axum::extract::path::InvalidUtf8InPathParam)
   [9]: ../../../src/axum/extract/path/mod.rs.html#569-571
   [10]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [11]: ../../../src/axum/extract/path/mod.rs.html#575-577
   [12]: ../../../src/axum/extract/path/mod.rs.html#561
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [15]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [17]: ../../../src/axum/extract/path/mod.rs.html#580-584
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [19]: ../../../src/axum/extract/path/mod.rs.html#581-583
   [20]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [21]: ../../../src/axum/extract/path/mod.rs.html#586
   [22]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [23]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#111
   [24]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [25]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [26]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#137
   [27]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [28]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [29]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [30]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [31]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [32]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [33]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [34]: ../../../src/axum/extract/rejection.rs.html#162-171
   [35]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [36]: ../rejection/enum.RawPathParamsRejection.html (enum axum::extract::rejection::RawPathParamsRejection)
   [37]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [38]: ../../../src/axum/extract/path/mod.rs.html#588-598
   [39]: ../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [40]: ../../../src/axum/extract/path/mod.rs.html#589-597
   [41]: ../../response/trait.IntoResponse.html#tymethod.into_response
   [42]: ../../response/type.Response.html (type axum::response::Response)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [44]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [46]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [47]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [48]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [49]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [50]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [51]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [52]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [53]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [54]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [57]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [58]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [59]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [60]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [61]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [62]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [63]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [66]: super::Span::current()
   [67]: crate::Span
   [68]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [69]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [70]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [71]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [72]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [73]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [74]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [75]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [76]: https://docs.rs/http/latest/http/struct.Extensions.html
   [77]: crate::follow_redirect::policy::Standard
   [78]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [79]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [80]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [81]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [82]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [83]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [84]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [85]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [86]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [87]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [92]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [98]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [100]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [101]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [102]: super::Subscriber
   [103]: dispatcher#setting-the-default-subscriber

