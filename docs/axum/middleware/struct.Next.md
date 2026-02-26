<!-- Generated from rustdoc HTML: middleware/struct.Next.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Next

## [axum][1]0.8.8

## Next

### Methods

  * run



### Trait Implementations

  * Clone
  * Debug
  * Service<Request<Body>>



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



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Struct Next Copy item path

[Source][4]
``` 
pub struct Next { /* private fields */ }
```

Expand description

The remainder of a middleware stack, including the handler.

## Implementations§

[Source][5]§

### impl [Next][6]

[Source][7]

#### pub async fn run(self, req: [Request][8]) -> [Response][9]

Execute the remaining middleware stack.

## Trait Implementations§

[Source][10]§

### impl [Clone][11] for [Next][6]

[Source][10]§

#### fn [clone][12](&self) -> [Next][6]

Returns a duplicate of the value. [Read more][12]

1.0.0 · [Source][13]§

#### fn [clone_from][14](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][14]

[Source][10]§

### impl [Debug][15] for [Next][6]

[Source][10]§

#### fn [fmt][16](&self, f: &mut [Formatter][17]<'_>) -> [Result][18]

Formats the value using the given formatter. [Read more][16]

[Source][19]§

### impl Service<Request<[Body][20]>> for [Next][6]

[Source][21]§

#### type Response = Response<[Body][20]>

Responses given by the service.

[Source][22]§

#### type Error = [Infallible][23]

Errors produced by the service.

[Source][24]§

#### type Future = [Pin][25]<[Box][26]<dyn [Future][27]<Output = [Result][28]<<[Next][6] as Service<Request<[Body][20]>>>::Response, <[Next][6] as Service<Request<[Body][20]>>>::Error>> \+ [Send][29]>>

The future response value.

[Source][30]§

#### fn poll_ready(&mut self, cx: &mut [Context][31]<'_>) -> [Poll][32]<[Result][28]<[()][33], Self::Error>>

Returns `Poll::Ready(Ok(()))` when the service is able to process requests. Read more

[Source][34]§

#### fn call(&mut self, req: [Request][8]) -> Self::Future

Process the request and return the response asynchronously. Read more

## Auto Trait Implementations§

§

### impl [Freeze][35] for [Next][6]

§

### impl ![RefUnwindSafe][36] for [Next][6]

§

### impl [Send][29] for [Next][6]

§

### impl [Sync][37] for [Next][6]

§

### impl [Unpin][38] for [Next][6]

§

### impl ![UnwindSafe][39] for [Next][6]

## Blanket Implementations§

[Source][40]§

### impl<T> [Any][41] for T

where T: 'static + ?[Sized][42],

[Source][43]§

#### fn [type_id][44](&self) -> [TypeId][45]

Gets the `TypeId` of `self`. [Read more][44]

[Source][46]§

### impl<T> [Borrow][47]<T> for T

where T: ?[Sized][42],

[Source][48]§

#### fn [borrow][49](&self) -> [&T][50]

Immutably borrows from an owned value. [Read more][49]

[Source][51]§

### impl<T> [BorrowMut][52]<T> for T

where T: ?[Sized][42],

[Source][53]§

#### fn [borrow_mut][54](&mut self) -> [&mut T][50]

Mutably borrows from an owned value. [Read more][54]

[Source][55]§

### impl<T> [CloneToUninit][56] for T

where T: [Clone][11],

[Source][57]§

#### unsafe fn [clone_to_uninit][58](&self, dest: [*mut ][59][u8][60])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][58]

[Source][61]§

### impl<T> [From][62]<T> for T

[Source][63]§

#### fn [from][64](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][65]<T> for T

where T: [Clone][11],

§

#### fn [from_ref][66](input: [&T][50]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][67] [`Span`][68], returning an `Instrumented` wrapper. Read more

[Source][69]§

### impl<T, U> [Into][70]<U> for T

where U: [From][62]<T>,

[Source][71]§

#### fn [into][72](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][62]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][42],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][73]§

### impl<T> [Same][74] for T

[Source][75]§

#### type [Output][76] = T

Should always be `Self`

[Source][77]§

### impl<S, R> [ServiceExt][78]<R> for S

where S: Service<R>,

[Source][79]§

#### fn [into_make_service][80](self) -> [IntoMakeService][81]<S>

Convert this service into a [`MakeService`][82], that is a [`Service`] whose response is another service. [Read more][80]

[Source][83]§

#### fn [into_make_service_with_connect_info][84]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][85]<S, C>

Available on **crate feature`tokio`** only.

