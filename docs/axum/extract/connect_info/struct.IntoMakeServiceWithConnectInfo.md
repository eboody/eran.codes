<!-- Generated from rustdoc HTML: extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IntoMakeServiceWithConnectInfo

## [axum][1]0.8.8

## IntoMakeServiceWithConnectInfo

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



## [In axum::extract::connect_info][2]

[axum][3]::[extract][4]::[connect_info][2]

# Struct IntoMakeServiceWithConnectInfo Copy item path

[Source][5]
``` 
pub struct IntoMakeServiceWithConnectInfo<S, C> { /* private fields */ }
```

Available on **crate feature`tokio`** only.

Expand description

A [`MakeService`][6] created from a router.

See [`Router::into_make_service_with_connect_info`][7] for more details.

## Trait Implementations§

[Source][8]§

### impl<S, C> [Clone][9] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [Clone][9],

[Source][11]§

#### fn [clone][12](&self) -> Self

Returns a duplicate of the value. [Read more][12]

1.0.0 · [Source][13]§

#### fn [clone_from][14](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][14]

[Source][15]§

### impl<S, C> [Debug][16] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [Debug][16],

[Source][17]§

#### fn [fmt][18](&self, f: &mut [Formatter][19]<'_>) -> [Result][20]

Formats the value using the given formatter. [Read more][18]

[Source][21]§

### impl<S, C, T> Service<T> for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [Clone][9], C: [Connected][22]<T>,

[Source][23]§

#### type Response = [AddExtension][24]<S, [ConnectInfo][25]<C>>

Responses given by the service.

[Source][26]§

#### type Error = [Infallible][27]

Errors produced by the service.

[Source][28]§

#### type Future = [ResponseFuture][29]<S, C>

The future response value.

[Source][30]§

#### fn poll_ready(&mut self, _cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][33]<[()][34], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][35]§

#### fn call(&mut self, target: T) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl<S, C> [Freeze][36] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [Freeze][36],

§

### impl<S, C> [RefUnwindSafe][37] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [RefUnwindSafe][37],

§

### impl<S, C> [Send][38] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [Send][38],

§

### impl<S, C> [Sync][39] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [Sync][39],

§

### impl<S, C> [Unpin][40] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [Unpin][40],

§

### impl<S, C> [UnwindSafe][41] for [IntoMakeServiceWithConnectInfo][10]<S, C>

where S: [UnwindSafe][41],

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

### impl<T, U> [Into][72]<U> for T

where U: [From][64]<T>,

[Source][73]§

#### fn [into][74](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][64]<T> for U` chooses to do.

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

#### fn poll_ready( &mut self, cx: &mut [Context][31]<'_>, ) -> [Poll][32]<[Result][33]<[()][34], <M as MakeService<Target, Request>>::MakeError>>

Returns [`Poll::Ready`][75] when the factory is able to create more services. Read more

§

#### fn make_service( &mut self, target: Target, ) -> <M as MakeService<Target, Request>>::Future

Create and return a new service value asynchronously.

§

#### fn into_service(self) -> IntoService<Self, Request>

where Self: [Sized][44],

Consume this [`MakeService`] and convert it into a [`Service`]. Read more

§

#### fn as_service(&mut self) -> AsService<'_, Self, Request>

where Self: [Sized][44],

Convert this [`MakeService`] into a [`Service`] without consuming the original [`MakeService`]. Read more

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

[Source][76]§

### impl<T> [Same][77] for T

[Source][78]§

#### type [Output][79] = T

Should always be `Self`

[Source][80]§

### impl<S, R> [ServiceExt][81]<R> for S

where S: Service<R>,

[Source][82]§

#### fn [into_make_service][83](self) -> [IntoMakeService][84]<S>

Convert this service into a [`MakeService`][6], that is a [`Service`] whose response is another service. [Read more][83]

[Source][85]§

#### fn [into_make_service_with_connect_info][86]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][10]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][6], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][25] can extract it. [Read more][86]

[Source][87]§

#### fn [handle_error][88]<F, T>(self, f: F) -> [HandleError][89]<Self, F, T>

Convert this service into a [`HandleError`][89], that will handle errors by converting them into responses. [Read more][88]

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

Process all requests from the given [`Stream`][90], and produce a [`Stream`][90] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][44], F: [Clone][9],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][91] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][44], F: [FnOnce][92](Self::Response) -> Response + [Clone][9],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][91] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][44], F: [FnOnce][92](Self::Error) -> Error + [Clone][9],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][91] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][44], Error: [From][64]<Self::Error>, F: [FnOnce][92]([Result][33]<Self::Response, Self::Error>) -> [Result][33]<Response, Error> \+ [Clone][9],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][44], F: [FnMut][93](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][44], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][94] that conditionally accepts or rejects requests based on a [predicate][95]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][44], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][96] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][44], Error: [From][64]<Self::Error>, F: [FnOnce][92]([Result][33]<Self::Response, Self::Error>) -> Fut + [Clone][9], Fut: [Future][97]<Output = [Result][33]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][44], F: [FnMut][93](Self::Future) -> Fut, Error: [From][64]<Self::Error>, Fut: [Future][97]<Output = [Result][33]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][44] \+ [Send][38] \+ 'static, Self::Future: [Send][38] \+ 'static,

