<!-- Generated from rustdoc HTML: serve/struct.ConnLimiterIo.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ConnLimiterIo

## [axum][1]0.8.8

## ConnLimiterIo

### Trait Implementations

  * AsyncRead
  * AsyncWrite
  * Debug
  * Unpin



### Auto Trait Implementations

  * Freeze
  * RefUnwindSafe
  * Send
  * Sync
  * UnwindSafe



### Blanket Implementations

  * Any
  * AsyncReadExt
  * AsyncWriteExt
  * Borrow<T>
  * BorrowMut<T>
  * From<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::serve][2]

[axum][3]::[serve][2]

# Struct ConnLimiterIo Copy item path

[Source][4]
``` 
pub struct ConnLimiterIo<T> { /* private fields */ }
```

Available on **crate feature`tokio` and (crate features `http1` or `http2`)** only.

Expand description

A connection counted by [`ConnLimiter`][5].

See [`ListenerExt::limit_connections`][6] for details.

## Trait Implementations§

[Source][7]§

### impl<T: AsyncRead> AsyncRead for [ConnLimiterIo][8]<T>

[Source][9]§

#### fn poll_read( self: [Pin][10]<&mut Self>, cx: &mut [Context][11]<'_>, buf: &mut ReadBuf<'_>, ) -> [Poll][12]<[Result][13]<[()][14]>>

Attempts to read from the `AsyncRead` into `buf`. Read more

[Source][15]§

### impl<T: AsyncWrite> AsyncWrite for [ConnLimiterIo][8]<T>

[Source][16]§

#### fn is_write_vectored(&self) -> [bool][17]

Determines if this writer has an efficient [`poll_write_vectored`][18] implementation. Read more

[Source][19]§

#### fn poll_flush(self: [Pin][10]<&mut Self>, cx: &mut [Context][11]<'_>) -> [Poll][12]<[Result][13]<[()][14]>>

Attempts to flush the object, ensuring that any buffered data reach their destination. Read more

[Source][20]§

#### fn poll_shutdown(self: [Pin][10]<&mut Self>, cx: &mut [Context][11]<'_>) -> [Poll][12]<[Result][13]<[()][14]>>

Initiates or attempts to shut down this writer, returning success when the I/O connection has completely shut down. Read more

[Source][21]§

#### fn poll_write( self: [Pin][10]<&mut Self>, cx: &mut [Context][11]<'_>, buf: &[[u8][22]], ) -> [Poll][12]<[Result][13]<[usize][23]>>

Attempt to write bytes from `buf` into the object. Read more

[Source][24]§

#### fn poll_write_vectored( self: [Pin][10]<&mut Self>, cx: &mut [Context][11]<'_>, bufs: &[[IoSlice][25]<'_>], ) -> [Poll][12]<[Result][13]<[usize][23]>>

Like [`poll_write`][26], except that it writes from a slice of buffers. Read more

[Source][27]§

### impl<T: [Debug][28]> [Debug][28] for [ConnLimiterIo][8]<T>

[Source][27]§

#### fn [fmt][29](&self, f: &mut [Formatter][30]<'_>) -> [Result][31]

Formats the value using the given formatter. [Read more][29]

[Source][4]§

### impl<'__pin, T> [Unpin][32] for [ConnLimiterIo][8]<T>

where PinnedFieldsOf<__Origin<'__pin, T>>: [Unpin][32],

## Auto Trait Implementations§

§

### impl<T> [Freeze][33] for [ConnLimiterIo][8]<T>

where T: [Freeze][33],

§

### impl<T> [RefUnwindSafe][34] for [ConnLimiterIo][8]<T>

where T: [RefUnwindSafe][34],

§

### impl<T> [Send][35] for [ConnLimiterIo][8]<T>

where T: [Send][35],

§

### impl<T> [Sync][36] for [ConnLimiterIo][8]<T>

where T: [Sync][36],

§

### impl<T> [UnwindSafe][37] for [ConnLimiterIo][8]<T>

where T: [UnwindSafe][37],

## Blanket Implementations§

[Source][38]§

### impl<T> [Any][39] for T

where T: 'static + ?[Sized][40],

[Source][41]§

#### fn [type_id][42](&self) -> [TypeId][43]

Gets the `TypeId` of `self`. [Read more][42]

§

### impl<R> AsyncReadExt for R

where R: AsyncRead + ?[Sized][40],

§

#### fn chain<R>(self, next: R) -> Chain<Self, R>

where Self: [Sized][40], R: AsyncRead,

Creates a new `AsyncRead` instance that chains this stream with `next`. Read more

§

#### fn read<'a>(&'a mut self, buf: &'a mut [[u8][22]]) -> Read<'a, Self>

