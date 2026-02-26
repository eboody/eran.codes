<!-- Generated from rustdoc HTML: middleware/struct.ResponseAxumBodyFuture.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ResponseAxumBodyFuture

## [axum][1]0.8.8

## ResponseAxumBodyFuture

### Trait Implementations

  * Future
  * Unpin



### Auto Trait Implementations

  * Freeze
  * RefUnwindSafe
  * Send
  * Sync
  * UnwindSafe



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



## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Struct ResponseAxumBodyFuture Copy item path

[Source][4]
``` 
pub struct ResponseAxumBodyFuture<Fut> { /* private fields */ }
```

Expand description

Response future for [`ResponseAxumBody`][5].

## Trait Implementations§

[Source][6]§

### impl<Fut, ResBody, E> [Future][7] for [ResponseAxumBodyFuture][8]<Fut>

where Fut: [Future][7]<Output = [Result][9]<[Response][10]<ResBody>, E>>, ResBody: HttpBody<Data = Bytes> \+ [Send][11] \+ 'static, <ResBody as HttpBody>::Error: [Error][12] \+ [Send][11] \+ [Sync][13],

[Source][14]§

#### type [Output][15] = [Result][9]<Response<[Body][16]>, E>

The type of value produced on completion.

[Source][17]§

#### fn [poll][18](self: [Pin][19]<&mut Self>, cx: &mut [Context][20]<'_>) -> [Poll][21]<Self::[Output][22]>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available. [Read more][18]

[Source][4]§

### impl<'__pin, Fut> [Unpin][23] for [ResponseAxumBodyFuture][8]<Fut>

where PinnedFieldsOf<__Origin<'__pin, Fut>>: [Unpin][23],

## Auto Trait Implementations§

§

### impl<Fut> [Freeze][24] for [ResponseAxumBodyFuture][8]<Fut>

where Fut: [Freeze][24],

§

### impl<Fut> [RefUnwindSafe][25] for [ResponseAxumBodyFuture][8]<Fut>

where Fut: [RefUnwindSafe][25],

§

### impl<Fut> [Send][11] for [ResponseAxumBodyFuture][8]<Fut>

where Fut: [Send][11],

§

### impl<Fut> [Sync][13] for [ResponseAxumBodyFuture][8]<Fut>

where Fut: [Sync][13],

§

### impl<Fut> [UnwindSafe][26] for [ResponseAxumBodyFuture][8]<Fut>

where Fut: [UnwindSafe][26],

## Blanket Implementations§

[Source][27]§

### impl<T> [Any][28] for T

where T: 'static + ?[Sized][29],

[Source][30]§

#### fn [type_id][31](&self) -> [TypeId][32]

Gets the `TypeId` of `self`. [Read more][31]

[Source][33]§

### impl<T> [Borrow][34]<T> for T

where T: ?[Sized][29],

[Source][35]§

#### fn [borrow][36](&self) -> [&T][37]

Immutably borrows from an owned value. [Read more][36]

[Source][38]§

### impl<T> [BorrowMut][39]<T> for T

where T: ?[Sized][29],

[Source][40]§

#### fn [borrow_mut][41](&mut self) -> [&mut T][37]

Mutably borrows from an owned value. [Read more][41]

[Source][42]§

### impl<T> [From][43]<T> for T

[Source][44]§

#### fn [from][45](t: T) -> T

Returns the argument unchanged.

§

### impl<T> FutureExt for T

where T: [Future][7] \+ ?[Sized][29],

§

#### fn map<U, F>(self, f: F) -> Map<Self, F>

where F: [FnOnce][46](Self::[Output][22]) -> U, Self: [Sized][29],

Map this future’s output to a different type, returning a new future of the resulting type. Read more

§

#### fn map_into<U>(self) -> MapInto<Self, U>

where Self::[Output][22]: [Into][47]<U>, Self: [Sized][29],

Map this future’s output to a different type, returning a new future of the resulting type. Read more

§

#### fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>

where F: [FnOnce][46](Self::[Output][22]) -> Fut, Fut: [Future][7], Self: [Sized][29],

