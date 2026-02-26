<!-- Generated from rustdoc HTML: extract/struct.RawQuery.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RawQuery

## [axum][1]0.8.8

## RawQuery

### Sections

  * Example



### Tuple Fields

  * 0



### Trait Implementations

  * Debug
  * FromRequestParts<S>



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
  * FromRequest<S, ViaParts>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct RawQuery Copy item path

[Source][4]
``` 
pub struct RawQuery(pub [Option][5]<[String][6]>);
```

Expand description

Extractor that extracts the raw query string, without parsing it.

## §Example
``` 
use axum::{
    extract::RawQuery,
    routing::get,
    Router,
};
use futures_util::StreamExt;

async fn handler(RawQuery(query): RawQuery) {
    // ...
}

let app = Router::new().route("/users", get(handler));
```

## Tuple Fields§

§`0: [Option][5]<[String][6]>`

## Trait Implementations§

[Source][7]§

### impl [Debug][8] for [RawQuery][9]

[Source][7]§

#### fn [fmt][10](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]

Formats the value using the given formatter. [Read more][10]

[Source][13]§

### impl<S> [FromRequestParts][14]<S> for [RawQuery][9]

where S: [Send][15] \+ [Sync][16],

[Source][17]§

#### type [Rejection][18] = [Infallible][19]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][20]§

#### async fn [from_request_parts][21]( parts: &mut Parts, _state: [&S][22], ) -> [Result][23]<Self, Self::[Rejection][24]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl [Freeze][25] for [RawQuery][9]

§

### impl [RefUnwindSafe][26] for [RawQuery][9]

§

### impl [Send][15] for [RawQuery][9]

§

### impl [Sync][16] for [RawQuery][9]

§

### impl [Unpin][27] for [RawQuery][9]

§

### impl [UnwindSafe][28] for [RawQuery][9]

## Blanket Implementations§

[Source][29]§

### impl<T> [Any][30] for T

where T: 'static + ?[Sized][31],

[Source][32]§

#### fn [type_id][33](&self) -> [TypeId][34]

Gets the `TypeId` of `self`. [Read more][33]

[Source][35]§

### impl<T> [Borrow][36]<T> for T

where T: ?[Sized][31],

[Source][37]§

#### fn [borrow][38](&self) -> [&T][22]

Immutably borrows from an owned value. [Read more][38]

[Source][39]§

### impl<T> [BorrowMut][40]<T> for T

where T: ?[Sized][31],

[Source][41]§

#### fn [borrow_mut][42](&mut self) -> [&mut T][22]

Mutably borrows from an owned value. [Read more][42]

[Source][43]§

### impl<T> [From][44]<T> for T

[Source][45]§

#### fn [from][46](t: T) -> T

Returns the argument unchanged.

§

### impl<S, T> [FromRequest][47]<S, ViaParts> for T

where S: [Send][15] \+ [Sync][16], T: [FromRequestParts][14]<S>,

§

#### type [Rejection][48] = <T as [FromRequestParts][14]<S>>::[Rejection][24]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][49]( req: Request<[Body][50]>, state: [&S][22], ) -> impl [Future][51]<Output = [Result][23]<T, <T as [FromRequest][47]<S, ViaParts>>::[Rejection][52]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][53] [`Span`][54], returning an `Instrumented` wrapper. Read more

[Source][55]§

### impl<T, U> [Into][56]<U> for T

where U: [From][44]<T>,

[Source][57]§

#### fn [into][58](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][44]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][31],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][59]§

### impl<T> [Same][60] for T

[Source][61]§

#### type [Output][62] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][31],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][31],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][63]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][31],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][31],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][31],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][31],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][31],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][31],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][31],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][64] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][65]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][66] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][65]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][66] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][65]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][66] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][31], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][31], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][31],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][31],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][31],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][67]) -> RequestBodyLimit<Self>

where Self: [Sized][31],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][31],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][31],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][68]§

### impl<T, U> [TryFrom][69]<U> for T

where U: [Into][56]<T>,

[Source][70]§

#### type [Error][71] = [Infallible][19]

The type returned in the event of a conversion error.

[Source][72]§

#### fn [try_from][73](value: U) -> [Result][23]<T, <T as [TryFrom][69]<U>>::[Error][74]>

Performs the conversion.

[Source][75]§

### impl<T, U> [TryInto][76]<U> for T

where U: [TryFrom][69]<T>,

[Source][77]§

#### type [Error][78] = <U as [TryFrom][69]<T>>::[Error][74]

The type returned in the event of a conversion error.

[Source][79]§

#### fn [try_into][80](self) -> [Result][23]<U, <U as [TryFrom][69]<T>>::[Error][74]>

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

where S: [Into][56]<Dispatch>,

Attaches the provided [`Subscriber`][81] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][82] [`Subscriber`][81] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/raw_query.rs.html#25
   [5]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [6]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [7]: ../../src/axum/extract/raw_query.rs.html#24
   [8]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [9]: struct.RawQuery.html (struct axum::extract::RawQuery)
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [11]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [13]: ../../src/axum/extract/raw_query.rs.html#27-37
   [14]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [15]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [16]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [17]: ../../src/axum/extract/raw_query.rs.html#31
   [18]: trait.FromRequestParts.html#associatedtype.Rejection
   [19]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [20]: ../../src/axum/extract/raw_query.rs.html#33-36
   [21]: trait.FromRequestParts.html#tymethod.from_request_parts
   [22]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [23]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [24]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [26]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [28]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [29]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [30]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [32]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [33]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [34]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [35]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [36]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [37]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [38]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [39]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [40]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [41]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [42]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [43]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [44]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [45]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [46]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [47]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [48]: trait.FromRequest.html#associatedtype.Rejection
   [49]: trait.FromRequest.html#tymethod.from_request
   [50]: ../body/struct.Body.html (struct axum::body::Body)
   [51]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [52]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [53]: super::Span::current()
   [54]: crate::Span
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [56]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [59]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [60]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [61]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [62]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [63]: https://docs.rs/http/latest/http/struct.Extensions.html
   [64]: crate::follow_redirect::policy::Standard
   [65]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [66]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [67]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [68]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [69]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [70]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [71]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [72]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [73]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [81]: super::Subscriber
   [82]: dispatcher#setting-the-default-subscriber

