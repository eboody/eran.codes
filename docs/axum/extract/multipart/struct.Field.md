<!-- Generated from rustdoc HTML: extract/multipart/struct.Field.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Field

## [axum][1]0.8.8

## Field

### Methods

  * bytes
  * chunk
  * content_type
  * file_name
  * headers
  * name
  * text



### Trait Implementations

  * Debug
  * Stream



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
  * StreamExt
  * TryFrom<U>
  * TryInto<U>
  * TryStream
  * TryStreamExt
  * VZip<V>
  * WithSubscriber



## [In axum::extract::multipart][2]

[axum][3]::[extract][4]::[multipart][2]

# Struct Field Copy item path

[Source][5]
``` 
pub struct Field<'a> { /* private fields */ }
```

Available on **crate feature`multipart`** only.

Expand description

A single field in a multipart stream.

## Implementations§

[Source][6]§

### impl [Field][7]<'_>

[Source][8]

#### pub fn name(&self) -> [Option][9]<&[str][10]>

The field name found in the [`Content-Disposition`][11] header.

[Source][12]

#### pub fn file_name(&self) -> [Option][9]<&[str][10]>

The file name found in the [`Content-Disposition`][11] header.

[Source][13]

#### pub fn content_type(&self) -> [Option][9]<&[str][10]>

Get the [content type][14] of the field.

[Source][15]

#### pub fn headers(&self) -> &HeaderMap

Get a map of headers as [`HeaderMap`].

[Source][16]

#### pub async fn bytes(self) -> [Result][17]<Bytes, [MultipartError][18]>

Get the full data of the field as [`Bytes`].

[Source][19]

#### pub async fn text(self) -> [Result][17]<[String][20], [MultipartError][18]>

Get the full field data as text.

[Source][21]

#### pub async fn chunk(&mut self) -> [Result][17]<[Option][9]<Bytes>, [MultipartError][18]>

Stream a chunk of the field data.

When the field data has been exhausted, this will return [`None`][22].

Note this does the same thing as `Field`’s [`Stream`] implementation.

##### §Example
``` 
use axum::{
   extract::Multipart,
   routing::post,
   response::IntoResponse,
   http::StatusCode,
   Router,
};

async fn upload(mut multipart: Multipart) -> Result<(), (StatusCode, String)> {
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
    {
        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?
        {
            println!("received {} bytes", chunk.len());
        }
    }

    Ok(())
}

let app = Router::new().route("/upload", post(upload));
```

## Trait Implementations§

[Source][23]§

### impl<'a> [Debug][24] for [Field][7]<'a>

[Source][23]§

#### fn [fmt][25](&self, f: &mut [Formatter][26]<'_>) -> [Result][27]

Formats the value using the given formatter. [Read more][25]

[Source][28]§

### impl Stream for [Field][7]<'_>

[Source][29]§

#### type Item = [Result][17]<Bytes, [MultipartError][18]>

Values yielded by the stream.

[Source][30]§

#### fn poll_next( self: [Pin][31]<&mut Self>, cx: &mut [Context][32]<'_>, ) -> [Poll][33]<[Option][9]<Self::Item>>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. Read more

§

#### fn size_hint(&self) -> ([usize][34], [Option][9]<[usize][34]>)

Returns the bounds on the remaining length of the stream. Read more

## Auto Trait Implementations§

§

### impl<'a> [Freeze][35] for [Field][7]<'a>

§

### impl<'a> ![RefUnwindSafe][36] for [Field][7]<'a>

§

### impl<'a> [Send][37] for [Field][7]<'a>

§

### impl<'a> [Sync][38] for [Field][7]<'a>

§

### impl<'a> [Unpin][39] for [Field][7]<'a>

§

### impl<'a> ![UnwindSafe][40] for [Field][7]<'a>

## Blanket Implementations§

[Source][41]§

### impl<T> [Any][42] for T

where T: 'static + ?[Sized][43],

[Source][44]§

#### fn [type_id][45](&self) -> [TypeId][46]

Gets the `TypeId` of `self`. [Read more][45]

[Source][47]§

### impl<T> [Borrow][48]<T> for T

where T: ?[Sized][43],

[Source][49]§

