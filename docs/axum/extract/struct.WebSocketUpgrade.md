<!-- Generated from rustdoc HTML: extract/struct.WebSocketUpgrade.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## WebSocketUpgrade

## [axum][1]0.8.8

## WebSocketUpgrade

### Methods

  * accept_unmasked_frames
  * max_frame_size
  * max_message_size
  * max_write_buffer_size
  * on_failed_upgrade
  * on_upgrade
  * protocols
  * read_buffer_size
  * requested_protocols
  * selected_protocol
  * set_selected_protocol
  * write_buffer_size



### Trait Implementations

  * Debug
  * FromRequestParts<S>



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
  * From<T>
  * FromRequest<S, ViaParts>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct WebSocketUpgrade Copy item path

[Source][4]
``` 
pub struct WebSocketUpgrade<F = [DefaultOnFailedUpgrade][5]> { /* private fields */ }
```

Available on **crate feature`ws`** only.

Expand description

Extractor for establishing WebSocket connections.

For HTTP/1.1 requests, this extractor requires the request method to be `GET`; in later versions, `CONNECT` is used instead. To support both, it should be used with [`any`][6].

See the [module docs][7] for an example.

## Implementations§

[Source][8]§

### impl<F> [WebSocketUpgrade][9]<F>

[Source][10]

#### pub fn read_buffer_size(self, size: [usize][11]) -> Self

Read buffer capacity. The default value is 128KiB

[Source][12]

#### pub fn write_buffer_size(self, size: [usize][11]) -> Self

The target minimum size of the write buffer to reach before writing the data to the underlying stream.

The default value is 128 KiB.

If set to `0` each message will be eagerly written to the underlying stream. It is often more optimal to allow them to buffer a little, hence the default value.

Note: [`flush`][13] will always fully write the buffer regardless.

[Source][14]

#### pub fn max_write_buffer_size(self, max: [usize][11]) -> Self

The max size of the write buffer in bytes. Setting this can provide backpressure in the case the write buffer is filling up due to write errors.

The default value is unlimited.

Note: The write buffer only builds up past [`write_buffer_size`][15] when writes to the underlying stream are failing. So the **write buffer can not fill up if you are not observing write errors even if not flushing**.

Note: Should always be at least [`write_buffer_size + 1 message`][15] and probably a little more depending on error handling strategy.

[Source][16]

#### pub fn max_message_size(self, max: [usize][11]) -> Self

Set the maximum message size (defaults to 64 megabytes)

[Source][17]

#### pub fn max_frame_size(self, max: [usize][11]) -> Self

Set the maximum frame size (defaults to 16 megabytes)

[Source][18]

#### pub fn accept_unmasked_frames(self, accept: [bool][19]) -> Self

Allow server to accept unmasked frames (defaults to false)

[Source][20]

#### pub fn protocols<I>(self, protocols: I) -> Self

where I: [IntoIterator][21], I::[Item][22]: [Into][23]<[Cow][24]<'static, [str][25]>>,

Set the known protocols.

If the protocol name specified by `Sec-WebSocket-Protocol` header to match any of them, the upgrade response will include `Sec-WebSocket-Protocol` header and return the protocol name.

The protocols should be listed in decreasing order of preference: if the client offers multiple protocols that the server could support, the server will pick the first one in this list.

##### §Examples
``` 
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::any,
    response::{IntoResponse, Response},
    Router,
};

let app = Router::new().route("/ws", any(handler));

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.protocols(["graphql-ws", "graphql-transport-ws"])
        .on_upgrade(|socket| async {
            // ...
        })
}
```

[Source][26]

#### pub fn requested_protocols(&self) -> impl [Iterator][27]<Item = &HeaderValue>

Return the WebSocket subprotocols requested by the client.

##### §Examples

If the client sends the following HTTP header in the WebSocket upgrade request:
``` 
Sec-WebSocket-Protocol: soap, wamp
```

this method returns an iterator yielding `"soap"` and `"wamp"`.

[Source][28]

#### pub fn set_selected_protocol(&mut self, protocol: HeaderValue)

Set the chosen WebSocket subprotocol.

Another method, [`protocols()`][29], also sets the chosen WebSocket subprotocol. If both methods are called, only the latter call takes effect.

##### §Notes

  * The chosen protocol is echoed back in the WebSocket upgrade response as required by RFC 6455. Some browsers may reject a value that was not present in the client’s request.



[Source][30]

#### pub fn selected_protocol(&self) -> [Option][31]<&HeaderValue>

Return the selected WebSocket subprotocol, if one has been chosen.

If [`protocols()`][29] selects a matching protocol, or [`set_selected_protocol()`][32] has been called, the return value will be `Some` containing the selected protocol. Otherwise, it will be `None`.

