<!-- Generated from rustdoc HTML: routing/struct.RouterIntoService.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RouterIntoService

## [axum][1]0.8.8

## RouterIntoService

### Trait Implementations

  * Clone
  * Debug
  * Service<Request<B>>



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



## [In axum::routing][2]

[axum][3]::[routing][2]

# Struct RouterIntoService Copy item path

[Source][4]
``` 
pub struct RouterIntoService<B, S = [()][5]> { /* private fields */ }
```

Expand description

A [`Router`][6] converted into an owned [`Service`] with a fixed body type.

See [`Router::into_service`][7] for more details.

## Trait Implementations§

[Source][8]§

### impl<B, S> [Clone][9] for [RouterIntoService][10]<B, S>

where [Router][6]<S>: [Clone][9],

[Source][11]§

#### fn [clone][12](&self) -> Self

Returns a duplicate of the value. [Read more][12]

1.0.0 · [Source][13]§

#### fn [clone_from][14](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][14]

[Source][15]§

### impl<B, S> [Debug][16] for [RouterIntoService][10]<B, S>

where S: [Debug][16],

[Source][17]§

#### fn [fmt][18](&self, f: &mut [Formatter][19]<'_>) -> [Result][20]

Formats the value using the given formatter. [Read more][18]

[Source][21]§

### impl<B> Service<Request<B>> for [RouterIntoService][10]<B, [()][5]>

where B: HttpBody<Data = Bytes> \+ [Send][22] \+ 'static, B::Error: [Into][23]<[BoxError][24]>,

[Source][25]§

#### type Response = Response<[Body][26]>

Responses given by the service.

[Source][27]§

#### type Error = [Infallible][28]

Errors produced by the service.

[Source][29]§

#### type Future = [RouteFuture][30]<[Infallible][28]>

The future response value.

[Source][31]§

#### fn poll_ready(&mut self, cx: &mut [Context][32]<'_>) -> [Poll][33]<[Result][34]<[()][5], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][35]§

#### fn call(&mut self, req: [Request][36]<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<B, S> [Freeze][37] for [RouterIntoService][10]<B, S>

§

### impl<B, S = [()][5]> ![RefUnwindSafe][38] for [RouterIntoService][10]<B, S>

§

### impl<B, S> [Send][22] for [RouterIntoService][10]<B, S>

§

### impl<B, S> [Sync][39] for [RouterIntoService][10]<B, S>

§

### impl<B, S> [Unpin][40] for [RouterIntoService][10]<B, S>

§

### impl<B, S = [()][5]> ![UnwindSafe][41] for [RouterIntoService][10]<B, S>

## Blanket Implementations§

[Source][42]§

### impl<T> [Any][43] for T

where T: 'static + ?[Sized][44],

[Source][45]§

#### fn [type_id][46](&self) -> [TypeId][47]

Gets the `TypeId` of `self`. [Read more][46]

[Source][48]§

### impl<T> [Borrow][49]<T> for T

where T: ?[Sized][44],

[Source][50]§

#### fn [borrow][51](&self) -> [&T][52]

Immutably borrows from an owned value. [Read more][51]

[Source][53]§

### impl<T> [BorrowMut][54]<T> for T

where T: ?[Sized][44],

[Source][55]§

#### fn [borrow_mut][56](&mut self) -> [&mut T][52]

Mutably borrows from an owned value. [Read more][56]

[Source][57]§

### impl<T> [CloneToUninit][58] for T

where T: [Clone][9],

[Source][59]§

#### unsafe fn [clone_to_uninit][60](&self, dest: [*mut ][61][u8][62])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][60]

[Source][63]§

### impl<T> [From][64]<T> for T

[Source][65]§

#### fn [from][66](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][67]<T> for T

where T: [Clone][9],

§

#### fn [from_ref][68](input: [&T][52]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][69] [`Span`][70], returning an `Instrumented` wrapper. Read more

[Source][71]§

### impl<T, U> [Into][23]<U> for T

where U: [From][64]<T>,

[Source][72]§

#### fn [into][73](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][64]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][44],

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

[Source][78]§

### impl<S, R> [ServiceExt][79]<R> for S

where S: Service<R>,

[Source][80]§

#### fn [into_make_service][81](self) -> [IntoMakeService][82]<S>

Convert this service into a [`MakeService`][83], that is a [`Service`] whose response is another service. [Read more][81]

[Source][84]§

#### fn [into_make_service_with_connect_info][85]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][86]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][83], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][87] can extract it. [Read more][85]

[Source][88]§

#### fn [handle_error][89]<F, T>(self, f: F) -> [HandleError][90]<Self, F, T>

Convert this service into a [`HandleError`][90], that will handle errors by converting them into responses. [Read more][89]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][44],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][44],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][44],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][44],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][44], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][91], and produce a [`Stream`][91] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][44], F: [Clone][9],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][92] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][44], F: [FnOnce][93](Self::Response) -> Response + [Clone][9],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][92] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][44], F: [FnOnce][93](Self::Error) -> Error + [Clone][9],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][92] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][44], Error: [From][64]<Self::Error>, F: [FnOnce][93]([Result][34]<Self::Response, Self::Error>) -> [Result][34]<Response, Error> \+ [Clone][9],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][44], F: [FnMut][94](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][44], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][95] that conditionally accepts or rejects requests based on a [predicate][96]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][44], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][97] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][44], Error: [From][64]<Self::Error>, F: [FnOnce][93]([Result][34]<Self::Response, Self::Error>) -> Fut + [Clone][9], Fut: [Future][98]<Output = [Result][34]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][44], F: [FnMut][94](Self::Future) -> Fut, Error: [From][64]<Self::Error>, Fut: [Future][98]<Output = [Result][34]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][44] \+ [Send][22] \+ 'static, Self::Future: [Send][22] \+ 'static,

