<!-- Generated from rustdoc HTML: middleware/struct.ResponseAxumBodyLayer.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ResponseAxumBodyLayer

## [axum][1]0.8.8

## ResponseAxumBodyLayer

### Trait Implementations

  * Clone
  * Debug
  * Layer<S>



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



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Struct ResponseAxumBodyLayer Copy item path

[Source][4]
``` 
pub struct ResponseAxumBodyLayer;
```

Expand description

Layer that transforms the Response body to [`crate::body::Body`][5].

This is useful when another layer maps the body to some other type to convert it back.

## Trait Implementations§

[Source][6]§

### impl [Clone][7] for [ResponseAxumBodyLayer][8]

[Source][6]§

#### fn [clone][9](&self) -> [ResponseAxumBodyLayer][8]

Returns a duplicate of the value. [Read more][9]

1.0.0 · [Source][10]§

#### fn [clone_from][11](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][11]

[Source][6]§

### impl [Debug][12] for [ResponseAxumBodyLayer][8]

[Source][6]§

#### fn [fmt][13](&self, f: &mut [Formatter][14]<'_>) -> [Result][15]

Formats the value using the given formatter. [Read more][13]

[Source][16]§

### impl<S> Layer<S> for [ResponseAxumBodyLayer][8]

[Source][17]§

#### type Service = [ResponseAxumBody][18]<S>

The wrapped service

[Source][19]§

#### fn layer(&self, inner: S) -> Self::Service

Wrap the given service with the middleware, returning a new service that has been decorated with the middleware.

## Auto Trait Implementations§

§

### impl [Freeze][20] for [ResponseAxumBodyLayer][8]

§

### impl [RefUnwindSafe][21] for [ResponseAxumBodyLayer][8]

§

### impl [Send][22] for [ResponseAxumBodyLayer][8]

§

### impl [Sync][23] for [ResponseAxumBodyLayer][8]

§

### impl [Unpin][24] for [ResponseAxumBodyLayer][8]

§

### impl [UnwindSafe][25] for [ResponseAxumBodyLayer][8]

## Blanket Implementations§

[Source][26]§

### impl<T> [Any][27] for T

where T: 'static + ?[Sized][28],

[Source][29]§

#### fn [type_id][30](&self) -> [TypeId][31]

Gets the `TypeId` of `self`. [Read more][30]

[Source][32]§

### impl<T> [Borrow][33]<T> for T

where T: ?[Sized][28],

[Source][34]§

#### fn [borrow][35](&self) -> [&T][36]

Immutably borrows from an owned value. [Read more][35]

[Source][37]§

### impl<T> [BorrowMut][38]<T> for T

where T: ?[Sized][28],

[Source][39]§

#### fn [borrow_mut][40](&mut self) -> [&mut T][36]

Mutably borrows from an owned value. [Read more][40]

[Source][41]§

### impl<T> [CloneToUninit][42] for T

where T: [Clone][7],

[Source][43]§

#### unsafe fn [clone_to_uninit][44](&self, dest: [*mut ][45][u8][46])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][44]

[Source][47]§

### impl<T> [From][48]<T> for T

[Source][49]§

#### fn [from][50](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][51]<T> for T

where T: [Clone][7],

§

#### fn [from_ref][52](input: [&T][36]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][53] [`Span`][54], returning an `Instrumented` wrapper. Read more

[Source][55]§

### impl<T, U> [Into][56]<U> for T

where U: [From][48]<T>,

[Source][57]§

#### fn [into][58](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][48]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][28],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][59]§

### impl<T> [Same][60] for T

[Source][61]§

#### type [Output][62] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][28],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][28],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][63]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][28],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][28],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][28],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][28],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][28],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][28],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][28],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][64] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][65]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][28],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][66] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][65]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][28],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][66] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][65]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][28],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][66] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][28],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][28],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][28],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][28],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][28],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][28],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][28], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][28], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][28],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][28],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][28],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][67]) -> RequestBodyLimit<Self>

where Self: [Sized][28],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][28],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][28],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][68]§

### impl<T> [ToOwned][69] for T

where T: [Clone][7],

[Source][70]§

#### type [Owned][71] = T

The resulting type after obtaining ownership.

[Source][72]§

#### fn [to_owned][73](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][73]

[Source][74]§

#### fn [clone_into][75](&self, target: [&mut T][36])

Uses borrowed data to replace owned data, usually by cloning. [Read more][75]

[Source][76]§

### impl<T, U> [TryFrom][77]<U> for T

where U: [Into][56]<T>,

[Source][78]§

#### type [Error][79] = [Infallible][80]

The type returned in the event of a conversion error.

[Source][81]§

#### fn [try_from][82](value: U) -> [Result][83]<T, <T as [TryFrom][77]<U>>::[Error][84]>

Performs the conversion.

[Source][85]§

### impl<T, U> [TryInto][86]<U> for T

where U: [TryFrom][77]<T>,

[Source][87]§

#### type [Error][88] = <U as [TryFrom][77]<T>>::[Error][84]

The type returned in the event of a conversion error.

[Source][89]§

#### fn [try_into][90](self) -> [Result][83]<U, <U as [TryFrom][77]<T>>::[Error][84]>

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

where S: [Into][56]<Dispatch>,

Attaches the provided [`Subscriber`][91] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][92] [`Subscriber`][91] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/response_axum_body.rs.html#18
   [5]: ../body/struct.Body.html (struct axum::body::Body)
   [6]: ../../src/axum/middleware/response_axum_body.rs.html#17
   [7]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [8]: struct.ResponseAxumBodyLayer.html (struct axum::middleware::ResponseAxumBodyLayer)
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [10]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [16]: ../../src/axum/middleware/response_axum_body.rs.html#20-26
   [17]: ../../src/axum/middleware/response_axum_body.rs.html#21
   [18]: struct.ResponseAxumBody.html (struct axum::middleware::ResponseAxumBody)
   [19]: ../../src/axum/middleware/response_axum_body.rs.html#23-25
   [20]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [21]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [22]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [25]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [26]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [27]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [29]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [30]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [31]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [32]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [33]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [34]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [35]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [36]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [37]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [38]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [39]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [40]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [41]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [42]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [43]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [44]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [45]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [46]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [47]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [48]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [49]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [50]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [51]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [52]: ../extract/trait.FromRef.html#tymethod.from_ref
   [53]: super::Span::current()
   [54]: crate::Span
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [56]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [59]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [60]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [61]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [62]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [63]: https://docs.rs/http/latest/http/struct.Extensions.html
   [64]: crate::follow_redirect::policy::Standard
   [65]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [66]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [67]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [68]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [69]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [70]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [71]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [72]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [73]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [74]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [75]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [76]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [77]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [80]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [83]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [90]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [91]: super::Subscriber
   [92]: dispatcher#setting-the-default-subscriber