[Source][33]

#### pub fn on_failed_upgrade<C>(self, callback: C) -> [WebSocketUpgrade][9]<C>

where C: [OnFailedUpgrade][34],

Provide a callback to call if upgrading the connection fails.

The connection upgrade is performed in a background task. If that fails this callback will be called.

By default any errors will be silently ignored.

##### §Example
``` 
use axum::{
    extract::{WebSocketUpgrade},
    response::Response,
};

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_failed_upgrade(|error| {
        report_error(error);
    })
    .on_upgrade(|socket| async { /* ... */ })
}
```

[Source][35]

#### pub fn on_upgrade<C, Fut>(self, callback: C) -> [Response][36]

where C: [FnOnce][37]([WebSocket][38]) -> Fut + [Send][39] \+ 'static, Fut: [Future][40]<Output = [()][41]> \+ [Send][39] \+ 'static, F: [OnFailedUpgrade][34],

Finalize upgrading the connection and call the provided callback with the stream.

## Trait Implementations§

[Source][42]§

### impl<F> [Debug][43] for [WebSocketUpgrade][9]<F>

[Source][44]§

#### fn [fmt][45](&self, f: &mut [Formatter][46]<'_>) -> [Result][47]

Formats the value using the given formatter. [Read more][45]

[Source][48]§

### impl<S> [FromRequestParts][49]<S> for [WebSocketUpgrade][9]<[DefaultOnFailedUpgrade][5]>

where S: [Send][39] \+ [Sync][50],

[Source][51]§

#### type [Rejection][52] = [WebSocketUpgradeRejection][53]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][54]§

#### async fn [from_request_parts][55]( parts: &mut Parts, _state: [&S][56], ) -> [Result][57]<Self, Self::[Rejection][58]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl<F = [DefaultOnFailedUpgrade][5]> ![Freeze][59] for [WebSocketUpgrade][9]<F>

§

### impl<F> [RefUnwindSafe][60] for [WebSocketUpgrade][9]<F>

where F: [RefUnwindSafe][60],

§

### impl<F> [Send][39] for [WebSocketUpgrade][9]<F>

where F: [Send][39],

§

### impl<F> [Sync][50] for [WebSocketUpgrade][9]<F>

where F: [Sync][50],

§

### impl<F> [Unpin][61] for [WebSocketUpgrade][9]<F>

where F: [Unpin][61],

§

### impl<F> [UnwindSafe][62] for [WebSocketUpgrade][9]<F>

where F: [UnwindSafe][62],

## Blanket Implementations§

[Source][63]§

### impl<T> [Any][64] for T

where T: 'static + ?[Sized][65],

[Source][66]§

#### fn [type_id][67](&self) -> [TypeId][68]

Gets the `TypeId` of `self`. [Read more][67]

[Source][69]§

### impl<T> [Borrow][70]<T> for T

where T: ?[Sized][65],

[Source][71]§

#### fn [borrow][72](&self) -> [&T][56]

Immutably borrows from an owned value. [Read more][72]

[Source][73]§

### impl<T> [BorrowMut][74]<T> for T

where T: ?[Sized][65],

[Source][75]§

#### fn [borrow_mut][76](&mut self) -> [&mut T][56]

Mutably borrows from an owned value. [Read more][76]

[Source][77]§

### impl<T> [From][78]<T> for T

[Source][79]§

#### fn [from][80](t: T) -> T

Returns the argument unchanged.

§

### impl<S, T> [FromRequest][81]<S, ViaParts> for T

where S: [Send][39] \+ [Sync][50], T: [FromRequestParts][49]<S>,

§

#### type [Rejection][82] = <T as [FromRequestParts][49]<S>>::[Rejection][58]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][83]( req: Request<[Body][84]>, state: [&S][56], ) -> impl [Future][40]<Output = [Result][57]<T, <T as [FromRequest][81]<S, ViaParts>>::[Rejection][85]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][86] [`Span`][87], returning an `Instrumented` wrapper. Read more

[Source][88]§

### impl<T, U> [Into][23]<U> for T

where U: [From][78]<T>,

[Source][89]§

#### fn [into][90](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][78]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][65],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][91]§

### impl<T> [Same][92] for T

[Source][93]§

#### type [Output][94] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][65],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][65],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][95]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][65],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][65],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][65],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][65],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][65],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][65],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][65],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][96] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][21]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][65],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][97] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][21]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][65],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][97] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][21]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][65],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][97] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][65],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][65],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][65],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][65],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][65],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][65],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][65], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][65], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][65],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][65],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][65],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][11]) -> RequestBodyLimit<Self>

where Self: [Sized][65],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][65],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][65],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][98]§

### impl<T, U> [TryFrom][99]<U> for T

where U: [Into][23]<T>,