Chain on a computation for when a future finished, passing the result of the future to the provided closure `f`. Read more

§

#### fn left_future<B>(self) -> Either<Self, B>

where B: [Future][7]<Output = Self::[Output][22]>, Self: [Sized][29],

Wrap this future in an `Either` future, making it the left-hand variant of that `Either`. Read more

§

#### fn right_future<A>(self) -> Either<A, Self>

where A: [Future][7]<Output = Self::[Output][22]>, Self: [Sized][29],

Wrap this future in an `Either` future, making it the right-hand variant of that `Either`. Read more

§

#### fn into_stream(self) -> IntoStream<Self>

where Self: [Sized][29],

Convert this future into a single element stream. Read more

§

#### fn flatten(self) -> Flatten<Self>

where Self::[Output][22]: [Future][7], Self: [Sized][29],

Flatten the execution of this future when the output of this future is itself another future. Read more

§

#### fn flatten_stream(self) -> FlattenStream<Self>

where Self::[Output][22]: Stream, Self: [Sized][29],

Flatten the execution of this future when the successful result of this future is a stream. Read more

§

#### fn fuse(self) -> Fuse<Self>

where Self: [Sized][29],

Fuse a future such that `poll` will never again be called once it has completed. This method can be used to turn any `Future` into a `FusedFuture`. Read more

§

#### fn inspect<F>(self, f: F) -> Inspect<Self, F>

where F: [FnOnce][46](&Self::[Output][22]), Self: [Sized][29],

Do something with the output of a future before passing it on. Read more

§

#### fn catch_unwind(self) -> CatchUnwind<Self>

where Self: [Sized][29] \+ [UnwindSafe][26],

Available on **crate feature`std`** only.

Catches unwinding panics while polling the future. Read more

§

#### fn shared(self) -> Shared<Self>

where Self: [Sized][29], Self::[Output][22]: [Clone][48],

Available on **crate feature`std`** only.

Create a cloneable handle to this future where all handles will resolve to the same result. Read more

§

#### fn boxed<'a>(self) -> [Pin][19]<[Box][49]<dyn [Future][7]<Output = Self::[Output][22]> \+ [Send][11] \+ 'a>>

where Self: [Sized][29] \+ [Send][11] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the future in a Box, pinning it. Read more

§

#### fn boxed_local<'a>(self) -> [Pin][19]<[Box][49]<dyn [Future][7]<Output = Self::[Output][22]> \+ 'a>>

where Self: [Sized][29] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the future in a Box, pinning it. Read more

§

#### fn unit_error(self) -> UnitError<Self>

where Self: [Sized][29],

Turns a [`Future<Output = T>`][7] into a [`TryFuture<Ok = T, Error = ()`>][50].

§

#### fn never_error(self) -> NeverError<Self>

where Self: [Sized][29],

Turns a [`Future<Output = T>`][7] into a [`TryFuture<Ok = T, Error = Never`>][50].

§

#### fn poll_unpin(&mut self, cx: &mut [Context][20]<'_>) -> [Poll][21]<Self::[Output][22]>

where Self: [Unpin][23],

A convenience for calling `Future::poll` on `Unpin` future types.

§

#### fn now_or_never(self) -> [Option][51]<Self::[Output][22]>

where Self: [Sized][29],

Evaluates and consumes the future, returning the resulting output if the future is ready after the first call to `Future::poll`. Read more

§

### impl<T> FutureExt for T

where T: [Future][7] \+ ?[Sized][29],

§

#### fn with_cancellation_token( self, cancellation_token: &CancellationToken, ) -> WithCancellationTokenFuture<'_, Self>

where Self: [Sized][29],

Similar to [`CancellationToken::run_until_cancelled`], but with the advantage that it is easier to write fluent call chains. Read more

§

#### fn with_cancellation_token_owned( self, cancellation_token: CancellationToken, ) -> WithCancellationTokenFutureOwned<Self>

where Self: [Sized][29],

