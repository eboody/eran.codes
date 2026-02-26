<!-- Generated from rustdoc HTML: extract/struct.RawForm.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RawForm

## [axum][1]0.8.8

## RawForm

### Sections

  * Example



### Tuple Fields

  * 0



### Trait Implementations

  * Debug
  * FromRequest<S>



### Auto Trait Implementations

  * !Freeze
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
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct RawForm Copy item path

[Source][4]
``` 
pub struct RawForm(pub Bytes);
```

Expand description

Extractor that extracts raw form requests.

For `GET` requests it will extract the raw query. For other methods it extracts the raw `application/x-www-form-urlencoded` encoded request body.

## §Example
``` 
use axum::{
    extract::RawForm,
    routing::get,
    Router
};

async fn handler(RawForm(form): RawForm) {}

let app = Router::new().route("/", get(handler));
```

## Tuple Fields§

§`0: Bytes`

## Trait Implementations§

[Source][5]§

### impl [Debug][6] for [RawForm][7]

[Source][5]§

#### fn [fmt][8](&self, f: &mut [Formatter][9]<'_>) -> [Result][10]

Formats the value using the given formatter. [Read more][8]

[Source][11]§

### impl<S> [FromRequest][12]<S> for [RawForm][7]

where S: [Send][13] \+ [Sync][14],

[Source][15]§

#### type [Rejection][16] = [RawFormRejection][17]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][18]§

#### async fn [from_request][19](req: [Request][20], state: [&S][21]) -> [Result][22]<Self, Self::[Rejection][23]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl ![Freeze][24] for [RawForm][7]

§

### impl [RefUnwindSafe][25] for [RawForm][7]

§

### impl [Send][13] for [RawForm][7]

§

### impl [Sync][14] for [RawForm][7]

§

### impl [Unpin][26] for [RawForm][7]

§

### impl [UnwindSafe][27] for [RawForm][7]

## Blanket Implementations§

[Source][28]§

### impl<T> [Any][29] for T

where T: 'static + ?[Sized][30],

[Source][31]§

#### fn [type_id][32](&self) -> [TypeId][33]

Gets the `TypeId` of `self`. [Read more][32]

[Source][34]§

### impl<T> [Borrow][35]<T> for T

where T: ?[Sized][30],

[Source][36]§

#### fn [borrow][37](&self) -> [&T][21]

Immutably borrows from an owned value. [Read more][37]

[Source][38]§

### impl<T> [BorrowMut][39]<T> for T

where T: ?[Sized][30],

[Source][40]§

#### fn [borrow_mut][41](&mut self) -> [&mut T][21]

Mutably borrows from an owned value. [Read more][41]

[Source][42]§

### impl<T> [From][43]<T> for T

[Source][44]§

#### fn [from][45](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][46] [`Span`][47], returning an `Instrumented` wrapper. Read more

[Source][48]§

### impl<T, U> [Into][49]<U> for T

where U: [From][43]<T>,

[Source][50]§

#### fn [into][51](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][43]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][30],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][52]§

### impl<T> [Same][53] for T

[Source][54]§

#### type [Output][55] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][30],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][30],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][56]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][30],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][30],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][30],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][30],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][30],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][30],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][30],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][57] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][58]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][30],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][59] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][58]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][30],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][59] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][58]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][30],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][59] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][30],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][30],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][30],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][30],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][30],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][30],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][30], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][30], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][30],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][30],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][30],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][60]) -> RequestBodyLimit<Self>

where Self: [Sized][30],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][30],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][30],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][61]§

### impl<T, U> [TryFrom][62]<U> for T

where U: [Into][49]<T>,

[Source][63]§

#### type [Error][64] = [Infallible][65]

The type returned in the event of a conversion error.

[Source][66]§

#### fn [try_from][67](value: U) -> [Result][22]<T, <T as [TryFrom][62]<U>>::[Error][68]>

Performs the conversion.

[Source][69]§

### impl<T, U> [TryInto][70]<U> for T

where U: [TryFrom][62]<T>,

[Source][71]§

#### type [Error][72] = <U as [TryFrom][62]<T>>::[Error][68]

The type returned in the event of a conversion error.

[Source][73]§

#### fn [try_into][74](self) -> [Result][22]<U, <U as [TryFrom][62]<T>>::[Error][68]>

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

where S: [Into][49]<Dispatch>,

Attaches the provided [`Subscriber`][75] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][76] [`Subscriber`][75] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/raw_form.rs.html#30
   [5]: ../../src/axum/extract/raw_form.rs.html#29
   [6]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [7]: struct.RawForm.html (struct axum::extract::RawForm)
   [8]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [9]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [10]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [11]: ../../src/axum/extract/raw_form.rs.html#32-53
   [12]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [13]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [14]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [15]: ../../src/axum/extract/raw_form.rs.html#36
   [16]: trait.FromRequest.html#associatedtype.Rejection
   [17]: rejection/enum.RawFormRejection.html (enum axum::extract::rejection::RawFormRejection)
   [18]: ../../src/axum/extract/raw_form.rs.html#38-52
   [19]: trait.FromRequest.html#tymethod.from_request
   [20]: type.Request.html (type axum::extract::Request)
   [21]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [22]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [23]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [25]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [27]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [28]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [29]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [31]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [32]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [33]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [34]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [35]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [36]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [37]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [38]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [39]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [42]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [43]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [44]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [45]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [46]: super::Span::current()
   [47]: crate::Span
   [48]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [49]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [50]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [51]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [52]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [53]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [54]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [55]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [56]: https://docs.rs/http/latest/http/struct.Extensions.html
   [57]: crate::follow_redirect::policy::Standard
   [58]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [59]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [60]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [65]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [66]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [67]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [68]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [69]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [70]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [71]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [72]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [75]: super::Subscriber
   [76]: dispatcher#setting-the-default-subscriber

