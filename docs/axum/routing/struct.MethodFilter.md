<!-- Generated from rustdoc HTML: routing/struct.MethodFilter.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MethodFilter

## [axum][1]0.8.8

## MethodFilter

### Associated Constants

  * CONNECT
  * DELETE
  * GET
  * HEAD
  * OPTIONS
  * PATCH
  * POST
  * PUT
  * TRACE



### Methods

  * or



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * PartialEq
  * StructuralPartialEq
  * TryFrom<Method>



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



## [In axum::routing][2]

[axum][3]::[routing][2]

# Struct MethodFilter Copy item path

[Source][4]
``` 
pub struct MethodFilter(/* private fields */);
```

Expand description

A filter that matches one or more HTTP methods.

## Implementations§

[Source][5]§

### impl [MethodFilter][6]

[Source][7]

#### pub const CONNECT: Self

Match `CONNECT` requests.

This is useful for implementing HTTP/2’s [extended CONNECT method], in which the `:protocol` pseudoheader is read (using [`hyper::ext::Protocol`]) and the connection upgraded to a bidirectional byte stream (using [`hyper::upgrade::on`]).

As seen in the [HTTP Upgrade Token Registry][8], common uses include WebSockets and proxying UDP or IP – though note that when using [`WebSocketUpgrade`][9] it’s more useful to use [`any`][10] as HTTP/1.1 WebSockets need to support `GET`.

[Source][11]

#### pub const DELETE: Self

Match `DELETE` requests.

[Source][12]

#### pub const GET: Self

Match `GET` requests.

[Source][13]

#### pub const HEAD: Self

Match `HEAD` requests.

[Source][14]

#### pub const OPTIONS: Self

Match `OPTIONS` requests.

[Source][15]

#### pub const PATCH: Self

Match `PATCH` requests.

[Source][16]

#### pub const POST: Self

Match `POST` requests.

[Source][17]

#### pub const PUT: Self

Match `PUT` requests.

[Source][18]

#### pub const TRACE: Self

Match `TRACE` requests.

[Source][19]

#### pub const fn or(self, other: Self) -> Self

Performs the OR operation between the [`MethodFilter`][6] in `self` with `other`.

## Trait Implementations§

[Source][20]§

### impl [Clone][21] for [MethodFilter][6]

[Source][20]§

#### fn [clone][22](&self) -> [MethodFilter][6]

Returns a duplicate of the value. [Read more][22]

1.0.0 · [Source][23]§

#### fn [clone_from][24](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][24]

[Source][20]§

### impl [Debug][25] for [MethodFilter][6]

[Source][20]§

#### fn [fmt][26](&self, f: &mut [Formatter][27]<'_>) -> [Result][28]

Formats the value using the given formatter. [Read more][26]

[Source][20]§

### impl [PartialEq][29] for [MethodFilter][6]

[Source][20]§

#### fn [eq][30](&self, other: &[MethodFilter][6]) -> [bool][31]

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source][32]§

#### fn [ne][33](&self, other: [&Rhs][34]) -> [bool][31]

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source][35]§

### impl [TryFrom][36]<Method> for [MethodFilter][6]

[Source][37]§

#### type [Error][38] = NoMatchingMethodFilter

The type returned in the event of a conversion error.

[Source][39]§

#### fn [try_from][40](m: Method) -> [Result][41]<Self, NoMatchingMethodFilter>

Performs the conversion.

[Source][20]§

### impl [Copy][42] for [MethodFilter][6]

[Source][20]§

### impl [StructuralPartialEq][43] for [MethodFilter][6]

## Auto Trait Implementations§

§

### impl [Freeze][44] for [MethodFilter][6]

§

### impl [RefUnwindSafe][45] for [MethodFilter][6]

§

### impl [Send][46] for [MethodFilter][6]

§

### impl [Sync][47] for [MethodFilter][6]

§

### impl [Unpin][48] for [MethodFilter][6]

§

### impl [UnwindSafe][49] for [MethodFilter][6]

## Blanket Implementations§

[Source][50]§

### impl<T> [Any][51] for T

where T: 'static + ?[Sized][52],

[Source][53]§

#### fn [type_id][54](&self) -> [TypeId][55]

Gets the `TypeId` of `self`. [Read more][54]

[Source][56]§

### impl<T> [Borrow][57]<T> for T

where T: ?[Sized][52],

[Source][58]§

#### fn [borrow][59](&self) -> [&T][34]

Immutably borrows from an owned value. [Read more][59]

[Source][60]§

### impl<T> [BorrowMut][61]<T> for T

where T: ?[Sized][52],

[Source][62]§

#### fn [borrow_mut][63](&mut self) -> [&mut T][34]

