<!-- Generated from rustdoc HTML: middleware/future/struct.MapResponseResponseFuture.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## MapResponseResponseFuture

## [axum][1]0.8.8

## MapResponseResponseFuture

### Trait Implementations

  * Debug
  * Future



### Auto Trait Implementations

  * !RefUnwindSafe
  * !Sync
  * !UnwindSafe
  * Freeze
  * Send
  * Unpin



### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * From<T>
  * FutureExt
  * FutureExt
  * Instrument
  * Into<U>
  * IntoFuture
  * PolicyExt
  * Same
  * ServiceExt
  * TryFrom<U>
  * TryFuture
  * TryFutureExt
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::middleware::future][2]

[axum][3]::[middleware][4]::[future][2]

# Struct MapResponseResponseFuture Copy item path

[Source][5]
``` 
pub struct MapResponseResponseFuture { /* private fields */ }
```

Expand description

Response future for [`MapResponse`][6].

## Trait Implementations§

[Source][7]§

### impl [Debug][8] for [ResponseFuture][9]

[Source][10]§

#### fn [fmt][11](&self, f: &mut [Formatter][12]<'_>) -> [Result][13]

Formats the value using the given formatter. [Read more][11]

[Source][14]§

### impl [Future][15] for [ResponseFuture][9]

[Source][16]§

#### type [Output][17] = [Result][18]<Response<[Body][19]>, [Infallible][20]>

The type of value produced on completion.

[Source][21]§

#### fn [poll][22](self: [Pin][23]<&mut Self>, cx: &mut [Context][24]<'_>) -> [Poll][25]<Self::[Output][26]>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available. [Read more][22]

## Auto Trait Implementations§

§

### impl [Freeze][27] for [ResponseFuture][9]

§

### impl ![RefUnwindSafe][28] for [ResponseFuture][9]

§

### impl [Send][29] for [ResponseFuture][9]

§

### impl ![Sync][30] for [ResponseFuture][9]

§

### impl [Unpin][31] for [ResponseFuture][9]

§

### impl ![UnwindSafe][32] for [ResponseFuture][9]

## Blanket Implementations§

[Source][33]§

### impl<T> [Any][34] for T

where T: 'static + ?[Sized][35],

[Source][36]§

#### fn [type_id][37](&self) -> [TypeId][38]

Gets the `TypeId` of `self`. [Read more][37]

[Source][39]§

### impl<T> [Borrow][40]<T> for T

where T: ?[Sized][35],

[Source][41]§

#### fn [borrow][42](&self) -> [&T][43]

Immutably borrows from an owned value. [Read more][42]

[Source][44]§

### impl<T> [BorrowMut][45]<T> for T

where T: ?[Sized][35],

[Source][46]§

#### fn [borrow_mut][47](&mut self) -> [&mut T][43]

Mutably borrows from an owned value. [Read more][47]

[Source][48]§

### impl<T> [From][49]<T> for T

[Source][50]§

#### fn [from][51](t: T) -> T

Returns the argument unchanged.

§

### impl<T> FutureExt for T

where T: [Future][15] \+ ?[Sized][35],

§

#### fn map<U, F>(self, f: F) -> Map<Self, F>

where F: [FnOnce][52](Self::[Output][26]) -> U, Self: [Sized][35],

Map this future’s output to a different type, returning a new future of the resulting type. Read more

§

#### fn map_into<U>(self) -> MapInto<Self, U>

where Self::[Output][26]: [Into][53]<U>, Self: [Sized][35],

Map this future’s output to a different type, returning a new future of the resulting type. Read more

§

#### fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>

where F: [FnOnce][52](Self::[Output][26]) -> Fut, Fut: [Future][15], Self: [Sized][35],

Chain on a computation for when a future finished, passing the result of the future to the provided closure `f`. Read more

§

#### fn left_future<B>(self) -> Either<Self, B>

where B: [Future][15]<Output = Self::[Output][26]>, Self: [Sized][35],

