<!-- Generated from rustdoc HTML: response/struct.Html.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Html

## [axum][1]0.8.8

## Html

### Tuple Fields

  * 0



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * From<T>
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
  * From<!>
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

# Struct Html Copy item path

[Source][4]
``` 
pub struct Html<T>(pub T);
```

Expand description

An HTML response.

Will automatically get `Content-Type: text/html`.

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][5]§

### impl<T: [Clone][6]> [Clone][6] for [Html][7]<T>

[Source][5]§

#### fn [clone][8](&self) -> [Html][7]<T>

Returns a duplicate of the value. [Read more][8]

1.0.0 · [Source][9]§

#### fn [clone_from][10](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][10]

[Source][5]§

### impl<T: [Debug][11]> [Debug][11] for [Html][7]<T>

[Source][5]§

#### fn [fmt][12](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]

Formats the value using the given formatter. [Read more][12]

[Source][15]§

### impl<T> [From][16]<T> for [Html][7]<T>

[Source][17]§

#### fn [from][18](inner: T) -> Self

Converts to this type from the input type.

[Source][19]§

### impl<T> [IntoResponse][20] for [Html][7]<T>

where T: [IntoResponse][20],

[Source][21]§

#### fn [into_response][22](self) -> [Response][23]

Create a response.

[Source][5]§

### impl<T: [Copy][24]> [Copy][24] for [Html][7]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][25] for [Html][7]<T>

where T: [Freeze][25],

§

### impl<T> [RefUnwindSafe][26] for [Html][7]<T>

where T: [RefUnwindSafe][26],

§

### impl<T> [Send][27] for [Html][7]<T>

where T: [Send][27],

§

### impl<T> [Sync][28] for [Html][7]<T>

where T: [Sync][28],

§

### impl<T> [Unpin][29] for [Html][7]<T>

where T: [Unpin][29],

§

### impl<T> [UnwindSafe][30] for [Html][7]<T>

where T: [UnwindSafe][30],

## Blanket Implementations§

[Source][31]§

### impl<T> [Any][32] for T

where T: 'static + ?[Sized][33],

[Source][34]§

#### fn [type_id][35](&self) -> [TypeId][36]

Gets the `TypeId` of `self`. [Read more][35]

[Source][37]§

### impl<T> [Borrow][38]<T> for T

where T: ?[Sized][33],

[Source][39]§

#### fn [borrow][40](&self) -> [&T][41]

Immutably borrows from an owned value. [Read more][40]

[Source][42]§

### impl<T> [BorrowMut][43]<T> for T

where T: ?[Sized][33],

[Source][44]§

#### fn [borrow_mut][45](&mut self) -> [&mut T][41]

Mutably borrows from an owned value. [Read more][45]

[Source][46]§

### impl<T> [CloneToUninit][47] for T

where T: [Clone][6],

[Source][48]§

#### unsafe fn [clone_to_uninit][49](&self, dest: [*mut ][50][u8][51])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][49]

[Source][52]§

### impl<T> [From][16]<[!][53]> for T

[Source][54]§

#### fn [from][18](t: [!][53]) -> T

Converts to this type from the input type.

[Source][55]§

### impl<T> [From][16]<T> for T

[Source][56]§

#### fn [from][18](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][57]<T> for T

where T: [Clone][6],

§

#### fn [from_ref][58](input: [&T][41]) -> T

Converts to this type from a reference to the input type.

[Source][59]§

### impl<H, T> [HandlerWithoutStateExt][60]<T> for H

where H: [Handler][61]<T, [()][62]>,

[Source][63]§

#### fn [into_service][64](self) -> [HandlerService][65]<H, T, [()][62]>

Convert the handler into a [`Service`] and no state.

[Source][66]§

#### fn [into_make_service][67](self) -> [IntoMakeService][68]<[HandlerService][65]<H, T, [()][62]>>

Convert the handler into a [`MakeService`][69] and no state. [Read more][67]

[Source][70]§

#### fn [into_make_service_with_connect_info][71]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][72]<[HandlerService][65]<H, T, [()][62]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][69] which stores information about the incoming connection and has no state. [Read more][71]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][73] [`Span`][74], returning an `Instrumented` wrapper. Read more

