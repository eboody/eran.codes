<!-- Generated from rustdoc HTML: handler/struct.HandlerService.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## HandlerService

## [axum][1]0.8.8

## HandlerService

### Methods

  * into_make_service
  * into_make_service_with_connect_info
  * state



### Trait Implementations

  * Clone
  * Debug
  * Service<IncomingStream<'_, L>>
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



## [In axum::handler][2]

[axum][3]::[handler][2]

# Struct HandlerService Copy item path

[Source][4]
``` 
pub struct HandlerService<H, T, S> { /* private fields */ }
```

Expand description

An adapter that makes a [`Handler`][5] into a [`Service`].

Created with [`Handler::with_state`][6] or [`HandlerWithoutStateExt::into_service`][7].

## Implementations§

[Source][8]§

### impl<H, T, S> [HandlerService][9]<H, T, S>

[Source][10]

#### pub fn state(&self) -> [&S][11]

Get a reference to the state.

[Source][12]

#### pub fn into_make_service(self) -> [IntoMakeService][13]<Self>

Convert the handler into a [`MakeService`][14].

This allows you to serve a single handler if you don’t need any routing:
``` 
use axum::{
    handler::Handler,
    extract::State,
    http::{Uri, Method},
    response::IntoResponse,
};
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {}

async fn handler(State(state): State<AppState>) {
    // ...
}

let app = handler.with_state(AppState {});

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app.into_make_service()).await;
```

[Source][15]

#### pub fn into_make_service_with_connect_info<C>( self, ) -> [IntoMakeServiceWithConnectInfo][16]<Self, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][14] which stores information about the incoming connection.

See [`Router::into_make_service_with_connect_info`][17] for more details.
``` 
use axum::{
    handler::Handler,
    response::IntoResponse,
    extract::{ConnectInfo, State},
};
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {};

async fn handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> String {
    format!("Hello {addr}")
}

let app = handler.with_state(AppState {});

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>(),
).await;
```

## Trait Implementations§

[Source][18]§

### impl<H, T, S> [Clone][19] for [HandlerService][9]<H, T, S>

where H: [Clone][19], S: [Clone][19],

[Source][20]§

#### fn [clone][21](&self) -> Self

Returns a duplicate of the value. [Read more][21]

1.0.0 · [Source][22]§

#### fn [clone_from][23](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][23]

[Source][24]§

### impl<H, T, S> [Debug][25] for [HandlerService][9]<H, T, S>

[Source][26]§

#### fn [fmt][27](&self, f: &mut [Formatter][28]<'_>) -> [Result][29]

Formats the value using the given formatter. [Read more][27]

[Source][30]§

### impl<H, T, S, L> Service<[IncomingStream][31]<'_, L>> for [HandlerService][9]<H, T, S>

where H: [Clone][19], S: [Clone][19], L: [Listener][32],

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

[Source][33]§

#### type Response = [HandlerService][9]<H, T, S>

Responses given by the service.

[Source][34]§

#### type Error = [Infallible][35]

Errors produced by the service.

[Source][36]§

#### type Future = [Ready][37]<[Result][38]<<[HandlerService][9]<H, T, S> as Service<[IncomingStream][31]<'_, L>>>::Response, <[HandlerService][9]<H, T, S> as Service<[IncomingStream][31]<'_, L>>>::Error>>

The future response value.

[Source][39]§

#### fn poll_ready(&mut self, _cx: &mut [Context][40]<'_>) -> [Poll][41]<[Result][38]<[()][42], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][43]§

#### fn call(&mut self, _req: [IncomingStream][31]<'_, L>) -> Self::Future

Process the request and return the response asynchronously. Read more

[Source][44]§

### impl<H, T, S, B> Service<Request<B>> for [HandlerService][9]<H, T, S>

where H: [Handler][5]<T, S> \+ [Clone][19] \+ [Send][45] \+ 'static, B: HttpBody<Data = Bytes> \+ [Send][45] \+ 'static, B::Error: [Into][46]<[BoxError][47]>, S: [Clone][19] \+ [Send][45] \+ [Sync][48],

[Source][49]§

#### type Response = Response<[Body][50]>

Responses given by the service.

[Source][51]§

#### type Error = [Infallible][35]

Errors produced by the service.

[Source][52]§

#### type Future = [IntoServiceFuture][53]<<H as [Handler][5]<T, S>>::[Future][54]>

The future response value.

[Source][55]§

#### fn poll_ready(&mut self, _cx: &mut [Context][40]<'_>) -> [Poll][41]<[Result][38]<[()][42], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][56]§

#### fn call(&mut self, req: Request<B>) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<H, T, S> [Freeze][57] for [HandlerService][9]<H, T, S>

where H: [Freeze][57], S: [Freeze][57],

§

### impl<H, T, S> [RefUnwindSafe][58] for [HandlerService][9]<H, T, S>

where H: [RefUnwindSafe][58], S: [RefUnwindSafe][58],

§

### impl<H, T, S> [Send][45] for [HandlerService][9]<H, T, S>

where H: [Send][45], S: [Send][45],

§

### impl<H, T, S> [Sync][48] for [HandlerService][9]<H, T, S>

