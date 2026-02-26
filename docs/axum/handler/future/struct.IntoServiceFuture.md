<!-- Generated from rustdoc HTML: handler/future/struct.IntoServiceFuture.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IntoServiceFuture

## [axum][1]0.8.8

## IntoServiceFuture

### Trait Implementations

  * Debug
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



## [In axum::handler::future][2]

[axum][3]::[handler][4]::[future][2]

# Struct IntoServiceFuture Copy item path

[Source][5]
``` 
pub struct IntoServiceFuture<F> { /* private fields */ }
```

Expand description

The response future for [`IntoService`][6].

## Trait Implementations§

[Source][5]§

### impl<F> [Debug][7] for [IntoServiceFuture][8]<F>

[Source][5]§

#### fn [fmt][9](&self, f: &mut [Formatter][10]<'_>) -> [Result][11]

Formats the value using the given formatter. [Read more][9]

[Source][5]§

### impl<F> [Future][12] for [IntoServiceFuture][8]<F>

where Map<F, [fn][13]([Response][14]) -> [Result][15]<[Response][14], [Infallible][16]>>: [Future][12],

[Source][5]§

#### type [Output][17] = <Map<F, [fn][13](Response<[Body][18]>) -> [Result][15]<Response<[Body][18]>, [Infallible][16]>> as [Future][12]>::[Output][19]

The type of value produced on completion.

[Source][5]§

#### fn [poll][20](self: [Pin][21]<&mut Self>, cx: &mut [Context][22]<'_>) -> [Poll][23]<Self::[Output][19]>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available. [Read more][20]

[Source][5]§

### impl<'__pin, F> [Unpin][24] for [IntoServiceFuture][8]<F>

where PinnedFieldsOf<__Origin<'__pin, F>>: [Unpin][24],

## Auto Trait Implementations§

§

### impl<F> [Freeze][25] for [IntoServiceFuture][8]<F>

where F: [Freeze][25],

§

### impl<F> [RefUnwindSafe][26] for [IntoServiceFuture][8]<F>

where F: [RefUnwindSafe][26],

§

### impl<F> [Send][27] for [IntoServiceFuture][8]<F>

where F: [Send][27],

§

### impl<F> [Sync][28] for [IntoServiceFuture][8]<F>

where F: [Sync][28],

§

### impl<F> [UnwindSafe][29] for [IntoServiceFuture][8]<F>

where F: [UnwindSafe][29],

## Blanket Implementations§

[Source][30]§

### impl<T> [Any][31] for T

where T: 'static + ?[Sized][32],

[Source][33]§

#### fn [type_id][34](&self) -> [TypeId][35]

Gets the `TypeId` of `self`. [Read more][34]

[Source][36]§

### impl<T> [Borrow][37]<T> for T

where T: ?[Sized][32],

[Source][38]§

#### fn [borrow][39](&self) -> [&T][40]

Immutably borrows from an owned value. [Read more][39]

[Source][41]§

### impl<T> [BorrowMut][42]<T> for T

where T: ?[Sized][32],

[Source][43]§

#### fn [borrow_mut][44](&mut self) -> [&mut T][40]

Mutably borrows from an owned value. [Read more][44]

[Source][45]§

### impl<T> [From][46]<T> for T

[Source][47]§

#### fn [from][48](t: T) -> T

Returns the argument unchanged.

§

### impl<T> FutureExt for T

where T: [Future][12] \+ ?[Sized][32],

§

#### fn map<U, F>(self, f: F) -> Map<Self, F>

where F: [FnOnce][49](Self::[Output][19]) -> U, Self: [Sized][32],

Map this future’s output to a different type, returning a new future of the resulting type. Read more

§

#### fn map_into<U>(self) -> MapInto<Self, U>

where Self::[Output][19]: [Into][50]<U>, Self: [Sized][32],

Map this future’s output to a different type, returning a new future of the resulting type. Read more

§

#### fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>

where F: [FnOnce][49](Self::[Output][19]) -> Fut, Fut: [Future][12], Self: [Sized][32],