[Source][75]§

### impl<T, U> [Into][76]<U> for T

where U: [From][16]<T>,

[Source][77]§

#### fn [into][78](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][16]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][33],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][79]§

### impl<T> [Same][80] for T

[Source][81]§

#### type [Output][82] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][33],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][33],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][83]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][33],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][33],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][33],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][33],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][33],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][33],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][33],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][84] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][85]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][86] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][85]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][86] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][85]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][86] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][33],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][33], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][33], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][33],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][33],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][33],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][87]) -> RequestBodyLimit<Self>

where Self: [Sized][33],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][33],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][33],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][88]§

### impl<T> [ToOwned][89] for T

where T: [Clone][6],

[Source][90]§

#### type [Owned][91] = T

The resulting type after obtaining ownership.

[Source][92]§

#### fn [to_owned][93](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][93]

[Source][94]§

#### fn [clone_into][95](&self, target: [&mut T][41])

Uses borrowed data to replace owned data, usually by cloning. [Read more][95]

[Source][96]§

### impl<T, U> [TryFrom][97]<U> for T

where U: [Into][76]<T>,

[Source][98]§

#### type [Error][99] = [Infallible][100]

The type returned in the event of a conversion error.

[Source][101]§

#### fn [try_from][102](value: U) -> [Result][103]<T, <T as [TryFrom][97]<U>>::[Error][104]>

Performs the conversion.

[Source][105]§

### impl<T, U> [TryInto][106]<U> for T

where U: [TryFrom][97]<T>,

[Source][107]§

#### type [Error][108] = <U as [TryFrom][97]<T>>::[Error][104]

The type returned in the event of a conversion error.

[Source][109]§

#### fn [try_into][110](self) -> [Result][103]<U, <U as [TryFrom][97]<T>>::[Error][104]>

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

where S: [Into][76]<Dispatch>,

Attaches the provided [`Subscriber`][111] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][112] [`Subscriber`][111] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/response/mod.rs.html#37
   [5]: ../../src/axum/response/mod.rs.html#35
   [6]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [7]: struct.Html.html (struct axum::response::Html)
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [9]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [13]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [15]: ../../src/axum/response/mod.rs.html#55-59
   [16]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [17]: ../../src/axum/response/mod.rs.html#56-58
   [18]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [19]: ../../src/axum/response/mod.rs.html#39-53
   [20]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [21]: ../../src/axum/response/mod.rs.html#43-52
   [22]: trait.IntoResponse.html#tymethod.into_response
   [23]: type.Response.html (type axum::response::Response)
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [26]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [30]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [31]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [32]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [34]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [35]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [36]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [37]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [38]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [39]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [40]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [41]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [46]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [47]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [48]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [49]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [50]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [51]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#802
   [53]: https://doc.rust-lang.org/nightly/std/primitive.never.html
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#803
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [57]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [58]: ../extract/trait.FromRef.html#tymethod.from_ref
   [59]: ../../src/axum/handler/mod.rs.html#380-398
   [60]: ../handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [61]: ../handler/trait.Handler.html (trait axum::handler::Handler)
   [62]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [63]: ../../src/axum/handler/mod.rs.html#384-386
   [64]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [65]: ../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [66]: ../../src/axum/handler/mod.rs.html#388-390
   [67]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [68]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [69]: tower::make::MakeService
   [70]: ../../src/axum/handler/mod.rs.html#393-397
   [71]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [72]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [73]: super::Span::current()
   [74]: crate::Span
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [79]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [80]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [81]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [82]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [83]: https://docs.rs/http/latest/http/struct.Extensions.html
   [84]: crate::follow_redirect::policy::Standard
   [85]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [86]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [87]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [88]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [89]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [90]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [91]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [92]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [93]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [94]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [95]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [98]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [100]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [101]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [102]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [103]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [105]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [106]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [107]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [108]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [109]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [110]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [111]: super::Subscriber
   [112]: dispatcher#setting-the-default-subscriber