#### fn [borrow][50](&self) -> [&T][51]

Immutably borrows from an owned value. [Read more][50]

[Source][52]§

### impl<T> [BorrowMut][53]<T> for T

where T: ?[Sized][43],

[Source][54]§

#### fn [borrow_mut][55](&mut self) -> [&mut T][51]

Mutably borrows from an owned value. [Read more][55]

[Source][56]§

### impl<T> [From][57]<T> for T

[Source][58]§

#### fn [from][59](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][60] [`Span`][61], returning an `Instrumented` wrapper. Read more

[Source][62]§

### impl<T, U> [Into][63]<U> for T

where U: [From][57]<T>,

[Source][64]§

#### fn [into][65](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][57]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][43],

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

where Self: [Sized][43],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][43],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][70]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][43],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][43],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][43],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][71] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][72]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][73] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][43],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][34]) -> RequestBodyLimit<Self>

where Self: [Sized][43],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

§

### impl<T> StreamExt for T

where T: Stream + ?[Sized][43],

§

#### fn next(&mut self) -> Next<'_, Self>

where Self: [Unpin][39],

Creates a future that resolves to the next item in the stream. Read more

§

#### fn into_future(self) -> StreamFuture<Self>

where Self: [Sized][43] \+ [Unpin][39],

Converts this stream into a future of `(next_item, tail_of_stream)`. If the stream terminates, then the next item is [`None`][22]. Read more

§

#### fn map<T, F>(self, f: F) -> Map<Self, F>

where F: [FnMut][74](Self::Item) -> T, Self: [Sized][43],

Maps this stream’s items to a different type, returning a new stream of the resulting type. Read more

§

#### fn enumerate(self) -> Enumerate<Self>

where Self: [Sized][43],

Creates a stream which gives the current iteration count as well as the next value. Read more

§

#### fn filter<Fut, F>(self, f: F) -> Filter<Self, Fut, F>

where F: [FnMut][74](&Self::Item) -> Fut, Fut: [Future][75]<Output = [bool][76]>, Self: [Sized][43],

Filters the values produced by this stream according to the provided asynchronous predicate. Read more

§

#### fn filter_map<Fut, T, F>(self, f: F) -> FilterMap<Self, Fut, F>

where F: [FnMut][74](Self::Item) -> Fut, Fut: [Future][75]<Output = [Option][9]<T>>, Self: [Sized][43],

Filters the values produced by this stream while simultaneously mapping them to a different type according to the provided asynchronous closure. Read more

§

#### fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>

where F: [FnMut][74](Self::Item) -> Fut, Fut: [Future][75], Self: [Sized][43],

Computes from this stream’s items new items of a different type using an asynchronous closure. Read more

§

#### fn collect<C>(self) -> Collect<Self, C>

where C: [Default][77] \+ [Extend][78]<Self::Item>, Self: [Sized][43],

Transforms a stream into a collection, returning a future representing the result of that computation. Read more

§

#### fn unzip<A, B, FromA, FromB>(self) -> Unzip<Self, FromA, FromB>

where FromA: [Default][77] \+ [Extend][78]<A>, FromB: [Default][77] \+ [Extend][78]<B>, Self: [Sized][43] \+ Stream<Item = [(A, B)][79]>,

Converts a stream of pairs into a future, which resolves to pair of containers. Read more

§

#### fn concat(self) -> Concat<Self>

where Self: [Sized][43], Self::Item: [Extend][78]<<Self::Item as [IntoIterator][72]>::[Item][80]> \+ [IntoIterator][72] \+ [Default][77],

Concatenate all items of a stream into a single extendable destination, returning a future representing the end result. Read more

§

#### fn count(self) -> Count<Self>

where Self: [Sized][43],

Drives the stream to completion, counting the number of items. Read more

§

#### fn cycle(self) -> Cycle<Self>

where Self: [Sized][43] \+ [Clone][81],

Repeats a stream endlessly. Read more

§

#### fn fold<T, Fut, F>(self, init: T, f: F) -> Fold<Self, Fut, T, F>

where F: [FnMut][74](T, Self::Item) -> Fut, Fut: [Future][75]<Output = T>, Self: [Sized][43],