Similar to [`CancellationToken::run_until_cancelled_owned`], but with the advantage that it is easier to write fluent call chains. Read more

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][52] [`Span`][53], returning an `Instrumented` wrapper. Read more

[Source][54]§

### impl<T, U> [Into][47]<U> for T

where U: [From][43]<T>,

[Source][55]§

#### fn [into][56](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][43]<T> for U` chooses to do.

[Source][57]§

### impl<F> [IntoFuture][58] for F

where F: [Future][7],

[Source][59]§

#### type [Output][60] = <F as [Future][7]>::[Output][22]

The output that the future will produce on completion.

[Source][61]§

#### type [IntoFuture][62] = F

Which kind of future are we turning this into?

[Source][63]§

#### fn [into_future][64](self) -> <F as [IntoFuture][58]>::[IntoFuture][65]

Creates a future from a value. [Read more][64]

§

### impl<T> PolicyExt for T

where T: ?[Sized][29],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][66]§

### impl<T> [Same][67] for T

[Source][68]§

#### type [Output][69] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][29],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][29],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][70]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][29],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][29],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][29],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][29],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][29],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][29],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][29],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][71] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][29],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][29],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][29],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][29],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][29], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][29], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][29],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][29],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][29],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][74]) -> RequestBodyLimit<Self>

where Self: [Sized][29],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][29],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][29],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][75]§

### impl<T, U> [TryFrom][76]<U> for T

where U: [Into][47]<T>,

[Source][77]§

#### type [Error][78] = [Infallible][79]

The type returned in the event of a conversion error.

[Source][80]§

#### fn [try_from][81](value: U) -> [Result][9]<T, <T as [TryFrom][76]<U>>::[Error][82]>

Performs the conversion.

§

### impl<F, T, E> TryFuture for F

where F: [Future][7]<Output = [Result][9]<T, E>> \+ ?[Sized][29],

§

#### type Ok = T

The type of successful values yielded by this future

§

#### type Error = E

The type of failures yielded by this future

§

#### fn try_poll( self: [Pin][19]<[&mut F][37]>, cx: &mut [Context][20]<'_>, ) -> [Poll][21]<<F as [Future][7]>::[Output][22]>

Poll this `TryFuture` as if it were a `Future`. Read more

§

### impl<Fut> TryFutureExt for Fut

where Fut: TryFuture + ?[Sized][29],

§

#### fn flatten_sink<Item>(self) -> FlattenSink<Self, Self::Ok>

where Self::Ok: Sink<Item, Error = Self::Error>, Self: [Sized][29],

Available on **crate feature`sink`** only.

Flattens the execution of this future when the successful result of this future is a [`Sink`]. Read more

§

#### fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>

where F: [FnOnce][46](Self::Ok) -> T, Self: [Sized][29],

Maps this future’s success value to a different value. Read more

§

#### fn map_ok_or_else<T, E, F>(self, e: E, f: F) -> MapOkOrElse<Self, F, E>

where F: [FnOnce][46](Self::Ok) -> T, E: [FnOnce][46](Self::Error) -> T, Self: [Sized][29],

Maps this future’s success value to a different value, and permits for error handling resulting in the same type. Read more

§

#### fn map_err<E, F>(self, f: F) -> MapErr<Self, F>

where F: [FnOnce][46](Self::Error) -> E, Self: [Sized][29],

Maps this future’s error value to a different value. Read more

§

#### fn err_into<E>(self) -> ErrInto<Self, E>

where Self: [Sized][29], Self::Error: [Into][47]<E>,

Maps this future’s [`Error`][83] to a new error type using the [`Into`][47] trait. Read more

§

#### fn ok_into<U>(self) -> OkInto<Self, U>

where Self: [Sized][29], Self::Ok: [Into][47]<U>,

Maps this future’s [`Ok`][84] to a new type using the [`Into`][47] trait.

§

#### fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>

where F: [FnOnce][46](Self::Ok) -> Fut, Fut: TryFuture<Error = Self::Error>, Self: [Sized][29],

Executes another future after this one resolves successfully. The success value is passed to a closure to create this subsequent future. Read more

§

#### fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>

