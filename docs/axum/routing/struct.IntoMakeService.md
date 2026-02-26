<!-- Generated from rustdoc HTML: routing/struct.IntoMakeService.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IntoMakeService

## [axum][1]0.8.8

## IntoMakeService

### Trait Implementations

  * Clone
  * Debug
  * Service<T>



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
  * MakeService<Target, Request>
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

# Struct IntoMakeService Copy item path

[Source][4]
``` 
pub struct IntoMakeService<S> { /* private fields */ }
```

Expand description

A [`MakeService`][5] that produces axum router services.

## Trait Implementations§

[Source][6]§

### impl<S: [Clone][7]> [Clone][7] for [IntoMakeService][8]<S>

[Source][6]§

#### fn [clone][9](&self) -> [IntoMakeService][8]<S>

Returns a duplicate of the value. [Read more][9]

1.0.0 · [Source][10]§

#### fn [clone_from][11](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][11]

[Source][6]§

### impl<S: [Debug][12]> [Debug][12] for [IntoMakeService][8]<S>

[Source][6]§

#### fn [fmt][13](&self, f: &mut [Formatter][14]<'_>) -> [Result][15]

Formats the value using the given formatter. [Read more][13]

[Source][16]§

### impl<S, T> Service<T> for [IntoMakeService][8]<S>

where S: [Clone][7],

[Source][17]§

#### type Response = S

Responses given by the service.

[Source][18]§

#### type Error = [Infallible][19]

Errors produced by the service.

[Source][20]§

#### type Future = [IntoMakeServiceFuture][21]<S>

The future response value.

[Source][22]§

#### fn poll_ready(&mut self, _cx: &mut [Context][23]<'_>) -> [Poll][24]<[Result][25]<[()][26], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][27]§

#### fn call(&mut self, _target: T) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<S> [Freeze][28] for [IntoMakeService][8]<S>

where S: [Freeze][28],

§

### impl<S> [RefUnwindSafe][29] for [IntoMakeService][8]<S>

where S: [RefUnwindSafe][29],

§

### impl<S> [Send][30] for [IntoMakeService][8]<S>

where S: [Send][30],

§

### impl<S> [Sync][31] for [IntoMakeService][8]<S>

where S: [Sync][31],

§

### impl<S> [Unpin][32] for [IntoMakeService][8]<S>

where S: [Unpin][32],

§

### impl<S> [UnwindSafe][33] for [IntoMakeService][8]<S>

where S: [UnwindSafe][33],

## Blanket Implementations§

[Source][34]§

### impl<T> [Any][35] for T

where T: 'static + ?[Sized][36],

[Source][37]§

#### fn [type_id][38](&self) -> [TypeId][39]

Gets the `TypeId` of `self`. [Read more][38]

[Source][40]§

### impl<T> [Borrow][41]<T> for T

where T: ?[Sized][36],

[Source][42]§

#### fn [borrow][43](&self) -> [&T][44]

Immutably borrows from an owned value. [Read more][43]

[Source][45]§

### impl<T> [BorrowMut][46]<T> for T

where T: ?[Sized][36],

[Source][47]§

#### fn [borrow_mut][48](&mut self) -> [&mut T][44]

Mutably borrows from an owned value. [Read more][48]

[Source][49]§

### impl<T> [CloneToUninit][50] for T

where T: [Clone][7],

[Source][51]§

#### unsafe fn [clone_to_uninit][52](&self, dest: [*mut ][53][u8][54])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][52]

[Source][55]§

### impl<T> [From][56]<T> for T

[Source][57]§

#### fn [from][58](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][59]<T> for T

where T: [Clone][7],

§

#### fn [from_ref][60](input: [&T][44]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][61] [`Span`][62], returning an `Instrumented` wrapper. Read more

[Source][63]§

### impl<T, U> [Into][64]<U> for T

where U: [From][56]<T>,

[Source][65]§

#### fn [into][66](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][56]<T> for U` chooses to do.

§

### impl<M, S, Target, Request> MakeService<Target, Request> for M

where M: Service<Target, Response = S>, S: Service<Request>,

§

#### type Response = <S as Service<Request>>::Response

Responses given by the service

§

#### type Error = <S as Service<Request>>::Error

Errors produced by the service

§

#### type Service = S

The [`Service`] value created by this factory

§

#### type MakeError = <M as Service<Target>>::Error

Errors produced while building a service.

§

#### type Future = <M as Service<Target>>::Future

The future of the [`Service`] instance.

§

#### fn poll_ready( &mut self, cx: &mut [Context][23]<'_>, ) -> [Poll][24]<[Result][25]<[()][26], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][67] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][36],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][36],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][36],

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

[Source][72]§

### impl<S, R> [ServiceExt][73]<R> for S

where S: Service<R>,

[Source][74]§

#### fn [into_make_service][75](self) -> [IntoMakeService][8]<S>

Convert this service into a [`MakeService`][5], that is a [`Service`] whose response is another service. [Read more][75]

[Source][76]§

#### fn [into_make_service_with_connect_info][77]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][78]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][5], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][79] can extract it. [Read more][77]

[Source][80]§

#### fn [handle_error][81]<F, T>(self, f: F) -> [HandleError][82]<Self, F, T>