Wrap this future in an `Either` future, making it the left-hand variant of that `Either`. Read more

§

#### fn right_future<A>(self) -> Either<A, Self>

where A: [Future][15]<Output = Self::[Output][26]>, Self: [Sized][35],

Wrap this future in an `Either` future, making it the right-hand variant of that `Either`. Read more

§

#### fn into_stream(self) -> IntoStream<Self>

where Self: [Sized][35],

Convert this future into a single element stream. Read more

§

#### fn flatten(self) -> Flatten<Self>

where Self::[Output][26]: [Future][15], Self: [Sized][35],

Flatten the execution of this future when the output of this future is itself another future. Read more

§

#### fn flatten_stream(self) -> FlattenStream<Self>

where Self::[Output][26]: Stream, Self: [Sized][35],

Flatten the execution of this future when the successful result of this future is a stream. Read more

§

#### fn fuse(self) -> Fuse<Self>

where Self: [Sized][35],

Fuse a future such that `poll` will never again be called once it has completed. This method can be used to turn any `Future` into a `FusedFuture`. Read more

§

#### fn inspect<F>(self, f: F) -> Inspect<Self, F>

where F: [FnOnce][52](&Self::[Output][26]), Self: [Sized][35],

Do something with the output of a future before passing it on. Read more

§

#### fn catch_unwind(self) -> CatchUnwind<Self>

where Self: [Sized][35] \+ [UnwindSafe][32],

Available on **crate feature`std`** only.

Catches unwinding panics while polling the future. Read more

§

#### fn shared(self) -> Shared<Self>

where Self: [Sized][35], Self::[Output][26]: [Clone][54],

Available on **crate feature`std`** only.

Create a cloneable handle to this future where all handles will resolve to the same result. Read more

§

#### fn boxed<'a>(self) -> [Pin][23]<[Box][55]<dyn [Future][15]<Output = Self::[Output][26]> \+ [Send][29] \+ 'a>>

where Self: [Sized][35] \+ [Send][29] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the future in a Box, pinning it. Read more

§

#### fn boxed_local<'a>(self) -> [Pin][23]<[Box][55]<dyn [Future][15]<Output = Self::[Output][26]> \+ 'a>>

where Self: [Sized][35] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the future in a Box, pinning it. Read more

§

#### fn unit_error(self) -> UnitError<Self>

where Self: [Sized][35],

Turns a [`Future<Output = T>`][15] into a [`TryFuture<Ok = T, Error = ()`>][56].

§

#### fn never_error(self) -> NeverError<Self>

where Self: [Sized][35],

Turns a [`Future<Output = T>`][15] into a [`TryFuture<Ok = T, Error = Never`>][56].

§

#### fn poll_unpin(&mut self, cx: &mut [Context][24]<'_>) -> [Poll][25]<Self::[Output][26]>

where Self: [Unpin][31],

A convenience for calling `Future::poll` on `Unpin` future types.

§

#### fn now_or_never(self) -> [Option][57]<Self::[Output][26]>

where Self: [Sized][35],

Evaluates and consumes the future, returning the resulting output if the future is ready after the first call to `Future::poll`. Read more

§

### impl<T> FutureExt for T

where T: [Future][15] \+ ?[Sized][35],

§

#### fn with_cancellation_token( self, cancellation_token: &CancellationToken, ) -> WithCancellationTokenFuture<'_, Self>

where Self: [Sized][35],

Similar to [`CancellationToken::run_until_cancelled`], but with the advantage that it is easier to write fluent call chains. Read more

§

#### fn with_cancellation_token_owned( self, cancellation_token: CancellationToken, ) -> WithCancellationTokenFutureOwned<Self>

where Self: [Sized][35],

Similar to [`CancellationToken::run_until_cancelled_owned`], but with the advantage that it is easier to write fluent call chains. Read more

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][58] [`Span`][59], returning an `Instrumented` wrapper. Read more

[Source][60]§

### impl<T, U> [Into][53]<U> for T

where U: [From][49]<T>,

[Source][61]§