Execute an accumulating asynchronous computation over a stream, collecting all the values into one final result. Read more

§

#### fn any<Fut, F>(self, f: F) -> Any<Self, Fut, F>

where F: [FnMut][74](Self::Item) -> Fut, Fut: [Future][75]<Output = [bool][76]>, Self: [Sized][43],

Execute predicate over asynchronous stream, and return `true` if any element in stream satisfied a predicate. Read more

§

#### fn all<Fut, F>(self, f: F) -> All<Self, Fut, F>

where F: [FnMut][74](Self::Item) -> Fut, Fut: [Future][75]<Output = [bool][76]>, Self: [Sized][43],

Execute predicate over asynchronous stream, and return `true` if all element in stream satisfied a predicate. Read more

§

#### fn flatten(self) -> Flatten<Self>

where Self::Item: Stream, Self: [Sized][43],

Flattens a stream of streams into just one continuous stream. Read more

§

#### fn flatten_unordered( self, limit: impl [Into][63]<[Option][9]<[usize][34]>>, ) -> FlattenUnorderedWithFlowController<Self, [()][82]>

where Self::Item: Stream + [Unpin][39], Self: [Sized][43],

Available on **crate feature`alloc`** only.

Flattens a stream of streams into just one continuous stream. Polls inner streams produced by the base stream concurrently. Read more

§

#### fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>

where F: [FnMut][74](Self::Item) -> U, U: Stream, Self: [Sized][43],

Maps a stream like [`StreamExt::map`] but flattens nested `Stream`s. Read more

§

#### fn flat_map_unordered<U, F>( self, limit: impl [Into][63]<[Option][9]<[usize][34]>>, f: F, ) -> FlatMapUnordered<Self, U, F>

where U: Stream + [Unpin][39], F: [FnMut][74](Self::Item) -> U, Self: [Sized][43],

Available on **crate feature`alloc`** only.

Maps a stream like [`StreamExt::map`] but flattens nested `Stream`s and polls them concurrently, yielding items in any order, as they made available. Read more

§

#### fn scan<S, B, Fut, F>(self, initial_state: S, f: F) -> Scan<Self, S, Fut, F>

where F: [FnMut][74]([&mut S][51], Self::Item) -> Fut, Fut: [Future][75]<Output = [Option][9]<B>>, Self: [Sized][43],

Combinator similar to [`StreamExt::fold`] that holds internal state and produces a new stream. Read more

§

#### fn skip_while<Fut, F>(self, f: F) -> SkipWhile<Self, Fut, F>

where F: [FnMut][74](&Self::Item) -> Fut, Fut: [Future][75]<Output = [bool][76]>, Self: [Sized][43],

Skip elements on this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn take_while<Fut, F>(self, f: F) -> TakeWhile<Self, Fut, F>

where F: [FnMut][74](&Self::Item) -> Fut, Fut: [Future][75]<Output = [bool][76]>, Self: [Sized][43],

Take elements from this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn take_until<Fut>(self, fut: Fut) -> TakeUntil<Self, Fut>

where Fut: [Future][75], Self: [Sized][43],

Take elements from this stream until the provided future resolves. Read more

§

#### fn for_each<Fut, F>(self, f: F) -> ForEach<Self, Fut, F>

where F: [FnMut][74](Self::Item) -> Fut, Fut: [Future][75]<Output = [()][82]>, Self: [Sized][43],

Runs this stream to completion, executing the provided asynchronous closure for each element on the stream. Read more

§

#### fn for_each_concurrent<Fut, F>( self, limit: impl [Into][63]<[Option][9]<[usize][34]>>, f: F, ) -> ForEachConcurrent<Self, Fut, F>

where F: [FnMut][74](Self::Item) -> Fut, Fut: [Future][75]<Output = [()][82]>, Self: [Sized][43],

Available on **crate feature`alloc`** only.

Runs this stream to completion, executing the provided asynchronous closure for each element on the stream concurrently as elements become available. Read more

§

#### fn take(self, n: [usize][34]) -> Take<Self>

where Self: [Sized][43],

Creates a new stream of at most `n` items of the underlying stream. Read more

§

#### fn skip(self, n: [usize][34]) -> Skip<Self>