Convert this service into a [`HandleError`][82], that will handle errors by converting them into responses. [Read more][81]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][36],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][36],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][36],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][36],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][36], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][83], and produce a [`Stream`][83] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][36], F: [Clone][7],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][84] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][36], F: [FnOnce][85](Self::Response) -> Response + [Clone][7],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][84] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][36], F: [FnOnce][85](Self::Error) -> Error + [Clone][7],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][84] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][36], Error: [From][56]<Self::Error>, F: [FnOnce][85]([Result][25]<Self::Response, Self::Error>) -> [Result][25]<Response, Error> \+ [Clone][7],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][36], F: [FnMut][86](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][36], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][87] that conditionally accepts or rejects requests based on a [predicate][88]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][36], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][89] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][36], Error: [From][56]<Self::Error>, F: [FnOnce][85]([Result][25]<Self::Response, Self::Error>) -> Fut + [Clone][7], Fut: [Future][90]<Output = [Result][25]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][36], F: [FnMut][86](Self::Future) -> Fut, Error: [From][56]<Self::Error>, Fut: [Future][90]<Output = [Result][25]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][36] \+ [Send][30] \+ 'static, Self::Future: [Send][30] \+ 'static,

Convert the service into a [`Service`][91] \+ [`Send`][30] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][36] \+ [Clone][7] \+ [Send][30] \+ 'static, Self::Future: [Send][30] \+ 'static,

Convert the service into a [`Service`][91] \+ [`Clone`][7] \+ [`Send`][30] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][36],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][36],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][92]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][36],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][36],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][36],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][36],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][36],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][36],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][36],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][93] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][94]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][36],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][95] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][94]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][36],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][95] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][94]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][36],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][95] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][36],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][36], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][36], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][36],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][36],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][36],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][96]) -> RequestBodyLimit<Self>

where Self: [Sized][36],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][36],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][36],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][97]§

### impl<T> [ToOwned][98] for T

where T: [Clone][7],

[Source][99]§

#### type [Owned][100] = T

The resulting type after obtaining ownership.

[Source][101]§

#### fn [to_owned][102](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][102]

[Source][103]§

#### fn [clone_into][104](&self, target: [&mut T][44])

Uses borrowed data to replace owned data, usually by cloning. [Read more][104]

[Source][105]§

### impl<T, U> [TryFrom][106]<U> for T

where U: [Into][64]<T>,

[Source][107]§

#### type [Error][108] = [Infallible][19]

The type returned in the event of a conversion error.

[Source][109]§

#### fn [try_from][110](value: U) -> [Result][25]<T, <T as [TryFrom][106]<U>>::[Error][111]>

Performs the conversion.

[Source][112]§

### impl<T, U> [TryInto][113]<U> for T

where U: [TryFrom][106]<T>,

[Source][114]§

#### type [Error][115] = <U as [TryFrom][106]<T>>::[Error][111]

The type returned in the event of a conversion error.

[Source][116]§

#### fn [try_into][117](self) -> [Result][25]<U, <U as [TryFrom][106]<T>>::[Error][111]>

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

where S: [Into][64]<Dispatch>,

Attaches the provided [`Subscriber`][118] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][119] [`Subscriber`][118] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/routing/into_make_service.rs.html#12-14
   [5]: tower::make::MakeService
   [6]: ../../src/axum/routing/into_make_service.rs.html#11
   [7]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [8]: struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [10]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [16]: ../../src/axum/routing/into_make_service.rs.html#22-38
   [17]: ../../src/axum/routing/into_make_service.rs.html#26
   [18]: ../../src/axum/routing/into_make_service.rs.html#27
   [19]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [20]: ../../src/axum/routing/into_make_service.rs.html#28
   [21]: future/struct.IntoMakeServiceFuture.html (struct axum::routing::future::IntoMakeServiceFuture)
   [22]: ../../src/axum/routing/into_make_service.rs.html#31-33
   [23]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [24]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [25]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [26]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [27]: ../../src/axum/routing/into_make_service.rs.html#35-37
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [29]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [33]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [34]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [35]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [37]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [38]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [39]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [44]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [45]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [46]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [49]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [50]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [51]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [52]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [53]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [54]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [56]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [58]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [59]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [60]: ../extract/trait.FromRef.html#tymethod.from_ref
   [61]: super::Span::current()
   [62]: crate::Span
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [67]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [70]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [71]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [72]: ../../src/axum/service_ext.rs.html#47-59
   [73]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [74]: ../../src/axum/service_ext.rs.html#51-53
   [75]: ../trait.ServiceExt.html#tymethod.into_make_service
   [76]: ../../src/axum/service_ext.rs.html#56-58
   [77]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [78]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [79]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [80]: ../../src/axum/service_ext.rs.html#42-44
   [81]: ../trait.ServiceExt.html#method.handle_error
   [82]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [83]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [84]: crate::Service::poll_ready
   [85]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [86]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [87]: crate::filter::Filter
   [88]: crate::filter::Predicate
   [89]: crate::filter::AsyncFilter
   [90]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [91]: crate::Service
   [92]: https://docs.rs/http/latest/http/struct.Extensions.html
   [93]: crate::follow_redirect::policy::Standard
   [94]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [95]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [96]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [97]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [98]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [99]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [100]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [101]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [102]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [103]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [104]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [105]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [106]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [107]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [108]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [109]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [110]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [111]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [112]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [113]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [114]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [115]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [116]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [117]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [118]: super::Subscriber
   [119]: dispatcher#setting-the-default-subscriber

