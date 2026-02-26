<!-- Generated from rustdoc HTML: response/struct.Redirect.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Redirect

## [axum][1]0.8.8

## Redirect

### Sections

  * Example



### Methods

  * location
  * permanent
  * status_code
  * temporary
  * to



### Trait Implementations

  * Clone
  * Debug
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
  * CloneToUninit
  * From<T>
  * FromRef<T>
  * HandlerWithoutStateExt<T>
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



## [In axum::response][2]

[axum][3]::[response][2]

# Struct Redirect Copy item path

[Source][4]
``` 
pub struct Redirect { /* private fields */ }
```

Expand description

Response that redirects the request to another location.

## §Example
``` 
use axum::{
    routing::get,
    response::Redirect,
    Router,
};

let app = Router::new()
    .route("/old", get(|| async { Redirect::permanent("/new") }))
    .route("/new", get(|| async { "Hello!" }));
```

## Implementations§

[Source][5]§

### impl [Redirect][6]

[Source][7]

#### pub fn to(uri: impl [Into][8]<[String][9]>) -> Self

Create a new [`Redirect`][6] that uses a [`303 See Other`][10] status code.

This redirect instructs the client to change the method to GET for the subsequent request to the given `uri`, which is useful after successful form submission, file upload or when you generally don’t want the redirected-to page to observe the original request method and body (if non-empty). If you want to preserve the request method and body, [`Redirect::temporary`][11] should be used instead.

[Source][12]

#### pub fn temporary(uri: impl [Into][8]<[String][9]>) -> Self

Create a new [`Redirect`][6] that uses a [`307 Temporary Redirect`][13] status code.

This has the same behavior as [`Redirect::to`][14], except it will preserve the original HTTP method and body.

[Source][15]

#### pub fn permanent(uri: impl [Into][8]<[String][9]>) -> Self

Create a new [`Redirect`][6] that uses a [`308 Permanent Redirect`][16] status code.

[Source][17]

#### pub fn status_code(&self) -> StatusCode

Returns the HTTP status code of the `Redirect`.

[Source][18]

#### pub fn location(&self) -> &[str][19]

Returns the `Redirect`’s URI.

## Trait Implementations§

[Source][20]§

### impl [Clone][21] for [Redirect][6]

[Source][20]§

#### fn [clone][22](&self) -> [Redirect][6]

Returns a duplicate of the value. [Read more][22]

1.0.0 · [Source][23]§

#### fn [clone_from][24](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][24]

[Source][20]§

### impl [Debug][25] for [Redirect][6]

[Source][20]§

#### fn [fmt][26](&self, f: &mut [Formatter][27]<'_>) -> [Result][28]

Formats the value using the given formatter. [Read more][26]

[Source][29]§

### impl [IntoResponse][30] for [Redirect][6]

[Source][31]§

#### fn [into_response][32](self) -> [Response][33]

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][34] for [Redirect][6]

§

### impl [RefUnwindSafe][35] for [Redirect][6]

§

### impl [Send][36] for [Redirect][6]

§

### impl [Sync][37] for [Redirect][6]

§

### impl [Unpin][38] for [Redirect][6]

§

### impl [UnwindSafe][39] for [Redirect][6]

## Blanket Implementations§

[Source][40]§

### impl<T> [Any][41] for T

where T: 'static + ?[Sized][42],

[Source][43]§

#### fn [type_id][44](&self) -> [TypeId][45]

Gets the `TypeId` of `self`. [Read more][44]

[Source][46]§

### impl<T> [Borrow][47]<T> for T

where T: ?[Sized][42],

[Source][48]§

#### fn [borrow][49](&self) -> [&T][50]

Immutably borrows from an owned value. [Read more][49]

[Source][51]§

### impl<T> [BorrowMut][52]<T> for T

where T: ?[Sized][42],

[Source][53]§

#### fn [borrow_mut][54](&mut self) -> [&mut T][50]

Mutably borrows from an owned value. [Read more][54]

[Source][55]§

### impl<T> [CloneToUninit][56] for T

where T: [Clone][21],

[Source][57]§

#### unsafe fn [clone_to_uninit][58](&self, dest: [*mut ][59][u8][60])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][58]

[Source][61]§

### impl<T> [From][62]<T> for T

[Source][63]§

#### fn [from][64](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][65]<T> for T

where T: [Clone][21],

§

#### fn [from_ref][66](input: [&T][50]) -> T

Converts to this type from a reference to the input type.

[Source][67]§

### impl<H, T> [HandlerWithoutStateExt][68]<T> for H

where H: [Handler][69]<T, [()][70]>,

[Source][71]§

#### fn [into_service][72](self) -> [HandlerService][73]<H, T, [()][70]>

Convert the handler into a [`Service`] and no state.

[Source][74]§

#### fn [into_make_service][75](self) -> [IntoMakeService][76]<[HandlerService][73]<H, T, [()][70]>>

