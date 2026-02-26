<!-- Generated from rustdoc HTML: extract/ws/struct.WebSocket.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## WebSocket

## [axum][1]0.8.8

## WebSocket

### Methods

  * protocol
  * recv
  * send



### Trait Implementations

  * Debug
  * FusedStream
  * Sink<Message>
  * Stream



### Auto Trait Implementations

  * !Freeze
  * !RefUnwindSafe
  * !Sync
  * !UnwindSafe
  * Send
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
  * SinkExt<Item>
  * StreamExt
  * TryFrom<U>
  * TryInto<U>
  * TryStream
  * TryStreamExt
  * VZip<V>
  * WithSubscriber



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Struct WebSocket Copy item path

[Source][5]
``` 
pub struct WebSocket { /* private fields */ }
```

Available on **crate feature`ws`** only.

Expand description

A stream of WebSocket messages.

See [the module level documentation][6] for more details.

## Implementations§

[Source][7]§

### impl [WebSocket][8]

[Source][9]

#### pub async fn recv(&mut self) -> [Option][10]<[Result][11]<[Message][12], [Error][13]>>

Receive another message.

Returns `None` if the stream has closed.

[Source][14]

#### pub async fn send(&mut self, msg: [Message][12]) -> [Result][11]<[()][15], [Error][13]>

Send a message.

[Source][16]

#### pub fn protocol(&self) -> [Option][10]<&HeaderValue>

Return the selected WebSocket subprotocol, if one has been chosen.

## Trait Implementations§

[Source][17]§

### impl [Debug][18] for [WebSocket][8]

[Source][17]§

#### fn [fmt][19](&self, f: &mut [Formatter][20]<'_>) -> [Result][21]

Formats the value using the given formatter. [Read more][19]

[Source][22]§

### impl FusedStream for [WebSocket][8]

[Source][23]§

#### fn is_terminated(&self) -> [bool][24]

Returns true if the websocket has been terminated.

[Source][25]§

### impl Sink<[Message][12]> for [WebSocket][8]

[Source][26]§

#### type Error = [Error][13]

The type of value produced by the sink when an error occurs.

[Source][27]§

#### fn poll_ready( self: [Pin][28]<&mut Self>, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Result][11]<[()][15], Self::Error>>

Attempts to prepare the `Sink` to receive a value. Read more

[Source][31]§

#### fn start_send(self: [Pin][28]<&mut Self>, item: [Message][12]) -> [Result][11]<[()][15], Self::Error>

Begin the process of sending a value to the sink. Each call to this function must be preceded by a successful call to `poll_ready` which returned `Poll::Ready(Ok(()))`. Read more

[Source][32]§

#### fn poll_flush( self: [Pin][28]<&mut Self>, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Result][11]<[()][15], Self::Error>>

Flush any remaining output from this sink. Read more

[Source][33]§

#### fn poll_close( self: [Pin][28]<&mut Self>, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Result][11]<[()][15], Self::Error>>

Flush any remaining output and close this sink, if necessary. Read more

[Source][34]§

### impl Stream for [WebSocket][8]

[Source][35]§

#### type Item = [Result][11]<[Message][12], [Error][13]>

Values yielded by the stream.

[Source][36]§

#### fn poll_next( self: [Pin][28]<&mut Self>, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Option][10]<Self::Item>>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. Read more

§

#### fn size_hint(&self) -> ([usize][37], [Option][10]<[usize][37]>)

Returns the bounds on the remaining length of the stream. Read more

## Auto Trait Implementations§

§

### impl ![Freeze][38] for [WebSocket][8]

§

### impl ![RefUnwindSafe][39] for [WebSocket][8]

§

### impl [Send][40] for [WebSocket][8]

§

### impl ![Sync][41] for [WebSocket][8]

§

### impl [Unpin][42] for [WebSocket][8]

§

### impl ![UnwindSafe][43] for [WebSocket][8]

## Blanket Implementations§

[Source][44]§

### impl<T> [Any][45] for T

where T: 'static + ?[Sized][46],

[Source][47]§

#### fn [type_id][48](&self) -> [TypeId][49]

Gets the `TypeId` of `self`. [Read more][48]

[Source][50]§

