<!-- Generated from rustdoc HTML: extract/struct.MatchedPath.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MatchedPath

## [axum][1]0.8.8

## MatchedPath

### Sections

  * Accessing `MatchedPath` via extensions



### Methods

  * as_str



### Trait Implementations

  * Clone
  * Debug
  * FromRequestParts<S>
  * OptionalFromRequestParts<S>



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

# Struct MatchedPath Copy item path

[Source][4]
``` 
pub struct MatchedPath(/* private fields */);
```

Available on **crate feature`matched-path`** only.

Expand description

Access the path in the router that matches the request.
``` 
use axum::{
    Router,
    extract::MatchedPath,
    routing::get,
};

let app = Router::new().route(
    "/users/{id}",
    get(|path: MatchedPath| async move {
        let path = path.as_str();
        // `path` will be "/users/{id}"
    })
);
```

## §Accessing `MatchedPath` via extensions

`MatchedPath` can also be accessed from middleware via request extensions.

This is useful for example with [`Trace`][5] to create a span that contains the matched path:
``` 
use axum::{
    Router,
    extract::{Request, MatchedPath},
    routing::get,
};
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/users/{id}", get(|| async { /* ... */ }))
    .layer(
        TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
            let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
                path.as_str()
            } else {
                req.uri().path()
            };
            tracing::info_span!("http-request", %path)
        }),
    );
```

## Implementations§

[Source][6]§

### impl [MatchedPath][7]

[Source][8]

#### pub fn as_str(&self) -> &[str][9]

Returns a `str` representation of the path.

## Trait Implementations§

[Source][10]§

### impl [Clone][11] for [MatchedPath][7]

[Source][10]§

#### fn [clone][12](&self) -> [MatchedPath][7]

Returns a duplicate of the value. [Read more][12]

1.0.0 · [Source][13]§

#### fn [clone_from][14](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][14]

[Source][10]§

### impl [Debug][15] for [MatchedPath][7]

[Source][10]§

#### fn [fmt][16](&self, f: &mut [Formatter][17]<'_>) -> [Result][18]

Formats the value using the given formatter. [Read more][16]

[Source][19]§

### impl<S> [FromRequestParts][20]<S> for [MatchedPath][7]

where S: [Send][21] \+ [Sync][22],

[Source][23]§

#### type [Rejection][24] = [MatchedPathRejection][25]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][26]§

#### async fn [from_request_parts][27]( parts: &mut Parts, _state: [&S][28], ) -> [Result][29]<Self, Self::[Rejection][30]>

Perform the extraction.

[Source][31]§

### impl<S> [OptionalFromRequestParts][32]<S> for [MatchedPath][7]

where S: [Send][21] \+ [Sync][22],

[Source][33]§

#### type [Rejection][34] = [Infallible][35]

If the extractor fails, it will use this “rejection” type. [Read more][34]

[Source][36]§

#### async fn [from_request_parts][37]( parts: &mut Parts, _state: [&S][28], ) -> [Result][29]<[Option][38]<Self>, Self::[Rejection][39]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl [Freeze][40] for [MatchedPath][7]

§

### impl [RefUnwindSafe][41] for [MatchedPath][7]

§

### impl [Send][21] for [MatchedPath][7]

§

### impl [Sync][22] for [MatchedPath][7]

§

### impl [Unpin][42] for [MatchedPath][7]

§

### impl [UnwindSafe][43] for [MatchedPath][7]

## Blanket Implementations§

[Source][44]§

### impl<T> [Any][45] for T

where T: 'static + ?[Sized][46],

[Source][47]§

#### fn [type_id][48](&self) -> [TypeId][49]

Gets the `TypeId` of `self`. [Read more][48]

[Source][50]§

### impl<T> [Borrow][51]<T> for T

where T: ?[Sized][46],

[Source][52]§

#### fn [borrow][53](&self) -> [&T][28]

Immutably borrows from an owned value. [Read more][53]

[Source][54]§

### impl<T> [BorrowMut][55]<T> for T

where T: ?[Sized][46],

[Source][56]§

#### fn [borrow_mut][57](&mut self) -> [&mut T][28]

Mutably borrows from an owned value. [Read more][57]

[Source][58]§

### impl<T> [CloneToUninit][59] for T

where T: [Clone][11],

[Source][60]§

#### unsafe fn [clone_to_uninit][61](&self, dest: [*mut ][62][u8][63])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][61]

[Source][64]§

### impl<T> [From][65]<T> for T

[Source][66]§

#### fn [from][67](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][68]<T> for T

where T: [Clone][11],

§

#### fn [from_ref][69](input: [&T][28]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][70]<S, ViaParts> for T

where S: [Send][21] \+ [Sync][22], T: [FromRequestParts][20]<S>,

§