Convert the service into a [`Service`][98] \+ [`Send`][38] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][44] \+ [Clone][9] \+ [Send][38] \+ 'static, Self::Future: [Send][38] \+ 'static,

Convert the service into a [`Service`][98] \+ [`Clone`][9] \+ [`Send`][38] trait object. Read more

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

Add some shareable value to [request extensions][99]. Read more

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

Follow redirect resposes using the [`Standard`][100] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][101]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][102] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][101]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][102] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][101]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][102] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][103]) -> RequestBodyLimit<Self>

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

[Source][104]§

### impl<T> [ToOwned][105] for T

where T: [Clone][9],

[Source][106]§

#### type [Owned][107] = T

The resulting type after obtaining ownership.

[Source][108]§

#### fn [to_owned][109](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][109]

[Source][110]§

#### fn [clone_into][111](&self, target: [&mut T][52])

Uses borrowed data to replace owned data, usually by cloning. [Read more][111]

[Source][112]§

### impl<T, U> [TryFrom][113]<U> for T

where U: [Into][72]<T>,

[Source][114]§

#### type [Error][115] = [Infallible][27]

The type returned in the event of a conversion error.

[Source][116]§

#### fn [try_from][117](value: U) -> [Result][33]<T, <T as [TryFrom][113]<U>>::[Error][118]>

Performs the conversion.

[Source][119]§

### impl<T, U> [TryInto][120]<U> for T

where U: [TryFrom][113]<T>,

[Source][121]§

#### type [Error][122] = <U as [TryFrom][113]<T>>::[Error][118]

The type returned in the event of a conversion error.

[Source][123]§

#### fn [try_into][124](self) -> [Result][33]<U, <U as [TryFrom][113]<T>>::[Error][118]>

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

where S: [Into][72]<Dispatch>,

Attaches the provided [`Subscriber`][125] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][126] [`Subscriber`][125] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/connect_info.rs.html#28-31
   [6]: tower::make::MakeService
   [7]: ../../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [8]: ../../../src/axum/extract/connect_info.rs.html#53-63
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [10]: struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [11]: ../../../src/axum/extract/connect_info.rs.html#57-62
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [13]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [15]: ../../../src/axum/extract/connect_info.rs.html#42-51
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [17]: ../../../src/axum/extract/connect_info.rs.html#46-50
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [19]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [20]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [21]: ../../../src/axum/extract/connect_info.rs.html#99-118
   [22]: trait.Connected.html (trait axum::extract::connect_info::Connected)
   [23]: ../../../src/axum/extract/connect_info.rs.html#104
   [24]: ../../middleware/struct.AddExtension.html (struct axum::middleware::AddExtension)
   [25]: ../struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [26]: ../../../src/axum/extract/connect_info.rs.html#105
   [27]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [28]: ../../../src/axum/extract/connect_info.rs.html#106
   [29]: struct.ResponseFuture.html (struct axum::extract::connect_info::ResponseFuture)
   [30]: ../../../src/axum/extract/connect_info.rs.html#109-111
   [31]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [32]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [33]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [34]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [35]: ../../../src/axum/extract/connect_info.rs.html#113-117
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [37]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
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
   [67]: ../trait.FromRef.html (trait axum::extract::FromRef)
   [68]: ../trait.FromRef.html#tymethod.from_ref
   [69]: super::Span::current()
   [70]: crate::Span
   [71]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [72]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [75]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready (variant core::task::poll::Poll::Ready)
   [76]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [77]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [78]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [79]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [80]: ../../../src/axum/service_ext.rs.html#47-59
   [81]: ../../trait.ServiceExt.html (trait axum::ServiceExt)
   [82]: ../../../src/axum/service_ext.rs.html#51-53
   [83]: ../../trait.ServiceExt.html#tymethod.into_make_service
   [84]: ../../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [85]: ../../../src/axum/service_ext.rs.html#56-58
   [86]: ../../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [87]: ../../../src/axum/service_ext.rs.html#42-44
   [88]: ../../trait.ServiceExt.html#method.handle_error
   [89]: ../../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [90]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [91]: crate::Service::poll_ready
   [92]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [93]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [94]: crate::filter::Filter
   [95]: crate::filter::Predicate
   [96]: crate::filter::AsyncFilter
   [97]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [98]: crate::Service
   [99]: https://docs.rs/http/latest/http/struct.Extensions.html
   [100]: crate::follow_redirect::policy::Standard
   [101]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [102]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [103]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [104]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [105]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [106]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [107]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [108]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [109]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [110]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [111]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [112]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [113]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [114]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [115]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
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

