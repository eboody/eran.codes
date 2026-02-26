<!-- Generated from rustdoc HTML: routing/struct.RouterAsService.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RouterAsService

## [axum][1]0.8.8

## RouterAsService

### Trait Implementations

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
  * From<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ServiceExt<R>
  * ServiceExt<Request>
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::routing][2]

[axum][3]::[routing][2]

# Struct RouterAsService Copy item path

[Source][4]
``` 
pub struct RouterAsService<'a, B, S = [()][5]> { /* private fields */ }
```

Expand description

A [`Router`][6] converted into a borrowed [`Service`] with a fixed body type.

See [`Router::as_service`][7] for more details.

## Trait Implementations§

[Source][8]§

### impl<B, S> [Debug][9] for [RouterAsService][10]<'_, B, S>

where S: [Debug][9],

[Source][11]§

#### fn [fmt][12](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]

Formats the value using the given formatter. [Read more][12]

[Source][15]§

### impl<B> Service<Request<B>> for [RouterAsService][10]<'_, B, [()][5]>

where B: HttpBody<Data = Bytes> \+ [Send][16] \+ 'static, B::Error: [Into][17]<[BoxError][18]>,

[Source][19]§

#### type Response = Response<[Body][20]>

Responses given by the service.

[Source][21]§

#### type Error = [Infallible][22]

Errors produced by the service.

[Source][23]§

#### type Future = [RouteFuture][24]<[Infallible][22]>

The future response value.

[Source][25]§

#### fn poll_ready(&mut self, cx: &mut [Context][26]<'_>) -> [Poll][27]<[Result][28]<[()][5], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][29]§

#### fn call(&mut self, req: [Request][30]<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<'a, B, S> [Freeze][31] for [RouterAsService][10]<'a, B, S>

§

### impl<'a, B, S = [()][5]> ![RefUnwindSafe][32] for [RouterAsService][10]<'a, B, S>

§

### impl<'a, B, S> [Send][16] for [RouterAsService][10]<'a, B, S>

§

### impl<'a, B, S> [Sync][33] for [RouterAsService][10]<'a, B, S>

§

### impl<'a, B, S> [Unpin][34] for [RouterAsService][10]<'a, B, S>

§

### impl<'a, B, S = [()][5]> ![UnwindSafe][35] for [RouterAsService][10]<'a, B, S>

## Blanket Implementations§

[Source][36]§

### impl<T> [Any][37] for T

where T: 'static + ?[Sized][38],

[Source][39]§

#### fn [type_id][40](&self) -> [TypeId][41]

Gets the `TypeId` of `self`. [Read more][40]

[Source][42]§

### impl<T> [Borrow][43]<T> for T

where T: ?[Sized][38],

[Source][44]§

#### fn [borrow][45](&self) -> [&T][46]

Immutably borrows from an owned value. [Read more][45]

[Source][47]§

### impl<T> [BorrowMut][48]<T> for T

where T: ?[Sized][38],

[Source][49]§

#### fn [borrow_mut][50](&mut self) -> [&mut T][46]

Mutably borrows from an owned value. [Read more][50]

[Source][51]§

### impl<T> [From][52]<T> for T

[Source][53]§

#### fn [from][54](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][55] [`Span`][56], returning an `Instrumented` wrapper. Read more

[Source][57]§

### impl<T, U> [Into][17]<U> for T

where U: [From][52]<T>,

[Source][58]§

#### fn [into][59](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][52]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][38],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][60]§

### impl<T> [Same][61] for T

[Source][62]§

#### type [Output][63] = T

Should always be `Self`

[Source][64]§

### impl<S, R> [ServiceExt][65]<R> for S

where S: Service<R>,

[Source][66]§

#### fn [into_make_service][67](self) -> [IntoMakeService][68]<S>

Convert this service into a [`MakeService`][69], that is a [`Service`] whose response is another service. [Read more][67]

[Source][70]§

#### fn [into_make_service_with_connect_info][71]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][72]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][69], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][73] can extract it. [Read more][71]

[Source][74]§

#### fn [handle_error][75]<F, T>(self, f: F) -> [HandleError][76]<Self, F, T>