where H: [Sync][48], S: [Sync][48],

§

### impl<H, T, S> [Unpin][59] for [HandlerService][9]<H, T, S>

where H: [Unpin][59], S: [Unpin][59],

§

### impl<H, T, S> [UnwindSafe][60] for [HandlerService][9]<H, T, S>

where H: [UnwindSafe][60], S: [UnwindSafe][60],

## Blanket Implementations§

[Source][61]§

### impl<T> [Any][62] for T

where T: 'static + ?[Sized][63],

[Source][64]§

#### fn [type_id][65](&self) -> [TypeId][66]

Gets the `TypeId` of `self`. [Read more][65]

[Source][67]§

### impl<T> [Borrow][68]<T> for T

where T: ?[Sized][63],

[Source][69]§

#### fn [borrow][70](&self) -> [&T][11]

Immutably borrows from an owned value. [Read more][70]

[Source][71]§

### impl<T> [BorrowMut][72]<T> for T

where T: ?[Sized][63],

[Source][73]§

#### fn [borrow_mut][74](&mut self) -> [&mut T][11]

Mutably borrows from an owned value. [Read more][74]

[Source][75]§

### impl<T> [CloneToUninit][76] for T

where T: [Clone][19],

[Source][77]§

#### unsafe fn [clone_to_uninit][78](&self, dest: [*mut ][79][u8][80])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][78]

[Source][81]§

### impl<T> [From][82]<T> for T

[Source][83]§

#### fn [from][84](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][85]<T> for T

where T: [Clone][19],

§

#### fn [from_ref][86](input: [&T][11]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][87] [`Span`][88], returning an `Instrumented` wrapper. Read more

[Source][89]§

### impl<T, U> [Into][46]<U> for T

where U: [From][82]<T>,

[Source][90]§

#### fn [into][91](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][82]<T> for U` chooses to do.

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

#### fn poll_ready( &mut self, cx: &mut [Context][40]<'_>, ) -> [Poll][41]<[Result][38]<[()][42], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][92] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][63],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][63],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

§

### impl<T> PolicyExt for T

where T: ?[Sized][63],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][93]§

### impl<T> [Same][94] for T

[Source][95]§

#### type [Output][96] = T

Should always be `Self`

[Source][97]§

### impl<S, R> [ServiceExt][98]<R> for S

where S: Service<R>,

[Source][99]§

#### fn [into_make_service][100](self) -> [IntoMakeService][13]<S>

Convert this service into a [`MakeService`][14], that is a [`Service`] whose response is another service. [Read more][100]

[Source][101]§

#### fn [into_make_service_with_connect_info][102]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][16]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][14], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][103] can extract it. [Read more][102]

[Source][104]§

#### fn [handle_error][105]<F, T>(self, f: F) -> [HandleError][106]<Self, F, T>

Convert this service into a [`HandleError`][106], that will handle errors by converting them into responses. [Read more][105]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][63],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][63],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][63],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][63],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][63], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][107], and produce a [`Stream`][107] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][63], F: [Clone][19],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][108] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][63], F: [FnOnce][109](Self::Response) -> Response + [Clone][19],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][108] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][63], F: [FnOnce][109](Self::Error) -> Error + [Clone][19],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][108] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][63], Error: [From][82]<Self::Error>, F: [FnOnce][109]([Result][38]<Self::Response, Self::Error>) -> [Result][38]<Response, Error> \+ [Clone][19],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][63], F: [FnMut][110](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][63], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][111] that conditionally accepts or rejects requests based on a [predicate][112]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][63], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][113] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][63], Error: [From][82]<Self::Error>, F: [FnOnce][109]([Result][38]<Self::Response, Self::Error>) -> Fut + [Clone][19], Fut: [Future][114]<Output = [Result][38]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][63], F: [FnMut][110](Self::Future) -> Fut, Error: [From][82]<Self::Error>, Fut: [Future][114]<Output = [Result][38]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][63] \+ [Send][45] \+ 'static, Self::Future: [Send][45] \+ 'static,

Convert the service into a [`Service`][115] \+ [`Send`][45] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][63] \+ [Clone][19] \+ [Send][45] \+ 'static, Self::Future: [Send][45] \+ 'static,

Convert the service into a [`Service`][115] \+ [`Clone`][19] \+ [`Send`][45] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][63],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][63],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][116]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][63],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][63],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][63],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][63],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][63],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][63],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][63],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][117] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][118]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][63],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][119] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][118]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][63],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][119] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][118]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][63],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][119] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][63],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][63], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][63], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][63],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][63],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][63],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][120]) -> RequestBodyLimit<Self>

where Self: [Sized][63],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][63],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][63],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][121]§

### impl<T> [ToOwned][122] for T

where T: [Clone][19],

[Source][123]§

#### type [Owned][124] = T

The resulting type after obtaining ownership.

[Source][125]§

#### fn [to_owned][126](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][126]

[Source][127]§

#### fn [clone_into][128](&self, target: [&mut T][11])

Uses borrowed data to replace owned data, usually by cloning. [Read more][128]

[Source][129]§