Chain on a computation for when a future finished, passing the result of the future to the provided closure `f`. Read more

§

#### fn left_future<B>(self) -> Either<Self, B>

where B: [Future][12]<Output = Self::[Output][19]>, Self: [Sized][32],

Wrap this future in an `Either` future, making it the left-hand variant of that `Either`. Read more

§

#### fn right_future<A>(self) -> Either<A, Self>

where A: [Future][12]<Output = Self::[Output][19]>, Self: [Sized][32],

Wrap this future in an `Either` future, making it the right-hand variant of that `Either`. Read more

§

#### fn into_stream(self) -> IntoStream<Self>

where Self: [Sized][32],

Convert this future into a single element stream. Read more

§

#### fn flatten(self) -> Flatten<Self>

where Self::[Output][19]: [Future][12], Self: [Sized][32],

Flatten the execution of this future when the output of this future is itself another future. Read more

§

#### fn flatten_stream(self) -> FlattenStream<Self>

where Self::[Output][19]: Stream, Self: [Sized][32],

Flatten the execution of this future when the successful result of this future is a stream. Read more

§

#### fn fuse(self) -> Fuse<Self>

where Self: [Sized][32],

Fuse a future such that `poll` will never again be called once it has completed. This method can be used to turn any `Future` into a `FusedFuture`. Read more

§

#### fn inspect<F>(self, f: F) -> Inspect<Self, F>

where F: [FnOnce][49](&Self::[Output][19]), Self: [Sized][32],

Do something with the output of a future before passing it on. Read more

§

#### fn catch_unwind(self) -> CatchUnwind<Self>

where Self: [Sized][32] \+ [UnwindSafe][29],

Available on **crate feature`std`** only.

Catches unwinding panics while polling the future. Read more

§

#### fn shared(self) -> Shared<Self>

where Self: [Sized][32], Self::[Output][19]: [Clone][51],

Available on **crate feature`std`** only.

Create a cloneable handle to this future where all handles will resolve to the same result. Read more

§

#### fn boxed<'a>(self) -> [Pin][21]<[Box][52]<dyn [Future][12]<Output = Self::[Output][19]> \+ [Send][27] \+ 'a>>

where Self: [Sized][32] \+ [Send][27] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the future in a Box, pinning it. Read more

§

#### fn boxed_local<'a>(self) -> [Pin][21]<[Box][52]<dyn [Future][12]<Output = Self::[Output][19]> \+ 'a>>

where Self: [Sized][32] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the future in a Box, pinning it. Read more

§

#### fn unit_error(self) -> UnitError<Self>

where Self: [Sized][32],

Turns a [`Future<Output = T>`][12] into a [`TryFuture<Ok = T, Error = ()`>][53].

§

#### fn never_error(self) -> NeverError<Self>

where Self: [Sized][32],

Turns a [`Future<Output = T>`][12] into a [`TryFuture<Ok = T, Error = Never`>][53].

§

#### fn poll_unpin(&mut self, cx: &mut [Context][22]<'_>) -> [Poll][23]<Self::[Output][19]>

where Self: [Unpin][24],

A convenience for calling `Future::poll` on `Unpin` future types.

§

#### fn now_or_never(self) -> [Option][54]<Self::[Output][19]>

where Self: [Sized][32],

Evaluates and consumes the future, returning the resulting output if the future is ready after the first call to `Future::poll`. Read more

§

### impl<T> FutureExt for T

where T: [Future][12] \+ ?[Sized][32],

§

#### fn with_cancellation_token( self, cancellation_token: &CancellationToken, ) -> WithCancellationTokenFuture<'_, Self>

where Self: [Sized][32],

Similar to [`CancellationToken::run_until_cancelled`], but with the advantage that it is easier to write fluent call chains. Read more

§

#### fn with_cancellation_token_owned( self, cancellation_token: CancellationToken, ) -> WithCancellationTokenFutureOwned<Self>

where Self: [Sized][32],

Similar to [`CancellationToken::run_until_cancelled_owned`], but with the advantage that it is easier to write fluent call chains. Read more

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][55] [`Span`][56], returning an `Instrumented` wrapper. Read more

