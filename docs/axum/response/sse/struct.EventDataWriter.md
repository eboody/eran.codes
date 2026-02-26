<!-- Generated from rustdoc HTML: response/sse/struct.EventDataWriter.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## EventDataWriter

## [axum][1]0.8.8

## EventDataWriter

### Sections

  * Panics



### Methods

  * into_event



### Trait Implementations

  * Debug
  * Write



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
  * From<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::response::sse][2]

[axum][3]::[response][4]::[sse][2]

# Struct EventDataWriter Copy item path

[Source][5]
``` 
pub struct EventDataWriter { /* private fields */ }
```

Expand description

Expose [`Event`][6] as a [`std::fmt::Write`][7] such that any form of data can be written as data safely.

This also ensures that newline characters `\r` and `\n` correctly trigger a split with a new `data: ` prefix.

## §Panics

Panics if any `data` has already been written prior to the first write of this [`EventDataWriter`][8] instance.

## Implementations§

[Source][9]§

### impl [EventDataWriter][8]

[Source][10]

#### pub fn into_event(self) -> [Event][6]

Consume the [`EventDataWriter`][8] and return the [`Event`][6] once again.

In case any data was written by this instance it will also write the trailing `\n` character.

## Trait Implementations§

[Source][11]§

### impl [Debug][12] for [EventDataWriter][8]

[Source][11]§

#### fn [fmt][13](&self, f: &mut [Formatter][14]<'_>) -> [Result][15]

Formats the value using the given formatter. [Read more][13]

[Source][16]§

### impl [Write][7] for [EventDataWriter][8]

[Source][17]§

#### fn [write_str][18](&mut self, s: &[str][19]) -> [Result][15]

Writes a string slice into this writer, returning whether the write succeeded. [Read more][18]

1.1.0 · [Source][20]§

#### fn [write_char][21](&mut self, c: [char][22]) -> [Result][23]<[()][24], [Error][25]>

Writes a [`char`][26] into this writer, returning whether the write succeeded. [Read more][21]

1.0.0 · [Source][27]§

#### fn [write_fmt][28](&mut self, args: [Arguments][29]<'_>) -> [Result][23]<[()][24], [Error][25]>

Glue for usage of the [`write!`][30] macro with implementors of this trait. [Read more][28]

## Auto Trait Implementations§

§

### impl ![Freeze][31] for [EventDataWriter][8]

§

### impl [RefUnwindSafe][32] for [EventDataWriter][8]

§

### impl [Send][33] for [EventDataWriter][8]

§

### impl [Sync][34] for [EventDataWriter][8]

§

### impl [Unpin][35] for [EventDataWriter][8]

§

### impl [UnwindSafe][36] for [EventDataWriter][8]

## Blanket Implementations§

[Source][37]§

### impl<T> [Any][38] for T

where T: 'static + ?[Sized][39],

[Source][40]§

#### fn [type_id][41](&self) -> [TypeId][42]

Gets the `TypeId` of `self`. [Read more][41]

[Source][43]§

### impl<T> [Borrow][44]<T> for T

where T: ?[Sized][39],

[Source][45]§

#### fn [borrow][46](&self) -> [&T][47]

Immutably borrows from an owned value. [Read more][46]

[Source][48]§

### impl<T> [BorrowMut][49]<T> for T

where T: ?[Sized][39],

[Source][50]§

#### fn [borrow_mut][51](&mut self) -> [&mut T][47]

Mutably borrows from an owned value. [Read more][51]

[Source][52]§

### impl<T> [From][53]<T> for T

[Source][54]§

#### fn [from][55](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][56] [`Span`][57], returning an `Instrumented` wrapper. Read more

[Source][58]§

### impl<T, U> [Into][59]<U> for T

where U: [From][53]<T>,

[Source][60]§

#### fn [into][61](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][53]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][39],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][62]§

### impl<T> [Same][63] for T

[Source][64]§

#### type [Output][65] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][39],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][39],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][66]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][39],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][39],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][39],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][39],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][39],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][39],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][39],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][67] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][39],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][39],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][39],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][39],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][39],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][39],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][39],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][39],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][39],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][39], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][39], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][39],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][39],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][39],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][70]) -> RequestBodyLimit<Self>

where Self: [Sized][39],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][39],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][39],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][71]§

### impl<T, U> [TryFrom][72]<U> for T

where U: [Into][59]<T>,

[Source][73]§

#### type [Error][74] = [Infallible][75]

The type returned in the event of a conversion error.

[Source][76]§

#### fn [try_from][77](value: U) -> [Result][23]<T, <T as [TryFrom][72]<U>>::[Error][78]>

Performs the conversion.

[Source][79]§

### impl<T, U> [TryInto][80]<U> for T

where U: [TryFrom][72]<T>,

[Source][81]§

#### type [Error][82] = <U as [TryFrom][72]<T>>::[Error][78]

The type returned in the event of a conversion error.

[Source][83]§

#### fn [try_into][84](self) -> [Result][23]<U, <U as [TryFrom][72]<T>>::[Error][78]>

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

where S: [Into][59]<Dispatch>,

Attaches the provided [`Subscriber`][85] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][86] [`Subscriber`][85] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/response/sse.rs.html#191-198
   [6]: struct.Event.html (struct axum::response::sse::Event)
   [7]: https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html (trait core::fmt::Write)
   [8]: struct.EventDataWriter.html (struct axum::response::sse::EventDataWriter)
   [9]: ../../../src/axum/response/sse.rs.html#425-437
   [10]: ../../../src/axum/response/sse.rs.html#430-436
   [11]: ../../../src/axum/response/sse.rs.html#189
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [16]: ../../../src/axum/response/sse.rs.html#472-477
   [17]: ../../../src/axum/response/sse.rs.html#473-476
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#tymethod.write_str
   [19]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [20]: https://doc.rust-lang.org/nightly/src/core/fmt/mod.rs.html#183
   [21]: https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_char
   [22]: https://doc.rust-lang.org/nightly/std/primitive.char.html
   [23]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [24]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [25]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [26]: https://doc.rust-lang.org/nightly/std/primitive.char.html (primitive char)
   [27]: https://doc.rust-lang.org/nightly/src/core/fmt/mod.rs.html#212
   [28]: https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_fmt
   [29]: https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html (struct core::fmt::Arguments)
   [30]: https://doc.rust-lang.org/nightly/core/macro.write.html (macro core::write)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [32]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [34]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [36]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [37]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [38]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [40]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [41]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [42]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [43]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [44]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [45]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [46]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [47]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [55]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [56]: super::Span::current()
   [57]: crate::Span
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [62]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [63]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [64]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [65]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [66]: https://docs.rs/http/latest/http/struct.Extensions.html
   [67]: crate::follow_redirect::policy::Standard
   [68]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [69]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [70]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
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

