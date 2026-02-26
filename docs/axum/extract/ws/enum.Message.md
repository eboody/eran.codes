<!-- Generated from rustdoc HTML: extract/ws/enum.Message.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Message

## [axum][1]0.8.8

## Message

### Variants

  * Binary
  * Close
  * Ping
  * Pong
  * Text



### Methods

  * binary
  * into_data
  * into_text
  * text
  * to_text



### Trait Implementations

  * Clone
  * Debug
  * Eq
  * From<&'b [u8]>
  * From<&'s str>
  * From<Bytes>
  * From<Message>
  * From<String>
  * From<Vec<u8>>
  * PartialEq
  * Sink<Message>
  * StructuralPartialEq



### Auto Trait Implementations

  * !Freeze
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
  * Equivalent<K>
  * Equivalent<K>
  * From<T>
  * FromRef<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Enum Message Copy item path

[Source][5]
``` 
pub enum Message {
    Text([Utf8Bytes][6]),
    Binary(Bytes),
    Ping(Bytes),
    Pong(Bytes),
    Close([Option][7]<[CloseFrame][8]>),
}
```

Available on **crate feature`ws`** only.

Expand description

A WebSocket message.

## Variants§

§

### Text([Utf8Bytes][6])

A text WebSocket message

§

### Binary(Bytes)

A binary WebSocket message

§

### Ping(Bytes)

A ping message with the specified payload

The payload here must have a length less than 125 bytes.

Ping messages will be automatically responded to by the server, so you do not have to worry about dealing with them yourself.

§

### Pong(Bytes)

A pong message with the specified payload

The payload here must have a length less than 125 bytes.

Pong messages will be automatically sent to the client if a ping message is received, so you do not have to worry about constructing them yourself unless you want to implement a [unidirectional heartbeat][9].

§

### Close([Option][7]<[CloseFrame][8]>)

A close message with the optional close frame.

You may “uncleanly” close a WebSocket connection at any time by simply dropping the [`WebSocket`][10]. However, you may also use the graceful closing protocol, in which

  1. peer A sends a close frame, and does not send any further messages;
  2. peer B responds with a close frame, and does not send any further messages;
  3. peer A processes the remaining messages sent by peer B, before finally
  4. both peers close the connection.



After sending a close frame, you may still read messages, but attempts to send another message will error. After receiving a close frame, axum will automatically respond with a close frame if necessary (you do not have to deal with this yourself). Since no further messages will be received, you may either do nothing or explicitly drop the connection.

## Implementations§

[Source][11]§

### impl [Message][12]

[Source][13]

#### pub fn into_data(self) -> Bytes

Consume the WebSocket and return it as binary data.

[Source][14]

#### pub fn into_text(self) -> [Result][15]<[Utf8Bytes][6], [Error][16]>

Attempt to consume the WebSocket message and convert it to a Utf8Bytes.

[Source][17]

#### pub fn to_text(&self) -> [Result][15]<&[str][18], [Error][16]>

Attempt to get a &str from the WebSocket message, this will try to convert binary data to utf8.

[Source][19]

#### pub fn text<S>(string: S) -> Self

where S: [Into][20]<[Utf8Bytes][6]>,

Create a new text WebSocket message from a stringable.

[Source][21]

#### pub fn binary<B>(bin: B) -> Self

where B: [Into][20]<Bytes>,

Create a new binary WebSocket message by converting to `Bytes`.

## Trait Implementations§

[Source][22]§

### impl [Clone][23] for [Message][12]

[Source][22]§

#### fn [clone][24](&self) -> [Message][12]

Returns a duplicate of the value. [Read more][24]

1.0.0 · [Source][25]§

#### fn [clone_from][26](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][26]

[Source][22]§

### impl [Debug][27] for [Message][12]

[Source][22]§

#### fn [fmt][28](&self, f: &mut [Formatter][29]<'_>) -> [Result][30]

Formats the value using the given formatter. [Read more][28]

[Source][31]§

### impl<'b> [From][32]<&'b [[u8][33]]> for [Message][12]

[Source][34]§

#### fn [from][35](data: &'b [[u8][33]]) -> Self

Converts to this type from the input type.

[Source][36]§

### impl<'s> [From][32]<&'s [str][18]> for [Message][12]

[Source][37]§

#### fn [from][35](string: &'s [str][18]) -> Self

Converts to this type from the input type.

[Source][38]§

### impl [From][32]<Bytes> for [Message][12]

[Source][39]§

#### fn [from][35](data: Bytes) -> Self

Converts to this type from the input type.

[Source][40]§

### impl [From][32]<[Message][12]> for [Vec][41]<[u8][33]>

[Source][42]§

#### fn [from][35](msg: [Message][12]) -> Self

Converts to this type from the input type.

[Source][43]§

### impl [From][32]<[String][44]> for [Message][12]

[Source][45]§

#### fn [from][35](string: [String][44]) -> Self

Converts to this type from the input type.

[Source][46]§

### impl [From][32]<[Vec][41]<[u8][33]>> for [Message][12]