where Self: [Sized][43],

Creates a new stream which skips `n` items of the underlying stream. Read more

§

#### fn fuse(self) -> Fuse<Self>

where Self: [Sized][43],

Fuse a stream such that [`poll_next`][83] will never again be called once it has finished. This method can be used to turn any `Stream` into a `FusedStream`. Read more

§

#### fn by_ref(&mut self) -> &mut Self

Borrows a stream, rather than consuming it. Read more

§

#### fn catch_unwind(self) -> CatchUnwind<Self>

where Self: [Sized][43] \+ [UnwindSafe][40],

Available on **crate feature`std`** only.

Catches unwinding panics while polling the stream. Read more

§

#### fn boxed<'a>(self) -> [Pin][31]<[Box][84]<dyn Stream<Item = Self::Item> \+ [Send][37] \+ 'a>>

where Self: [Sized][43] \+ [Send][37] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the stream in a Box, pinning it. Read more

§

#### fn boxed_local<'a>(self) -> [Pin][31]<[Box][84]<dyn Stream<Item = Self::Item> \+ 'a>>

where Self: [Sized][43] \+ 'a,

Available on **crate feature`alloc`** only.

Wrap the stream in a Box, pinning it. Read more

§

#### fn buffered(self, n: [usize][34]) -> Buffered<Self>

where Self::Item: [Future][75], Self: [Sized][43],

Available on **crate feature`alloc`** only.

An adaptor for creating a buffered list of pending futures. Read more

§

#### fn buffer_unordered(self, n: [usize][34]) -> BufferUnordered<Self>

where Self::Item: [Future][75], Self: [Sized][43],

Available on **crate feature`alloc`** only.

An adaptor for creating a buffered list of pending futures (unordered). Read more

§

#### fn zip<St>(self, other: St) -> Zip<Self, St>

where St: Stream, Self: [Sized][43],

An adapter for zipping two streams together. Read more

§

#### fn chain<St>(self, other: St) -> Chain<Self, St>

where St: Stream<Item = Self::Item>, Self: [Sized][43],

Adapter for chaining two streams. Read more

§

#### fn peekable(self) -> Peekable<Self>

where Self: [Sized][43],

Creates a new stream which exposes a `peek` method. Read more

§

#### fn chunks(self, capacity: [usize][34]) -> Chunks<Self>

where Self: [Sized][43],

Available on **crate feature`alloc`** only.

An adaptor for chunking up items of the stream inside a vector. Read more

§

#### fn ready_chunks(self, capacity: [usize][34]) -> ReadyChunks<Self>

where Self: [Sized][43],

Available on **crate feature`alloc`** only.

An adaptor for chunking up ready items of the stream inside a vector. Read more

§

#### fn forward<S>(self, sink: S) -> Forward<Self, S>

where S: Sink<Self::Ok, Error = Self::Error>, Self: [Sized][43] \+ TryStream,

Available on **crate feature`sink`** only.

A future that completes after the given stream has been fully processed into the sink and the sink has been flushed and closed. Read more

§

#### fn split<Item>(self) -> (SplitSink<Self, Item>, SplitStream<Self>)

where Self: [Sized][43] \+ Sink<Item>,

Available on **crate features`sink` and `alloc`** only.

Splits this `Stream + Sink` object into separate `Sink` and `Stream` objects. Read more

§

#### fn inspect<F>(self, f: F) -> Inspect<Self, F>

where F: [FnMut][74](&Self::Item), Self: [Sized][43],

Do something with each item of this stream, afterwards passing it on. Read more

§

#### fn left_stream<B>(self) -> Either<Self, B>

where B: Stream<Item = Self::Item>, Self: [Sized][43],

Wrap this stream in an `Either` stream, making it the left-hand variant of that `Either`. Read more

§

#### fn right_stream<B>(self) -> Either<B, Self>

where B: Stream<Item = Self::Item>, Self: [Sized][43],

Wrap this stream in an `Either` stream, making it the right-hand variant of that `Either`. Read more

§

#### fn poll_next_unpin(&mut self, cx: &mut [Context][32]<'_>) -> [Poll][33]<[Option][9]<Self::Item>>

