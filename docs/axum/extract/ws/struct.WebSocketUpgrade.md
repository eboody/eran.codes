<!-- Generated from rustdoc HTML: extract/ws/struct.WebSocketUpgrade.html -->
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



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Struct WebSocketUpgrade Copy item path

[Source][5]
``` 
pub struct WebSocketUpgrade<F = [DefaultOnFailedUpgrade][6]> { /* private fields */ }
```

Available on **crate feature`ws`** only.

Expand description

Extractor for establishing WebSocket connections.

For HTTP/1.1 requests, this extractor requires the request method to be `GET`; in later versions, `CONNECT` is used instead. To support both, it should be used with [`any`][7].

See the [module docs][8] for an example.

## Implementations§

[Source][9]§

### impl<F> [WebSocketUpgrade][10]<F>

[Source][11]

#### pub fn read_buffer_size(self, size: [usize][12]) -> Self

Read buffer capacity. The default value is 128KiB

[Source][13]

#### pub fn write_buffer_size(self, size: [usize][12]) -> Self

The target minimum size of the write buffer to reach before writing the data to the underlying stream.

The default value is 128 KiB.

If set to `0` each message will be eagerly written to the underlying stream. It is often more optimal to allow them to buffer a little, hence the default value.

Note: [`flush`][14] will always fully write the buffer regardless.

[Source][15]

#### pub fn max_write_buffer_size(self, max: [usize][12]) -> Self

The max size of the write buffer in bytes. Setting this can provide backpressure in the case the write buffer is filling up due to write errors.

The default value is unlimited.

Note: The write buffer only builds up past [`write_buffer_size`][16] when writes to the underlying stream are failing. So the **write buffer can not fill up if you are not observing write errors even if not flushing**.

Note: Should always be at least [`write_buffer_size + 1 message`][16] and probably a little more depending on error handling strategy.

[Source][17]

#### pub fn max_message_size(self, max: [usize][12]) -> Self

Set the maximum message size (defaults to 64 megabytes)

[Source][18]

#### pub fn max_frame_size(self, max: [usize][12]) -> Self

Set the maximum frame size (defaults to 16 megabytes)

[Source][19]

#### pub fn accept_unmasked_frames(self, accept: [bool][20]) -> Self

Allow server to accept unmasked frames (defaults to false)

[Source][21]

#### pub fn protocols<I>(self, protocols: I) -> Self

where I: [IntoIterator][22], I::[Item][23]: [Into][24]<[Cow][25]<'static, [str][26]>>,

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

[Source][27]

#### pub fn requested_protocols(&self) -> impl [Iterator][28]<Item = &HeaderValue>

Return the WebSocket subprotocols requested by the client.

##### §Examples

If the client sends the following HTTP header in the WebSocket upgrade request:
``` 
Sec-WebSocket-Protocol: soap, wamp
```

this method returns an iterator yielding `"soap"` and `"wamp"`.

[Source][29]

#### pub fn set_selected_protocol(&mut self, protocol: HeaderValue)

Set the chosen WebSocket subprotocol.

Another method, [`protocols()`][30], also sets the chosen WebSocket subprotocol. If both methods are called, only the latter call takes effect.

##### §Notes

  * The chosen protocol is echoed back in the WebSocket upgrade response as required by RFC 6455. Some browsers may reject a value that was not present in the client’s request.



[Source][31]

#### pub fn selected_protocol(&self) -> [Option][32]<&HeaderValue>

Return the selected WebSocket subprotocol, if one has been chosen.

If [`protocols()`][30] selects a matching protocol, or [`set_selected_protocol()`][33] has been called, the return value will be `Some` containing the selected protocol. Otherwise, it will be `None`.

[Source][34]

#### pub fn on_failed_upgrade<C>(self, callback: C) -> [WebSocketUpgrade][10]<C>

where C: [OnFailedUpgrade][35],

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

[Source][36]

#### pub fn on_upgrade<C, Fut>(self, callback: C) -> [Response][37]

where C: [FnOnce][38]([WebSocket][39]) -> Fut + [Send][40] \+ 'static, Fut: [Future][41]<Output = [()][42]> \+ [Send][40] \+ 'static, F: [OnFailedUpgrade][35],

Finalize upgrading the connection and call the provided callback with the stream.

## Trait Implementations§

[Source][43]§

### impl<F> [Debug][44] for [WebSocketUpgrade][10]<F>