### impl<T> [Borrow][51]<T> for T

where T: ?[Sized][46],

[Source][52]§

#### fn [borrow][53](&self) -> [&T][54]

Immutably borrows from an owned value. [Read more][53]

[Source][55]§

### impl<T> [BorrowMut][56]<T> for T

where T: ?[Sized][46],

[Source][57]§

#### fn [borrow_mut][58](&mut self) -> [&mut T][54]

Mutably borrows from an owned value. [Read more][58]

[Source][59]§

### impl<T> [From][60]<T> for T

[Source][61]§

#### fn [from][62](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][63] [`Span`][64], returning an `Instrumented` wrapper. Read more

[Source][65]§

### impl<T, U> [Into][66]<U> for T

where U: [From][60]<T>,

[Source][67]§

#### fn [into][68](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][60]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][46],

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

where Self: [Sized][46],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][46],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][73]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][46],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][46],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][46],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][46],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][46],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][46],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][46],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][74] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][46],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][46],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][75]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][46],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][76] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][46],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][46], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][46], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][46],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][46],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][46],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][37]) -> RequestBodyLimit<Self>

where Self: [Sized][46],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][46],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][46],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

§

### impl<T, Item> SinkExt<Item> for T

where T: Sink<Item> \+ ?[Sized][46],

§

#### fn with<U, Fut, F, E>(self, f: F) -> With<Self, Item, U, Fut, F>

where F: [FnMut][77](U) -> Fut, Fut: [Future][78]<Output = [Result][11]<Item, E>>, E: [From][60]<Self::Error>, Self: [Sized][46],

Composes a function _in front of_ the sink. Read more

§

#### fn with_flat_map<U, St, F>(self, f: F) -> WithFlatMap<Self, Item, U, St, F>

where F: [FnMut][77](U) -> St, St: Stream<Item = [Result][11]<Item, Self::Error>>, Self: [Sized][46],

Composes a function _in front of_ the sink. Read more

§

#### fn sink_map_err<E, F>(self, f: F) -> SinkMapErr<Self, F>

where F: [FnOnce][79](Self::Error) -> E, Self: [Sized][46],

Transforms the error returned by the sink.

§

#### fn sink_err_into<E>(self) -> SinkErrInto<Self, Item, E>

where Self: [Sized][46], Self::Error: [Into][66]<E>,

Map this sink’s error to a different error type using the `Into` trait. Read more

§

#### fn buffer(self, capacity: [usize][37]) -> Buffer<Self, Item>

where Self: [Sized][46],

Available on **crate feature`alloc`** only.

Adds a fixed-size buffer to the current sink. Read more

§

#### fn close(&mut self) -> Close<'_, Self, Item>

where Self: [Unpin][42],

Close the sink.

§

#### fn fanout<Si>(self, other: Si) -> Fanout<Self, Si>

where Self: [Sized][46], Item: [Clone][80], Si: Sink<Item, Error = Self::Error>,

Fanout items to multiple sinks. Read more

§

#### fn flush(&mut self) -> Flush<'_, Self, Item>

where Self: [Unpin][42],

Flush the sink, processing all pending items. Read more

§

#### fn send(&mut self, item: Item) -> Send<'_, Self, Item>

where Self: [Unpin][42],

A future that completes after the given item has been fully processed into the sink, including flushing. Read more

§

#### fn feed(&mut self, item: Item) -> Feed<'_, Self, Item>

where Self: [Unpin][42],

A future that completes after the given item has been received by the sink. Read more

§

#### fn send_all<'a, St>(&'a mut self, stream: [&'a mut St][54]) -> SendAll<'a, Self, St>

where St: TryStream<Ok = Item, Error = Self::Error> \+ Stream + [Unpin][42] \+ ?[Sized][46], Self: [Unpin][42],

A future that completes after the given stream has been fully processed into the sink, including flushing. Read more

§

#### fn left_sink<Si2>(self) -> Either<Self, Si2>

where Si2: Sink<Item, Error = Self::Error>, Self: [Sized][46],

Wrap this sink in an `Either` sink, making it the left-hand variant of that `Either`. Read more

§

#### fn right_sink<Si1>(self) -> Either<Si1, Self>

