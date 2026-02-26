<!-- Generated from rustdoc HTML: response/sse/struct.KeepAlive.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## KeepAlive

## [axum][1]0.8.8

## KeepAlive

### Methods

  * event
  * interval
  * new
  * text



### Trait Implementations

  * Clone
  * Debug
  * Default



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



## [In axum::response::sse][2]

[axum][3]::[response][4]::[sse][2]

# Struct KeepAlive Copy item path

[Source][5]
``` 
pub struct KeepAlive { /* private fields */ }
```

Expand description

Configure the interval between keep-alive messages, the content of each message, and the associated stream.

## Implementations§

[Source][6]§

### impl [KeepAlive][7]

[Source][8]

#### pub fn new() -> Self

Create a new `KeepAlive`.

[Source][9]

#### pub fn interval(self, time: [Duration][10]) -> Self

Customize the interval between keep-alive messages.

Default is 15 seconds.

[Source][11]

#### pub fn text<I>(self, text: I) -> Self

where I: [AsRef][12]<[str][13]>,

Customize the text of the keep-alive message.

Default is an empty comment.

##### §Panics

Panics if `text` contains any newline or carriage returns, as they are not allowed in SSE comments.

[Source][14]

#### pub fn event(self, event: [Event][15]) -> Self

Customize the event of the keep-alive message.

Default is an empty comment.

##### §Panics

Panics if `event` contains any newline or carriage returns, as they are not allowed in SSE comments.

## Trait Implementations§

[Source][16]§

### impl [Clone][17] for [KeepAlive][7]

[Source][16]§

#### fn [clone][18](&self) -> [KeepAlive][7]

Returns a duplicate of the value. [Read more][18]

1.0.0 · [Source][19]§

#### fn [clone_from][20](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][20]

[Source][16]§

### impl [Debug][21] for [KeepAlive][7]

[Source][16]§

#### fn [fmt][22](&self, f: &mut [Formatter][23]<'_>) -> [Result][24]

Formats the value using the given formatter. [Read more][22]

[Source][25]§

### impl [Default][26] for [KeepAlive][7]

[Source][27]§

#### fn [default][28]() -> Self

Returns the “default value” for a type. [Read more][28]

## Auto Trait Implementations§

§

### impl ![Freeze][29] for [KeepAlive][7]

§

### impl [RefUnwindSafe][30] for [KeepAlive][7]

§

### impl [Send][31] for [KeepAlive][7]

§

### impl [Sync][32] for [KeepAlive][7]

§

### impl [Unpin][33] for [KeepAlive][7]

§

### impl [UnwindSafe][34] for [KeepAlive][7]

## Blanket Implementations§

[Source][35]§

### impl<T> [Any][36] for T

where T: 'static + ?[Sized][37],

[Source][38]§

#### fn [type_id][39](&self) -> [TypeId][40]

Gets the `TypeId` of `self`. [Read more][39]

[Source][41]§

### impl<T> [Borrow][42]<T> for T

where T: ?[Sized][37],

[Source][43]§

#### fn [borrow][44](&self) -> [&T][45]

Immutably borrows from an owned value. [Read more][44]

[Source][46]§

### impl<T> [BorrowMut][47]<T> for T

where T: ?[Sized][37],

[Source][48]§

#### fn [borrow_mut][49](&mut self) -> [&mut T][45]

Mutably borrows from an owned value. [Read more][49]

[Source][50]§

### impl<T> [CloneToUninit][51] for T

where T: [Clone][17],

[Source][52]§

#### unsafe fn [clone_to_uninit][53](&self, dest: [*mut ][54][u8][55])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][53]

[Source][56]§

### impl<T> [From][57]<T> for T

[Source][58]§

#### fn [from][59](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][60]<T> for T

where T: [Clone][17],

§

#### fn [from_ref][61](input: [&T][45]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][62] [`Span`][63], returning an `Instrumented` wrapper. Read more

[Source][64]§

### impl<T, U> [Into][65]<U> for T

where U: [From][57]<T>,

[Source][66]§

#### fn [into][67](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][57]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][37],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][68]§

### impl<T> [Same][69] for T

[Source][70]§

#### type [Output][71] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][37],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][37],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][72]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][37],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][37],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][37],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][37],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][37],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][37],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][37],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][73] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][74]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][37],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][75] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][74]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][37],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][75] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][74]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][37],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][75] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][37], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][37], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][37],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][37],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][37],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][76]) -> RequestBodyLimit<Self>

where Self: [Sized][37],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][37],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][37],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][77]§

### impl<T> [ToOwned][78] for T

where T: [Clone][17],

[Source][79]§

#### type [Owned][80] = T

The resulting type after obtaining ownership.

[Source][81]§

#### fn [to_owned][82](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][82]

[Source][83]§

#### fn [clone_into][84](&self, target: [&mut T][45])

Uses borrowed data to replace owned data, usually by cloning. [Read more][84]

[Source][85]§

### impl<T, U> [TryFrom][86]<U> for T

where U: [Into][65]<T>,

[Source][87]§

#### type [Error][88] = [Infallible][89]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_from][91](value: U) -> [Result][92]<T, <T as [TryFrom][86]<U>>::[Error][93]>

Performs the conversion.

[Source][94]§

### impl<T, U> [TryInto][95]<U> for T

where U: [TryFrom][86]<T>,

[Source][96]§

#### type [Error][97] = <U as [TryFrom][86]<T>>::[Error][93]

The type returned in the event of a conversion error.

[Source][98]§

#### fn [try_into][99](self) -> [Result][92]<U, <U as [TryFrom][86]<T>>::[Error][93]>

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

where S: [Into][65]<Dispatch>,

Attaches the provided [`Subscriber`][100] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][101] [`Subscriber`][100] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/response/sse.rs.html#518-521
   [6]: ../../../src/axum/response/sse.rs.html#523-567
   [7]: struct.KeepAlive.html (struct axum::response::sse::KeepAlive)
   [8]: ../../../src/axum/response/sse.rs.html#525-530
   [9]: ../../../src/axum/response/sse.rs.html#535-538
   [10]: https://doc.rust-lang.org/nightly/core/time/struct.Duration.html (struct core::time::Duration)
   [11]: ../../../src/axum/response/sse.rs.html#548-553
   [12]: https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html (trait core::convert::AsRef)
   [13]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [14]: ../../../src/axum/response/sse.rs.html#563-566
   [15]: struct.Event.html (struct axum::response::sse::Event)
   [16]: ../../../src/axum/response/sse.rs.html#516
   [17]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [18]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [19]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [20]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [21]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [22]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [23]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [24]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [25]: ../../../src/axum/response/sse.rs.html#569-573
   [26]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [27]: ../../../src/axum/response/sse.rs.html#570-572
   [28]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [30]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [34]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [35]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [36]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [38]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [39]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [40]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [41]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [42]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [43]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [44]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [45]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [50]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [51]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [52]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [53]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [54]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [55]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [57]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [60]: ../../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [61]: ../../extract/trait.FromRef.html#tymethod.from_ref
   [62]: super::Span::current()
   [63]: crate::Span
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [66]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [67]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [70]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [71]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [72]: https://docs.rs/http/latest/http/struct.Extensions.html
   [73]: crate::follow_redirect::policy::Standard
   [74]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [75]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [76]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [77]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [78]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [79]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [80]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [81]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [82]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [83]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [84]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [89]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [92]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [98]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [100]: super::Subscriber
   [101]: dispatcher#setting-the-default-subscriber

