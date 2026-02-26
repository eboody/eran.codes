<!-- Generated from rustdoc HTML: extract/path/struct.RawPathParams.html -->
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



## [In axum::extract::path][2]

[axum][3]::[extract][4]::[path][2]

# Struct RawPathParams Copy item path

[Source][5]
``` 
pub struct RawPathParams(/* private fields */);
```

Expand description

Extractor that will get captures from the URL without deserializing them.

In general you should prefer to use [`Path`][6] as it is higher level, however `RawPathParams` is suitable if just want the raw params without deserializing them and thus saving some allocations.

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

[Source][7]§

### impl [RawPathParams][8]

[Source][9]

#### pub fn iter(&self) -> [RawPathParamsIter][10]<'_> ⓘ

Get an iterator over the path parameters.

## Trait Implementations§

[Source][11]§

### impl [Debug][12] for [RawPathParams][8]

[Source][11]§

#### fn [fmt][13](&self, f: &mut [Formatter][14]<'_>) -> [Result][15]

Formats the value using the given formatter. [Read more][13]

[Source][16]§

### impl<S> [FromRequestParts][17]<S> for [RawPathParams][8]

where S: [Send][18] \+ [Sync][19],

[Source][20]§

#### type [Rejection][21] = [RawPathParamsRejection][22]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][23]§

#### async fn [from_request_parts][24]( parts: &mut Parts, _state: [&S][25], ) -> [Result][26]<Self, Self::[Rejection][27]>

Perform the extraction.

[Source][28]§

### impl<'a> [IntoIterator][29] for &'a [RawPathParams][8]

[Source][30]§

#### type [Item][31] = (&'a [str][32], &'a [str][32])

The type of the elements being iterated over.

[Source][33]§

#### type [IntoIter][34] = [RawPathParamsIter][10]<'a>

Which kind of iterator are we turning this into?

[Source][35]§

#### fn [into_iter][36](self) -> Self::[IntoIter][37]

Creates an iterator from a value. [Read more][36]

## Auto Trait Implementations§

§

### impl [Freeze][38] for [RawPathParams][8]

§

### impl [RefUnwindSafe][39] for [RawPathParams][8]

§

### impl [Send][18] for [RawPathParams][8]

§

### impl [Sync][19] for [RawPathParams][8]

§

### impl [Unpin][40] for [RawPathParams][8]

§

### impl [UnwindSafe][41] for [RawPathParams][8]

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

#### fn [borrow][51](&self) -> [&T][25]

Immutably borrows from an owned value. [Read more][51]

[Source][52]§

### impl<T> [BorrowMut][53]<T> for T

where T: ?[Sized][44],

[Source][54]§

#### fn [borrow_mut][55](&mut self) -> [&mut T][25]

Mutably borrows from an owned value. [Read more][55]

[Source][56]§

### impl<T> [From][57]<T> for T

[Source][58]§

#### fn [from][59](t: T) -> T

Returns the argument unchanged.

§

### impl<S, T> [FromRequest][60]<S, ViaParts> for T

where S: [Send][18] \+ [Sync][19], T: [FromRequestParts][17]<S>,

§

#### type [Rejection][61] = <T as [FromRequestParts][17]<S>>::[Rejection][27]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][62]( req: Request<[Body][63]>, state: [&S][25], ) -> impl [Future][64]<Output = [Result][26]<T, <T as [FromRequest][60]<S, ViaParts>>::[Rejection][65]>>

Perform the extraction.

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

where U: [From][57]<T>,

[Source][70]§

#### fn [into][71](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][57]<T> for U` chooses to do.

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

[Source][72]§

### impl<T> [Same][73] for T

[Source][74]§

#### type [Output][75] = T

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

Add some shareable value to [request extensions][76]. Read more

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

Follow redirect resposes using the [`Standard`][77] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][29]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][78] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][29]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][78] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][29]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][78] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][79]) -> RequestBodyLimit<Self>

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

[Source][80]§

### impl<T, U> [TryFrom][81]<U> for T

where U: [Into][69]<T>,

[Source][82]§

#### type [Error][83] = [Infallible][84]

The type returned in the event of a conversion error.

[Source][85]§

#### fn [try_from][86](value: U) -> [Result][26]<T, <T as [TryFrom][81]<U>>::[Error][87]>

Performs the conversion.

[Source][88]§

### impl<T, U> [TryInto][89]<U> for T

where U: [TryFrom][81]<T>,

[Source][90]§

#### type [Error][91] = <U as [TryFrom][81]<T>>::[Error][87]

The type returned in the event of a conversion error.

[Source][92]§

#### fn [try_into][93](self) -> [Result][26]<U, <U as [TryFrom][81]<T>>::[Error][87]>

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

Attaches the provided [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][95] [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/path/mod.rs.html#501
   [6]: ../struct.Path.html (struct axum::extract::Path)
   [7]: ../../../src/axum/extract/path/mod.rs.html#527-533
   [8]: ../struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [9]: ../../../src/axum/extract/path/mod.rs.html#530-532
   [10]: struct.RawPathParamsIter.html (struct axum::extract::path::RawPathParamsIter)
   [11]: ../../../src/axum/extract/path/mod.rs.html#500
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [16]: ../../../src/axum/extract/path/mod.rs.html#503-525
   [17]: ../trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [18]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [19]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [20]: ../../../src/axum/extract/path/mod.rs.html#507
   [21]: ../trait.FromRequestParts.html#associatedtype.Rejection
   [22]: ../rejection/enum.RawPathParamsRejection.html (enum axum::extract::rejection::RawPathParamsRejection)
   [23]: ../../../src/axum/extract/path/mod.rs.html#509-524
   [24]: ../trait.FromRequestParts.html#tymethod.from_request_parts
   [25]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [26]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [27]: ../trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [28]: ../../../src/axum/extract/path/mod.rs.html#535-542
   [29]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [30]: ../../../src/axum/extract/path/mod.rs.html#536
   [31]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item
   [32]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [33]: ../../../src/axum/extract/path/mod.rs.html#537
   [34]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter
   [35]: ../../../src/axum/extract/path/mod.rs.html#539-541
   [36]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter
   [37]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter (type core::iter::traits::collect::IntoIterator::IntoIter)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [39]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
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
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [57]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [60]: ../trait.FromRequest.html (trait axum::extract::FromRequest)
   [61]: ../trait.FromRequest.html#associatedtype.Rejection
   [62]: ../trait.FromRequest.html#tymethod.from_request
   [63]: ../../body/struct.Body.html (struct axum::body::Body)
   [64]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [65]: ../trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
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
   [78]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [79]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [84]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [94]: super::Subscriber
   [95]: dispatcher#setting-the-default-subscriber