Convert the service into a [`Service`][99] \+ [`Send`][22] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][44] \+ [Clone][9] \+ [Send][22] \+ 'static, Self::Future: [Send][22] \+ 'static,

Convert the service into a [`Service`][99] \+ [`Clone`][9] \+ [`Send`][22] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][44],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][44],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][100]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][44],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][44],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][44],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][101] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][102]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][103] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][44],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][104]) -> RequestBodyLimit<Self>

where Self: [Sized][44],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][105]§

### impl<T> [ToOwned][106] for T

where T: [Clone][9],

[Source][107]§

#### type [Owned][108] = T

The resulting type after obtaining ownership.

[Source][109]§

#### fn [to_owned][110](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][110]

[Source][111]§

#### fn [clone_into][112](&self, target: [&mut T][52])

Uses borrowed data to replace owned data, usually by cloning. [Read more][112]

[Source][113]§

### impl<T, U> [TryFrom][114]<U> for T

where U: [Into][23]<T>,

[Source][115]§

#### type [Error][116] = [Infallible][28]

The type returned in the event of a conversion error.

[Source][117]§

#### fn [try_from][118](value: U) -> [Result][34]<T, <T as [TryFrom][114]<U>>::[Error][119]>

Performs the conversion.

[Source][120]§

### impl<T, U> [TryInto][121]<U> for T

where U: [TryFrom][114]<T>,

[Source][122]§

#### type [Error][123] = <U as [TryFrom][114]<T>>::[Error][119]

The type returned in the event of a conversion error.

[Source][124]§

#### fn [try_into][125](self) -> [Result][34]<U, <U as [TryFrom][114]<T>>::[Error][119]>

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

where S: [Into][23]<Dispatch>,

Attaches the provided [`Subscriber`][126] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][127] [`Subscriber`][126] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/routing/mod.rs.html#662-665
   [5]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [6]: ../struct.Router.html (struct axum::Router)
   [7]: ../struct.Router.html#method.into_service (method axum::Router::into_service)
   [8]: ../../src/axum/routing/mod.rs.html#667-677
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [10]: struct.RouterIntoService.html (struct axum::routing::RouterIntoService)
   [11]: ../../src/axum/routing/mod.rs.html#671-676
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [13]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [15]: ../../src/axum/routing/mod.rs.html#699-708
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [17]: ../../src/axum/routing/mod.rs.html#703-707
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [19]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [20]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [21]: ../../src/axum/routing/mod.rs.html#679-697
   [22]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [23]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [24]: ../type.BoxError.html (type axum::BoxError)
   [25]: ../../src/axum/routing/mod.rs.html#684
   [26]: ../body/struct.Body.html (struct axum::body::Body)
   [27]: ../../src/axum/routing/mod.rs.html#685
   [28]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [29]: ../../src/axum/routing/mod.rs.html#686
   [30]: future/struct.RouteFuture.html (struct axum::routing::future::RouteFuture)
   [31]: ../../src/axum/routing/mod.rs.html#689-691
   [32]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [33]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [34]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [35]: ../../src/axum/routing/mod.rs.html#694-696
   [36]: ../extract/type.Request.html (type axum::extract::Request)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [41]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [42]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [43]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [45]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [46]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [47]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [52]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [57]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [58]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [59]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [60]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [61]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [62]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [67]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [68]: ../extract/trait.FromRef.html#tymethod.from_ref
   [69]: super::Span::current()
   [70]: crate::Span
   [71]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [72]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [73]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [74]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [75]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [76]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [77]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [78]: ../../src/axum/service_ext.rs.html#47-59
   [79]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [80]: ../../src/axum/service_ext.rs.html#51-53
   [81]: ../trait.ServiceExt.html#tymethod.into_make_service
   [82]: struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [83]: tower::make::MakeService
   [84]: ../../src/axum/service_ext.rs.html#56-58
   [85]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [86]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [87]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [88]: ../../src/axum/service_ext.rs.html#42-44
   [89]: ../trait.ServiceExt.html#method.handle_error
   [90]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [91]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [92]: crate::Service::poll_ready
   [93]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [94]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [95]: crate::filter::Filter
   [96]: crate::filter::Predicate
   [97]: crate::filter::AsyncFilter
   [98]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [99]: crate::Service
   [100]: https://docs.rs/http/latest/http/struct.Extensions.html
   [101]: crate::follow_redirect::policy::Standard
   [102]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [103]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [104]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [105]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [106]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [107]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [108]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [109]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [110]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [111]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [112]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [113]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [114]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [115]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [116]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [117]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [118]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [119]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [120]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [121]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [122]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [123]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [124]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [125]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [126]: super::Subscriber
   [127]: dispatcher#setting-the-default-subscriber