where Self: [Unpin][32],

Pulls some bytes from this source into the specified buffer, returning how many bytes were read. Read more

§

#### fn read_buf<'a, B>(&'a mut self, buf: [&'a mut B][44]) -> ReadBuf<'a, Self, B>

where Self: [Unpin][32], B: BufMut + ?[Sized][40],

Pulls some bytes from this source into the specified buffer, advancing the buffer’s internal cursor. Read more

§

#### fn read_exact<'a>(&'a mut self, buf: &'a mut [[u8][22]]) -> ReadExact<'a, Self>

where Self: [Unpin][32],

Reads the exact number of bytes required to fill `buf`. Read more

§

#### fn read_u8(&mut self) -> ReadU8<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 8 bit integer from the underlying reader. Read more

§

#### fn read_i8(&mut self) -> ReadI8<&mut Self>

where Self: [Unpin][32],

Reads a signed 8 bit integer from the underlying reader. Read more

§

#### fn read_u16(&mut self) -> ReadU16<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 16-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_i16(&mut self) -> ReadI16<&mut Self>

where Self: [Unpin][32],

Reads a signed 16-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_u32(&mut self) -> ReadU32<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 32-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_i32(&mut self) -> ReadI32<&mut Self>

where Self: [Unpin][32],

Reads a signed 32-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_u64(&mut self) -> ReadU64<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 64-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_i64(&mut self) -> ReadI64<&mut Self>

where Self: [Unpin][32],

Reads an signed 64-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_u128(&mut self) -> ReadU128<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 128-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_i128(&mut self) -> ReadI128<&mut Self>

where Self: [Unpin][32],

Reads an signed 128-bit integer in big-endian order from the underlying reader. Read more

§

#### fn read_f32(&mut self) -> ReadF32<&mut Self>

where Self: [Unpin][32],

Reads an 32-bit floating point type in big-endian order from the underlying reader. Read more

§

#### fn read_f64(&mut self) -> ReadF64<&mut Self>

where Self: [Unpin][32],

Reads an 64-bit floating point type in big-endian order from the underlying reader. Read more

§

#### fn read_u16_le(&mut self) -> ReadU16Le<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 16-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_i16_le(&mut self) -> ReadI16Le<&mut Self>

where Self: [Unpin][32],

Reads a signed 16-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_u32_le(&mut self) -> ReadU32Le<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 32-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_i32_le(&mut self) -> ReadI32Le<&mut Self>

where Self: [Unpin][32],

Reads a signed 32-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_u64_le(&mut self) -> ReadU64Le<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 64-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_i64_le(&mut self) -> ReadI64Le<&mut Self>

where Self: [Unpin][32],

Reads an signed 64-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_u128_le(&mut self) -> ReadU128Le<&mut Self>

where Self: [Unpin][32],

Reads an unsigned 128-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_i128_le(&mut self) -> ReadI128Le<&mut Self>

where Self: [Unpin][32],

Reads an signed 128-bit integer in little-endian order from the underlying reader. Read more

§

#### fn read_f32_le(&mut self) -> ReadF32Le<&mut Self>

where Self: [Unpin][32],

Reads an 32-bit floating point type in little-endian order from the underlying reader. Read more

§

#### fn read_f64_le(&mut self) -> ReadF64Le<&mut Self>

where Self: [Unpin][32],

Reads an 64-bit floating point type in little-endian order from the underlying reader. Read more

§

#### fn read_to_end<'a>(&'a mut self, buf: &'a mut [Vec][45]<[u8][22]>) -> ReadToEnd<'a, Self>

where Self: [Unpin][32],

Reads all bytes until EOF in this source, placing them into `buf`. Read more

§

#### fn read_to_string<'a>( &'a mut self, dst: &'a mut [String][46], ) -> ReadToString<'a, Self>

where Self: [Unpin][32],

Reads all bytes until EOF in this source, appending them to `buf`. Read more

§

#### fn take(self, limit: [u64][47]) -> Take<Self>

where Self: [Sized][40],

Creates an adaptor which reads at most `limit` bytes from it. Read more

§

### impl<W> AsyncWriteExt for W

where W: AsyncWrite + ?[Sized][40],

§

#### fn write<'a>(&'a mut self, src: &'a [[u8][22]]) -> Write<'a, Self>

where Self: [Unpin][32],

Writes a buffer into this writer, returning how many bytes were written. Read more

§

#### fn write_vectored<'a, 'b>( &'a mut self, bufs: &'a [[IoSlice][25]<'b>], ) -> WriteVectored<'a, 'b, Self>

where Self: [Unpin][32],