#### type [Rejection][71] = <T as [FromRequestParts][20]<S>>::[Rejection][30]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][72]( req: Request<[Body][73]>, state: [&S][28], ) -> impl [Future][74]<Output = [Result][29]<T, <T as [FromRequest][70]<S, ViaParts>>::[Rejection][75]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][76] [`Span`][77], returning an `Instrumented` wrapper. Read more

[Source][78]§

### impl<T, U> [Into][79]<U> for T

where U: [From][65]<T>,

[Source][80]§

#### fn [into][81](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][65]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][46],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][82]§

### impl<T> [Same][83] for T

[Source][84]§

#### type [Output][85] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][46],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][46],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][86]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][46],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][46],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][46],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][46],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][46],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][46],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][46],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][87] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][46],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][46],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][46],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][46], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][46], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][46],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][46],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][46],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][90]) -> RequestBodyLimit<Self>

where Self: [Sized][46],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][46],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][46],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][91]§

### impl<T> [ToOwned][92] for T

where T: [Clone][11],

[Source][93]§

#### type [Owned][94] = T

The resulting type after obtaining ownership.

[Source][95]§

#### fn [to_owned][96](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][96]

[Source][97]§

#### fn [clone_into][98](&self, target: [&mut T][28])

Uses borrowed data to replace owned data, usually by cloning. [Read more][98]

[Source][99]§

### impl<T, U> [TryFrom][100]<U> for T

where U: [Into][79]<T>,

[Source][101]§

#### type [Error][102] = [Infallible][35]

The type returned in the event of a conversion error.

[Source][103]§

#### fn [try_from][104](value: U) -> [Result][29]<T, <T as [TryFrom][100]<U>>::[Error][105]>

Performs the conversion.

[Source][106]§

### impl<T, U> [TryInto][107]<U> for T

where U: [TryFrom][100]<T>,

[Source][108]§

#### type [Error][109] = <U as [TryFrom][100]<T>>::[Error][105]

The type returned in the event of a conversion error.

[Source][110]§

#### fn [try_into][111](self) -> [Result][29]<U, <U as [TryFrom][100]<T>>::[Error][105]>

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

where S: [Into][79]<Dispatch>,

Attaches the provided [`Subscriber`][112] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][113] [`Subscriber`][112] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/matched_path.rs.html#57
   [5]: tower_http::trace::Trace
   [6]: ../../src/axum/extract/matched_path.rs.html#59-65
   [7]: struct.MatchedPath.html (struct axum::extract::MatchedPath)
   [8]: ../../src/axum/extract/matched_path.rs.html#62-64
   [9]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [10]: ../../src/axum/extract/matched_path.rs.html#56
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [13]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [19]: ../../src/axum/extract/matched_path.rs.html#67-82
   [20]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [21]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [22]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [23]: ../../src/axum/extract/matched_path.rs.html#71
   [24]: trait.FromRequestParts.html#associatedtype.Rejection
   [25]: rejection/enum.MatchedPathRejection.html (enum axum::extract::rejection::MatchedPathRejection)
   [26]: ../../src/axum/extract/matched_path.rs.html#73-81
   [27]: trait.FromRequestParts.html#tymethod.from_request_parts
   [28]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [29]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [30]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [31]: ../../src/axum/extract/matched_path.rs.html#84-96
   [32]: trait.OptionalFromRequestParts.html (trait axum::extract::OptionalFromRequestParts)
   [33]: ../../src/axum/extract/matched_path.rs.html#88
   [34]: trait.OptionalFromRequestParts.html#associatedtype.Rejection
   [35]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [36]: ../../src/axum/extract/matched_path.rs.html#90-95
   [37]: trait.OptionalFromRequestParts.html#tymethod.from_request_parts
   [38]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [39]: trait.OptionalFromRequestParts.html#associatedtype.Rejection (type axum::extract::OptionalFromRequestParts::Rejection)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [41]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [43]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [44]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [45]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [46]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [47]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [48]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [49]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [56]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [57]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [58]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [59]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [60]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [61]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [62]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [63]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [66]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [67]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [68]: trait.FromRef.html (trait axum::extract::FromRef)
   [69]: trait.FromRef.html#tymethod.from_ref
   [70]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [71]: trait.FromRequest.html#associatedtype.Rejection
   [72]: trait.FromRequest.html#tymethod.from_request
   [73]: ../body/struct.Body.html (struct axum::body::Body)
   [74]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [75]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [76]: super::Span::current()
   [77]: crate::Span
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [82]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [83]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [84]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [85]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [86]: https://docs.rs/http/latest/http/struct.Extensions.html
   [87]: crate::follow_redirect::policy::Standard
   [88]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [89]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [90]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [91]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [92]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [93]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [94]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [95]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [96]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [97]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [98]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [99]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [100]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [101]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [102]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [103]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [105]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [106]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [107]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [108]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [109]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [110]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [111]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [112]: super::Subscriber
   [113]: dispatcher#setting-the-default-subscriber