Convert this service into a [`MakeService`][82], that will store `C`’s associated `ConnectInfo` in a request extension such that [`ConnectInfo`][86] can extract it. [Read more][84]

[Source][87]§

#### fn [handle_error][88]<F, T>(self, f: F) -> [HandleError][89]<Self, F, T>

Convert this service into a [`HandleError`][89], that will handle errors by converting them into responses. [Read more][88]

§

### impl<T, Request> ServiceExt<Request> for T

where T: Service<Request> \+ ?[Sized][42],

§

#### fn ready(&mut self) -> Ready<'_, Self, Request>

where Self: [Sized][42],

Yields a mutable reference to the service when it is ready to accept a request.

§

#### fn ready_oneshot(self) -> ReadyOneshot<Self, Request>

where Self: [Sized][42],

Yields the service when it is ready to accept a request.

§

#### fn oneshot(self, req: Request) -> Oneshot<Self, Request>

where Self: [Sized][42],

Consume this `Service`, calling it with the provided request once it is ready.

§

#### fn call_all<S>(self, reqs: S) -> CallAll<Self, S>

where Self: [Sized][42], S: Stream<Item = Request>,

Process all requests from the given [`Stream`][90], and produce a [`Stream`][90] of their responses. Read more

§

#### fn and_then<F>(self, f: F) -> AndThen<Self, F>

where Self: [Sized][42], F: [Clone][11],

Executes a new future after this service’s future resolves. This does not alter the behaviour of the [`poll_ready`][91] method. Read more

§

#### fn map_response<F, Response>(self, f: F) -> MapResponse<Self, F>

where Self: [Sized][42], F: [FnOnce][92](Self::Response) -> Response + [Clone][11],

Maps this service’s response value to a different value. This does not alter the behaviour of the [`poll_ready`][91] method. Read more

§

#### fn map_err<F, Error>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][42], F: [FnOnce][92](Self::Error) -> Error + [Clone][11],

Maps this service’s error value to a different value. This does not alter the behaviour of the [`poll_ready`][91] method. Read more

§

#### fn map_result<F, Response, Error>(self, f: F) -> MapResult<Self, F>

where Self: [Sized][42], Error: [From][62]<Self::Error>, F: [FnOnce][92]([Result][28]<Self::Response, Self::Error>) -> [Result][28]<Response, Error> \+ [Clone][11],

Maps this service’s result type (`Result<Self::Response, Self::Error>`) to a different value, regardless of whether the future succeeds or fails. Read more

§

#### fn map_request<F, NewRequest>(self, f: F) -> MapRequest<Self, F>

where Self: [Sized][42], F: [FnMut][93](NewRequest) -> Request,

Composes a function _in front of_ the service. Read more

§

#### fn filter<F, NewRequest>(self, filter: F) -> Filter<Self, F>

where Self: [Sized][42], F: Predicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with a [`Filter`][94] that conditionally accepts or rejects requests based on a [predicate][95]. Read more

§

#### fn filter_async<F, NewRequest>(self, filter: F) -> AsyncFilter<Self, F>

where Self: [Sized][42], F: AsyncPredicate<NewRequest>,

Available on **crate feature`filter`** only.

Composes this service with an [`AsyncFilter`][96] that conditionally accepts or rejects requests based on an [async predicate]. Read more

§

#### fn then<F, Response, Error, Fut>(self, f: F) -> Then<Self, F>

where Self: [Sized][42], Error: [From][62]<Self::Error>, F: [FnOnce][92]([Result][28]<Self::Response, Self::Error>) -> Fut + [Clone][11], Fut: [Future][27]<Output = [Result][28]<Response, Error>>,

Composes an asynchronous function _after_ this service. Read more

§

#### fn map_future<F, Fut, Response, Error>(self, f: F) -> MapFuture<Self, F>

where Self: [Sized][42], F: [FnMut][93](Self::Future) -> Fut, Error: [From][62]<Self::Error>, Fut: [Future][27]<Output = [Result][28]<Response, Error>>,

Composes a function that transforms futures produced by the service. Read more

§

#### fn boxed(self) -> BoxService<Request, Self::Response, Self::Error>

where Self: [Sized][42] \+ [Send][29] \+ 'static, Self::Future: [Send][29] \+ 'static,

Convert the service into a [`Service`][97] \+ [`Send`][29] trait object. Read more

§

#### fn boxed_clone(self) -> BoxCloneService<Request, Self::Response, Self::Error>