Like [`write`][48], except that it writes from a slice of buffers. Read more

§

#### fn write_buf<'a, B>(&'a mut self, src: [&'a mut B][44]) -> WriteBuf<'a, Self, B>

where Self: [Sized][40] \+ [Unpin][32], B: Buf,

Writes a buffer into this writer, advancing the buffer’s internal cursor. Read more

§

#### fn write_all_buf<'a, B>( &'a mut self, src: [&'a mut B][44], ) -> WriteAllBuf<'a, Self, B>

where Self: [Sized][40] \+ [Unpin][32], B: Buf,

Attempts to write an entire buffer into this writer. Read more

§

#### fn write_all<'a>(&'a mut self, src: &'a [[u8][22]]) -> WriteAll<'a, Self>

where Self: [Unpin][32],

Attempts to write an entire buffer into this writer. Read more

§

#### fn write_u8(&mut self, n: [u8][22]) -> WriteU8<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 8-bit integer to the underlying writer. Read more

§

#### fn write_i8(&mut self, n: [i8][49]) -> WriteI8<&mut Self>

where Self: [Unpin][32],

Writes a signed 8-bit integer to the underlying writer. Read more

§

#### fn write_u16(&mut self, n: [u16][50]) -> WriteU16<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 16-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_i16(&mut self, n: [i16][51]) -> WriteI16<&mut Self>

where Self: [Unpin][32],

Writes a signed 16-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_u32(&mut self, n: [u32][52]) -> WriteU32<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 32-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_i32(&mut self, n: [i32][53]) -> WriteI32<&mut Self>

where Self: [Unpin][32],

Writes a signed 32-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_u64(&mut self, n: [u64][47]) -> WriteU64<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 64-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_i64(&mut self, n: [i64][54]) -> WriteI64<&mut Self>

where Self: [Unpin][32],

Writes an signed 64-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_u128(&mut self, n: [u128][55]) -> WriteU128<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 128-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_i128(&mut self, n: [i128][56]) -> WriteI128<&mut Self>

where Self: [Unpin][32],

Writes an signed 128-bit integer in big-endian order to the underlying writer. Read more

§

#### fn write_f32(&mut self, n: [f32][57]) -> WriteF32<&mut Self>

where Self: [Unpin][32],

Writes an 32-bit floating point type in big-endian order to the underlying writer. Read more

§

#### fn write_f64(&mut self, n: [f64][58]) -> WriteF64<&mut Self>

where Self: [Unpin][32],

Writes an 64-bit floating point type in big-endian order to the underlying writer. Read more

§

#### fn write_u16_le(&mut self, n: [u16][50]) -> WriteU16Le<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 16-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_i16_le(&mut self, n: [i16][51]) -> WriteI16Le<&mut Self>

where Self: [Unpin][32],

Writes a signed 16-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_u32_le(&mut self, n: [u32][52]) -> WriteU32Le<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 32-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_i32_le(&mut self, n: [i32][53]) -> WriteI32Le<&mut Self>

where Self: [Unpin][32],

Writes a signed 32-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_u64_le(&mut self, n: [u64][47]) -> WriteU64Le<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 64-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_i64_le(&mut self, n: [i64][54]) -> WriteI64Le<&mut Self>

where Self: [Unpin][32],

Writes an signed 64-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_u128_le(&mut self, n: [u128][55]) -> WriteU128Le<&mut Self>

where Self: [Unpin][32],

Writes an unsigned 128-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_i128_le(&mut self, n: [i128][56]) -> WriteI128Le<&mut Self>

where Self: [Unpin][32],

Writes an signed 128-bit integer in little-endian order to the underlying writer. Read more

§

#### fn write_f32_le(&mut self, n: [f32][57]) -> WriteF32Le<&mut Self>

where Self: [Unpin][32],

Writes an 32-bit floating point type in little-endian order to the underlying writer. Read more

§

#### fn write_f64_le(&mut self, n: [f64][58]) -> WriteF64Le<&mut Self>

where Self: [Unpin][32],

Writes an 64-bit floating point type in little-endian order to the underlying writer. Read more

§

#### fn flush(&mut self) -> Flush<'_, Self>

where Self: [Unpin][32],

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. Read more

§

#### fn shutdown(&mut self) -> Shutdown<'_, Self>

where Self: [Unpin][32],

Shuts down the output stream, ensuring that the value can be dropped cleanly. Read more

[Source][59]§

### impl<T> [Borrow][60]<T> for T

where T: ?[Sized][40],

[Source][61]§

#### fn [borrow][62](&self) -> [&T][44]

Immutably borrows from an owned value. [Read more][62]

[Source][63]§