[Source][45]§

#### fn [fmt][46](&self, f: &mut [Formatter][47]<'_>) -> [Result][48]

Formats the value using the given formatter. [Read more][46]

[Source][49]§

### impl<S> [FromRequestParts][50]<S> for [WebSocketUpgrade][10]<[DefaultOnFailedUpgrade][6]>

where S: [Send][40] \+ [Sync][51],

[Source][52]§

#### type [Rejection][53] = [WebSocketUpgradeRejection][54]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][55]§

#### async fn [from_request_parts][56]( parts: &mut Parts, _state: [&S][57], ) -> [Result][58]<Self, Self::[Rejection][59]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl<F = [DefaultOnFailedUpgrade][6]> ![Freeze][60] for [WebSocketUpgrade][10]<F>

§

### impl<F> [RefUnwindSafe][61] for [WebSocketUpgrade][10]<F>

where F: [RefUnwindSafe][61],

§

### impl<F> [Send][40] for [WebSocketUpgrade][10]<F>

where F: [Send][40],

§

### impl<F> [Sync][51] for [WebSocketUpgrade][10]<F>

where F: [Sync][51],

§

### impl<F> [Unpin][62] for [WebSocketUpgrade][10]<F>

where F: [Unpin][62],

§

### impl<F> [UnwindSafe][63] for [WebSocketUpgrade][10]<F>

where F: [UnwindSafe][63],

## Blanket Implementations§

[Source][64]§

### impl<T> [Any][65] for T

where T: 'static + ?[Sized][66],

[Source][67]§

#### fn [type_id][68](&self) -> [TypeId][69]

Gets the `TypeId` of `self`. [Read more][68]

[Source][70]§

### impl<T> [Borrow][71]<T> for T

where T: ?[Sized][66],

[Source][72]§

#### fn [borrow][73](&self) -> [&T][57]

Immutably borrows from an owned value. [Read more][73]

[Source][74]§

### impl<T> [BorrowMut][75]<T> for T

where T: ?[Sized][66],

[Source][76]§

#### fn [borrow_mut][77](&mut self) -> [&mut T][57]

Mutably borrows from an owned value. [Read more][77]

[Source][78]§

### impl<T> [From][79]<T> for T

[Source][80]§

#### fn [from][81](t: T) -> T

Returns the argument unchanged.

§

### impl<S, T> [FromRequest][82]<S, ViaParts> for T

where S: [Send][40] \+ [Sync][51], T: [FromRequestParts][50]<S>,

§

#### type [Rejection][83] = <T as [FromRequestParts][50]<S>>::[Rejection][59]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][84]( req: Request<[Body][85]>, state: [&S][57], ) -> impl [Future][41]<Output = [Result][58]<T, <T as [FromRequest][82]<S, ViaParts>>::[Rejection][86]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][87] [`Span`][88], returning an `Instrumented` wrapper. Read more

[Source][89]§

### impl<T, U> [Into][24]<U> for T

where U: [From][79]<T>,

[Source][90]§

#### fn [into][91](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][79]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][66],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][92]§

### impl<T> [Same][93] for T

[Source][94]§

#### type [Output][95] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][66],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][66],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][96]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][66],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][66],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][66],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][66],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][66],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][66],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][66],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][97] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][22]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][66],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][98] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][22]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][66],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][98] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][22]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][66],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][98] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][66],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][66],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][66],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][66],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][66],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][66],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][66], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][66], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][66],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][66],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][66],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][12]) -> RequestBodyLimit<Self>

where Self: [Sized][66],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][66],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][66],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][99]§

### impl<T, U> [TryFrom][100]<U> for T

where U: [Into][24]<T>,

[Source][101]§

#### type [Error][102] = [Infallible][103]

The type returned in the event of a conversion error.

[Source][104]§

#### fn [try_from][105](value: U) -> [Result][58]<T, <T as [TryFrom][100]<U>>::[Error][106]>

Performs the conversion.

[Source][107]§

### impl<T, U> [TryInto][108]<U> for T

where U: [TryFrom][100]<T>,

[Source][109]§

#### type [Error][110] = <U as [TryFrom][100]<T>>::[Error][106]

The type returned in the event of a conversion error.

[Source][111]§

#### fn [try_into][112](self) -> [Result][58]<U, <U as [TryFrom][100]<T>>::[Error][106]>

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