where Si1: Sink<Item, Error = Self::Error>, Self: [Sized][46],

Wrap this stream in an `Either` stream, making it the right-hand variant of that `Either`. Read more

§

#### fn poll_ready_unpin( &mut self, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Result][11]<[()][15], Self::Error>>

where Self: [Unpin][42],

A convenience method for calling [`Sink::poll_ready`] on [`Unpin`][42] sink types.

§

#### fn start_send_unpin(&mut self, item: Item) -> [Result][11]<[()][15], Self::Error>

where Self: [Unpin][42],

A convenience method for calling [`Sink::start_send`] on [`Unpin`][42] sink types.

§

#### fn poll_flush_unpin( &mut self, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Result][11]<[()][15], Self::Error>>

where Self: [Unpin][42],

A convenience method for calling [`Sink::poll_flush`] on [`Unpin`][42] sink types.

§

#### fn poll_close_unpin( &mut self, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Result][11]<[()][15], Self::Error>>

where Self: [Unpin][42],

A convenience method for calling [`Sink::poll_close`] on [`Unpin`][42] sink types.

§

### impl<T> StreamExt for T

where T: Stream + ?[Sized][46],

§

#### fn next(&mut self) -> Next<'_, Self>

where Self: [Unpin][42],

Creates a future that resolves to the next item in the stream. Read more

§

#### fn into_future(self) -> StreamFuture<Self>

where Self: [Sized][46] \+ [Unpin][42],

Converts this stream into a future of `(next_item, tail_of_stream)`. If the stream terminates, then the next item is [`None`][81]. Read more

§

#### fn map<T, F>(self, f: F) -> Map<Self, F>

where F: [FnMut][77](Self::Item) -> T, Self: [Sized][46],

Maps this stream’s items to a different type, returning a new stream of the resulting type. Read more

§

#### fn enumerate(self) -> Enumerate<Self>

where Self: [Sized][46],

Creates a stream which gives the current iteration count as well as the next value. Read more

§

#### fn filter<Fut, F>(self, f: F) -> Filter<Self, Fut, F>

where F: [FnMut][77](&Self::Item) -> Fut, Fut: [Future][78]<Output = [bool][24]>, Self: [Sized][46],

Filters the values produced by this stream according to the provided asynchronous predicate. Read more

§

#### fn filter_map<Fut, T, F>(self, f: F) -> FilterMap<Self, Fut, F>

where F: [FnMut][77](Self::Item) -> Fut, Fut: [Future][78]<Output = [Option][10]<T>>, Self: [Sized][46],

Filters the values produced by this stream while simultaneously mapping them to a different type according to the provided asynchronous closure. Read more

§

#### fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>

where F: [FnMut][77](Self::Item) -> Fut, Fut: [Future][78], Self: [Sized][46],

Computes from this stream’s items new items of a different type using an asynchronous closure. Read more

§

#### fn collect<C>(self) -> Collect<Self, C>

where C: [Default][82] \+ [Extend][83]<Self::Item>, Self: [Sized][46],

Transforms a stream into a collection, returning a future representing the result of that computation. Read more

§

#### fn unzip<A, B, FromA, FromB>(self) -> Unzip<Self, FromA, FromB>

where FromA: [Default][82] \+ [Extend][83]<A>, FromB: [Default][82] \+ [Extend][83]<B>, Self: [Sized][46] \+ Stream<Item = [(A, B)][84]>,

Converts a stream of pairs into a future, which resolves to pair of containers. Read more

§

#### fn concat(self) -> Concat<Self>

where Self: [Sized][46], Self::Item: [Extend][83]<<Self::Item as [IntoIterator][75]>::[Item][85]> \+ [IntoIterator][75] \+ [Default][82],

Concatenate all items of a stream into a single extendable destination, returning a future representing the end result. Read more

§

#### fn count(self) -> Count<Self>

where Self: [Sized][46],

Drives the stream to completion, counting the number of items. Read more

§

#### fn cycle(self) -> Cycle<Self>

where Self: [Sized][46] \+ [Clone][80],

Repeats a stream endlessly. Read more

§

#### fn fold<T, Fut, F>(self, init: T, f: F) -> Fold<Self, Fut, T, F>