Mutably borrows from an owned value. [Read more][63]

[Source][64]§

### impl<T> [CloneToUninit][65] for T

where T: [Clone][21],

[Source][66]§

#### unsafe fn [clone_to_uninit][67](&self, dest: [*mut ][68][u8][69])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][67]

[Source][70]§

### impl<T> [From][71]<T> for T

[Source][72]§

#### fn [from][73](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][74]<T> for T

where T: [Clone][21],

§

#### fn [from_ref][75](input: [&T][34]) -> T

Converts to this type from a reference to the input type.

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

where U: [From][71]<T>,

[Source][80]§

#### fn [into][81](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][71]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][52],

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

where Self: [Sized][52],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][52],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][86]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][52],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][52],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][52],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][52],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][52],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][52],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][52],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][87] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][52],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][52],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][52],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][52],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][52],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][52],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][52],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][52],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][52],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][52], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][52], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][52],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][52],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][52],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][90]) -> RequestBodyLimit<Self>

where Self: [Sized][52],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][52],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][52],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][91]§

### impl<T> [ToOwned][92] for T

where T: [Clone][21],

[Source][93]§

#### type [Owned][94] = T

The resulting type after obtaining ownership.

[Source][95]§

#### fn [to_owned][96](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][96]

[Source][97]§

#### fn [clone_into][98](&self, target: [&mut T][34])

Uses borrowed data to replace owned data, usually by cloning. [Read more][98]

[Source][99]§

### impl<T, U> [TryFrom][36]<U> for T

where U: [Into][79]<T>,

[Source][100]§

#### type [Error][38] = [Infallible][101]

The type returned in the event of a conversion error.

[Source][102]§

#### fn [try_from][40](value: U) -> [Result][41]<T, <T as [TryFrom][36]<U>>::[Error][103]>

Performs the conversion.

[Source][104]§

### impl<T, U> [TryInto][105]<U> for T

where U: [TryFrom][36]<T>,

[Source][106]§

#### type [Error][107] = <U as [TryFrom][36]<T>>::[Error][103]

The type returned in the event of a conversion error.

[Source][108]§

#### fn [try_into][109](self) -> [Result][41]<U, <U as [TryFrom][36]<T>>::[Error][103]>

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

Attaches the provided [`Subscriber`][110] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][111] [`Subscriber`][110] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/routing/method_filter.rs.html#9
   [5]: ../../src/axum/routing/method_filter.rs.html#11-65
   [6]: struct.MethodFilter.html (struct axum::routing::MethodFilter)
   [7]: ../../src/axum/routing/method_filter.rs.html#29
   [8]: https://www.iana.org/assignments/http-upgrade-tokens/http-upgrade-tokens.xhtml
   [9]: ../extract/struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [10]: method_routing/fn.any.html (fn axum::routing::method_routing::any)
   [11]: ../../src/axum/routing/method_filter.rs.html#31
   [12]: ../../src/axum/routing/method_filter.rs.html#33
   [13]: ../../src/axum/routing/method_filter.rs.html#35
   [14]: ../../src/axum/routing/method_filter.rs.html#37
   [15]: ../../src/axum/routing/method_filter.rs.html#39
   [16]: ../../src/axum/routing/method_filter.rs.html#41
   [17]: ../../src/axum/routing/method_filter.rs.html#43
   [18]: ../../src/axum/routing/method_filter.rs.html#45
   [19]: ../../src/axum/routing/method_filter.rs.html#62-64
   [20]: ../../src/axum/routing/method_filter.rs.html#8
   [21]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [22]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [23]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [24]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [25]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [26]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [27]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [28]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [29]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html (trait core::cmp::PartialEq)
   [30]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq
   [31]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [32]: https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264
   [33]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne
   [34]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [35]: ../../src/axum/routing/method_filter.rs.html#88-105
   [36]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [37]: ../../src/axum/routing/method_filter.rs.html#89
   [38]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [39]: ../../src/axum/routing/method_filter.rs.html#91-104
   [40]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [41]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html (trait core::marker::StructuralPartialEq)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [45]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [46]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [47]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [48]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [49]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [50]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [51]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [52]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [53]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [54]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [55]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [56]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [57]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [58]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [59]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [60]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [61]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [62]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [63]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [64]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [65]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [66]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [67]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [68]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [69]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [70]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [71]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [72]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [73]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [74]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [75]: ../extract/trait.FromRef.html#tymethod.from_ref
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
   [100]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [101]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [102]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [103]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [104]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [105]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [106]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [107]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [108]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [109]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [110]: super::Subscriber
   [111]: dispatcher#setting-the-default-subscriber

