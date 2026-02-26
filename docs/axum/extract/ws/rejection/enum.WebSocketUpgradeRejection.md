<!-- Generated from rustdoc HTML: extract/ws/rejection/enum.WebSocketUpgradeRejection.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## WebSocketUpgradeRejection

## [axum][1]0.8.8

## WebSocketUpgradeRejection

### Variants

  * ConnectionNotUpgradable
  * InvalidConnectionHeader
  * InvalidProtocolPseudoheader
  * InvalidUpgradeHeader
  * InvalidWebSocketVersionHeader
  * MethodNotConnect
  * MethodNotGet
  * WebSocketKeyHeaderMissing



### Methods

  * body_text
  * status



### Trait Implementations

  * Debug
  * Display
  * Error
  * From<ConnectionNotUpgradable>
  * From<InvalidConnectionHeader>
  * From<InvalidProtocolPseudoheader>
  * From<InvalidUpgradeHeader>
  * From<InvalidWebSocketVersionHeader>
  * From<MethodNotConnect>
  * From<MethodNotGet>
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

# Enum WebSocketUpgradeRejection Copy item path

[Source][6]
``` 
#[non_exhaustive]

pub enum WebSocketUpgradeRejection {
    MethodNotGet([MethodNotGet][7]),
    MethodNotConnect([MethodNotConnect][8]),
    InvalidConnectionHeader([InvalidConnectionHeader][9]),
    InvalidUpgradeHeader([InvalidUpgradeHeader][10]),
    InvalidProtocolPseudoheader([InvalidProtocolPseudoheader][11]),
    InvalidWebSocketVersionHeader([InvalidWebSocketVersionHeader][12]),
    WebSocketKeyHeaderMissing([WebSocketKeyHeaderMissing][13]),
    ConnectionNotUpgradable([ConnectionNotUpgradable][14]),
}
```

Available on **crate feature`ws`** only.

Expand description

Rejection used for [`WebSocketUpgrade`][15].

Contains one variant for each way the [`WebSocketUpgrade`][15] extractor can fail.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

§

### MethodNotGet([MethodNotGet][7])

§

### MethodNotConnect([MethodNotConnect][8])

§

### InvalidConnectionHeader([InvalidConnectionHeader][9])

§

### InvalidUpgradeHeader([InvalidUpgradeHeader][10])

§

### InvalidProtocolPseudoheader([InvalidProtocolPseudoheader][11])

§

### InvalidWebSocketVersionHeader([InvalidWebSocketVersionHeader][12])

§

### WebSocketKeyHeaderMissing([WebSocketKeyHeaderMissing][13])

§

### ConnectionNotUpgradable([ConnectionNotUpgradable][14])

## Implementations§

[Source][6]§

### impl [WebSocketUpgradeRejection][16]

[Source][6]

#### pub fn body_text(&self) -> [String][17]

Get the response body text used for this rejection.

[Source][6]

#### pub fn status(&self) -> StatusCode

Get the status code used for this rejection.

## Trait Implementations§

[Source][6]§

### impl [Debug][18] for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [fmt][19](&self, f: &mut [Formatter][20]<'_>) -> [Result][21]

Formats the value using the given formatter. [Read more][19]

[Source][6]§

### impl [Display][22] for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [fmt][23](&self, f: &mut [Formatter][20]<'_>) -> [Result][21]

Formats the value using the given formatter. [Read more][23]

[Source][6]§

### impl [Error][24] for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [source][25](&self) -> [Option][26]<&(dyn [Error][24] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][25]

1.0.0 · [Source][27]§

#### fn [description][28](&self) -> &[str][29]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][28]

1.0.0 · [Source][30]§

#### fn [cause][31](&self) -> [Option][26]<&dyn [Error][24]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][32]§

#### fn [provide][33]<'a>(&'a self, request: &mut [Request][34]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][33]

[Source][6]§

### impl [From][35]<[ConnectionNotUpgradable][14]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [ConnectionNotUpgradable][14]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [From][35]<[InvalidConnectionHeader][9]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [InvalidConnectionHeader][9]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [From][35]<[InvalidProtocolPseudoheader][11]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [InvalidProtocolPseudoheader][11]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [From][35]<[InvalidUpgradeHeader][10]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [InvalidUpgradeHeader][10]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [From][35]<[InvalidWebSocketVersionHeader][12]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [InvalidWebSocketVersionHeader][12]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [From][35]<[MethodNotConnect][8]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [MethodNotConnect][8]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [From][35]<[MethodNotGet][7]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [MethodNotGet][7]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [From][35]<[WebSocketKeyHeaderMissing][13]> for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [from][36](inner: [WebSocketKeyHeaderMissing][13]) -> Self

Converts to this type from the input type.

[Source][6]§

### impl [IntoResponse][37] for [WebSocketUpgradeRejection][16]

[Source][6]§

#### fn [into_response][38](self) -> [Response][39]

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][40] for [WebSocketUpgradeRejection][16]

§

### impl [RefUnwindSafe][41] for [WebSocketUpgradeRejection][16]

§

### impl [Send][42] for [WebSocketUpgradeRejection][16]

§

### impl [Sync][43] for [WebSocketUpgradeRejection][16]

§

### impl [Unpin][44] for [WebSocketUpgradeRejection][16]

§