where F: [FnMut][77](T, Self::Item) -> Fut, Fut: [Future][78]<Output = T>, Self: [Sized][46],

Execute an accumulating asynchronous computation over a stream, collecting all the values into one final result. Read more

§

#### fn any<Fut, F>(self, f: F) -> Any<Self, Fut, F>

where F: [FnMut][77](Self::Item) -> Fut, Fut: [Future][78]<Output = [bool][24]>, Self: [Sized][46],

Execute predicate over asynchronous stream, and return `true` if any element in stream satisfied a predicate. Read more

§

#### fn all<Fut, F>(self, f: F) -> All<Self, Fut, F>

where F: [FnMut][77](Self::Item) -> Fut, Fut: [Future][78]<Output = [bool][24]>, Self: [Sized][46],

Execute predicate over asynchronous stream, and return `true` if all element in stream satisfied a predicate. Read more

§

#### fn flatten(self) -> Flatten<Self>

where Self::Item: Stream, Self: [Sized][46],

Flattens a stream of streams into just one continuous stream. Read more

§

#### fn flatten_unordered( self, limit: impl [Into][66]<[Option][10]<[usize][37]>>, ) -> FlattenUnorderedWithFlowController<Self, [()][15]>

where Self::Item: Stream + [Unpin][42], Self: [Sized][46],

Available on **crate feature`alloc`** only.

Flattens a stream of streams into just one continuous stream. Polls inner streams produced by the base stream concurrently. Read more

§

#### fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>

where F: [FnMut][77](Self::Item) -> U, U: Stream, Self: [Sized][46],

Maps a stream like [`StreamExt::map`] but flattens nested `Stream`s. Read more

§

#### fn flat_map_unordered<U, F>( self, limit: impl [Into][66]<[Option][10]<[usize][37]>>, f: F, ) -> FlatMapUnordered<Self, U, F>

where U: Stream + [Unpin][42], F: [FnMut][77](Self::Item) -> U, Self: [Sized][46],

Available on **crate feature`alloc`** only.

Maps a stream like [`StreamExt::map`] but flattens nested `Stream`s and polls them concurrently, yielding items in any order, as they made available. Read more

§

#### fn scan<S, B, Fut, F>(self, initial_state: S, f: F) -> Scan<Self, S, Fut, F>

where F: [FnMut][77]([&mut S][54], Self::Item) -> Fut, Fut: [Future][78]<Output = [Option][10]<B>>, Self: [Sized][46],

Combinator similar to [`StreamExt::fold`] that holds internal state and produces a new stream. Read more

§

#### fn skip_while<Fut, F>(self, f: F) -> SkipWhile<Self, Fut, F>

where F: [FnMut][77](&Self::Item) -> Fut, Fut: [Future][78]<Output = [bool][24]>, Self: [Sized][46],

Skip elements on this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn take_while<Fut, F>(self, f: F) -> TakeWhile<Self, Fut, F>

where F: [FnMut][77](&Self::Item) -> Fut, Fut: [Future][78]<Output = [bool][24]>, Self: [Sized][46],

Take elements from this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn take_until<Fut>(self, fut: Fut) -> TakeUntil<Self, Fut>

where Fut: [Future][78], Self: [Sized][46],

Take elements from this stream until the provided future resolves. Read more

§

#### fn for_each<Fut, F>(self, f: F) -> ForEach<Self, Fut, F>

where F: [FnMut][77](Self::Item) -> Fut, Fut: [Future][78]<Output = [()][15]>, Self: [Sized][46],

Runs this stream to completion, executing the provided asynchronous closure for each element on the stream. Read more

§

#### fn for_each_concurrent<Fut, F>( self, limit: impl [Into][66]<[Option][10]<[usize][37]>>, f: F, ) -> ForEachConcurrent<Self, Fut, F>

where F: [FnMut][77](Self::Item) -> Fut, Fut: [Future][78]<Output = [()][15]>, Self: [Sized][46],

Available on **crate feature`alloc`** only.

Runs this stream to completion, executing the provided asynchronous closure for each element on the stream concurrently as elements become available. Read more

§

#### fn take(self, n: [usize][37]) -> Take<Self>

