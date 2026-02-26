<!-- Generated from rustdoc HTML: extract/struct.NestedPath.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## NestedPath

## [axum][1]0.8.8

## NestedPath

### Sections

  * Example



### Methods

  * as_str



### Trait Implementations

  * Clone
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
  * CloneToUninit
  * From<T>
  * FromRef<T>
  * FromRequest<S, ViaParts>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct NestedPath Copy item path

[Source][4]
``` 
pub struct NestedPath(/* private fields */);
```

Expand description

Access the path the matched the route is nested at.

This can for example be used when doing redirects.

## §Example
``` 
use axum::{
    Router,
    extract::NestedPath,
    routing::get,
};

let api = Router::new().route(
    "/users",
    get(|path: NestedPath| async move {
        // `path` will be "/api" because that's what this
        // router is nested at when we build `app`
        let path = path.as_str();
    })
);

let app = Router::new().nest("/api", api);
```

## Implementations§

[Source][5]§

### impl [NestedPath][6]

[Source][7]

#### pub fn as_str(&self) -> &[str][8]

Returns a `str` representation of the path.

## Trait Implementations§

[Source][9]§

### impl [Clone][10] for [NestedPath][6]

[Source][9]§

#### fn [clone][11](&self) -> [NestedPath][6]

Returns a duplicate of the value. [Read more][11]

1.0.0 · [Source][12]§

#### fn [clone_from][13](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][13]

[Source][9]§

### impl [Debug][14] for [NestedPath][6]

[Source][9]§

#### fn [fmt][15](&self, f: &mut [Formatter][16]<'_>) -> [Result][17]

Formats the value using the given formatter. [Read more][15]

[Source][18]§

### impl<S> [FromRequestParts][19]<S> for [NestedPath][6]

where S: [Send][20] \+ [Sync][21],

[Source][22]§

#### type [Rejection][23] = [NestedPathRejection][24]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][25]§

#### async fn [from_request_parts][26]( parts: &mut Parts, _state: [&S][27], ) -> [Result][28]<Self, Self::[Rejection][29]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl [Freeze][30] for [NestedPath][6]

§

### impl [RefUnwindSafe][31] for [NestedPath][6]

§

### impl [Send][20] for [NestedPath][6]

§

### impl [Sync][21] for [NestedPath][6]

§

### impl [Unpin][32] for [NestedPath][6]

§

### impl [UnwindSafe][33] for [NestedPath][6]

## Blanket Implementations§

[Source][34]§

### impl<T> [Any][35] for T

where T: 'static + ?[Sized][36],

[Source][37]§

#### fn [type_id][38](&self) -> [TypeId][39]

Gets the `TypeId` of `self`. [Read more][38]

[Source][40]§

### impl<T> [Borrow][41]<T> for T

where T: ?[Sized][36],

[Source][42]§

#### fn [borrow][43](&self) -> [&T][27]

Immutably borrows from an owned value. [Read more][43]

[Source][44]§

### impl<T> [BorrowMut][45]<T> for T

where T: ?[Sized][36],

[Source][46]§

#### fn [borrow_mut][47](&mut self) -> [&mut T][27]

Mutably borrows from an owned value. [Read more][47]

[Source][48]§

### impl<T> [CloneToUninit][49] for T

where T: [Clone][10],

[Source][50]§

#### unsafe fn [clone_to_uninit][51](&self, dest: [*mut ][52][u8][53])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][51]

[Source][54]§

### impl<T> [From][55]<T> for T

[Source][56]§

#### fn [from][57](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][58]<T> for T

where T: [Clone][10],

§

#### fn [from_ref][59](input: [&T][27]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][60]<S, ViaParts> for T

where S: [Send][20] \+ [Sync][21], T: [FromRequestParts][19]<S>,

§

#### type [Rejection][61] = <T as [FromRequestParts][19]<S>>::[Rejection][29]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][62]( req: Request<[Body][63]>, state: [&S][27], ) -> impl [Future][64]<Output = [Result][28]<T, <T as [FromRequest][60]<S, ViaParts>>::[Rejection][65]>>

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

where U: [From][55]<T>,

[Source][70]§

#### fn [into][71](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][55]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][36],

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

where Self: [Sized][36],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][36],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][76]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][36],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][36],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][36],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][36],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][36],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][36],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][36],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][77] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][36],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][36],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][36],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][36], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][36], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][36],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][36],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][36],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][80]) -> RequestBodyLimit<Self>

where Self: [Sized][36],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][36],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][36],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][81]§

### impl<T> [ToOwned][82] for T

where T: [Clone][10],

[Source][83]§

#### type [Owned][84] = T

The resulting type after obtaining ownership.

[Source][85]§

#### fn [to_owned][86](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][86]

[Source][87]§

#### fn [clone_into][88](&self, target: [&mut T][27])

Uses borrowed data to replace owned data, usually by cloning. [Read more][88]

[Source][89]§

### impl<T, U> [TryFrom][90]<U> for T

where U: [Into][69]<T>,

[Source][91]§

#### type [Error][92] = [Infallible][93]

The type returned in the event of a conversion error.

[Source][94]§

#### fn [try_from][95](value: U) -> [Result][28]<T, <T as [TryFrom][90]<U>>::[Error][96]>

Performs the conversion.

[Source][97]§

### impl<T, U> [TryInto][98]<U> for T

where U: [TryFrom][90]<T>,

[Source][99]§

#### type [Error][100] = <U as [TryFrom][90]<T>>::[Error][96]

The type returned in the event of a conversion error.

[Source][101]§

#### fn [try_into][102](self) -> [Result][28]<U, <U as [TryFrom][90]<T>>::[Error][96]>

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

Attaches the provided [`Subscriber`][103] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][104] [`Subscriber`][103] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/nested_path.rs.html#40
   [5]: ../../src/axum/extract/nested_path.rs.html#42-48
   [6]: struct.NestedPath.html (struct axum::extract::NestedPath)
   [7]: ../../src/axum/extract/nested_path.rs.html#45-47
   [8]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [9]: ../../src/axum/extract/nested_path.rs.html#39
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [12]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [13]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [16]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [18]: ../../src/axum/extract/nested_path.rs.html#51-63
   [19]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [20]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [21]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [22]: ../../src/axum/extract/nested_path.rs.html#55
   [23]: trait.FromRequestParts.html#associatedtype.Rejection
   [24]: rejection/struct.NestedPathRejection.html (struct axum::extract::rejection::NestedPathRejection)
   [25]: ../../src/axum/extract/nested_path.rs.html#57-62
   [26]: trait.FromRequestParts.html#tymethod.from_request_parts
   [27]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [28]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [29]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [31]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [33]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [34]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [35]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [37]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [38]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [39]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [48]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [49]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [50]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [51]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [52]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [53]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [55]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [57]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [58]: trait.FromRef.html (trait axum::extract::FromRef)
   [59]: trait.FromRef.html#tymethod.from_ref
   [60]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [61]: trait.FromRequest.html#associatedtype.Rejection
   [62]: trait.FromRequest.html#tymethod.from_request
   [63]: ../body/struct.Body.html (struct axum::body::Body)
   [64]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [65]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
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
   [81]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [82]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [83]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [84]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [85]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [86]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [87]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [88]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [90]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [93]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [96]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [97]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [98]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [99]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [100]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [101]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [102]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [103]: super::Subscriber
   [104]: dispatcher#setting-the-default-subscriber

