<!-- Generated from rustdoc HTML: struct.Error.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Error

## [axum][1]0.8.8

## Error

### Methods

  * into_inner
  * new



### Trait Implementations

  * Debug
  * Display
  * Error



### Auto Trait Implementations

  * !RefUnwindSafe
  * !UnwindSafe
  * Freeze
  * Send
  * Sync
  * Unpin



### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * From<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ToString
  * ToStringFallible
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In crate axum][2]

[axum][2]

# Struct Error Copy item path
```
pub struct Error { /* private fields */ }
```

Expand description

Errors that can happen when using axum.

## Implementations§

§

### impl [Error][3]

#### pub fn new(error: impl [Into][4]<[Box][5]<dyn [Error][6] \+ [Send][7] \+ [Sync][8]>>) -> [Error][3]

Create a new `Error` from a boxable error.

#### pub fn into_inner(self) -> [Box][5]<dyn [Error][6] \+ [Send][7] \+ [Sync][8]>

Convert an `Error` back into the underlying boxed trait object.

## Trait Implementations§

§

### impl [Debug][9] for [Error][3]

§

#### fn [fmt][10](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]<[()][13], [Error][14]>

Formats the value using the given formatter. [Read more][10]

§

### impl [Display][15] for [Error][3]

§

#### fn [fmt][16](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]<[()][13], [Error][14]>

Formats the value using the given formatter. [Read more][16]

§

### impl [Error][6] for [Error][3]

§

#### fn [source][17](&self) -> [Option][18]<&(dyn [Error][6] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][17]

1.0.0 · [Source][19]§

#### fn [description][20](&self) -> &[str][21]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][20]

1.0.0 · [Source][22]§

#### fn [cause][23](&self) -> [Option][18]<&dyn [Error][6]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][24]§

#### fn [provide][25]<'a>(&'a self, request: &mut [Request][26]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][25]

## Auto Trait Implementations§

§

### impl [Freeze][27] for [Error][3]

§

### impl ![RefUnwindSafe][28] for [Error][3]

§

### impl [Send][7] for [Error][3]

§

### impl [Sync][8] for [Error][3]

§

### impl [Unpin][29] for [Error][3]

§

### impl ![UnwindSafe][30] for [Error][3]

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

### impl<T> [From][47]<T> for T

[Source][48]§

#### fn [from][49](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][50] [`Span`][51], returning an `Instrumented` wrapper. Read more

[Source][52]§

### impl<T, U> [Into][4]<U> for T

where U: [From][47]<T>,

[Source][53]§

#### fn [into][54](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][47]<T> for U` chooses to do.

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

[Source][55]§

### impl<T> [Same][56] for T

[Source][57]§

#### type [Output][58] = T

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

Add some shareable value to [request extensions][59]. Read more

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

Follow redirect resposes using the [`Standard`][60] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][61]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][62] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][61]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][62] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][61]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][33],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][62] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][63]) -> RequestBodyLimit<Self>

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

[Source][64]§

### impl<T> [ToString][65] for T

where T: [Display][15] \+ ?[Sized][33],

[Source][66]§

#### fn [to_string][67](&self) -> [String][68]

Converts the given value to a `String`. [Read more][67]

§

### impl<T> ToStringFallible for T

where T: [Display][15],

§

#### fn try_to_string(&self) -> [Result][12]<[String][68], [TryReserveError][69]>

[`ToString::to_string`][70], but without panic on OOM.

[Source][71]§

### impl<T, U> [TryFrom][72]<U> for T

where U: [Into][4]<T>,

[Source][73]§

#### type [Error][74] = [Infallible][75]

The type returned in the event of a conversion error.

[Source][76]§

#### fn [try_from][77](value: U) -> [Result][12]<T, <T as [TryFrom][72]<U>>::[Error][78]>

Performs the conversion.

[Source][79]§

### impl<T, U> [TryInto][80]<U> for T

where U: [TryFrom][72]<T>,

[Source][81]§

#### type [Error][82] = <U as [TryFrom][72]<T>>::[Error][78]

The type returned in the event of a conversion error.

[Source][83]§

#### fn [try_into][84](self) -> [Result][12]<U, <U as [TryFrom][72]<T>>::[Error][78]>

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

where S: [Into][4]<Dispatch>,

Attaches the provided [`Subscriber`][85] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][86] [`Subscriber`][85] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../axum/index.html
   [2]: index.html
   [3]: struct.Error.html (struct axum::Error)
   [4]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [5]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [6]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [7]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [8]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [9]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [11]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [12]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [13]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [18]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [19]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#137
   [20]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [21]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [22]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [23]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [24]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [25]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [26]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [28]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
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
   [46]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [47]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [48]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [49]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [50]: super::Span::current()
   [51]: crate::Span
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [53]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [54]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [55]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [56]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [57]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [58]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [59]: https://docs.rs/http/latest/http/struct.Extensions.html
   [60]: crate::follow_redirect::policy::Standard
   [61]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [62]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [63]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [64]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [65]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [66]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [67]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [68]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [69]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [70]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [71]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [72]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [75]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [76]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [77]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [85]: super::Subscriber
   [86]: dispatcher#setting-the-default-subscriber