[Source][100]§

#### type [Error][101] = [Infallible][102]

The type returned in the event of a conversion error.

[Source][103]§

#### fn [try_from][104](value: U) -> [Result][57]<T, <T as [TryFrom][99]<U>>::[Error][105]>

Performs the conversion.

[Source][106]§

### impl<T, U> [TryInto][107]<U> for T

where U: [TryFrom][99]<T>,

[Source][108]§

#### type [Error][109] = <U as [TryFrom][99]<T>>::[Error][105]

The type returned in the event of a conversion error.

[Source][110]§

#### fn [try_into][111](self) -> [Result][57]<U, <U as [TryFrom][99]<T>>::[Error][105]>

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

where S: [Into][23]<Dispatch>,

Attaches the provided [`Subscriber`][112] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][113] [`Subscriber`][112] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/ws.rs.html#134-143
   [5]: ws/struct.DefaultOnFailedUpgrade.html (struct axum::extract::ws::DefaultOnFailedUpgrade)
   [6]: ../routing/method_routing/fn.any.html (fn axum::routing::method_routing::any)
   [7]: ws/index.html (mod axum::extract::ws)
   [8]: ../../src/axum/extract/ws.rs.html#156-411
   [9]: struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [10]: ../../src/axum/extract/ws.rs.html#158-161
   [11]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [12]: ../../src/axum/extract/ws.rs.html#172-175
   [13]: SinkExt::flush
   [14]: ../../src/axum/extract/ws.rs.html#188-191
   [15]: struct.WebSocketUpgrade.html#method.write_buffer_size (method axum::extract::WebSocketUpgrade::write_buffer_size)
   [16]: ../../src/axum/extract/ws.rs.html#194-197
   [17]: ../../src/axum/extract/ws.rs.html#200-203
   [18]: ../../src/axum/extract/ws.rs.html#206-209
   [19]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [20]: ../../src/axum/extract/ws.rs.html#241-265
   [21]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [22]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item (type core::iter::traits::collect::IntoIterator::Item)
   [23]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [24]: https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html (enum alloc::borrow::Cow)
   [25]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [26]: ../../src/axum/extract/ws.rs.html#278-280
   [27]: https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html (trait core::iter::traits::iterator::Iterator)
   [28]: ../../src/axum/extract/ws.rs.html#292-294
   [29]: struct.WebSocketUpgrade.html#method.protocols (method axum::extract::WebSocketUpgrade::protocols)
   [30]: ../../src/axum/extract/ws.rs.html#301-303
   [31]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [32]: struct.WebSocketUpgrade.html#method.set_selected_protocol (method axum::extract::WebSocketUpgrade::set_selected_protocol)
   [33]: ../../src/axum/extract/ws.rs.html#329-341
   [34]: ws/trait.OnFailedUpgrade.html (trait axum::extract::ws::OnFailedUpgrade)
   [35]: ../../src/axum/extract/ws.rs.html#346-410
   [36]: ../response/type.Response.html (type axum::response::Response)
   [37]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [38]: ws/struct.WebSocket.html (struct axum::extract::ws::WebSocket)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [40]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [41]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [42]: ../../src/axum/extract/ws.rs.html#145-154
   [43]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [44]: ../../src/axum/extract/ws.rs.html#146-153
   [45]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [46]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [47]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [48]: ../../src/axum/extract/ws.rs.html#442-517
   [49]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [50]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [51]: ../../src/axum/extract/ws.rs.html#446
   [52]: trait.FromRequestParts.html#associatedtype.Rejection
   [53]: ws/rejection/enum.WebSocketUpgradeRejection.html (enum axum::extract::ws::rejection::WebSocketUpgradeRejection)
   [54]: ../../src/axum/extract/ws.rs.html#448-516
   [55]: trait.FromRequestParts.html#tymethod.from_request_parts
   [56]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [57]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [58]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [59]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [60]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [61]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [62]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [63]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [64]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [65]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [66]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [67]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [68]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [69]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [70]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [71]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [72]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [73]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [74]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [75]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [76]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [81]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [82]: trait.FromRequest.html#associatedtype.Rejection
   [83]: trait.FromRequest.html#tymethod.from_request
   [84]: ../body/struct.Body.html (struct axum::body::Body)
   [85]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [86]: super::Span::current()
   [87]: crate::Span
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [90]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [91]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [92]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [93]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [94]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [95]: https://docs.rs/http/latest/http/struct.Extensions.html
   [96]: crate::follow_redirect::policy::Standard
   [97]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [98]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [99]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [100]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [101]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [102]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [103]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [105]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [106]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [107]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [108]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [109]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [110]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [111]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [112]: super::Subscriber
   [113]: dispatcher#setting-the-default-subscriber

