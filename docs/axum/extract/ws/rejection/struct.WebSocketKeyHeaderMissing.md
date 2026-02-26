<!-- Generated from rustdoc HTML: extract/ws/rejection/struct.WebSocketKeyHeaderMissing.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## WebSocketKeyHeaderMissing

## [axum][1]0.8.8

## WebSocketKeyHeaderMissing

### Methods

  * body_text
  * status



### Trait Implementations

  * Debug
  * Default
  * Display
  * Error
  * From<WebSocketKeyHeaderMissing>
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



## [In axum::extract::ws::rejection][2]

[axum][3]::[extract][4]::[ws][5]::[rejection][2]

# Struct WebSocketKeyHeaderMissing Copy item path

[Source][6]
``` 
#[non_exhaustive]

pub struct WebSocketKeyHeaderMissing;
```

Available on **crate feature`ws`** only.

Expand description

Rejection type for [`WebSocketUpgrade`][7].

## Implementations§

[Source][6]§

### impl [WebSocketKeyHeaderMissing][8]

[Source][6]

#### pub fn body_text(&self) -> [String][9]

Get the response body text used for this rejection.

[Source][6]

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

[Source][6]§

### impl [Debug][10] for [WebSocketKeyHeaderMissing][8]

[Source][6]§

#### fn [fmt][11](&self, f: &mut [Formatter][12]<'_>) -> [Result][13]

Formats the value using the given formatter. [Read more][11]

[Source][6]§

### impl [Default][14] for [WebSocketKeyHeaderMissing][8]

[Source][6]§

#### fn [default][15]() -> Self

Returns the “default value” for a type. [Read more][15]

[Source][6]§

### impl [Display][16] for [WebSocketKeyHeaderMissing][8]

[Source][6]§

#### fn [fmt][17](&self, f: &mut [Formatter][12]<'_>) -> [Result][13]

Formats the value using the given formatter. [Read more][17]

[Source][6]§

### impl [Error][18] for [WebSocketKeyHeaderMissing][8]

1.30.0 · [Source][19]§

#### fn [source][20](&self) -> [Option][21]<&(dyn [Error][18] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][20]

1.0.0 · [Source][22]§

#### fn [description][23](&self) -> &[str][24]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][23]

1.0.0 · [Source][25]§

#### fn [cause][26](&self) -> [Option][21]<&dyn [Error][18]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][27]§

#### fn [provide][28]<'a>(&'a self, request: &mut [Request][29]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][28]

[Source][30]§

### impl [From][31]<[WebSocketKeyHeaderMissing][8]> for [WebSocketUpgradeRejection][32]

[Source][30]§

#### fn [from][33](inner: [WebSocketKeyHeaderMissing][8]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [IntoResponse][34] for [WebSocketKeyHeaderMissing][8]

[Source][6]§

#### fn [into_response][35](self) -> [Response][36]

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][37] for [WebSocketKeyHeaderMissing][8]

§

### impl [RefUnwindSafe][38] for [WebSocketKeyHeaderMissing][8]

§

### impl [Send][39] for [WebSocketKeyHeaderMissing][8]

§

### impl [Sync][40] for [WebSocketKeyHeaderMissing][8]

§

### impl [Unpin][41] for [WebSocketKeyHeaderMissing][8]

§

### impl [UnwindSafe][42] for [WebSocketKeyHeaderMissing][8]

## Blanket Implementations§

[Source][43]§

### impl<T> [Any][44] for T

where T: 'static + ?[Sized][45],

[Source][46]§

#### fn [type_id][47](&self) -> [TypeId][48]

Gets the `TypeId` of `self`. [Read more][47]

[Source][49]§

### impl<T> [Borrow][50]<T> for T

where T: ?[Sized][45],

[Source][51]§

#### fn [borrow][52](&self) -> [&T][53]

Immutably borrows from an owned value. [Read more][52]

[Source][54]§

### impl<T> [BorrowMut][55]<T> for T

where T: ?[Sized][45],

[Source][56]§

#### fn [borrow_mut][57](&mut self) -> [&mut T][53]

Mutably borrows from an owned value. [Read more][57]

[Source][58]§

### impl<T> [From][31]<T> for T

[Source][59]§

#### fn [from][33](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][60] [`Span`][61], returning an `Instrumented` wrapper. Read more

[Source][62]§

### impl<T, U> [Into][63]<U> for T

where U: [From][31]<T>,

[Source][64]§

#### fn [into][65](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][31]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][45],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][66]§

### impl<T> [Same][67] for T

[Source][68]§

#### type [Output][69] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][45],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][45],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][70]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][45],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][45],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][45],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][45],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][45],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][45],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][45],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][71] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][45], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][45], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][45],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][45],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][45],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][74]) -> RequestBodyLimit<Self>

where Self: [Sized][45],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][45],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][45],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][75]§

### impl<T> [ToString][76] for T

where T: [Display][16] \+ ?[Sized][45],

[Source][77]§

#### fn [to_string][78](&self) -> [String][9]

Converts the given value to a `String`. [Read more][78]

§

### impl<T> ToStringFallible for T

where T: [Display][16],

§

#### fn try_to_string(&self) -> [Result][79]<[String][9], [TryReserveError][80]>

[`ToString::to_string`][81], but without panic on OOM.

[Source][82]§

### impl<T, U> [TryFrom][83]<U> for T

where U: [Into][63]<T>,

[Source][84]§

#### type [Error][85] = [Infallible][86]

The type returned in the event of a conversion error.

[Source][87]§

#### fn [try_from][88](value: U) -> [Result][79]<T, <T as [TryFrom][83]<U>>::[Error][89]>

Performs the conversion.

[Source][90]§

### impl<T, U> [TryInto][91]<U> for T

where U: [TryFrom][83]<T>,

[Source][92]§

#### type [Error][93] = <U as [TryFrom][83]<T>>::[Error][89]

The type returned in the event of a conversion error.

[Source][94]§

#### fn [try_into][95](self) -> [Result][79]<U, <U as [TryFrom][83]<T>>::[Error][89]>

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

where S: [Into][63]<Dispatch>,

Attaches the provided [`Subscriber`][96] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][97] [`Subscriber`][96] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../../axum/index.html
   [2]: index.html
   [3]: ../../../index.html
   [4]: ../../index.html
   [5]: ../index.html
   [6]: ../../../../src/axum/extract/ws.rs.html#989-994
   [7]: ../../struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [8]: struct.WebSocketKeyHeaderMissing.html (struct axum::extract::ws::rejection::WebSocketKeyHeaderMissing)
   [9]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [12]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [14]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [15]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [18]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [19]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#111
   [20]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [21]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [22]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#137
   [23]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [24]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [25]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [26]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [27]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [28]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [29]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [30]: ../../../../src/axum/extract/ws.rs.html#1010-1025
   [31]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [32]: enum.WebSocketUpgradeRejection.html (enum axum::extract::ws::rejection::WebSocketUpgradeRejection)
   [33]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [34]: ../../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [35]: ../../../response/trait.IntoResponse.html#tymethod.into_response
   [36]: ../../../response/type.Response.html (type axum::response::Response)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [42]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [43]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [44]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [46]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [47]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [48]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [53]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [56]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [57]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [59]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [60]: super::Span::current()
   [61]: crate::Span
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [70]: https://docs.rs/http/latest/http/struct.Extensions.html
   [71]: crate::follow_redirect::policy::Standard
   [72]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [73]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [74]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [75]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [76]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [77]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [78]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [79]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [80]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [81]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [84]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [86]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [96]: super::Subscriber
   [97]: dispatcher#setting-the-default-subscriber