#### fn [into][62](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][49]<T> for U` chooses to do.

[Source][63]§

### impl<F> [IntoFuture][64] for F

where F: [Future][15],

[Source][65]§

#### type [Output][66] = <F as [Future][15]>::[Output][26]

The output that the future will produce on completion.

[Source][67]§

#### type [IntoFuture][68] = F

Which kind of future are we turning this into?

[Source][69]§

#### fn [into_future][70](self) -> <F as [IntoFuture][64]>::[IntoFuture][71]

Creates a future from a value. [Read more][70]

§

### impl<T> PolicyExt for T

where T: ?[Sized][35],

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

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][35],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][35],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][76]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][35],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][35],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][35],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][35],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][35],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][35],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][35],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][77] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][35],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][35],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][78]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][35],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][79] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][35],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][35], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][35], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][35],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][35],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][35],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][80]) -> RequestBodyLimit<Self>

where Self: [Sized][35],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][35],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][35],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][81]§

### impl<T, U> [TryFrom][82]<U> for T

where U: [Into][53]<T>,

[Source][83]§

#### type [Error][84] = [Infallible][20]

The type returned in the event of a conversion error.

[Source][85]§

#### fn [try_from][86](value: U) -> [Result][18]<T, <T as [TryFrom][82]<U>>::[Error][87]>

Performs the conversion.

§

### impl<F, T, E> TryFuture for F

where F: [Future][15]<Output = [Result][18]<T, E>> \+ ?[Sized][35],

§

#### type Ok = T

The type of successful values yielded by this future

§

#### type Error = E

The type of failures yielded by this future

§

#### fn try_poll( self: [Pin][23]<[&mut F][43]>, cx: &mut [Context][24]<'_>, ) -> [Poll][25]<<F as [Future][15]>::[Output][26]>

Poll this `TryFuture` as if it were a `Future`. Read more

§

### impl<Fut> TryFutureExt for Fut

where Fut: TryFuture + ?[Sized][35],

§

#### fn flatten_sink<Item>(self) -> FlattenSink<Self, Self::Ok>

where Self::Ok: Sink<Item, Error = Self::Error>, Self: [Sized][35],

Available on **crate feature`sink`** only.

Flattens the execution of this future when the successful result of this future is a [`Sink`]. Read more

§

#### fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>

where F: [FnOnce][52](Self::Ok) -> T, Self: [Sized][35],

Maps this future’s success value to a different value. Read more

§

#### fn map_ok_or_else<T, E, F>(self, e: E, f: F) -> MapOkOrElse<Self, F, E>

where F: [FnOnce][52](Self::Ok) -> T, E: [FnOnce][52](Self::Error) -> T, Self: [Sized][35],

Maps this future’s success value to a different value, and permits for error handling resulting in the same type. Read more

§

#### fn map_err<E, F>(self, f: F) -> MapErr<Self, F>

where F: [FnOnce][52](Self::Error) -> E, Self: [Sized][35],

Maps this future’s error value to a different value. Read more

§

#### fn err_into<E>(self) -> ErrInto<Self, E>

where Self: [Sized][35], Self::Error: [Into][53]<E>,

Maps this future’s [`Error`][88] to a new error type using the [`Into`][53] trait. Read more

§

#### fn ok_into<U>(self) -> OkInto<Self, U>

where Self: [Sized][35], Self::Ok: [Into][53]<U>,

Maps this future’s [`Ok`][89] to a new type using the [`Into`][53] trait.

§

#### fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>

where F: [FnOnce][52](Self::Ok) -> Fut, Fut: TryFuture<Error = Self::Error>, Self: [Sized][35],

Executes another future after this one resolves successfully. The success value is passed to a closure to create this subsequent future. Read more

§

#### fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>

where F: [FnOnce][52](Self::Error) -> Fut, Fut: TryFuture<Ok = Self::Ok>, Self: [Sized][35],

Executes another future if this one resolves to an error. The error value is passed to a closure to create this subsequent future. Read more

§

#### fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>

