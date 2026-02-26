<!-- Generated from rustdoc HTML: response/struct.NoContent.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## NoContent

## [axum][1]0.8.8

## NoContent

### Trait Implementations

  * Clone
  * Copy
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

# Struct NoContent Copy item path

[Source][4]
``` 
pub struct NoContent;
```

Expand description

An empty response with 204 No Content status.

Due to historical and implementation reasons, the `IntoResponse` implementation of `()` (unit type) returns an empty response with 200 [`StatusCode::OK`] status. If you specifically want a 204 [`StatusCode::NO_CONTENT`] status, you can use either `StatusCode` type directly, or this shortcut struct for self-documentation.
``` 
use axum::{extract::Path, response::NoContent};

async fn delete_user(Path(user): Path<String>) -> Result<NoContent, String> {
    // ...access database...
    Ok(NoContent)
}
```

## Trait Implementations§

[Source][5]§

### impl [Clone][6] for [NoContent][7]

[Source][5]§

#### fn [clone][8](&self) -> [NoContent][7]

Returns a duplicate of the value. [Read more][8]

1.0.0 · [Source][9]§

#### fn [clone_from][10](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][10]

[Source][5]§

### impl [Debug][11] for [NoContent][7]

[Source][5]§

#### fn [fmt][12](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]

Formats the value using the given formatter. [Read more][12]

[Source][15]§

### impl [IntoResponse][16] for [NoContent][7]

[Source][17]§

#### fn [into_response][18](self) -> [Response][19]

Create a response.

[Source][5]§

### impl [Copy][20] for [NoContent][7]

## Auto Trait Implementations§

§

### impl [Freeze][21] for [NoContent][7]

§

### impl [RefUnwindSafe][22] for [NoContent][7]

§

### impl [Send][23] for [NoContent][7]

§

### impl [Sync][24] for [NoContent][7]

§

### impl [Unpin][25] for [NoContent][7]

§

### impl [UnwindSafe][26] for [NoContent][7]

## Blanket Implementations§

[Source][27]§

### impl<T> [Any][28] for T

where T: 'static + ?[Sized][29],

[Source][30]§

#### fn [type_id][31](&self) -> [TypeId][32]

Gets the `TypeId` of `self`. [Read more][31]

[Source][33]§

### impl<T> [Borrow][34]<T> for T

where T: ?[Sized][29],

[Source][35]§

#### fn [borrow][36](&self) -> [&T][37]

Immutably borrows from an owned value. [Read more][36]

[Source][38]§

### impl<T> [BorrowMut][39]<T> for T

where T: ?[Sized][29],

[Source][40]§

#### fn [borrow_mut][41](&mut self) -> [&mut T][37]

Mutably borrows from an owned value. [Read more][41]

[Source][42]§

### impl<T> [CloneToUninit][43] for T

where T: [Clone][6],

[Source][44]§

#### unsafe fn [clone_to_uninit][45](&self, dest: [*mut ][46][u8][47])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][45]

[Source][48]§

### impl<T> [From][49]<T> for T

[Source][50]§

#### fn [from][51](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][52]<T> for T

where T: [Clone][6],

§

#### fn [from_ref][53](input: [&T][37]) -> T

Converts to this type from a reference to the input type.

[Source][54]§

### impl<H, T> [HandlerWithoutStateExt][55]<T> for H

where H: [Handler][56]<T, [()][57]>,

[Source][58]§

#### fn [into_service][59](self) -> [HandlerService][60]<H, T, [()][57]>

Convert the handler into a [`Service`] and no state.

[Source][61]§

#### fn [into_make_service][62](self) -> [IntoMakeService][63]<[HandlerService][60]<H, T, [()][57]>>

Convert the handler into a [`MakeService`][64] and no state. [Read more][62]

[Source][65]§

#### fn [into_make_service_with_connect_info][66]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][67]<[HandlerService][60]<H, T, [()][57]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][64] which stores information about the incoming connection and has no state. [Read more][66]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][68] [`Span`][69], returning an `Instrumented` wrapper. Read more