where Self: [Sized][46],

Creates a new stream of at most `n` items of the underlying stream. Read more

§

#### fn skip(self, n: [usize][37]) -> Skip<Self>

where Self: [Sized][46],

Creates a new stream which skips `n` items of the underlying stream. Read more

§

#### fn fuse(self) -> Fuse<Self>

where Self: [Sized][46],

Fuse a stream such that [`poll_next`][86] will never again be called once it has finished. This method can be used to turn any `Stream` into a `FusedStream`. Read more

§

#### fn by_ref(&mut self) -> &mut Self

Borrows a stream, rather than consuming it. Read more

§

#### fn catch_unwind(self) -> CatchUnwind<Self>

where Self: [Sized][46] \+ [UnwindSafe][43],

Available on **crate feature`std`** only.

Catches unwinding panics while polling the stream. Read more

§

#### fn boxed<'a>(self) -> [Pin][28]<[Box][87]<dyn Stream<Item = Self::Item> \+ [Send][40] \+ 'a>>

where Self: [Sized][46] \+ [Send][40] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the stream in a Box, pinning it. Read more

§

#### fn boxed_local<'a>(self) -> [Pin][28]<[Box][87]<dyn Stream<Item = Self::Item> \+ 'a>>

where Self: [Sized][46] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the stream in a Box, pinning it. Read more

§

#### fn buffered(self, n: [usize][37]) -> Buffered<Self>

where Self::Item: [Future][78], Self: [Sized][46],

Available on **crate feature`alloc`** only.

An adaptor for creating a buffered list of pending futures. Read more

§

#### fn buffer_unordered(self, n: [usize][37]) -> BufferUnordered<Self>

where Self::Item: [Future][78], Self: [Sized][46],

Available on **crate feature`alloc`** only.

An adaptor for creating a buffered list of pending futures (unordered). Read more

§

#### fn zip<St>(self, other: St) -> Zip<Self, St>

where St: Stream, Self: [Sized][46],

An adapter for zipping two streams together. Read more

§

#### fn chain<St>(self, other: St) -> Chain<Self, St>

where St: Stream<Item = Self::Item>, Self: [Sized][46],

Adapter for chaining two streams. Read more

§

#### fn peekable(self) -> Peekable<Self>

where Self: [Sized][46],

Creates a new stream which exposes a `peek` method. Read more

§

#### fn chunks(self, capacity: [usize][37]) -> Chunks<Self>

where Self: [Sized][46],

Available on **crate feature`alloc`** only.

An adaptor for chunking up items of the stream inside a vector. Read more

§

#### fn ready_chunks(self, capacity: [usize][37]) -> ReadyChunks<Self>

where Self: [Sized][46],

Available on **crate feature`alloc`** only.

An adaptor for chunking up ready items of the stream inside a vector. Read more

§

#### fn forward<S>(self, sink: S) -> Forward<Self, S>

where S: Sink<Self::Ok, Error = Self::Error>, Self: [Sized][46] \+ TryStream,

Available on **crate feature`sink`** only.

A future that completes after the given stream has been fully processed into the sink and the sink has been flushed and closed. Read more

§

#### fn split<Item>(self) -> (SplitSink<Self, Item>, SplitStream<Self>)

where Self: [Sized][46] \+ Sink<Item>,

Available on **crate features`sink` and `alloc`** only.

Splits this `Stream + Sink` object into separate `Sink` and `Stream` objects. Read more

§

#### fn inspect<F>(self, f: F) -> Inspect<Self, F>

where F: [FnMut][77](&Self::Item), Self: [Sized][46],

Do something with each item of this stream, afterwards passing it on. Read more

§

#### fn left_stream<B>(self) -> Either<Self, B>

where B: Stream<Item = Self::Item>, Self: [Sized][46],

Wrap this stream in an `Either` stream, making it the left-hand variant of that `Either`. Read more

§

#### fn right_stream<B>(self) -> Either<B, Self>

where B: Stream<Item = Self::Item>, Self: [Sized][46],

Wrap this stream in an `Either` stream, making it the right-hand variant of that `Either`. Read more

§