### impl<T> [BorrowMut][64]<T> for T

where T: ?[Sized][40],

[Source][65]§

#### fn [borrow_mut][66](&mut self) -> [&mut T][44]

Mutably borrows from an owned value. [Read more][66]

[Source][67]§

### impl<T> [From][68]<T> for T

[Source][69]§

#### fn [from][70](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][71] [`Span`][72], returning an `Instrumented` wrapper. Read more

[Source][73]§

### impl<T, U> [Into][74]<U> for T

where U: [From][68]<T>,

[Source][75]§

#### fn [into][76](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][68]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][40],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][77]§

### impl<T> [Same][78] for T

[Source][79]§

#### type [Output][80] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][40],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][40],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][81]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][40],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][40],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][40],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][40],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][40],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][40],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][40],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][82] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][83]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][40],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][84] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][83]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][40],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][84] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][83]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][40],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][84] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][40],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][40], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][40], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][40],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][40],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][40],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][23]) -> RequestBodyLimit<Self>

where Self: [Sized][40],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][40],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][40],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][85]§

### impl<T, U> [TryFrom][86]<U> for T

where U: [Into][74]<T>,

[Source][87]§

#### type [Error][88] = [Infallible][89]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_from][91](value: U) -> [Result][92]<T, <T as [TryFrom][86]<U>>::[Error][93]>

Performs the conversion.

[Source][94]§

### impl<T, U> [TryInto][95]<U> for T

where U: [TryFrom][86]<T>,

[Source][96]§

#### type [Error][97] = <U as [TryFrom][86]<T>>::[Error][93]

The type returned in the event of a conversion error.

[Source][98]§

#### fn [try_into][99](self) -> [Result][92]<U, <U as [TryFrom][86]<T>>::[Error][93]>

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

where S: [Into][74]<Dispatch>,

Attaches the provided [`Subscriber`][100] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][101] [`Subscriber`][100] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/serve/listener.rs.html#153-163
   [5]: struct.ConnLimiter.html (struct axum::serve::ConnLimiter)
   [6]: trait.ListenerExt.html#method.limit_connections (method axum::serve::ListenerExt::limit_connections)
   [7]: ../../src/axum/serve/listener.rs.html#166-174
   [8]: struct.ConnLimiterIo.html (struct axum::serve::ConnLimiterIo)
   [9]: ../../src/axum/serve/listener.rs.html#167-173
   [10]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [11]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [12]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [13]: https://doc.rust-lang.org/nightly/std/io/error/type.Result.html (type std::io::error::Result)
   [14]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [15]: ../../src/axum/serve/listener.rs.html#177-205
   [16]: ../../src/axum/serve/listener.rs.html#178-180
   [17]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [18]: AsyncWrite::poll_write_vectored
   [19]: ../../src/axum/serve/listener.rs.html#182-184
   [20]: ../../src/axum/serve/listener.rs.html#186-188
   [21]: ../../src/axum/serve/listener.rs.html#190-196
   [22]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [23]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [24]: ../../src/axum/serve/listener.rs.html#198-204
   [25]: https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html (struct std::io::IoSlice)
   [26]: AsyncWrite::poll_write
   [27]: ../../src/axum/serve/listener.rs.html#157
   [28]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [29]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [30]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [31]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [34]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [35]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [37]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [38]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [39]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [41]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [42]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [43]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [44]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [45]: https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html (struct alloc::vec::Vec)
   [46]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [47]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [48]: AsyncWriteExt::write
   [49]: https://doc.rust-lang.org/nightly/std/primitive.i8.html
   [50]: https://doc.rust-lang.org/nightly/std/primitive.u16.html
   [51]: https://doc.rust-lang.org/nightly/std/primitive.i16.html
   [52]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [53]: https://doc.rust-lang.org/nightly/std/primitive.i32.html
   [54]: https://doc.rust-lang.org/nightly/std/primitive.i64.html
   [55]: https://doc.rust-lang.org/nightly/std/primitive.u128.html
   [56]: https://doc.rust-lang.org/nightly/std/primitive.i128.html
   [57]: https://doc.rust-lang.org/nightly/std/primitive.f32.html
   [58]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [59]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [60]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [61]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [62]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [63]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [64]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [65]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [66]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [68]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [69]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [70]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [71]: super::Span::current()
   [72]: crate::Span
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [77]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [78]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [79]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [80]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [81]: https://docs.rs/http/latest/http/struct.Extensions.html
   [82]: crate::follow_redirect::policy::Standard
   [83]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [84]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [89]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [92]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [93]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [98]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [100]: super::Subscriber
   [101]: dispatcher#setting-the-default-subscriber

