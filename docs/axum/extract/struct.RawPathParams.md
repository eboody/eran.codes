<!-- Generated from rustdoc HTML: extract/struct.RawPathParams.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RawPathParams

## [axum][1]0.8.8

## RawPathParams

### Sections

  * Example



### Methods

  * iter



### Trait Implementations

  * Debug
  * FromRequestParts<S>
  * IntoIterator



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

# Struct RawPathParams Copy item path

[Source][4]
``` 
pub struct RawPathParams(/* private fields */);
```

Expand description

Extractor that will get captures from the URL without deserializing them.

In general you should prefer to use [`Path`][5] as it is higher level, however `RawPathParams` is suitable if just want the raw params without deserializing them and thus saving some allocations.

Any percent encoded parameters will be automatically decoded. The decoded parameters must be valid UTF-8, otherwise `RawPathParams` will fail and return a `400 Bad Request` response.

## §Example
``` 
use axum::{
    extract::RawPathParams,
    routing::get,
    Router,
};

async fn users_teams_show(params: RawPathParams) {
    for (key, value) in &params {
        println!("{key:?} = {value:?}");
    }
}

let app = Router::new().route("/users/{user_id}/team/{team_id}", get(users_teams_show));
```

## Implementations§

[Source][6]§

### impl [RawPathParams][7]

[Source][8]

#### pub fn iter(&self) -> [RawPathParamsIter][9]<'_> ⓘ

Get an iterator over the path parameters.

## Trait Implementations§

[Source][10]§

### impl [Debug][11] for [RawPathParams][7]

[Source][10]§

#### fn [fmt][12](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]

Formats the value using the given formatter. [Read more][12]

[Source][15]§

### impl<S> [FromRequestParts][16]<S> for [RawPathParams][7]

where S: [Send][17] \+ [Sync][18],

[Source][19]§

#### type [Rejection][20] = [RawPathParamsRejection][21]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][22]§

#### async fn [from_request_parts][23]( parts: &mut Parts, _state: [&S][24], ) -> [Result][25]<Self, Self::[Rejection][26]>

Perform the extraction.

[Source][27]§

### impl<'a> [IntoIterator][28] for &'a [RawPathParams][7]

[Source][29]§

#### type [Item][30] = (&'a [str][31], &'a [str][31])

The type of the elements being iterated over.

[Source][32]§

#### type [IntoIter][33] = [RawPathParamsIter][9]<'a>

Which kind of iterator are we turning this into?

[Source][34]§

#### fn [into_iter][35](self) -> Self::[IntoIter][36]

Creates an iterator from a value. [Read more][35]

## Auto Trait Implementations§

§

### impl [Freeze][37] for [RawPathParams][7]

§

### impl [RefUnwindSafe][38] for [RawPathParams][7]

§

### impl [Send][17] for [RawPathParams][7]

§

### impl [Sync][18] for [RawPathParams][7]

§

### impl [Unpin][39] for [RawPathParams][7]

§

### impl [UnwindSafe][40] for [RawPathParams][7]

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

#### fn [borrow][50](&self) -> [&T][24]

Immutably borrows from an owned value. [Read more][50]

[Source][51]§

### impl<T> [BorrowMut][52]<T> for T

where T: ?[Sized][43],

[Source][53]§

#### fn [borrow_mut][54](&mut self) -> [&mut T][24]

Mutably borrows from an owned value. [Read more][54]

[Source][55]§

### impl<T> [From][56]<T> for T

[Source][57]§

#### fn [from][58](t: T) -> T

Returns the argument unchanged.

§

### impl<S, T> [FromRequest][59]<S, ViaParts> for T

where S: [Send][17] \+ [Sync][18], T: [FromRequestParts][16]<S>,

§

#### type [Rejection][60] = <T as [FromRequestParts][16]<S>>::[Rejection][26]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][61]( req: Request<[Body][62]>, state: [&S][24], ) -> impl [Future][63]<Output = [Result][25]<T, <T as [FromRequest][59]<S, ViaParts>>::[Rejection][64]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][65] [`Span`][66], returning an `Instrumented` wrapper. Read more

[Source][67]§

### impl<T, U> [Into][68]<U> for T

where U: [From][56]<T>,

[Source][69]§

#### fn [into][70](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][56]<T> for U` chooses to do.

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

[Source][71]§

### impl<T> [Same][72] for T

[Source][73]§

#### type [Output][74] = T

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

Add some shareable value to [request extensions][75]. Read more

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

Follow redirect resposes using the [`Standard`][76] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][28]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][77] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][28]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][77] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][28]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][77] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][78]) -> RequestBodyLimit<Self>

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

[Source][79]§

### impl<T, U> [TryFrom][80]<U> for T

where U: [Into][68]<T>,

[Source][81]§

#### type [Error][82] = [Infallible][83]

The type returned in the event of a conversion error.

[Source][84]§

#### fn [try_from][85](value: U) -> [Result][25]<T, <T as [TryFrom][80]<U>>::[Error][86]>

Performs the conversion.

[Source][87]§

### impl<T, U> [TryInto][88]<U> for T

where U: [TryFrom][80]<T>,

[Source][89]§

#### type [Error][90] = <U as [TryFrom][80]<T>>::[Error][86]

The type returned in the event of a conversion error.

[Source][91]§

#### fn [try_into][92](self) -> [Result][25]<U, <U as [TryFrom][80]<T>>::[Error][86]>

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

where S: [Into][68]<Dispatch>,

Attaches the provided [`Subscriber`][93] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][94] [`Subscriber`][93] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/path/mod.rs.html#501
   [5]: struct.Path.html (struct axum::extract::Path)
   [6]: ../../src/axum/extract/path/mod.rs.html#527-533
   [7]: struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [8]: ../../src/axum/extract/path/mod.rs.html#530-532
   [9]: path/struct.RawPathParamsIter.html (struct axum::extract::path::RawPathParamsIter)
   [10]: ../../src/axum/extract/path/mod.rs.html#500
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [13]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [15]: ../../src/axum/extract/path/mod.rs.html#503-525
   [16]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [17]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [18]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [19]: ../../src/axum/extract/path/mod.rs.html#507
   [20]: trait.FromRequestParts.html#associatedtype.Rejection
   [21]: rejection/enum.RawPathParamsRejection.html (enum axum::extract::rejection::RawPathParamsRejection)
   [22]: ../../src/axum/extract/path/mod.rs.html#509-524
   [23]: trait.FromRequestParts.html#tymethod.from_request_parts
   [24]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [25]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [26]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [27]: ../../src/axum/extract/path/mod.rs.html#535-542
   [28]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [29]: ../../src/axum/extract/path/mod.rs.html#536
   [30]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item
   [31]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [32]: ../../src/axum/extract/path/mod.rs.html#537
   [33]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter
   [34]: ../../src/axum/extract/path/mod.rs.html#539-541
   [35]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter
   [36]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter (type core::iter::traits::collect::IntoIterator::IntoIter)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
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
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [56]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [59]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [60]: trait.FromRequest.html#associatedtype.Rejection
   [61]: trait.FromRequest.html#tymethod.from_request
   [62]: ../body/struct.Body.html (struct axum::body::Body)
   [63]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [64]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [65]: super::Span::current()
   [66]: crate::Span
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [68]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [69]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [70]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [71]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [72]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [73]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [74]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [75]: https://docs.rs/http/latest/http/struct.Extensions.html
   [76]: crate::follow_redirect::policy::Standard
   [77]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [78]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
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