where F: [FnOnce][46](Self::Error) -> Fut, Fut: TryFuture<Ok = Self::Ok>, Self: [Sized][29],

Executes another future if this one resolves to an error. The error value is passed to a closure to create this subsequent future. Read more

§

#### fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>

where F: [FnOnce][46](&Self::Ok), Self: [Sized][29],

Do something with the success value of a future before passing it on. Read more

§

#### fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>

where F: [FnOnce][46](&Self::Error), Self: [Sized][29],

Do something with the error value of a future before passing it on. Read more

§

#### fn try_flatten(self) -> TryFlatten<Self, Self::Ok>

where Self::Ok: TryFuture<Error = Self::Error>, Self: [Sized][29],

Flatten the execution of this future when the successful result of this future is another future. Read more

§

#### fn try_flatten_stream(self) -> TryFlattenStream<Self>

where Self::Ok: TryStream<Error = Self::Error>, Self: [Sized][29],

Flatten the execution of this future when the successful result of this future is a stream. Read more

§

#### fn unwrap_or_else<F>(self, f: F) -> UnwrapOrElse<Self, F>

where Self: [Sized][29], F: [FnOnce][46](Self::Error) -> Self::Ok,

Unwraps this future’s output, producing a future with this future’s [`Ok`][84] type as its [`Output`][85] type. Read more

§

#### fn into_future(self) -> IntoFuture<Self>

where Self: [Sized][29],

Wraps a [`TryFuture`] into a type that implements [`Future`][7]. Read more

§

#### fn try_poll_unpin( &mut self, cx: &mut [Context][20]<'_>, ) -> [Poll][21]<[Result][9]<Self::Ok, Self::Error>>

where Self: [Unpin][23],

A convenience method for calling [`TryFuture::try_poll`] on [`Unpin`][23] future types.

[Source][86]§

### impl<T, U> [TryInto][87]<U> for T

where U: [TryFrom][76]<T>,

[Source][88]§

#### type [Error][89] = <U as [TryFrom][76]<T>>::[Error][82]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_into][91](self) -> [Result][9]<U, <U as [TryFrom][76]<T>>::[Error][82]>

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

where S: [Into][47]<Dispatch>,

Attaches the provided [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][93] [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/response_axum_body.rs.html#55-61
   [5]: struct.ResponseAxumBody.html (struct axum::middleware::ResponseAxumBody)
   [6]: ../../src/axum/middleware/response_axum_body.rs.html#63-76
   [7]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [8]: struct.ResponseAxumBodyFuture.html (struct axum::middleware::ResponseAxumBodyFuture)
   [9]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [10]: ../response/type.Response.html (type axum::response::Response)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [12]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [13]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [14]: ../../src/axum/middleware/response_axum_body.rs.html#69
   [15]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output
   [16]: ../body/struct.Body.html (struct axum::body::Body)
   [17]: ../../src/axum/middleware/response_axum_body.rs.html#71-75
   [18]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll
   [19]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [20]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [21]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [22]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (type core::future::future::Future::Output)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [25]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [26]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [27]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [28]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [30]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [31]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [32]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [33]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [34]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [35]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [36]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [37]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [38]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [39]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [42]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [43]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [44]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [45]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [46]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [47]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [48]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [49]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [50]: futures_core::future::TryFuture
   [51]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [52]: super::Span::current()
   [53]: crate::Span
   [54]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [55]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [56]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [57]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#138
   [58]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html (trait core::future::into_future::IntoFuture)
   [59]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#139
   [60]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.Output
   [61]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#140
   [62]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture
   [63]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#142
   [64]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future
   [65]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture (type core::future::into_future::IntoFuture::IntoFuture)
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [70]: https://docs.rs/http/latest/http/struct.Extensions.html
   [71]: crate::follow_redirect::policy::Standard
   [72]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [73]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [74]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [79]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [83]: TryFuture::Error
   [84]: TryFuture::Ok
   [85]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (associated type core::future::future::Future::Output)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [92]: super::Subscriber
   [93]: dispatcher#setting-the-default-subscriber

