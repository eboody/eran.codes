<!-- Generated from rustdoc HTML: middleware/struct.FromExtractor.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromExtractor

## [axum][1]0.8.8

## FromExtractor

### Trait Implementations

  * Clone
  * Debug
  * Service<Request<B>>



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
  * ServiceExt<R>
  * ServiceExt<Request>
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Struct FromExtractor Copy item path

[Source][4]
``` 
pub struct FromExtractor<T, E, S> { /* private fields */ }
```

Expand description

Middleware that runs an extractor and discards the value.

See [`from_extractor`][5] for more details.

## Trait Implementations§

[Source][6]§

### impl<T, E, S> [Clone][7] for [FromExtractor][8]<T, E, S>

where T: [Clone][7], S: [Clone][7],

[Source][9]§

#### fn [clone][10](&self) -> Self

Returns a duplicate of the value. [Read more][10]

1.0.0 · [Source][11]§

#### fn [clone_from][12](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][12]

[Source][13]§

### impl<T, E, S> [Debug][14] for [FromExtractor][8]<T, E, S>

where T: [Debug][14], S: [Debug][14],

[Source][15]§

#### fn [fmt][16](&self, f: &mut [Formatter][17]<'_>) -> [Result][18]

Formats the value using the given formatter. [Read more][16]

[Source][19]§

### impl<T, E, B, S> Service<Request<B>> for [FromExtractor][8]<T, E, S>

where E: [FromRequestParts][20]<S> \+ 'static, B: [Send][21] \+ 'static, T: Service<Request<B>> \+ [Clone][7], T::Response: [IntoResponse][22], S: [Clone][7] \+ [Send][21] \+ [Sync][23] \+ 'static,

[Source][24]§

#### type Response = Response<[Body][25]>

Responses given by the service.

[Source][26]§

#### type Error = <T as Service<Request<B>>>::Error

Errors produced by the service.

[Source][27]§

#### type Future = [ResponseFuture][28]<B, T, E, S>

The future response value.

[Source][29]§

#### fn poll_ready(&mut self, cx: &mut [Context][30]<'_>) -> [Poll][31]<[Result][32]<[()][33], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][34]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<T, E, S> [Freeze][35] for [FromExtractor][8]<T, E, S>

where T: [Freeze][35], S: [Freeze][35],

§

### impl<T, E, S> [RefUnwindSafe][36] for [FromExtractor][8]<T, E, S>

where T: [RefUnwindSafe][36], S: [RefUnwindSafe][36],

§

### impl<T, E, S> [Send][21] for [FromExtractor][8]<T, E, S>

where T: [Send][21], S: [Send][21],

§

### impl<T, E, S> [Sync][23] for [FromExtractor][8]<T, E, S>

where T: [Sync][23], S: [Sync][23],

§

### impl<T, E, S> [Unpin][37] for [FromExtractor][8]<T, E, S>

where T: [Unpin][37], S: [Unpin][37],

§

### impl<T, E, S> [UnwindSafe][38] for [FromExtractor][8]<T, E, S>

where T: [UnwindSafe][38], S: [UnwindSafe][38],

## Blanket Implementations§

[Source][39]§

### impl<T> [Any][40] for T

where T: 'static + ?[Sized][41],

[Source][42]§

#### fn [type_id][43](&self) -> [TypeId][44]

Gets the `TypeId` of `self`. [Read more][43]

[Source][45]§

### impl<T> [Borrow][46]<T> for T

where T: ?[Sized][41],

[Source][47]§

#### fn [borrow][48](&self) -> [&T][49]

Immutably borrows from an owned value. [Read more][48]

[Source][50]§

### impl<T> [BorrowMut][51]<T> for T

where T: ?[Sized][41],

[Source][52]§

#### fn [borrow_mut][53](&mut self) -> [&mut T][49]

Mutably borrows from an owned value. [Read more][53]

[Source][54]§

### impl<T> [CloneToUninit][55] for T

where T: [Clone][7],

[Source][56]§

#### unsafe fn [clone_to_uninit][57](&self, dest: [*mut ][58][u8][59])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][57]

[Source][60]§

### impl<T> [From][61]<T> for T

[Source][62]§

#### fn [from][63](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][64]<T> for T

where T: [Clone][7],

§

#### fn [from_ref][65](input: [&T][49]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][66] [`Span`][67], returning an `Instrumented` wrapper. Read more

[Source][68]§

### impl<T, U> [Into][69]<U> for T

where U: [From][61]<T>,

[Source][70]§

#### fn [into][71](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][61]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][41],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][72]§

### impl<T> [Same][73] for T

[Source][74]§

#### type [Output][75] = T

Should always be `Self`

[Source][76]§

### impl<S, R> [ServiceExt][77]<R> for S

where S: Service<R>,

[Source][78]§

#### fn [into_make_service][79](self) -> [IntoMakeService][80]<S>

Convert this service into a [`MakeService`][81], that is a [`Service`] whose response is another service. [Read more][79]

[Source][82]§

#### fn [into_make_service_with_connect_info][83]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][84]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][81], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][85] can extract it. [Read more][83]

[Source][86]§

#### fn [handle_error][87]<F, T>(self, f: F) -> [HandleError][88]<Self, F, T>