where Self: [Unpin][39],

A convenience method for calling [`Stream::poll_next`] on [`Unpin`][39] stream types.

§

#### fn select_next_some(&mut self) -> SelectNextSome<'_, Self>

where Self: [Unpin][39] \+ FusedStream,

Returns a [`Future`][75] that resolves when the next item in this stream is ready. Read more

[Source][85]§

### impl<T, U> [TryFrom][86]<U> for T

where U: [Into][63]<T>,

[Source][87]§

#### type [Error][88] = [Infallible][89]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_from][91](value: U) -> [Result][17]<T, <T as [TryFrom][86]<U>>::[Error][92]>

Performs the conversion.

[Source][93]§

### impl<T, U> [TryInto][94]<U> for T

where U: [TryFrom][86]<T>,

[Source][95]§

#### type [Error][96] = <U as [TryFrom][86]<T>>::[Error][92]

The type returned in the event of a conversion error.

[Source][97]§

#### fn [try_into][98](self) -> [Result][17]<U, <U as [TryFrom][86]<T>>::[Error][92]>

Performs the conversion.

§

### impl<S, T, E> TryStream for S

where S: Stream<Item = [Result][17]<T, E>> \+ ?[Sized][43],

§

#### type Ok = T

The type of successful values yielded by this future

§

#### type Error = E

The type of failures yielded by this future

§

#### fn try_poll_next( self: [Pin][31]<[&mut S][51]>, cx: &mut [Context][32]<'_>, ) -> [Poll][33]<[Option][9]<[Result][17]<<S as TryStream>::Ok, <S as TryStream>::Error>>>

Poll this `TryStream` as if it were a `Stream`. Read more

§

### impl<S> TryStreamExt for S

where S: TryStream + ?[Sized][43],

§

#### fn err_into<E>(self) -> ErrInto<Self, E>

where Self: [Sized][43], Self::Error: [Into][63]<E>,

Wraps the current stream in a new stream which converts the error type into the one provided. Read more

§

#### fn map_ok<T, F>(self, f: F) -> MapOk<Self, F>

where Self: [Sized][43], F: [FnMut][74](Self::Ok) -> T,

Wraps the current stream in a new stream which maps the success value using the provided closure. Read more

§

#### fn map_err<E, F>(self, f: F) -> MapErr<Self, F>

where Self: [Sized][43], F: [FnMut][74](Self::Error) -> E,

Wraps the current stream in a new stream which maps the error value using the provided closure. Read more

§

#### fn and_then<Fut, F>(self, f: F) -> AndThen<Self, Fut, F>

where F: [FnMut][74](Self::Ok) -> Fut, Fut: TryFuture<Error = Self::Error>, Self: [Sized][43],

Chain on a computation for when a value is ready, passing the successful results to the provided closure `f`. Read more

§

#### fn or_else<Fut, F>(self, f: F) -> OrElse<Self, Fut, F>

where F: [FnMut][74](Self::Error) -> Fut, Fut: TryFuture<Ok = Self::Ok>, Self: [Sized][43],

Chain on a computation for when an error happens, passing the erroneous result to the provided closure `f`. Read more

§

#### fn inspect_ok<F>(self, f: F) -> InspectOk<Self, F>

where F: [FnMut][74](&Self::Ok), Self: [Sized][43],

Do something with the success value of this stream, afterwards passing it on. Read more

§

#### fn inspect_err<F>(self, f: F) -> InspectErr<Self, F>

where F: [FnMut][74](&Self::Error), Self: [Sized][43],

Do something with the error value of this stream, afterwards passing it on. Read more

§

#### fn into_stream(self) -> IntoStream<Self>

where Self: [Sized][43],

Wraps a [`TryStream`] into a type that implements [`Stream`][99] Read more

§

#### fn try_next(&mut self) -> TryNext<'_, Self>

where Self: [Unpin][39],

Creates a future that attempts to resolve the next item in the stream. If an error is encountered before the next item, the error is returned instead. Read more

§

#### fn try_for_each<Fut, F>(self, f: F) -> TryForEach<Self, Fut, F>

where F: [FnMut][74](Self::Ok) -> Fut, Fut: TryFuture<Ok = [()][82], Error = Self::Error>, Self: [Sized][43],