[Source][47]§

#### fn [from][35](data: [Vec][41]<[u8][33]>) -> Self

Converts to this type from the input type.

[Source][22]§

### impl [PartialEq][48] for [Message][12]

[Source][22]§

#### fn [eq][49](&self, other: &[Message][12]) -> [bool][50]

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source][51]§

#### fn [ne][52](&self, other: [&Rhs][53]) -> [bool][50]

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source][54]§

### impl Sink<[Message][12]> for [WebSocket][10]

[Source][55]§

#### type Error = [Error][16]

The type of value produced by the sink when an error occurs.

[Source][56]§

#### fn poll_ready( self: [Pin][57]<&mut Self>, cx: &mut [Context][58]<'_>, ) -> [Poll][59]<[Result][15]<[()][60], Self::Error>>

Attempts to prepare the `Sink` to receive a value. Read more

[Source][61]§

#### fn start_send(self: [Pin][57]<&mut Self>, item: [Message][12]) -> [Result][15]<[()][60], Self::Error>

Begin the process of sending a value to the sink. Each call to this function must be preceded by a successful call to `poll_ready` which returned `Poll::Ready(Ok(()))`. Read more

[Source][62]§

#### fn poll_flush( self: [Pin][57]<&mut Self>, cx: &mut [Context][58]<'_>, ) -> [Poll][59]<[Result][15]<[()][60], Self::Error>>

Flush any remaining output from this sink. Read more

[Source][63]§

#### fn poll_close( self: [Pin][57]<&mut Self>, cx: &mut [Context][58]<'_>, ) -> [Poll][59]<[Result][15]<[()][60], Self::Error>>

Flush any remaining output and close this sink, if necessary. Read more

[Source][22]§

### impl [Eq][64] for [Message][12]

[Source][22]§

### impl [StructuralPartialEq][65] for [Message][12]

## Auto Trait Implementations§

§

### impl ![Freeze][66] for [Message][12]

§

### impl [RefUnwindSafe][67] for [Message][12]

§

### impl [Send][68] for [Message][12]

§

### impl [Sync][69] for [Message][12]

§

### impl [Unpin][70] for [Message][12]

§

### impl [UnwindSafe][71] for [Message][12]

## Blanket Implementations§

[Source][72]§

### impl<T> [Any][73] for T

where T: 'static + ?[Sized][74],

[Source][75]§

#### fn [type_id][76](&self) -> [TypeId][77]

Gets the `TypeId` of `self`. [Read more][76]

[Source][78]§

### impl<T> [Borrow][79]<T> for T

where T: ?[Sized][74],

[Source][80]§

#### fn [borrow][81](&self) -> [&T][53]

Immutably borrows from an owned value. [Read more][81]

[Source][82]§

### impl<T> [BorrowMut][83]<T> for T

where T: ?[Sized][74],

[Source][84]§

#### fn [borrow_mut][85](&mut self) -> [&mut T][53]

Mutably borrows from an owned value. [Read more][85]

[Source][86]§

### impl<T> [CloneToUninit][87] for T

where T: [Clone][23],

[Source][88]§

#### unsafe fn [clone_to_uninit][89](&self, dest: [*mut ][90][u8][33])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][89]

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][64] \+ ?[Sized][74], K: [Borrow][79]<Q> \+ ?[Sized][74],

§

#### fn equivalent(&self, key: [&K][53]) -> [bool][50]

Checks if this value is equivalent to the given key. Read more

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][64] \+ ?[Sized][74], K: [Borrow][79]<Q> \+ ?[Sized][74],

§

#### fn equivalent(&self, key: [&K][53]) -> [bool][50]

Compare self to `key` and return `true` if they are equal.

[Source][91]§

### impl<T> [From][32]<T> for T

[Source][92]§

#### fn [from][35](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][93]<T> for T

where T: [Clone][23],

§

#### fn [from_ref][94](input: [&T][53]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][95] [`Span`][96], returning an `Instrumented` wrapper. Read more

[Source][97]§

### impl<T, U> [Into][20]<U> for T

where U: [From][32]<T>,

[Source][98]§

#### fn [into][99](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][32]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][74],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][100]§

### impl<T> [Same][101] for T

[Source][102]§

#### type [Output][103] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][74],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][74],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][104]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][74],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][74],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][74],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][74],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][74],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][74],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][74],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][105] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][106]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][74],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][107] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][106]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][74],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][107] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][106]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][74],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][107] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][74],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][74],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][74],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][74],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][74],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][74],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][74], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][74], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][74],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][74],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][74],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][108]) -> RequestBodyLimit<Self>

where Self: [Sized][74],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][74],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][74],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][109]§

### impl<T> [ToOwned][110] for T

where T: [Clone][23],

[Source][111]§

#### type [Owned][112] = T

The resulting type after obtaining ownership.

[Source][113]§

#### fn [to_owned][114](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][114]

[Source][115]§

#### fn [clone_into][116](&self, target: [&mut T][53])

