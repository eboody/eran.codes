<!-- Generated from rustdoc HTML: extract/multipart/struct.Multipart.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Multipart

## [axum][1]0.8.8

## Multipart

### Sections

  * Example
  * Large Files



### Methods

  * next_field



### Trait Implementations

  * Debug
  * FromRequest<S>
  * OptionalFromRequest<S>



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
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract::multipart][2]

[axum][3]::[extract][4]::[multipart][2]

# Struct Multipart Copy item path

[Source][5]
``` 
pub struct Multipart { /* private fields */ }
```

Available on **crate feature`multipart`** only.

Expand description

Extractor that parses `multipart/form-data` requests (commonly used with file uploads).

⚠️ Since extracting multipart form data from the request requires consuming the body, the `Multipart` extractor must be _last_ if there are multiple extractors in a handler. See [“the order of extractors”][6]

## §Example
``` 
use axum::{
    extract::Multipart,
    routing::post,
    Router,
};
use futures_util::stream::StreamExt;

async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}

let app = Router::new().route("/upload", post(upload));
```

## §Large Files

For security reasons, by default, `Multipart` limits the request body size to 2MB. See [`DefaultBodyLimit`][7] for how to configure this limit.

## Implementations§

[Source][8]§

### impl [Multipart][9]

[Source][10]

#### pub async fn next_field(&mut self) -> [Result][11]<[Option][12]<[Field][13]<'_>>, [MultipartError][14]>

Yields the next [`Field`][13] if available.

## Trait Implementations§

[Source][15]§

### impl [Debug][16] for [Multipart][9]

[Source][15]§

#### fn [fmt][17](&self, f: &mut [Formatter][18]<'_>) -> [Result][19]

Formats the value using the given formatter. [Read more][17]

[Source][20]§

### impl<S> [FromRequest][21]<S> for [Multipart][9]

where S: [Send][22] \+ [Sync][23],

[Source][24]§

#### type [Rejection][25] = [MultipartRejection][26]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][27]§

#### async fn [from_request][28](req: [Request][29], _state: [&S][30]) -> [Result][11]<Self, Self::[Rejection][31]>

Perform the extraction.

[Source][32]§

### impl<S> [OptionalFromRequest][33]<S> for [Multipart][9]

where S: [Send][22] \+ [Sync][23],

[Source][34]§

#### type [Rejection][35] = [MultipartRejection][26]

If the extractor fails, it will use this “rejection” type. [Read more][35]

[Source][36]§

#### async fn [from_request][37]( req: [Request][29], _state: [&S][30], ) -> [Result][11]<[Option][12]<Self>, Self::[Rejection][38]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl [Freeze][39] for [Multipart][9]

§

### impl ![RefUnwindSafe][40] for [Multipart][9]

§

### impl [Send][22] for [Multipart][9]

§

### impl [Sync][23] for [Multipart][9]

§

### impl [Unpin][41] for [Multipart][9]

§

### impl ![UnwindSafe][42] for [Multipart][9]

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

#### fn [borrow][52](&self) -> [&T][30]

Immutably borrows from an owned value. [Read more][52]

[Source][53]§

### impl<T> [BorrowMut][54]<T> for T

where T: ?[Sized][45],

[Source][55]§

#### fn [borrow_mut][56](&mut self) -> [&mut T][30]

Mutably borrows from an owned value. [Read more][56]

[Source][57]§

### impl<T> [From][58]<T> for T

[Source][59]§

#### fn [from][60](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][61] [`Span`][62], returning an `Instrumented` wrapper. Read more

[Source][63]§

### impl<T, U> [Into][64]<U> for T

where U: [From][58]<T>,

[Source][65]§

#### fn [into][66](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][58]<T> for U` chooses to do.

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

[Source][67]§

### impl<T> [Same][68] for T

[Source][69]§

#### type [Output][70] = T

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

Add some shareable value to [request extensions][71]. Read more

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

Follow redirect resposes using the [`Standard`][72] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][73]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][74] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][73]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][74] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][73]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][74] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][75]) -> RequestBodyLimit<Self>

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

[Source][76]§

### impl<T, U> [TryFrom][77]<U> for T

where U: [Into][64]<T>,

[Source][78]§

#### type [Error][79] = [Infallible][80]

The type returned in the event of a conversion error.

[Source][81]§

#### fn [try_from][82](value: U) -> [Result][11]<T, <T as [TryFrom][77]<U>>::[Error][83]>

Performs the conversion.

[Source][84]§

### impl<T, U> [TryInto][85]<U> for T

where U: [TryFrom][77]<T>,

[Source][86]§

#### type [Error][87] = <U as [TryFrom][77]<T>>::[Error][83]

The type returned in the event of a conversion error.

[Source][88]§

#### fn [try_into][89](self) -> [Result][11]<U, <U as [TryFrom][77]<T>>::[Error][83]>

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

where S: [Into][64]<Dispatch>,

Attaches the provided [`Subscriber`][90] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][91] [`Subscriber`][90] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/multipart.rs.html#64-66
   [6]: ../index.html#the-order-of-extractors (mod axum::extract)
   [7]: ../struct.DefaultBodyLimit.html (struct axum::extract::DefaultBodyLimit)
   [8]: ../../../src/axum/extract/multipart.rs.html#106-124
   [9]: ../struct.Multipart.html (struct axum::extract::Multipart)
   [10]: ../../../src/axum/extract/multipart.rs.html#108-123
   [11]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [12]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [13]: struct.Field.html (struct axum::extract::multipart::Field)
   [14]: struct.MultipartError.html (struct axum::extract::multipart::MultipartError)
   [15]: ../../../src/axum/extract/multipart.rs.html#63
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [18]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [20]: ../../../src/axum/extract/multipart.rs.html#68-82
   [21]: ../trait.FromRequest.html (trait axum::extract::FromRequest)
   [22]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [24]: ../../../src/axum/extract/multipart.rs.html#72
   [25]: ../trait.FromRequest.html#associatedtype.Rejection
   [26]: enum.MultipartRejection.html (enum axum::extract::multipart::MultipartRejection)
   [27]: ../../../src/axum/extract/multipart.rs.html#74-81
   [28]: ../trait.FromRequest.html#tymethod.from_request
   [29]: ../type.Request.html (type axum::extract::Request)
   [30]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [31]: ../trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [32]: ../../../src/axum/extract/multipart.rs.html#84-104
   [33]: ../trait.OptionalFromRequest.html (trait axum::extract::OptionalFromRequest)
   [34]: ../../../src/axum/extract/multipart.rs.html#88
   [35]: ../trait.OptionalFromRequest.html#associatedtype.Rejection
   [36]: ../../../src/axum/extract/multipart.rs.html#90-103
   [37]: ../trait.OptionalFromRequest.html#tymethod.from_request
   [38]: ../trait.OptionalFromRequest.html#associatedtype.Rejection (type axum::extract::OptionalFromRequest::Rejection)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
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
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [59]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [60]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [61]: super::Span::current()
   [62]: crate::Span
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [67]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [68]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [69]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [70]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [71]: https://docs.rs/http/latest/http/struct.Extensions.html
   [72]: crate::follow_redirect::policy::Standard
   [73]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [74]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [75]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [76]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [77]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [80]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [84]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [90]: super::Subscriber
   [91]: dispatcher#setting-the-default-subscriber