Attempts to run this stream to completion, executing the provided asynchronous closure for each element on the stream. Read more

§

#### fn try_skip_while<Fut, F>(self, f: F) -> TrySkipWhile<Self, Fut, F>

where F: [FnMut][74](&Self::Ok) -> Fut, Fut: TryFuture<Ok = [bool][76], Error = Self::Error>, Self: [Sized][43],

Skip elements on this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn try_take_while<Fut, F>(self, f: F) -> TryTakeWhile<Self, Fut, F>

where F: [FnMut][74](&Self::Ok) -> Fut, Fut: TryFuture<Ok = [bool][76], Error = Self::Error>, Self: [Sized][43],

Take elements on this stream while the provided asynchronous predicate resolves to `true`. Read more

§

#### fn try_for_each_concurrent<Fut, F>( self, limit: impl [Into][63]<[Option][9]<[usize][34]>>, f: F, ) -> TryForEachConcurrent<Self, Fut, F>

where F: [FnMut][74](Self::Ok) -> Fut, Fut: [Future][75]<Output = [Result][17]<[()][82], Self::Error>>, Self: [Sized][43],

Available on **crate feature`alloc`** only.

Attempts to run this stream to completion, executing the provided asynchronous closure for each element on the stream concurrently as elements become available, exiting as soon as an error occurs. Read more

§

#### fn try_collect<C>(self) -> TryCollect<Self, C>

where C: [Default][77] \+ [Extend][78]<Self::Ok>, Self: [Sized][43],

Attempt to transform a stream into a collection, returning a future representing the result of that computation. Read more

§

#### fn try_chunks(self, capacity: [usize][34]) -> TryChunks<Self>

where Self: [Sized][43],

Available on **crate feature`alloc`** only.

An adaptor for chunking up successful items of the stream inside a vector. Read more

§

#### fn try_ready_chunks(self, capacity: [usize][34]) -> TryReadyChunks<Self>

where Self: [Sized][43],

Available on **crate feature`alloc`** only.

An adaptor for chunking up successful, ready items of the stream inside a vector. Read more

§

#### fn try_filter<Fut, F>(self, f: F) -> TryFilter<Self, Fut, F>

where Fut: [Future][75]<Output = [bool][76]>, F: [FnMut][74](&Self::Ok) -> Fut, Self: [Sized][43],

Attempt to filter the values produced by this stream according to the provided asynchronous closure. Read more

§

#### fn try_filter_map<Fut, F, T>(self, f: F) -> TryFilterMap<Self, Fut, F>

where Fut: TryFuture<Ok = [Option][9]<T>, Error = Self::Error>, F: [FnMut][74](Self::Ok) -> Fut, Self: [Sized][43],

Attempt to filter the values produced by this stream while simultaneously mapping them to a different type according to the provided asynchronous closure. Read more

§

#### fn try_flatten_unordered( self, limit: impl [Into][63]<[Option][9]<[usize][34]>>, ) -> TryFlattenUnordered<Self>

where Self::Ok: TryStream + [Unpin][39], <Self::Ok as TryStream>::Error: [From][57]<Self::Error>, Self: [Sized][43],

Available on **crate feature`alloc`** only.

Flattens a stream of streams into just one continuous stream. Produced streams will be polled concurrently and any errors will be passed through without looking at them. If the underlying base stream returns an error, it will be **immediately** propagated. Read more

§

#### fn try_flatten(self) -> TryFlatten<Self>

where Self::Ok: TryStream, <Self::Ok as TryStream>::Error: [From][57]<Self::Error>, Self: [Sized][43],

Flattens a stream of streams into just one continuous stream. Read more

§

#### fn try_fold<T, Fut, F>(self, init: T, f: F) -> TryFold<Self, Fut, T, F>

where F: [FnMut][74](T, Self::Ok) -> Fut, Fut: TryFuture<Ok = T, Error = Self::Error>, Self: [Sized][43],

Attempt to execute an accumulating asynchronous computation over a stream, collecting all the values into one final result. Read more

§

#### fn try_concat(self) -> TryConcat<Self>