#### fn poll_next_unpin(&mut self, cx: &mut [Context][29]<'_>) -> [Poll][30]<[Option][10]<Self::Item>>

where Self: [Unpin][42],

A convenience method for calling [`Stream::poll_next`] on [`Unpin`][42] stream types.

§

#### fn select_next_some(&mut self) -> SelectNextSome<'_, Self>

where Self: [Unpin][42] \+ FusedStream,

Returns a [`Future`][78] that resolves when the next item in this stream is ready. Read more

[Source][88]§

### impl<T, U> [TryFrom][89]<U> for T

where U: [Into][66]<T>,

[Source][90]§

#### type [Error][91] = [Infallible][92]

The type returned in the event of a conversion error.

[Source][93]§

#### fn [try_from][94](value: U) -> [Result][11]<T, <T as [TryFrom][89]<U>>::[Error][95]>

Performs the conversion.

[Source][96]§

### impl<T, U> [TryInto][97]<U> for T

where U: [TryFrom][89]<T>,

[Source][98]§

#### type [Error][99] = <U as [TryFrom][89]<T>>::[Error][95]

The type returned in the event of a conversion error.

[Source][100]§

#### fn [try_into][101](self) -> [Result][11]<U, <U as [TryFrom][89]<T>>::[Error][95]>

Performs the conversion.

§

### impl<S, T, E> TryStream for S

where S: Stream<Item = [Result][11]<T, E>> \+ ?[Sized][46],

§

#### type Ok = T

The type of successful values yielded by this future

§

#### type Error = E

The type of failures yielded by this future

§

#### fn try_poll_next( self: [Pin][28]<[&mut S][54]>, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Option][10]<[Result][11]<<S as TryStream>::Ok, <S as TryStream>::Error>>>

Poll this `TryStream` as if it were a `Stream`. Read more

§

### impl<S> TryStreamExt for S

where S: TryStream + ?[Sized][46],

§

#### fn err_into<E>(self) -> ErrInto<Self, E>

where Self: [Sized][46], Self::Error: [Into][66]<E>,

Wraps the current stream in a new stream which converts the error type into the one provided. Read more

§

#### fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>

where Self: [Sized][46], F: [FnMut][77](Self::Ok) -> T,

Wraps the current stream in a new stream which maps the success value using the provided closure. Read more

§

#### fn map_err<E, F>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][46], F: [FnMut][77](Self::Error) -> E,

Wraps the current stream in a new stream which maps the error value using the provided closure. Read more

§

#### fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>

where F: [FnMut][77](Self::Ok) -> Fut, Fut: TryFuture<Error = Self::Error>, Self: [Sized][46],

Chain on a computation for when a value is ready, passing the successful results to the provided closure `f`. Read more

§

#### fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>

where F: [FnMut][77](Self::Error) -> Fut, Fut: TryFuture<Ok = Self::Ok>, Self: [Sized][46],

Chain on a computation for when an error happens, passing the erroneous result to the provided closure `f`. Read more

§

#### fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>

where F: [FnMut][77](&Self::Ok), Self: [Sized][46],

Do something with the success value of this stream, afterwards passing it on. Read more

§

#### fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>

where F: [FnMut][77](&Self::Error), Self: [Sized][46],

Do something with the error value of this stream, afterwards passing it on. Read more

§

#### fn into_stream(self) -> IntoStream<Self>

where Self: [Sized][46],

Wraps a [`TryStream`] into a type that implements [`Stream`][102] Read more

§

#### fn try_next(&mut self) -> TryNext<'_, Self>

where Self: [Unpin][42],

Creates a future that attempts to resolve the next item in the stream. If an error is encountered before the next item, the error is returned instead. Read more

§

#### fn try_for_each<Fut, F>(self, f: F) -> TryForEach<Self, Fut, F>

where F: [FnMut][77](Self::Ok) -> Fut, Fut: TryFuture<Ok = [()][15], Error = Self::Error>, Self: [Sized][46],

Attempts to run this stream to completion, executing the provided asynchronous closure for each element on the stream. Read more

§

#### fn try_skip_while<Fut, F>(self, f: F) -> TrySkipWhile<Self, Fut, F>