Uses borrowed data to replace owned data, usually by cloning. [Read more][116]

[Source][117]§

### impl<T, U> [TryFrom][118]<U> for T

where U: [Into][20]<T>,

[Source][119]§

#### type [Error][120] = [Infallible][121]

The type returned in the event of a conversion error.

[Source][122]§

#### fn [try_from][123](value: U) -> [Result][15]<T, <T as [TryFrom][118]<U>>::[Error][124]>

Performs the conversion.

[Source][125]§

### impl<T, U> [TryInto][126]<U> for T

where U: [TryFrom][118]<T>,

[Source][127]§

#### type [Error][128] = <U as [TryFrom][118]<T>>::[Error][124]

The type returned in the event of a conversion error.

[Source][129]§

#### fn [try_into][130](self) -> [Result][15]<U, <U as [TryFrom][118]<T>>::[Error][124]>

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

where S: [Into][20]<Dispatch>,

Attaches the provided [`Subscriber`][131] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][132] [`Subscriber`][131] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/ws.rs.html#769-809
   [6]: struct.Utf8Bytes.html (struct axum::extract::ws::Utf8Bytes)
   [7]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [8]: struct.CloseFrame.html (struct axum::extract::ws::CloseFrame)
   [9]: https://tools.ietf.org/html/rfc6455#section-5.5.3
   [10]: struct.WebSocket.html (struct axum::extract::ws::WebSocket)
   [11]: ../../../src/axum/extract/ws.rs.html#811-893
   [12]: enum.Message.html (enum axum::extract::ws::Message)
   [13]: ../../../src/axum/extract/ws.rs.html#844-851
   [14]: ../../../src/axum/extract/ws.rs.html#854-863
   [15]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [16]: ../../struct.Error.html (struct axum::Error)
   [17]: ../../../src/axum/extract/ws.rs.html#867-876
   [18]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [19]: ../../../src/axum/extract/ws.rs.html#879-884
   [20]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [21]: ../../../src/axum/extract/ws.rs.html#887-892
   [22]: ../../../src/axum/extract/ws.rs.html#768
   [23]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [24]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [25]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [26]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [27]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [28]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [29]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [30]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [31]: ../../../src/axum/extract/ws.rs.html#907-911
   [32]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [33]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [34]: ../../../src/axum/extract/ws.rs.html#908-910
   [35]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [36]: ../../../src/axum/extract/ws.rs.html#901-905
   [37]: ../../../src/axum/extract/ws.rs.html#902-904
   [38]: ../../../src/axum/extract/ws.rs.html#913-917
   [39]: ../../../src/axum/extract/ws.rs.html#914-916
   [40]: ../../../src/axum/extract/ws.rs.html#925-929
   [41]: https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html (struct alloc::vec::Vec)
   [42]: ../../../src/axum/extract/ws.rs.html#926-928
   [43]: ../../../src/axum/extract/ws.rs.html#895-899
   [44]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [45]: ../../../src/axum/extract/ws.rs.html#896-898
   [46]: ../../../src/axum/extract/ws.rs.html#919-923
   [47]: ../../../src/axum/extract/ws.rs.html#920-922
   [48]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html (trait core::cmp::PartialEq)
   [49]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq
   [50]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [51]: https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264
   [52]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne
   [53]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [54]: ../../../src/axum/extract/ws.rs.html#595-615
   [55]: ../../../src/axum/extract/ws.rs.html#596
   [56]: ../../../src/axum/extract/ws.rs.html#598-600
   [57]: https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html (struct core::pin::Pin)
   [58]: https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html (struct core::task::wake::Context)
   [59]: https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html (enum core::task::poll::Poll)
   [60]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [61]: ../../../src/axum/extract/ws.rs.html#602-606
   [62]: ../../../src/axum/extract/ws.rs.html#608-610
   [63]: ../../../src/axum/extract/ws.rs.html#612-614
   [64]: https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html (trait core::cmp::Eq)
   [65]: https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html (trait core::marker::StructuralPartialEq)
   [66]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [67]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [68]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [69]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [70]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [71]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [72]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [73]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [74]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [75]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [76]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [77]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [78]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [79]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [80]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [81]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [82]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [83]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [84]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [85]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [86]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [87]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [88]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [89]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [90]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [91]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [92]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [93]: ../trait.FromRef.html (trait axum::extract::FromRef)
   [94]: ../trait.FromRef.html#tymethod.from_ref
   [95]: super::Span::current()
   [96]: crate::Span
   [97]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [98]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [100]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [101]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [102]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [103]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [104]: https://docs.rs/http/latest/http/struct.Extensions.html
   [105]: crate::follow_redirect::policy::Standard
   [106]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [107]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [108]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [109]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [110]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [111]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [112]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [113]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [114]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [115]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [116]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [117]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [118]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [119]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [120]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [121]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [122]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [123]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [124]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [125]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [126]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [127]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [128]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [129]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [130]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [131]: super::Subscriber
   [132]: dispatcher#setting-the-default-subscriber