Convert this service into a [`HandleError`][88], that will handle errors by converting them into responses. [Read more][87]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][41],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][41],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][41],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][41],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][41], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][89], and produce a [`Stream`][89] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][41], F: [Clone][7],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][90] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][41], F: [FnOnce][91](Self::Response) -> Response + [Clone][7],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][90] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][41], F: [FnOnce][91](Self::Error) -> Error + [Clone][7],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][90] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][41], Error: [From][61]<Self::Error>, F: [FnOnce][91]([Result][32]<Self::Response, Self::Error>) -> [Result][32]<Response, Error> \+ [Clone][7],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][41], F: [FnMut][92](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][41], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][93] that conditionally accepts or rejects requests based on a [predicate][94]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][41], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][95] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][41], Error: [From][61]<Self::Error>, F: [FnOnce][91]([Result][32]<Self::Response, Self::Error>) -> Fut + [Clone][7], Fut: [Future][96]<Output = [Result][32]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][41], F: [FnMut][92](Self::Future) -> Fut, Error: [From][61]<Self::Error>, Fut: [Future][96]<Output = [Result][32]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][41] \+ [Send][21] \+ 'static, Self::Future: [Send][21] \+ 'static,

Convert the service into a [`Service`][97] \+ [`Send`][21] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][41] \+ [Clone][7] \+ [Send][21] \+ 'static, Self::Future: [Send][21] \+ 'static,

Convert the service into a [`Service`][97] \+ [`Clone`][7] \+ [`Send`][21] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][41],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][41],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][98]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][41],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][41],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][41],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][41],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][41],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][41],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][41],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][99] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][100]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][101] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][100]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][101] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][100]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][41],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][101] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][41],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][41], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][41], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][41],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][41],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][41],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][102]) -> RequestBodyLimit<Self>

where Self: [Sized][41],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][41],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][41],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][103]§

### impl<T> [ToOwned][104] for T

where T: [Clone][7],

[Source][105]§

#### type [Owned][106] = T

The resulting type after obtaining ownership.

[Source][107]§

#### fn [to_owned][108](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][108]

[Source][109]§

#### fn [clone_into][110](&self, target: [&mut T][49])

Uses borrowed data to replace owned data, usually by cloning. [Read more][110]

[Source][111]§

### impl<T, U> [TryFrom][112]<U> for T

where U: [Into][69]<T>,

[Source][113]§

#### type [Error][114] = [Infallible][115]

The type returned in the event of a conversion error.

[Source][116]§

#### fn [try_from][117](value: U) -> [Result][32]<T, <T as [TryFrom][112]<U>>::[Error][118]>

Performs the conversion.

[Source][119]§

### impl<T, U> [TryInto][120]<U> for T

where U: [TryFrom][112]<T>,

[Source][121]§

#### type [Error][122] = <U as [TryFrom][112]<T>>::[Error][118]

The type returned in the event of a conversion error.

[Source][123]§

#### fn [try_into][124](self) -> [Result][32]<U, <U as [TryFrom][112]<T>>::[Error][118]>

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

where S: [Into][69]<Dispatch>,

Attaches the provided [`Subscriber`][125] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][126] [`Subscriber`][125] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/from_extractor.rs.html#157-161
   [5]: fn.from_extractor.html (fn axum::middleware::from_extractor)
   [6]: ../../src/axum/middleware/from_extractor.rs.html#170-182
   [7]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [8]: struct.FromExtractor.html (struct axum::middleware::FromExtractor)
   [9]: ../../src/axum/middleware/from_extractor.rs.html#175-181
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [11]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [13]: ../../src/axum/middleware/from_extractor.rs.html#184-196
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: ../../src/axum/middleware/from_extractor.rs.html#189-195
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [19]: ../../src/axum/middleware/from_extractor.rs.html#198-232
   [20]: ../extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [21]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [22]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [24]: ../../src/axum/middleware/from_extractor.rs.html#206
   [25]: ../body/struct.Body.html (struct axum::body::Body)
   [26]: ../../src/axum/middleware/from_extractor.rs.html#207
   [27]: ../../src/axum/middleware/from_extractor.rs.html#208
   [28]: future/struct.FromExtractorResponseFuture.html (struct axum::middleware::future::FromExtractorResponseFuture)
   [29]: ../../src/axum/middleware/from_extractor.rs.html#211-213
   [30]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [31]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [32]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [33]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [34]: ../../src/axum/middleware/from_extractor.rs.html#215-231
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [36]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [39]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [40]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [42]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [43]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [44]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [45]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [46]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [49]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [54]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [55]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [56]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [57]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [58]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [59]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [64]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [65]: ../extract/trait.FromRef.html#tymethod.from_ref
   [66]: super::Span::current()
   [67]: crate::Span
   [68]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [69]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [70]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [71]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [72]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [73]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [74]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [75]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [76]: ../../src/axum/service_ext.rs.html#47-59
   [77]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [78]: ../../src/axum/service_ext.rs.html#51-53
   [79]: ../trait.ServiceExt.html#tymethod.into_make_service
   [80]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [81]: tower::make::MakeService
   [82]: ../../src/axum/service_ext.rs.html#56-58
   [83]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [84]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [85]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [86]: ../../src/axum/service_ext.rs.html#42-44
   [87]: ../trait.ServiceExt.html#method.handle_error
   [88]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [89]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [90]: crate::Service::poll_ready
   [91]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [92]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [93]: crate::filter::Filter
   [94]: crate::filter::Predicate
   [95]: crate::filter::AsyncFilter
   [96]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [97]: crate::Service
   [98]: https://docs.rs/http/latest/http/struct.Extensions.html
   [99]: crate::follow_redirect::policy::Standard
   [100]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [101]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [102]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [103]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [104]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [105]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [106]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [107]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [108]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [109]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [110]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [111]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [112]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [113]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [114]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [115]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [116]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [117]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [118]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [119]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [120]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [121]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [122]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [123]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [124]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [125]: super::Subscriber
   [126]: dispatcher#setting-the-default-subscriber