Convert this service into a [`HandleError`][76], that will handle errors by converting them into responses. [Read more][75]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][38],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][38],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][38],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][38],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][38], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][77], and produce a [`Stream`][77] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][38], F: [Clone][78],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][79] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][38], F: [FnOnce][80](Self::Response) -> Response + [Clone][78],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][79] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][38], F: [FnOnce][80](Self::Error) -> Error + [Clone][78],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][79] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][38], Error: [From][52]<Self::Error>, F: [FnOnce][80]([Result][28]<Self::Response, Self::Error>) -> [Result][28]<Response, Error> \+ [Clone][78],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][38], F: [FnMut][81](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][38], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][82] that conditionally accepts or rejects requests based on a [predicate][83]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][38], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][84] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][38], Error: [From][52]<Self::Error>, F: [FnOnce][80]([Result][28]<Self::Response, Self::Error>) -> Fut + [Clone][78], Fut: [Future][85]<Output = [Result][28]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][38], F: [FnMut][81](Self::Future) -> Fut, Error: [From][52]<Self::Error>, Fut: [Future][85]<Output = [Result][28]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][38] \+ [Send][16] \+ 'static, Self::Future: [Send][16] \+ 'static,

Convert the service into a [`Service`][86] \+ [`Send`][16] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][38] \+ [Clone][78] \+ [Send][16] \+ 'static, Self::Future: [Send][16] \+ 'static,

Convert the service into a [`Service`][86] \+ [`Clone`][78] \+ [`Send`][16] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][38],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][38],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][87]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][38],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][38],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][38],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][38],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][38],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][38],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][38],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][88] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][89]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][38],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][90] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][89]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][38],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][90] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][89]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][38],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][90] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][38], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][38], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][38],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][38],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][38],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][91]) -> RequestBodyLimit<Self>

where Self: [Sized][38],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][38],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][38],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][92]§

### impl<T, U> [TryFrom][93]<U> for T

where U: [Into][17]<T>,

[Source][94]§

#### type [Error][95] = [Infallible][22]

The type returned in the event of a conversion error.

[Source][96]§

#### fn [try_from][97](value: U) -> [Result][28]<T, <T as [TryFrom][93]<U>>::[Error][98]>

Performs the conversion.

[Source][99]§

### impl<T, U> [TryInto][100]<U> for T

where U: [TryFrom][93]<T>,

[Source][101]§

#### type [Error][102] = <U as [TryFrom][93]<T>>::[Error][98]

The type returned in the event of a conversion error.

[Source][103]§

#### fn [try_into][104](self) -> [Result][28]<U, <U as [TryFrom][93]<T>>::[Error][98]>

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

where S: [Into][17]<Dispatch>,

Attaches the provided [`Subscriber`][105] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][106] [`Subscriber`][105] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/routing/mod.rs.html#623-626
   [5]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [6]: ../struct.Router.html (struct axum::Router)
   [7]: ../struct.Router.html#method.as_service (method axum::Router::as_service)
   [8]: ../../src/axum/routing/mod.rs.html#648-657
   [9]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [10]: struct.RouterAsService.html (struct axum::routing::RouterAsService)
   [11]: ../../src/axum/routing/mod.rs.html#652-656
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [13]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [15]: ../../src/axum/routing/mod.rs.html#628-646
   [16]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [17]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [18]: ../type.BoxError.html (type axum::BoxError)
   [19]: ../../src/axum/routing/mod.rs.html#633
   [20]: ../body/struct.Body.html (struct axum::body::Body)
   [21]: ../../src/axum/routing/mod.rs.html#634
   [22]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [23]: ../../src/axum/routing/mod.rs.html#635
   [24]: future/struct.RouteFuture.html (struct axum::routing::future::RouteFuture)
   [25]: ../../src/axum/routing/mod.rs.html#638-640
   [26]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [27]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [28]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [29]: ../../src/axum/routing/mod.rs.html#643-645
   [30]: ../extract/type.Request.html (type axum::extract::Request)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [32]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [34]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [35]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [36]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [37]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [39]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [40]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [41]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [46]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [51]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [52]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [53]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [54]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [55]: super::Span::current()
   [56]: crate::Span
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [60]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [61]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [62]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [63]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [64]: ../../src/axum/service_ext.rs.html#47-59
   [65]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [66]: ../../src/axum/service_ext.rs.html#51-53
   [67]: ../trait.ServiceExt.html#tymethod.into_make_service
   [68]: struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [69]: tower::make::MakeService
   [70]: ../../src/axum/service_ext.rs.html#56-58
   [71]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [72]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [73]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [74]: ../../src/axum/service_ext.rs.html#42-44
   [75]: ../trait.ServiceExt.html#method.handle_error
   [76]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [77]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [78]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [79]: crate::Service::poll_ready
   [80]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [81]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [82]: crate::filter::Filter
   [83]: crate::filter::Predicate
   [84]: crate::filter::AsyncFilter
   [85]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [86]: crate::Service
   [87]: https://docs.rs/http/latest/http/struct.Extensions.html
   [88]: crate::follow_redirect::policy::Standard
   [89]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [90]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [91]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [98]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [99]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [100]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [101]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [102]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [103]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [105]: super::Subscriber
   [106]: dispatcher#setting-the-default-subscriber