where F: [FnMut][77](&Self::Ok) -> Fut, Fut: TryFuture<Ok = [bool][24], Error = Self::Error>, Self: [Sized][46],

Skip elements on this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn try_take_while<Fut, F>(self, f: F) -> TryTakeWhile<Self, Fut, F>

where F: [FnMut][77](&Self::Ok) -> Fut, Fut: TryFuture<Ok = [bool][24], Error = Self::Error>, Self: [Sized][46],

Take elements on this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn try_for_each_concurrent<Fut, F>( self, limit: impl [Into][66]<[Option][10]<[usize][37]>>, f: F, ) -> TryForEachConcurrent<Self, Fut, F>

where F: [FnMut][77](Self::Ok) -> Fut, Fut: [Future][78]<Output = [Result][11]<[()][15], Self::Error>>, Self: [Sized][46],

Available on **crate feature`alloc`** only.

Attempts to run this stream to completion, executing the provided asynchronous closure for each element on the stream concurrently as elements become available, exiting as soon as an error occurs. Read more

§

#### fn try_collect<C>(self) -> TryCollect<Self, C>

where C: [Default][82] \+ [Extend][83]<Self::Ok>, Self: [Sized][46],

Attempt to transform a stream into a collection, returning a future representing the result of that computation. Read more

§

#### fn try_chunks(self, capacity: [usize][37]) -> TryChunks<Self>

where Self: [Sized][46],

Available on **crate feature`alloc`** only.

An adaptor for chunking up successful items of the stream inside a vector. Read more

§

#### fn try_ready_chunks(self, capacity: [usize][37]) -> TryReadyChunks<Self>

where Self: [Sized][46],

Available on **crate feature`alloc`** only.

An adaptor for chunking up successful, ready items of the stream inside a vector. Read more

§

#### fn try_filter<Fut, F>(self, f: F) -> TryFilter<Self, Fut, F>

where Fut: [Future][78]<Output = [bool][24]>, F: [FnMut][77](&Self::Ok) -> Fut, Self: [Sized][46],

Attempt to filter the values produced by this stream according to the provided asynchronous closure. Read more

§

#### fn try_filter_map<Fut, F, T>(self, f: F) -> TryFilterMap<Self, Fut, F>

where Fut: TryFuture<Ok = [Option][10]<T>, Error = Self::Error>, F: [FnMut][77](Self::Ok) -> Fut, Self: [Sized][46],

Attempt to filter the values produced by this stream while simultaneously mapping them to a different type according to the provided asynchronous closure. Read more

§

#### fn try_flatten_unordered( self, limit: impl [Into][66]<[Option][10]<[usize][37]>>, ) -> TryFlattenUnordered<Self>

where Self::Ok: TryStream + [Unpin][42], <Self::Ok as TryStream>::Error: [From][60]<Self::Error>, Self: [Sized][46],

Available on **crate feature`alloc`** only.

Flattens a stream of streams into just one continuous stream. Produced streams will be polled concurrently and any errors will be passed through without looking at them. If the underlying base stream returns an error, it will be **immediately** propagated. Read more

§

#### fn try_flatten(self) -> TryFlatten<Self>

where Self::Ok: TryStream, <Self::Ok as TryStream>::Error: [From][60]<Self::Error>, Self: [Sized][46],

Flattens a stream of streams into just one continuous stream. Read more

§

#### fn try_fold<T, Fut, F>(self, init: T, f: F) -> TryFold<Self, Fut, T, F>

where F: [FnMut][77](T, Self::Ok) -> Fut, Fut: TryFuture<Ok = T, Error = Self::Error>, Self: [Sized][46],

Attempt to execute an accumulating asynchronous computation over a stream, collecting all the values into one final result. Read more

§

#### fn try_concat(self) -> TryConcat<Self>

where Self: [Sized][46], Self::Ok: [Extend][83]<<Self::Ok as [IntoIterator][75]>::[Item][85]> \+ [IntoIterator][75] \+ [Default][82],

Attempt to concatenate all items of a stream into a single extendable destination, returning a future representing the end result. Read more

§

#### fn try_buffer_unordered(self, n: [usize][37]) -> TryBufferUnordered<Self>

where Self::Ok: TryFuture<Error = Self::Error>, Self: [Sized][46],