[Source][57]§

### impl<T, U> [Into][50]<U> for T

where U: [From][46]<T>,

[Source][58]§

#### fn [into][59](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][46]<T> for U` chooses to do.

[Source][60]§

### impl<F> [IntoFuture][61] for F

where F: [Future][12],

[Source][62]§

#### type [Output][63] = <F as [Future][12]>::[Output][19]

The output that the future will produce on completion.

[Source][64]§

#### type [IntoFuture][65] = F

Which kind of future are we turning this into?

[Source][66]§

#### fn [into_future][67](self) -> <F as [IntoFuture][61]>::[IntoFuture][68]

Creates a future from a value. [Read more][67]

§

### impl<T> PolicyExt for T

where T: ?[Sized][32],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][69]§

### impl<T> [Same][70] for T

[Source][71]§

#### type [Output][72] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][32],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][32],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][73]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][32],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][32],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][32],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][32],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][32],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][32],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][32],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][74] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][32],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][32],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][32],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][32],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][32],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][32],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][32],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][32],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][32],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][32], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][32], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][32],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][32],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][32],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][77]) -> RequestBodyLimit<Self>

where Self: [Sized][32],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][32],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][32],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][78]§

### impl<T, U> [TryFrom][79]<U> for T

where U: [Into][50]<T>,

[Source][80]§

#### type [Error][81] = [Infallible][16]

The type returned in the event of a conversion error.

[Source][82]§

#### fn [try_from][83](value: U) -> [Result][15]<T, <T as [TryFrom][79]<U>>::[Error][84]>

Performs the conversion.

§

### impl<F, T, E> TryFuture for F

where F: [Future][12]<Output = [Result][15]<T, E>> \+ ?[Sized][32],

§

#### type Ok = T

The type of successful values yielded by this future

§

#### type Error = E

The type of failures yielded by this future

§

#### fn try_poll( self: [Pin][21]<[&mut F][40]>, cx: &mut [Context][22]<'_>, ) -> [Poll][23]<<F as [Future][12]>::[Output][19]>

Poll this `TryFuture` as if it were a `Future`. Read more

§

### impl<Fut> TryFutureExt for Fut

where Fut: TryFuture + ?[Sized][32],

§

#### fn flatten_sink<Item>(self) -> FlattenSink<Self, Self::Ok>

where Self::Ok: Sink<Item, Error = Self::Error>, Self: [Sized][32],

Available on **crate feature`sink`** only.

Flattens the execution of this future when the successful result of this future is a [`Sink`]. Read more

§

#### fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>

where F: [FnOnce][49](Self::Ok) -> T, Self: [Sized][32],

Maps this future’s success value to a different value. Read more

§

#### fn map_ok_or_else<T, E, F>(self, e: E, f: F) -> MapOkOrElse<Self, F, E>

where F: [FnOnce][49](Self::Ok) -> T, E: [FnOnce][49](Self::Error) -> T, Self: [Sized][32],

Maps this future’s success value to a different value, and permits for error handling resulting in the same type. Read more

§

#### fn map_err<E, F>(self, f: F) -> MapErr<Self, F>

where F: [FnOnce][49](Self::Error) -> E, Self: [Sized][32],

Maps this future’s error value to a different value. Read more

§

#### fn err_into<E>(self) -> ErrInto<Self, E>

where Self: [Sized][32], Self::Error: [Into][50]<E>,

Maps this future’s [`Error`][85] to a new error type using the [`Into`][50] trait. Read more

§

#### fn ok_into<U>(self) -> OkInto<Self, U>

where Self: [Sized][32], Self::Ok: [Into][50]<U>,

Maps this future’s [`Ok`][86] to a new type using the [`Into`][50] trait.

§

#### fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>

where F: [FnOnce][49](Self::Ok) -> Fut, Fut: TryFuture<Error = Self::Error>, Self: [Sized][32],

Executes another future after this one resolves successfully. The success value is passed to a closure to create this subsequent future. Read more