where S: [Into][24]<Dispatch>,

Attaches the provided [`Subscriber`][113] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][114] [`Subscriber`][113] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/ws.rs.html#134-143
   [6]: struct.DefaultOnFailedUpgrade.html (struct axum::extract::ws::DefaultOnFailedUpgrade)
   [7]: ../../routing/method_routing/fn.any.html (fn axum::routing::method_routing::any)
   [8]: index.html (mod axum::extract::ws)
   [9]: ../../../src/axum/extract/ws.rs.html#156-411
   [10]: ../struct.WebSocketUpgrade.html (struct axum::extract::WebSocketUpgrade)
   [11]: ../../../src/axum/extract/ws.rs.html#158-161
   [12]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [13]: ../../../src/axum/extract/ws.rs.html#172-175
   [14]: SinkExt::flush
   [15]: ../../../src/axum/extract/ws.rs.html#188-191
   [16]: ../struct.WebSocketUpgrade.html#method.write_buffer_size (method axum::extract::WebSocketUpgrade::write_buffer_size)
   [17]: ../../../src/axum/extract/ws.rs.html#194-197
   [18]: ../../../src/axum/extract/ws.rs.html#200-203
   [19]: ../../../src/axum/extract/ws.rs.html#206-209
   [20]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [21]: ../../../src/axum/extract/ws.rs.html#241-265
   [22]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [23]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item (type core::iter::traits::collect::IntoIterator::Item)
   [24]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [25]: https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html (enum alloc::borrow::Cow)
   [26]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [27]: ../../../src/axum/extract/ws.rs.html#278-280
   [28]: https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html (trait core::iter::traits::iterator::Iterator)
   [29]: ../../../src/axum/extract/ws.rs.html#292-294
   [30]: ../struct.WebSocketUpgrade.html#method.protocols (method axum::extract::WebSocketUpgrade::protocols)
   [31]: ../../../src/axum/extract/ws.rs.html#301-303
   [32]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [33]: ../struct.WebSocketUpgrade.html#method.set_selected_protocol (method axum::extract::WebSocketUpgrade::set_selected_protocol)
   [34]: ../../../src/axum/extract/ws.rs.html#329-341
   [35]: trait.OnFailedUpgrade.html (trait axum::extract::ws::OnFailedUpgrade)
   [36]: ../../../src/axum/extract/ws.rs.html#346-410
   [37]: ../../response/type.Response.html (type axum::response::Response)
   [38]: https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html (trait core::ops::function::FnOnce)
   [39]: struct.WebSocket.html (struct axum::extract::ws::WebSocket)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [41]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [42]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [43]: ../../../src/axum/extract/ws.rs.html#145-154
   [44]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [45]: ../../../src/axum/extract/ws.rs.html#146-153
   [46]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [47]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [48]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [49]: ../../../src/axum/extract/ws.rs.html#442-517
   [50]: ../trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [51]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [52]: ../../../src/axum/extract/ws.rs.html#446
   [53]: ../trait.FromRequestParts.html#associatedtype.Rejection
   [54]: rejection/enum.WebSocketUpgradeRejection.html (enum axum::extract::ws::rejection::WebSocketUpgradeRejection)
   [55]: ../../../src/axum/extract/ws.rs.html#448-516
   [56]: ../trait.FromRequestParts.html#tymethod.from_request_parts
   [57]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [58]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [59]: ../trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [60]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [61]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [62]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [63]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [64]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [65]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [66]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [67]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [68]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [69]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [70]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [71]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [72]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [73]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [74]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [75]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [76]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [77]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [82]: ../trait.FromRequest.html (trait axum::extract::FromRequest)
   [83]: ../trait.FromRequest.html#associatedtype.Rejection
   [84]: ../trait.FromRequest.html#tymethod.from_request
   [85]: ../../body/struct.Body.html (struct axum::body::Body)
   [86]: ../trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [87]: super::Span::current()
   [88]: crate::Span
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [92]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [93]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [94]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [95]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [96]: https://docs.rs/http/latest/http/struct.Extensions.html
   [97]: crate::follow_redirect::policy::Standard
   [98]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [99]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [100]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [101]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [102]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [103]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [104]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [105]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [106]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [107]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [108]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [109]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [110]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [111]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [112]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [113]: super::Subscriber
   [114]: dispatcher#setting-the-default-subscriber