Available on **crate feature`alloc`** only.

Attempt to execute several futures from a stream concurrently (unordered). Read more

§

#### fn try_buffered(self, n: [usize][37]) -> TryBuffered<Self>

where Self::Ok: TryFuture<Error = Self::Error>, Self: [Sized][46],

Available on **crate feature`alloc`** only.

Attempt to execute several futures from a stream concurrently. Read more

§

#### fn try_poll_next_unpin( &mut self, cx: &mut [Context][29]<'_>, ) -> [Poll][30]<[Option][10]<[Result][11]<Self::Ok, Self::Error>>>

where Self: [Unpin][42],

A convenience method for calling [`TryStream::try_poll_next`] on [`Unpin`][42] stream types.

§

#### fn try_all<Fut, F>(self, f: F) -> TryAll<Self, Fut, F>

where Self: [Sized][46], F: [FnMut][77](Self::Ok) -> Fut, Fut: [Future][78]<Output = [bool][24]>,

Attempt to execute a predicate over an asynchronous stream and evaluate if all items satisfy the predicate. Exits early if an `Err` is encountered or if an `Ok` item is found that does not satisfy the predicate. Read more

§

#### fn try_any<Fut, F>(self, f: F) -> TryAny<Self, Fut, F>

where Self: [Sized][46], F: [FnMut][77](Self::Ok) -> Fut, Fut: [Future][78]<Output = [bool][24]>,

Attempt to execute a predicate over an asynchronous stream and evaluate if any items satisfy the predicate. Exits early if an `Err` is encountered or if an `Ok` item is found that satisfies the predicate. Read more

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][66]<Dispatch>,

Attaches the provided [`Subscriber`][103] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][104] [`Subscriber`][103] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/ws.rs.html#543-546
   [6]: index.html (mod axum::extract::ws)
   [7]: ../../../src/axum/extract/ws.rs.html#548-568
   [8]: struct.WebSocket.html (struct axum::extract::ws::WebSocket)
   [9]: ../../../src/axum/extract/ws.rs.html#552-554
   [10]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [11]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [12]: enum.Message.html (enum axum::extract::ws::Message)
   [13]: ../../struct.Error.html (struct axum::Error)
   [14]: ../../../src/axum/extract/ws.rs.html#557-562
   [15]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [16]: ../../../src/axum/extract/ws.rs.html#565-567
   [17]: ../../../src/axum/extract/ws.rs.html#542
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [20]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [21]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [22]: ../../../src/axum/extract/ws.rs.html#570-575
   [23]: ../../../src/axum/extract/ws.rs.html#572-574
   [24]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [25]: ../../../src/axum/extract/ws.rs.html#595-615
   [26]: ../../../src/axum/extract/ws.rs.html#596
   [27]: ../../../src/axum/extract/ws.rs.html#598-600
   [28]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [29]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [30]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [31]: ../../../src/axum/extract/ws.rs.html#602-606
   [32]: ../../../src/axum/extract/ws.rs.html#608-610
   [33]: ../../../src/axum/extract/ws.rs.html#612-614
   [34]: ../../../src/axum/extract/ws.rs.html#577-593
   [35]: ../../../src/axum/extract/ws.rs.html#578
   [36]: ../../../src/axum/extract/ws.rs.html#580-592
   [37]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [39]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [42]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [43]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [44]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [45]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [46]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [47]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [48]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [49]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [54]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [57]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [58]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [59]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [60]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [63]: super::Span::current()
   [64]: crate::Span
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [68]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [69]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [70]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [71]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [72]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [73]: https://docs.rs/http/latest/http/struct.Extensions.html
   [74]: crate::follow_redirect::policy::Standard
   [75]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [76]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [77]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [78]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [79]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [80]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [81]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None (variant core::option::Option::None)
   [82]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [83]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html (trait core::iter::traits::collect::Extend)
   [84]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [85]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item (type core::iter::traits::collect::IntoIterator::Item)
   [86]: Stream::poll_next
   [87]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [92]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [98]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [100]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [101]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [102]: futures_core::stream::Stream
   [103]: super::Subscriber
   [104]: dispatcher#setting-the-default-subscriber

