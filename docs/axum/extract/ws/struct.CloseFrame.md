<!-- Generated from rustdoc HTML: extract/ws/struct.CloseFrame.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## CloseFrame

## [axum][1]0.8.8

## CloseFrame

### Fields

  * code
  * reason



### Trait Implementations

  * Clone
  * Debug
  * Eq
  * PartialEq
  * StructuralPartialEq



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
  * Equivalent<K>
  * Equivalent<K>
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



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Struct CloseFrame Copy item path

[Source][5]
``` 
pub struct CloseFrame {
    pub code: [CloseCode][6],
    pub reason: [Utf8Bytes][7],
}
```

Available on **crate feature`ws`** only.

Expand description

A struct representing the close command.

## Fields§

§`code: [CloseCode][6]`

The reason as a code.

§`reason: [Utf8Bytes][7]`

The reason as text string.

## Trait Implementations§

[Source][8]§

### impl [Clone][9] for [CloseFrame][10]

[Source][8]§

#### fn [clone][11](&self) -> [CloseFrame][10]

Returns a duplicate of the value. [Read more][11]

1.0.0 · [Source][12]§

#### fn [clone_from][13](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][13]

[Source][8]§

### impl [Debug][14] for [CloseFrame][10]

[Source][8]§

#### fn [fmt][15](&self, f: &mut [Formatter][16]<'_>) -> [Result][17]

Formats the value using the given formatter. [Read more][15]

[Source][8]§

### impl [PartialEq][18] for [CloseFrame][10]

[Source][8]§

#### fn [eq][19](&self, other: &[CloseFrame][10]) -> [bool][20]

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source][21]§

#### fn [ne][22](&self, other: [&Rhs][23]) -> [bool][20]

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source][8]§

### impl [Eq][24] for [CloseFrame][10]

[Source][8]§

### impl [StructuralPartialEq][25] for [CloseFrame][10]

## Auto Trait Implementations§

§

### impl ![Freeze][26] for [CloseFrame][10]

§

### impl [RefUnwindSafe][27] for [CloseFrame][10]

§

### impl [Send][28] for [CloseFrame][10]

§

### impl [Sync][29] for [CloseFrame][10]

§

### impl [Unpin][30] for [CloseFrame][10]

§

### impl [UnwindSafe][31] for [CloseFrame][10]

## Blanket Implementations§

[Source][32]§

### impl<T> [Any][33] for T

where T: 'static + ?[Sized][34],

[Source][35]§

#### fn [type_id][36](&self) -> [TypeId][37]

Gets the `TypeId` of `self`. [Read more][36]

[Source][38]§

### impl<T> [Borrow][39]<T> for T

where T: ?[Sized][34],

[Source][40]§

#### fn [borrow][41](&self) -> [&T][23]

Immutably borrows from an owned value. [Read more][41]

[Source][42]§

### impl<T> [BorrowMut][43]<T> for T

where T: ?[Sized][34],

[Source][44]§

#### fn [borrow_mut][45](&mut self) -> [&mut T][23]

Mutably borrows from an owned value. [Read more][45]

[Source][46]§

### impl<T> [CloneToUninit][47] for T

where T: [Clone][9],

[Source][48]§

#### unsafe fn [clone_to_uninit][49](&self, dest: [*mut ][50][u8][51])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][49]

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][24] \+ ?[Sized][34], K: [Borrow][39]<Q> \+ ?[Sized][34],

§

#### fn equivalent(&self, key: [&K][23]) -> [bool][20]

Checks if this value is equivalent to the given key. Read more

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][24] \+ ?[Sized][34], K: [Borrow][39]<Q> \+ ?[Sized][34],

§

#### fn equivalent(&self, key: [&K][23]) -> [bool][20]

Compare self to `key` and return `true` if they are equal.

[Source][52]§

### impl<T> [From][53]<T> for T

[Source][54]§

#### fn [from][55](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][56]<T> for T

where T: [Clone][9],

§

#### fn [from_ref][57](input: [&T][23]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][58] [`Span`][59], returning an `Instrumented` wrapper. Read more

[Source][60]§

### impl<T, U> [Into][61]<U> for T

where U: [From][53]<T>,

[Source][62]§

#### fn [into][63](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][53]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][34],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][64]§

### impl<T> [Same][65] for T

[Source][66]§

#### type [Output][67] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][34],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][34],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][68]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][34],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][34],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][34],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][34],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][34],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][34],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][34],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][69] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][34],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][34],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][70]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][34],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][71] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][34],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][34],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][34],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][34],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][34],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][34],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][34], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][34], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][34],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][34],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][34],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][72]) -> RequestBodyLimit<Self>

where Self: [Sized][34],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][34],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][34],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][73]§

### impl<T> [ToOwned][74] for T

where T: [Clone][9],

[Source][75]§

#### type [Owned][76] = T

The resulting type after obtaining ownership.

[Source][77]§

#### fn [to_owned][78](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][78]

[Source][79]§

#### fn [clone_into][80](&self, target: [&mut T][23])

Uses borrowed data to replace owned data, usually by cloning. [Read more][80]

[Source][81]§

### impl<T, U> [TryFrom][82]<U> for T

where U: [Into][61]<T>,

[Source][83]§

#### type [Error][84] = [Infallible][85]

The type returned in the event of a conversion error.

[Source][86]§

#### fn [try_from][87](value: U) -> [Result][88]<T, <T as [TryFrom][82]<U>>::[Error][89]>

Performs the conversion.

[Source][90]§

### impl<T, U> [TryInto][91]<U> for T

where U: [TryFrom][82]<T>,

[Source][92]§

#### type [Error][93] = <U as [TryFrom][82]<T>>::[Error][89]

The type returned in the event of a conversion error.

[Source][94]§

#### fn [try_into][95](self) -> [Result][88]<U, <U as [TryFrom][82]<T>>::[Error][89]>

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

where S: [Into][61]<Dispatch>,

Attaches the provided [`Subscriber`][96] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][97] [`Subscriber`][96] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/ws.rs.html#738-743
   [6]: type.CloseCode.html (type axum::extract::ws::CloseCode)
   [7]: struct.Utf8Bytes.html (struct axum::extract::ws::Utf8Bytes)
   [8]: ../../../src/axum/extract/ws.rs.html#737
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [10]: struct.CloseFrame.html (struct axum::extract::ws::CloseFrame)
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [12]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [13]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [16]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [18]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html (trait core::cmp::PartialEq)
   [19]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq
   [20]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [21]: https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264
   [22]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne
   [23]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [24]: https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html (trait core::cmp::Eq)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html (trait core::marker::StructuralPartialEq)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [27]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [31]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [32]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [33]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [34]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [35]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [36]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [37]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [38]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [39]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
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
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [55]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [56]: ../trait.FromRef.html (trait axum::extract::FromRef)
   [57]: ../trait.FromRef.html#tymethod.from_ref
   [58]: super::Span::current()
   [59]: crate::Span
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [64]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [65]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [68]: https://docs.rs/http/latest/http/struct.Extensions.html
   [69]: crate::follow_redirect::policy::Standard
   [70]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [71]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [72]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [73]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [74]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [75]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [76]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [77]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [78]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [79]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [80]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [85]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [88]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [96]: super::Subscriber
   [97]: dispatcher#setting-the-default-subscriber