where Self: [Sized][42] \+ [Clone][11] \+ [Send][29] \+ 'static, Self::Future: [Send][29] \+ 'static,

Convert the service into a [`Service`][97] \+ [`Clone`][11] \+ [`Send`][29] trait object. Read more

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][42],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][42],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][98]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][42],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][42],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][42],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][42],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][42],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][42],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][42],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][99] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][100]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][42],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][101] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][100]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][42],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][101] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][100]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][42],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][101] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][42],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][42], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][42], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][42],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][42],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][42],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][102]) -> RequestBodyLimit<Self>

where Self: [Sized][42],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][42],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][42],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][103]§

### impl<T> [ToOwned][104] for T

where T: [Clone][11],

[Source][105]§

#### type [Owned][106] = T

The resulting type after obtaining ownership.

[Source][107]§

#### fn [to_owned][108](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][108]

[Source][109]§

#### fn [clone_into][110](&self, target: [&mut T][50])

Uses borrowed data to replace owned data, usually by cloning. [Read more][110]

[Source][111]§

### impl<T, U> [TryFrom][112]<U> for T

where U: [Into][70]<T>,

[Source][113]§

#### type [Error][114] = [Infallible][23]

The type returned in the event of a conversion error.

[Source][115]§

#### fn [try_from][116](value: U) -> [Result][28]<T, <T as [TryFrom][112]<U>>::[Error][117]>

Performs the conversion.

[Source][118]§

### impl<T, U> [TryInto][119]<U> for T

where U: [TryFrom][112]<T>,

[Source][120]§

#### type [Error][121] = <U as [TryFrom][112]<T>>::[Error][117]

The type returned in the event of a conversion error.

[Source][122]§

#### fn [try_into][123](self) -> [Result][28]<U, <U as [TryFrom][112]<T>>::[Error][117]>

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

where S: [Into][70]<Dispatch>,

Attaches the provided [`Subscriber`][124] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][125] [`Subscriber`][124] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/from_fn.rs.html#338-340
   [5]: ../../src/axum/middleware/from_fn.rs.html#342-350
   [6]: struct.Next.html (struct axum::middleware::Next)
   [7]: ../../src/axum/middleware/from_fn.rs.html#344-349
   [8]: ../extract/type.Request.html (type axum::extract::Request)
   [9]: ../response/type.Response.html (type axum::response::Response)
   [10]: ../../src/axum/middleware/from_fn.rs.html#337
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [13]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [19]: ../../src/axum/middleware/from_fn.rs.html#352-364
   [20]: ../body/struct.Body.html (struct axum::body::Body)
   [21]: ../../src/axum/middleware/from_fn.rs.html#353
   [22]: ../../src/axum/middleware/from_fn.rs.html#354
   [23]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [24]: ../../src/axum/middleware/from_fn.rs.html#355
   [25]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [26]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [27]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [28]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [30]: ../../src/axum/middleware/from_fn.rs.html#357-359
   [31]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [32]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [33]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [34]: ../../src/axum/middleware/from_fn.rs.html#361-363
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [36]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [39]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [40]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [41]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [43]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [44]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [45]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [50]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [55]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [56]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [57]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [58]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [59]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [60]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [65]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [66]: ../extract/trait.FromRef.html#tymethod.from_ref
   [67]: super::Span::current()
   [68]: crate::Span
   [69]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [70]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [71]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [72]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [73]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [74]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [75]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [76]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [77]: ../../src/axum/service_ext.rs.html#47-59
   [78]: ../trait.ServiceExt.html (trait axum::ServiceExt)
   [79]: ../../src/axum/service_ext.rs.html#51-53
   [80]: ../trait.ServiceExt.html#tymethod.into_make_service
   [81]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [82]: tower::make::MakeService
   [83]: ../../src/axum/service_ext.rs.html#56-58
   [84]: ../trait.ServiceExt.html#tymethod.into_make_service_with_connect_info
   [85]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [86]: ../extract/struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [87]: ../../src/axum/service_ext.rs.html#42-44
   [88]: ../trait.ServiceExt.html#method.handle_error
   [89]: ../error_handling/struct.HandleError.html (struct axum::error_handling::HandleError)
   [90]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
   [91]: crate::Service::poll_ready
   [92]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [93]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [94]: crate::filter::Filter
   [95]: crate::filter::Predicate
   [96]: crate::filter::AsyncFilter
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
   [115]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [116]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [117]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [118]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [119]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [120]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [121]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [122]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [123]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [124]: super::Subscriber
   [125]: dispatcher#setting-the-default-subscriber