[Source][70]§

### impl<T, U> [Into][71]<U> for T

where U: [From][49]<T>,

[Source][72]§

#### fn [into][73](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][49]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][29],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][74]§

### impl<T> [Same][75] for T

[Source][76]§

#### type [Output][77] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][29],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][29],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][78]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][29],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][29],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][29],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][29],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][29],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][29],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][29],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][79] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][80]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][29],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][81] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][80]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][29],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][81] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][80]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][29],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][81] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][29], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][29], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][29],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][29],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][29],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][82]) -> RequestBodyLimit<Self>

where Self: [Sized][29],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][29],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][29],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][83]§

### impl<T> [ToOwned][84] for T

where T: [Clone][6],

[Source][85]§

#### type [Owned][86] = T

The resulting type after obtaining ownership.

[Source][87]§

#### fn [to_owned][88](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][88]

[Source][89]§

#### fn [clone_into][90](&self, target: [&mut T][37])

Uses borrowed data to replace owned data, usually by cloning. [Read more][90]

[Source][91]§

### impl<T, U> [TryFrom][92]<U> for T

where U: [Into][71]<T>,

[Source][93]§

#### type [Error][94] = [Infallible][95]

The type returned in the event of a conversion error.

[Source][96]§

#### fn [try_from][97](value: U) -> [Result][98]<T, <T as [TryFrom][92]<U>>::[Error][99]>

Performs the conversion.

[Source][100]§

### impl<T, U> [TryInto][101]<U> for T

where U: [TryFrom][92]<T>,

[Source][102]§

#### type [Error][103] = <U as [TryFrom][92]<T>>::[Error][99]

The type returned in the event of a conversion error.

[Source][104]§

#### fn [try_into][105](self) -> [Result][98]<U, <U as [TryFrom][92]<T>>::[Error][99]>

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

where S: [Into][71]<Dispatch>,

Attaches the provided [`Subscriber`][106] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][107] [`Subscriber`][106] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/response/mod.rs.html#78
   [5]: ../../src/axum/response/mod.rs.html#77
   [6]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [7]: struct.NoContent.html (struct axum::response::NoContent)
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [9]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [13]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [15]: ../../src/axum/response/mod.rs.html#80-84
   [16]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [17]: ../../src/axum/response/mod.rs.html#81-83
   [18]: trait.IntoResponse.html#tymethod.into_response
   [19]: type.Response.html (type axum::response::Response)
   [20]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [21]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [22]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [26]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [27]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [28]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [30]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [31]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [32]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [33]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [34]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [35]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [36]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [37]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [38]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [39]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [42]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [43]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [44]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [45]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [46]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [47]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [48]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [49]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [50]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [51]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [52]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [53]: ../extract/trait.FromRef.html#tymethod.from_ref
   [54]: ../../src/axum/handler/mod.rs.html#380-398
   [55]: ../handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [56]: ../handler/trait.Handler.html (trait axum::handler::Handler)
   [57]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [58]: ../../src/axum/handler/mod.rs.html#384-386
   [59]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [60]: ../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [61]: ../../src/axum/handler/mod.rs.html#388-390
   [62]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [63]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [64]: tower::make::MakeService
   [65]: ../../src/axum/handler/mod.rs.html#393-397
   [66]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [67]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [68]: super::Span::current()
   [69]: crate::Span
   [70]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [71]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [72]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [73]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [74]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [75]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [76]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [77]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [78]: https://docs.rs/http/latest/http/struct.Extensions.html
   [79]: crate::follow_redirect::policy::Standard
   [80]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [81]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [82]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [83]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [84]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [85]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [86]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [87]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [88]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [89]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [90]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [95]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [98]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [100]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [101]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [102]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [103]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [104]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [105]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [106]: super::Subscriber
   [107]: dispatcher#setting-the-default-subscriber

