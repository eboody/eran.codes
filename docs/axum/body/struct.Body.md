<!-- Generated from rustdoc HTML: body/struct.Body.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Body

## [axum][1]0.8.8

## Body

### Methods

  * empty
  * from_stream
  * into_data_stream
  * new



### Trait Implementations

  * Body
  * Debug
  * Default
  * From<&'static [u8]>
  * From<&'static str>
  * From<()>
  * From<Bytes>
  * From<Cow<'static, [u8]>>
  * From<Cow<'static, str>>
  * From<String>
  * From<Vec<u8>>
  * FromRequest<S>
  * IntoResponse



### Auto Trait Implementations

  * !RefUnwindSafe
  * !Sync
  * !UnwindSafe
  * Freeze
  * Send
  * Unpin



### Blanket Implementations

  * Any
  * BodyExt
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



## [In axum::body][2]

[axum][3]::[body][2]

# Struct Body Copy item path
```
pub struct Body(/* private fields */);
```

Expand description

The body type used in axum requests and responses.

## Implementations§

§

### impl [Body][4]

#### pub fn new<B>(body: B) -> [Body][4]

where B: Body<Data = Bytes> \+ [Send][5] \+ 'static, <B as Body>::Error: [Into][6]<[Box][7]<dyn [Error][8] \+ [Send][5] \+ [Sync][9]>>,

Create a new `Body` that wraps another [`http_body::Body`].

#### pub fn empty() -> [Body][4]

Create an empty body.

#### pub fn from_stream<S>(stream: S) -> [Body][4]

where S: TryStream + [Send][5] \+ 'static, <S as TryStream>::Ok: [Into][6]<Bytes>, <S as TryStream>::Error: [Into][6]<[Box][7]<dyn [Error][8] \+ [Send][5] \+ [Sync][9]>>,

Create a new `Body` from a [`Stream`][10].

#### pub fn into_data_stream(self) -> [BodyDataStream][11]

Convert the body into a [`Stream`] of data frames.

Non-data frames (such as trailers) will be discarded. Use [`http_body_util::BodyStream`][12] if you need a [`Stream`] of all frame types.

## Trait Implementations§

§

### impl Body for [Body][4]

§

#### type Data = Bytes

Values yielded by the `Body`.

§

#### type Error = [Error][13]

The error type this `Body` might generate.

§

#### fn poll_frame( self: [Pin][14]<&mut [Body][4]>, cx: &mut [Context][15]<'_>, ) -> [Poll][16]<[Option][17]<[Result][18]<Frame<<[Body][4] as Body>::Data>, <[Body][4] as Body>::Error>>>

Attempt to pull out the next data buffer of this stream.

§

#### fn size_hint(&self) -> SizeHint

Returns the bounds on the remaining length of the stream. Read more

§

#### fn is_end_stream(&self) -> [bool][19]

Returns `true` when the end of stream has been reached. Read more

§

### impl [Debug][20] for [Body][4]

§

#### fn [fmt][21](&self, f: &mut [Formatter][22]<'_>) -> [Result][18]<[()][23], [Error][24]>

Formats the value using the given formatter. [Read more][21]

§

### impl [Default][25] for [Body][4]

§

#### fn [default][26]() -> [Body][4]

Returns the “default value” for a type. [Read more][26]

§

### impl [From][27]<&'static [[u8][28]]> for [Body][4]

§

#### fn [from][29](buf: &'static [[u8][28]]) -> [Body][4]

Converts to this type from the input type.

§

### impl [From][27]<&'static [str][30]> for [Body][4]

§

#### fn [from][29](buf: &'static [str][30]) -> [Body][4]

Converts to this type from the input type.

§

### impl [From][27]<[()][23]> for [Body][4]

§

#### fn [from][29](_: [()][23]) -> [Body][4]

Converts to this type from the input type.

§

### impl [From][27]<Bytes> for [Body][4]

§

#### fn [from][29](buf: Bytes) -> [Body][4]

Converts to this type from the input type.

§

### impl [From][27]<[Cow][31]<'static, [[u8][28]]>> for [Body][4]

§

#### fn [from][29](buf: [Cow][31]<'static, [[u8][28]]>) -> [Body][4]

Converts to this type from the input type.

§

### impl [From][27]<[Cow][31]<'static, [str][30]>> for [Body][4]

§

#### fn [from][29](buf: [Cow][31]<'static, [str][30]>) -> [Body][4]

Converts to this type from the input type.

§

### impl [From][27]<[String][32]> for [Body][4]

§

#### fn [from][29](buf: [String][32]) -> [Body][4]

Converts to this type from the input type.

§

### impl [From][27]<[Vec][33]<[u8][28]>> for [Body][4]

§

#### fn [from][29](buf: [Vec][33]<[u8][28]>) -> [Body][4]

Converts to this type from the input type.

§

### impl<S> [FromRequest][34]<S> for [Body][4]

where S: [Send][5] \+ [Sync][9],

§

#### type [Rejection][35] = [Infallible][36]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### async fn [from_request][37]( req: Request<[Body][4]>, _: [&S][38], ) -> [Result][18]<[Body][4], <[Body][4] as [FromRequest][34]<S>>::[Rejection][39]>

Perform the extraction.

§

### impl [IntoResponse][40] for [Body][4]

§

#### fn [into_response][41](self) -> Response<[Body][4]>

Create a response.

## Auto Trait Implementations§

§

### impl [Freeze][42] for [Body][4]

§

### impl ![RefUnwindSafe][43] for [Body][4]

§

### impl [Send][5] for [Body][4]

§

### impl ![Sync][9] for [Body][4]

§

### impl [Unpin][44] for [Body][4]

§