where Self: [Sized][43], Self::Ok: [Extend][78]<<Self::Ok as [IntoIterator][72]>::[Item][80]> \+ [IntoIterator][72] \+ [Default][77],

Attempt to concatenate all items of a stream into a single extendable destination, returning a future representing the end result. Read more

§

#### fn try_buffer_unordered(self, n: [usize][34]) -> TryBufferUnordered<Self>

where Self::Ok: TryFuture<Error = Self::Error>, Self: [Sized][43],

Available on **crate feature`alloc`** only.

Attempt to execute several futures from a stream concurrently (unordered). Read more

§

#### fn try_buffered(self, n: [usize][34]) -> TryBuffered<Self>

where Self::Ok: TryFuture<Error = Self::Error>, Self: [Sized][43],

Available on **crate feature`alloc`** only.

Attempt to execute several futures from a stream concurrently. Read more

§

#### fn try_poll_next_unpin( &mut self, cx: &mut [Context][32]<'_>, ) -> [Poll][33]<[Option][9]<[Result][17]<Self::Ok, Self::Error>>>

where Self: [Unpin][39],

A convenience method for calling [`TryStream::try_poll_next`] on [`Unpin`][39] stream types.

§

#### fn try_all<Fut, F>(self, f: F) -> TryAll<Self, Fut, F>

where Self: [Sized][43], F: [FnMut][74](Self::Ok) -> Fut, Fut: [Future][75]<Output = [bool][76]>,

Attempt to execute a predicate over an asynchronous stream and evaluate if all items satisfy the predicate. Exits early if an `Err` is encountered or if an `Ok` item is found that does not satisfy the predicate. Read more

§

#### fn try_any<Fut, F>(self, f: F) -> TryAny<Self, Fut, F>

where Self: [Sized][43], F: [FnMut][74](Self::Ok) -> Fut, Fut: [Future][75]<Output = [bool][76]>,

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

where S: [Into][63]<Dispatch>,

Attaches the provided [`Subscriber`][100] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][101] [`Subscriber`][100] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/multipart.rs.html#128-133
   [6]: ../../../src/axum/extract/multipart.rs.html#145-231
   [7]: struct.Field.html (struct axum::extract::multipart::Field)
   [8]: ../../../src/axum/extract/multipart.rs.html#150-152
   [9]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [10]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [11]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition
   [12]: ../../../src/axum/extract/multipart.rs.html#158-160
   [13]: ../../../src/axum/extract/multipart.rs.html#164-166
   [14]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type
   [15]: ../../../src/axum/extract/multipart.rs.html#170-172
   [16]: ../../../src/axum/extract/multipart.rs.html#175-180
   [17]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [18]: struct.MultipartError.html (struct axum::extract::multipart::MultipartError)
   [19]: ../../../src/axum/extract/multipart.rs.html#183-185
   [20]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [21]: ../../../src/axum/extract/multipart.rs.html#225-230
   [22]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None (variant core::option::Option::None)
   [23]: ../../../src/axum/extract/multipart.rs.html#127
   [24]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [25]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [26]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [27]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [28]: ../../../src/axum/extract/multipart.rs.html#135-143
   [29]: ../../../src/axum/extract/multipart.rs.html#136
   [30]: ../../../src/axum/extract/multipart.rs.html#138-142
   [31]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [32]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [33]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [34]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [36]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [41]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [42]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [44]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [45]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [46]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [51]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [57]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [60]: super::Span::current()
   [61]: crate::Span
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [66]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [67]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [68]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [69]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [70]: https://docs.rs/http/latest/http/struct.Extensions.html
   [71]: crate::follow_redirect::policy::Standard
   [72]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [73]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [74]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html (trait core::ops::function::FnMut)
   [75]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [76]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [77]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [78]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html (trait core::iter::traits::collect::Extend)
   [79]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [80]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item (type core::iter::traits::collect::IntoIterator::Item)
   [81]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [82]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [83]: Stream::poll_next
   [84]: https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html (struct alloc::boxed::Box)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [89]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [92]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [93]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [94]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [95]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [96]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [97]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [98]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [99]: futures_core::stream::Stream
   [100]: super::Subscriber
   [101]: dispatcher#setting-the-default-subscriber