### impl<T, U> [TryFrom][130]<U> for T

where U: [Into][46]<T>,

[Source][131]§

#### type [Error][132] = [Infallible][35]

The type returned in the event of a conversion error.

[Source][133]§

#### fn [try_from][134](value: U) -> [Result][38]<T, <T as [TryFrom][130]<U>>::[Error][135]>

Performs the conversion.

[Source][136]§

### impl<T, U> [TryInto][137]<U> for T

where U: [TryFrom][130]<T>,

[Source][138]§

#### type [Error][139] = <U as [TryFrom][130]<T>>::[Error][135]

The type returned in the event of a conversion error.

[Source][140]§

#### fn [try_into][141](self) -> [Result][38]<U, <U as [TryFrom][130]<T>>::[Error][135]>

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

where S: [Into][46]<Dispatch>,

Attaches the provided [`Subscriber`][142] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][143] [`Subscriber`][142] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/handler/service.rs.html#22-26
   [5]: trait.Handler.html (trait axum::handler::Handler)
   [6]: trait.Handler.html#method.with_state (method axum::handler::Handler::with_state)
   [7]: trait.HandlerWithoutStateExt.html#tymethod.into_service (method axum::handler::HandlerWithoutStateExt::into_service)
   [8]: ../../src/axum/handler/service.rs.html#28-107
   [9]: struct.HandlerService.html (struct axum::handler::HandlerService)
   [10]: ../../src/axum/handler/service.rs.html#30-32
   [11]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [12]: ../../src/axum/handler/service.rs.html#63-65
   [13]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [14]: tower::make::MakeService
   [15]: ../../src/axum/handler/service.rs.html#104-106
   [16]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [17]: ../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [18]: ../../src/axum/handler/service.rs.html#132-144
   [19]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [20]: ../../src/axum/handler/service.rs.html#137-143
   [21]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [22]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [23]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [24]: ../../src/axum/handler/service.rs.html#126-130
   [25]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [26]: ../../src/axum/handler/service.rs.html#127-129
   [27]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [28]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [29]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [30]: ../../src/axum/handler/service.rs.html#183-200
   [31]: ../serve/struct.IncomingStream.html (struct axum::serve::IncomingStream)
   [32]: ../serve/trait.Listener.html (trait axum::serve::Listener)
   [33]: ../../src/axum/handler/service.rs.html#189
   [34]: ../../src/axum/handler/service.rs.html#190
   [35]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [36]: ../../src/axum/handler/service.rs.html#191
   [37]: https://doc.rust-lang.org/nightly/core/future/ready/struct.Ready.html (struct core::future::ready::Ready)
   [38]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [39]: ../../src/axum/handler/service.rs.html#193-195
   [40]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [41]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [42]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [43]: ../../src/axum/handler/service.rs.html#197-199
   [44]: ../../src/axum/handler/service.rs.html#146-176
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [46]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [47]: ../type.BoxError.html (type axum::BoxError)
   [48]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [49]: ../../src/axum/handler/service.rs.html#153
   [50]: ../body/struct.Body.html (struct axum::body::Body)
   [51]: ../../src/axum/handler/service.rs.html#154
   [52]: ../../src/axum/handler/service.rs.html#155
   [53]: future/struct.IntoServiceFuture.html (struct axum::handler::future::IntoServiceFuture)
   [54]: trait.Handler.html#associatedtype.Future (type axum::handler::Handler::Future)
   [55]: ../../src/axum/handler/service.rs.html#158-163
   [56]: ../../src/axum/handler/service.rs.html#165-175
   [57]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [58]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [59]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [60]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [61]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [62]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [63]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [64]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [65]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [66]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [67]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [68]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [69]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [70]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [71]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [72]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [73]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [74]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [75]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [76]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [77]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [78]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [79]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [80]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [85]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [86]: ../extract/trait.FromRef.html#tymethod.from_ref
   [87]: super::Span::current()
   [88]: crate::Span
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [92]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [93]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [94]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [95]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [96]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [97]: ../../src/axum/service_ext.rs.html#47-59
   [98]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [99]: ../../src/axum/service_ext.rs.html#51-53
   [100]: ../trait.ServiceExt.html#tymethod.into_make_service
   [101]: ../../src/axum/service_ext.rs.html#56-58
   [102]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [103]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [104]: ../../src/axum/service_ext.rs.html#42-44
   [105]: ../trait.ServiceExt.html#method.handle_error
   [106]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [107]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [108]: crate::Service::poll_ready
   [109]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [110]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [111]: crate::filter::Filter
   [112]: crate::filter::Predicate
   [113]: crate::filter::AsyncFilter
   [114]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [115]: crate::Service
   [116]: https://docs.rs/http/latest/http/struct.Extensions.html
   [117]: crate::follow_redirect::policy::Standard
   [118]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [119]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [120]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [121]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [122]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [123]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [124]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [125]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [126]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [127]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [128]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [129]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [130]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [131]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [132]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [133]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [134]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [135]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [136]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [137]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [138]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [139]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [140]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [141]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [142]: super::Subscriber
   [143]: dispatcher#setting-the-default-subscriber