§

#### fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>

where F: [FnOnce][49](Self::Error) -> Fut, Fut: TryFuture<Ok = Self::Ok>, Self: [Sized][32],

Executes another future if this one resolves to an error. The error value is passed to a closure to create this subsequent future. Read more

§

#### fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>

where F: [FnOnce][49](&Self::Ok), Self: [Sized][32],

Do something with the success value of a future before passing it on. Read more

§

#### fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>

where F: [FnOnce][49](&Self::Error), Self: [Sized][32],

Do something with the error value of a future before passing it on. Read more

§

#### fn try_flatten(self) -> TryFlatten<Self, Self::Ok>

where Self::Ok: TryFuture<Error = Self::Error>, Self: [Sized][32],

Flatten the execution of this future when the successful result of this future is another future. Read more

§

#### fn try_flatten_stream(self) -> TryFlattenStream<Self>

where Self::Ok: TryStream<Error = Self::Error>, Self: [Sized][32],

Flatten the execution of this future when the successful result of this future is a stream. Read more

§

#### fn unwrap_or_else<F>(self, f: F) -> UnwrapOrElse<Self, F>

where Self: [Sized][32], F: [FnOnce][49](Self::Error) -> Self::Ok,

Unwraps this future’s output, producing a future with this future’s [`Ok`][86] type as its [`Output`][87] type. Read more

§

#### fn into_future(self) -> IntoFuture<Self>

where Self: [Sized][32],

Wraps a [`TryFuture`] into a type that implements [`Future`][12]. Read more

§

#### fn try_poll_unpin( &mut self, cx: &mut [Context][22]<'_>, ) -> [Poll][23]<[Result][15]<Self::Ok, Self::Error>>

where Self: [Unpin][24],

A convenience method for calling [`TryFuture::try_poll`] on [`Unpin`][24] future types.

[Source][88]§

### impl<T, U> [TryInto][89]<U> for T

where U: [TryFrom][79]<T>,

[Source][90]§

#### type [Error][91] = <U as [TryFrom][79]<T>>::[Error][84]

The type returned in the event of a conversion error.

[Source][92]§

#### fn [try_into][93](self) -> [Result][15]<U, <U as [TryFrom][79]<T>>::[Error][84]>

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

where S: [Into][50]<Dispatch>,

Attaches the provided [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][95] [`Subscriber`][94] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/handler/future.rs.html#11-18
   [6]: super::IntoService
   [7]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [8]: struct.IntoServiceFuture.html (struct axum::handler::future::IntoServiceFuture)
   [9]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [10]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [11]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [12]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [13]: https://doc.rust-lang.org/nightly/std/primitive.fn.html
   [14]: ../../response/type.Response.html (type axum::response::Response)
   [15]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [16]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [17]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output
   [18]: ../../body/struct.Body.html (struct axum::body::Body)
   [19]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (type core::future::future::Future::Output)
   [20]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll
   [21]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [22]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [23]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [24]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [26]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [29]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [30]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [31]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [33]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [34]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [35]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [36]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [37]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [38]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [39]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [40]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [41]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [42]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [43]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [44]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [45]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [46]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [47]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [48]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [49]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [50]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [51]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [52]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [53]: futures_core::future::TryFuture
   [54]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [55]: super::Span::current()
   [56]: crate::Span
   [57]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [60]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#138
   [61]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html (trait core::future::into_future::IntoFuture)
   [62]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#139
   [63]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.Output
   [64]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#140
   [65]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture
   [66]: https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#142
   [67]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future
   [68]: https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture (type core::future::into_future::IntoFuture::IntoFuture)
   [69]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [70]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [71]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [72]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [73]: https://docs.rs/http/latest/http/struct.Extensions.html
   [74]: crate::follow_redirect::policy::Standard
   [75]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [76]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [77]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [82]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [83]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [85]: TryFuture::Error
   [86]: TryFuture::Ok
   [87]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output (associated type core::future::future::Future::Output)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [94]: super::Subscriber
   [95]: dispatcher#setting-the-default-subscriber