Convert the handler into a [`MakeService`][77] and no state. [Read more][75]

[Source][78]§

#### fn [into_make_service_with_connect_info][79]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][80]<[HandlerService][73]<H, T, [()][70]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][77] which stores information about the incoming connection and has no state. [Read more][79]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][81] [`Span`][82], returning an `Instrumented` wrapper. Read more

[Source][83]§

### impl<T, U> [Into][8]<U> for T

where U: [From][62]<T>,

[Source][84]§

#### fn [into][85](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][62]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][42],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][86]§

### impl<T> [Same][87] for T

[Source][88]§

#### type [Output][89] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][42],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][42],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][90]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][42],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][42],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][42],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][42],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][42],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][42],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][42],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][91] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][92]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][42],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][93] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][92]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][42],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][93] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][92]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][42],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][93] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][42], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][42], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][42],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][42],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][42],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][94]) -> RequestBodyLimit<Self>

where Self: [Sized][42],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][42],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][42],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][95]§

### impl<T> [ToOwned][96] for T

where T: [Clone][21],

[Source][97]§

#### type [Owned][98] = T

The resulting type after obtaining ownership.

[Source][99]§

#### fn [to_owned][100](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][100]

[Source][101]§

#### fn [clone_into][102](&self, target: [&mut T][50])

Uses borrowed data to replace owned data, usually by cloning. [Read more][102]

[Source][103]§

### impl<T, U> [TryFrom][104]<U> for T

where U: [Into][8]<T>,

[Source][105]§

#### type [Error][106] = [Infallible][107]

The type returned in the event of a conversion error.

[Source][108]§

#### fn [try_from][109](value: U) -> [Result][110]<T, <T as [TryFrom][104]<U>>::[Error][111]>

Performs the conversion.

[Source][112]§

### impl<T, U> [TryInto][113]<U> for T

where U: [TryFrom][104]<T>,

[Source][114]§

#### type [Error][115] = <U as [TryFrom][104]<T>>::[Error][111]

The type returned in the event of a conversion error.

[Source][116]§

#### fn [try_into][117](self) -> [Result][110]<U, <U as [TryFrom][104]<T>>::[Error][111]>

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

where S: [Into][8]<Dispatch>,

Attaches the provided [`Subscriber`][118] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][119] [`Subscriber`][118] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/response/redirect.rs.html#22-25
   [5]: ../../src/axum/response/redirect.rs.html#27-85
   [6]: struct.Redirect.html (struct axum::response::Redirect)
   [7]: ../../src/axum/response/redirect.rs.html#37-39
   [8]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [9]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [10]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/303
   [11]: struct.Redirect.html#method.temporary (associated function axum::response::Redirect::temporary)
   [12]: ../../src/axum/response/redirect.rs.html#47-49
   [13]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/307
   [14]: struct.Redirect.html#method.to (associated function axum::response::Redirect::to)
   [15]: ../../src/axum/response/redirect.rs.html#54-56
   [16]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/308
   [17]: ../../src/axum/response/redirect.rs.html#60-62
   [18]: ../../src/axum/response/redirect.rs.html#66-68
   [19]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [20]: ../../src/axum/response/redirect.rs.html#21
   [21]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [22]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [23]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [24]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [25]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [26]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [27]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [28]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [29]: ../../src/axum/response/redirect.rs.html#87-94
   [30]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [31]: ../../src/axum/response/redirect.rs.html#88-93
   [32]: trait.IntoResponse.html#tymethod.into_response
   [33]: type.Response.html (type axum::response::Response)
   [34]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [35]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [39]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [40]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [41]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [43]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [44]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [45]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [50]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [55]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [56]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [57]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [58]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [59]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [60]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [65]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [66]: ../extract/trait.FromRef.html#tymethod.from_ref
   [67]: ../../src/axum/handler/mod.rs.html#380-398
   [68]: ../handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [69]: ../handler/trait.Handler.html (trait axum::handler::Handler)
   [70]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [71]: ../../src/axum/handler/mod.rs.html#384-386
   [72]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [73]: ../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [74]: ../../src/axum/handler/mod.rs.html#388-390
   [75]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [76]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [77]: tower::make::MakeService
   [78]: ../../src/axum/handler/mod.rs.html#393-397
   [79]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [80]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [81]: super::Span::current()
   [82]: crate::Span
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [84]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [86]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [87]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [88]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [89]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [90]: https://docs.rs/http/latest/http/struct.Extensions.html
   [91]: crate::follow_redirect::policy::Standard
   [92]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [93]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [94]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [95]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [96]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [97]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [98]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [99]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [100]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [101]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [102]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [103]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [105]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [106]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [107]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [108]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [109]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [110]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [111]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [112]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [113]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [114]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [115]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [116]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [117]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [118]: super::Subscriber
   [119]: dispatcher#setting-the-default-subscriber