where F: [FnOnce][52](&Self::Ok), Self: [Sized][35],

Do something with the success value of a future before passing it on. Read more

§

#### fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>

where F: [FnOnce][52](&Self::Error), Self: [Sized][35],

Do something with the error value of a future before passing it on. Read more

§

#### fn try_flatten(self) -> TryFlatten<Self, Self::Ok>

where Self::Ok: TryFuture<Error = Self::Error>, Self: [Sized][35],

Flatten the execution of this future when the successful result of this future is another future. Read more

§

#### fn try_flatten_stream(self) -> TryFlattenStream<Self>

where Self::Ok: TryStream<Error = Self::Error>, Self: [Sized][35],

Flatten the execution of this future when the successful result of this future is a stream. Read more

§

#### fn unwrap_or_else<F>(self, f: F) -> UnwrapOrElse<Self, F>

where Self: [Sized][35], F: [FnOnce][52](Self::Error) -> Self::Ok,

Unwraps this future’s output, producing a future with this future’s [`Ok`][89] type as its [`Output`][90] type. Read more

§

#### fn into_future(self) -> IntoFuture<Self>

where Self: [Sized][35],

Wraps a [`TryFuture`] into a type that implements [`Future`][15]. Read more

§

#### fn try_poll_unpin( &mut self, cx: &mut [Context][24]<'_>, ) -> [Poll][25]<[Result][18]<Self::Ok, Self::Error>>

where Self: [Unpin][31],

A convenience method for calling [`TryFuture::try_poll`] on [`Unpin`][31] future types.

[Source][91]§

### impl<T, U> [TryInto][92]<U> for T

where U: [TryFrom][82]<T>,

[Source][93]§

#### type [Error][94] = <U as [TryFrom][82]<T>>::[Error][87]

The type returned in the event of a conversion error.

[Source][95]§

#### fn [try_into][96](self) -> [Result][18]<U, <U as [TryFrom][82]<T>>::[Error][87]>

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

where S: [Into][53]<Dispatch>,

Attaches the provided [`Subscriber`][97] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][98] [`Subscriber`][97] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/middleware/map_response.rs.html#325-327
   [6]: ../struct.MapResponse.html (struct axum::middleware::MapResponse)
   [7]: ../../../src/axum/middleware/map_response.rs.html#337-341
   [8]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [9]: struct.MapResponseResponseFuture.html (struct axum::middleware::future::MapResponseResponseFuture)
   [10]: ../../../src/axum/middleware/map_response.rs.html#338-340
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [12]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [14]: ../../../src/axum/middleware/map_response.rs.html#329-335
   [15]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [16]: ../../../src/axum/middleware/map_response.rs.html#330
   [17]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output
   [18]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [19]: ../../body/struct.Body.html (struct axum::body::Body)
   [20]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [21]: ../../../src/axum/middleware/map_response.rs.html#332-334
   [22]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll
   [23]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [24]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [25]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [26]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (type core::future::future::Future::Output)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [28]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [32]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [33]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [34]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [36]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [37]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [38]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [39]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [40]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [41]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [42]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [43]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [48]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [49]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [50]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [51]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [52]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [54]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [55]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [56]: futures_core::future::TryFuture
   [57]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [58]: super::Span::current()
   [59]: crate::Span
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [63]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#138
   [64]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html (trait core::future::into_future::IntoFuture)
   [65]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#139
   [66]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.Output
   [67]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#140
   [68]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture
   [69]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#142
   [70]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future
   [71]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture (type core::future::into_future::IntoFuture::IntoFuture)
   [72]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [73]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [74]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [75]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [76]: https://docs.rs/http/latest/http/struct.Extensions.html
   [77]: crate::follow_redirect::policy::Standard
   [78]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [79]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [80]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [88]: TryFuture::Error
   [89]: TryFuture::Ok
   [90]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (associated type core::future::future::Future::Output)
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [95]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [96]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [97]: super::Subscriber
   [98]: dispatcher#setting-the-default-subscriber