### impl [UnwindSafe][45] for [WebSocketUpgradeRejection][16]

## Blanket Implementations§

[Source][46]§

### impl<T> [Any][47] for T

where T: 'static + ?[Sized][48],

[Source][49]§

#### fn [type_id][50](&self) -> [TypeId][51]

Gets the `TypeId` of `self`. [Read more][50]

[Source][52]§

### impl<T> [Borrow][53]<T> for T

where T: ?[Sized][48],

[Source][54]§

#### fn [borrow][55](&self) -> [&T][56]

Immutably borrows from an owned value. [Read more][55]

[Source][57]§

### impl<T> [BorrowMut][58]<T> for T

where T: ?[Sized][48],

[Source][59]§

#### fn [borrow_mut][60](&mut self) -> [&mut T][56]

Mutably borrows from an owned value. [Read more][60]

[Source][61]§

### impl<T> [From][35]<T> for T

[Source][62]§

#### fn [from][36](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][63] [`Span`][64], returning an `Instrumented` wrapper. Read more

[Source][65]§

### impl<T, U> [Into][66]<U> for T

where U: [From][35]<T>,

[Source][67]§

#### fn [into][68](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][35]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][48],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][69]§

### impl<T> [Same][70] for T

[Source][71]§

#### type [Output][72] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][48],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][48],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][73]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][48],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][48],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][48],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][48],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][48],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][48],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][48],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][74] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][48],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][48],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][48],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][48], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][48], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][48],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][48],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][48],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][77]) -> RequestBodyLimit<Self>

where Self: [Sized][48],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][48],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][48],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][78]§

### impl<T> [ToString][79] for T

where T: [Display][22] \+ ?[Sized][48],

[Source][80]§

#### fn [to_string][81](&self) -> [String][17]

Converts the given value to a `String`. [Read more][81]

§

### impl<T> ToStringFallible for T

where T: [Display][22],

§

#### fn try_to_string(&self) -> [Result][82]<[String][17], [TryReserveError][83]>

[`ToString::to_string`][84], but without panic on OOM.

[Source][85]§

### impl<T, U> [TryFrom][86]<U> for T

where U: [Into][66]<T>,

[Source][87]§

#### type [Error][88] = [Infallible][89]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_from][91](value: U) -> [Result][82]<T, <T as [TryFrom][86]<U>>::[Error][92]>

Performs the conversion.

[Source][93]§

### impl<T, U> [TryInto][94]<U> for T

where U: [TryFrom][86]<T>,

[Source][95]§

#### type [Error][96] = <U as [TryFrom][86]<T>>::[Error][92]

The type returned in the event of a conversion error.

[Source][97]§

#### fn [try_into][98](self) -> [Result][82]<U, <U as [TryFrom][86]<T>>::[Error][92]>

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

where S: [Into][66]<Dispatch>,

Attaches the provided [`Subscriber`][99] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][100] [`Subscriber`][99] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../../axum/index.html
   [2]: index.html
   [3]: ../../../index.html
   [4]: ../../index.html
   [5]: ../index.html
   [6]: ../../../../src/axum/extract/ws.rs.html#1010-1025
   [7]: struct.MethodNotGet.html (struct axum::extract::ws::rejection::MethodNotGet)
   [8]: struct.MethodNotConnect.html (struct axum::extract::ws::rejection::MethodNotConnect)
   [9]: struct.InvalidConnectionHeader.html (struct axum::extract::ws::rejection::InvalidConnectionHeader)
   [10]: struct.InvalidUpgradeHeader.html (struct axum::extract::ws::rejection::InvalidUpgradeHeader)
   [11]: struct.InvalidProtocolPseudoheader.html (struct axum::extract::ws::rejection::InvalidProtocolPseudoheader)
   [12]: struct.InvalidWebSocketVersionHeader.html (struct axum::extract::ws::rejection::InvalidWebSocketVersionHeader)
   [13]: struct.WebSocketKeyHeaderMissing.html (struct axum::extract::ws::rejection::WebSocketKeyHeaderMissing)
   [14]: struct.ConnectionNotUpgradable.html (struct axum::extract::ws::rejection::ConnectionNotUpgradable)
   [15]: ../../struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [16]: enum.WebSocketUpgradeRejection.html (enum axum::extract::ws::rejection::WebSocketUpgradeRejection)
   [17]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [20]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [21]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [22]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [23]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [24]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [25]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [26]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [27]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#137
   [28]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [29]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [30]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [31]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [32]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [33]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [34]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [35]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [36]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [37]: ../../../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [38]: ../../../response/trait.IntoResponse.html#tymethod.into_response
   [39]: ../../../response/type.Response.html (type axum::response::Response)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [41]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [45]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [46]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [47]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [48]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [49]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [50]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [51]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [56]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [57]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [58]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [59]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [60]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [63]: super::Span::current()
   [64]: crate::Span
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [68]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [69]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [70]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [71]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [72]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [73]: https://docs.rs/http/latest/http/struct.Extensions.html
   [74]: crate::follow_redirect::policy::Standard
   [75]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [76]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [77]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [78]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [79]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [80]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [81]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [82]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [83]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [84]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [89]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [95]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [96]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [97]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [98]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [99]: super::Subscriber
   [100]: dispatcher#setting-the-default-subscriber