### impl ![UnwindSafe][45] for [Body][4]

## Blanket Implementations§

[Source][46]§

### impl<T> [Any][47] for T

where T: 'static + ?[Sized][48],

[Source][49]§

#### fn [type_id][50](&self) -> [TypeId][51]

Gets the `TypeId` of `self`. [Read more][50]

§

### impl<T> BodyExt for T

where T: Body + ?[Sized][48],

§

#### fn frame(&mut self) -> Frame<'_, Self>

where Self: [Unpin][44],

Returns a future that resolves to the next [`Frame`][52], if any.

§

#### fn map_frame<F, B>(self, f: F) -> MapFrame<Self, F>

where Self: [Sized][48], F: [FnMut][53](Frame<Self::Data>) -> Frame<B>, B: Buf,

Maps this body’s frame to a different kind.

§

#### fn map_err<F, E>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][48], F: [FnMut][53](Self::Error) -> E,

Maps this body’s error value to a different value.

§

#### fn boxed(self) -> BoxBody<Self::Data, Self::Error>

where Self: [Sized][48] \+ [Send][5] \+ [Sync][9] \+ 'static,

Turn this body into a boxed trait object.

§

#### fn boxed_unsync(self) -> UnsyncBoxBody<Self::Data, Self::Error>

where Self: [Sized][48] \+ [Send][5] \+ 'static,

Turn this body into a boxed trait object that is !Sync.

§

#### fn collect(self) -> Collect<Self>

where Self: [Sized][48],

Turn this body into [`Collected`] body which will collect all the DATA frames and trailers.

§

#### fn with_trailers<F>(self, trailers: F) -> WithTrailers<Self, F>

where Self: [Sized][48], F: [Future][54]<Output = [Option][17]<[Result][18]<HeaderMap, Self::Error>>>,

Add trailers to the body. Read more

§

#### fn into_data_stream(self) -> BodyDataStream<Self>

where Self: [Sized][48],

Turn this body into [`BodyDataStream`].

[Source][55]§

### impl<T> [Borrow][56]<T> for T

where T: ?[Sized][48],

[Source][57]§

#### fn [borrow][58](&self) -> [&T][38]

Immutably borrows from an owned value. [Read more][58]

[Source][59]§

### impl<T> [BorrowMut][60]<T> for T

where T: ?[Sized][48],

[Source][61]§

#### fn [borrow_mut][62](&mut self) -> [&mut T][38]

Mutably borrows from an owned value. [Read more][62]

[Source][63]§

### impl<T> [From][27]<T> for T

[Source][64]§

#### fn [from][29](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][65] [`Span`][66], returning an `Instrumented` wrapper. Read more

[Source][67]§

### impl<T, U> [Into][6]<U> for T

where U: [From][27]<T>,

[Source][68]§

#### fn [into][69](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][27]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][48],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][70]§

### impl<T> [Same][71] for T

[Source][72]§

#### type [Output][73] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][48],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][48],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][74]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][48],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][48],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][48],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][48],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][48],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][48],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][48],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][75] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][76]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][48],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][77] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][76]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][48],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][77] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][76]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][48],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][77] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][48],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][48], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][48], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][48],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][48],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][48],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][78]) -> RequestBodyLimit<Self>

where Self: [Sized][48],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][48],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][48],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][79]§

### impl<T, U> [TryFrom][80]<U> for T

where U: [Into][6]<T>,

[Source][81]§

#### type [Error][82] = [Infallible][36]

The type returned in the event of a conversion error.

[Source][83]§

#### fn [try_from][84](value: U) -> [Result][18]<T, <T as [TryFrom][80]<U>>::[Error][85]>

Performs the conversion.

[Source][86]§

### impl<T, U> [TryInto][87]<U> for T

where U: [TryFrom][80]<T>,

[Source][88]§

#### type [Error][89] = <U as [TryFrom][80]<T>>::[Error][85]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_into][91](self) -> [Result][18]<U, <U as [TryFrom][80]<T>>::[Error][85]>

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

where S: [Into][6]<Dispatch>,

Attaches the provided [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][93] [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: struct.Body.html (struct axum::body::Body)
   [5]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [6]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [7]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [8]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [9]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [10]: https://docs.rs/futures-core/latest/futures_core/stream/trait.Stream.html
   [11]: struct.BodyDataStream.html (struct axum::body::BodyDataStream)
   [12]: https://docs.rs/http-body-util/latest/http_body_util/struct.BodyStream.html
   [13]: ../struct.Error.html (struct axum::Error)
   [14]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [15]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [16]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [17]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [18]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [19]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [20]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [21]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [22]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [23]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [24]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [25]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [26]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [27]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [28]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [29]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [30]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [31]: https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html (enum alloc::borrow::Cow)
   [32]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [33]: https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html (struct alloc::vec::Vec)
   [34]: ../extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [35]: ../extract/trait.FromRequest.html#associatedtype.Rejection
   [36]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [37]: ../extract/trait.FromRequest.html#tymethod.from_request
   [38]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [39]: ../extract/trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [40]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [41]: ../response/trait.IntoResponse.html#tymethod.into_response
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [43]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [45]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [46]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [47]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [48]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [49]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [50]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [51]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [52]: combinators::Frame
   [53]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [54]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [57]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [58]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [59]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [60]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [61]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [62]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [65]: super::Span::current()
   [66]: crate::Span
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [68]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [69]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [70]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [71]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [72]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [73]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [74]: https://docs.rs/http/latest/http/struct.Extensions.html
   [75]: crate::follow_redirect::policy::Standard
   [76]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [77]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [78]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [92]: super::Subscriber
   [93]: dispatcher#setting-the-default-subscriber

