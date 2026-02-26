<!-- Generated from rustdoc HTML: response/sse/struct.Event.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Event

## [axum][1]0.8.8

## Event

### Associated Constants

  * DEFAULT_KEEP_ALIVE



### Methods

  * comment
  * data
  * event
  * id
  * into_data_writer
  * json_data
  * retry



### Trait Implementations

  * Clone
  * Debug
  * Default



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



## [In axum::response::sse][2]

[axum][3]::[response][4]::[sse][2]

# Struct Event Copy item path

[Source][5]
``` 
pub struct Event { /* private fields */ }
```

Expand description

Server-sent event

## Implementations§

[Source][6]§

### impl [Event][7]

[Source][8]

#### pub const DEFAULT_KEEP_ALIVE: Self

Default keep-alive event

[Source][9]

#### pub fn into_data_writer(self) -> [EventDataWriter][10]

Use this [`Event`][7] as a [`EventDataWriter`][10] to write custom data.

  * [`Self::data`][11] can be used as a shortcut to write `str` data
  * [`Self::json_data`][12] can be used as a shortcut to write `json` data



Turn it into an [`Event`][7] again using [`EventDataWriter::into_event`][13].

[Source][14]

#### pub fn data<T>(self, data: T) -> Self

where T: [AsRef][15]<[str][16]>,

Set the event’s data data field(s) (`data: <content>`)

Newlines in `data` will automatically be broken across `data: ` fields.

This corresponds to [`MessageEvent`’s data field][17].

Note that events with an empty data field will be ignored by the browser.

##### §Panics

Panics if any `data` has already been written before.

[Source][18]

#### pub fn json_data<T>(self, data: T) -> [Result][19]<Self, [Error][20]>

where T: [Serialize][21],

Available on **crate feature`json`** only.

Set the event’s data field to a value serialized as unformatted JSON (`data: <content>`).

This corresponds to [`MessageEvent`’s data field][17].

##### §Panics

Panics if any `data` has already been written before.

[Source][22]

#### pub fn comment<T>(self, comment: T) -> Self

where T: [AsRef][15]<[str][16]>,

Set the event’s comment field (`:<comment-text>`).

This field will be ignored by most SSE clients.

Unlike other functions, this function can be called multiple times to add many comments.

##### §Panics

Panics if `comment` contains any newlines or carriage returns, as they are not allowed in comments.

[Source][23]

#### pub fn event<T>(self, event: T) -> Self

where T: [AsRef][15]<[str][16]>,

Set the event’s name field (`event:<event-name>`).

This corresponds to the `type` parameter given when calling `addEventListener` on an [`EventSource`][24]. For example, `.event("update")` should correspond to `.addEventListener("update", ...)`. If no event type is given, browsers will fire a [`message` event][25] instead.

##### §Panics

  * Panics if `event` contains any newlines or carriage returns.
  * Panics if this function has already been called on this event.



[Source][26]

#### pub fn retry(self, duration: [Duration][27]) -> Self

Set the event’s retry timeout field (`retry: <timeout>`).

This sets how long clients will wait before reconnecting if they are disconnected from the SSE endpoint. Note that this is just a hint: clients are free to wait for longer if they wish, such as if they implement exponential backoff.

##### §Panics

Panics if this function has already been called on this event.

[Source][28]

#### pub fn id<T>(self, id: T) -> Self

where T: [AsRef][15]<[str][16]>,

Set the event’s identifier field (`id:<identifier>`).

This corresponds to [`MessageEvent`’s `lastEventId` field][29]. If no ID is in the event itself, the browser will set that field to the last known message ID, starting with the empty string.

##### §Panics

  * Panics if `id` contains any newlines, carriage returns or null characters.
  * Panics if this function has already been called on this event.



## Trait Implementations§

[Source][30]§

### impl [Clone][31] for [Event][7]

[Source][30]§

#### fn [clone][32](&self) -> [Event][7]

Returns a duplicate of the value. [Read more][32]

1.0.0 · [Source][33]§

#### fn [clone_from][34](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][34]

[Source][30]§

### impl [Debug][35] for [Event][7]

[Source][30]§

#### fn [fmt][36](&self, f: &mut [Formatter][37]<'_>) -> [Result][38]

Formats the value using the given formatter. [Read more][36]

[Source][39]§

### impl [Default][40] for [Event][7]

[Source][41]§

#### fn [default][42]() -> Self

Returns the “default value” for a type. [Read more][42]

## Auto Trait Implementations§

§

### impl ![Freeze][43] for [Event][7]

§

### impl [RefUnwindSafe][44] for [Event][7]

§

### impl [Send][45] for [Event][7]

§

### impl [Sync][46] for [Event][7]

§

### impl [Unpin][47] for [Event][7]

§

### impl [UnwindSafe][48] for [Event][7]

## Blanket Implementations§

[Source][49]§

### impl<T> [Any][50] for T

where T: 'static + ?[Sized][51],

[Source][52]§

#### fn [type_id][53](&self) -> [TypeId][54]

Gets the `TypeId` of `self`. [Read more][53]

[Source][55]§

### impl<T> [Borrow][56]<T> for T

where T: ?[Sized][51],

[Source][57]§

#### fn [borrow][58](&self) -> [&T][59]

Immutably borrows from an owned value. [Read more][58]

[Source][60]§

### impl<T> [BorrowMut][61]<T> for T

where T: ?[Sized][51],

[Source][62]§

#### fn [borrow_mut][63](&mut self) -> [&mut T][59]

Mutably borrows from an owned value. [Read more][63]

[Source][64]§

### impl<T> [CloneToUninit][65] for T

where T: [Clone][31],

[Source][66]§

#### unsafe fn [clone_to_uninit][67](&self, dest: [*mut ][68][u8][69])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][67]

[Source][70]§

### impl<T> [From][71]<T> for T

[Source][72]§

#### fn [from][73](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][74]<T> for T

where T: [Clone][31],

§

#### fn [from_ref][75](input: [&T][59]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][76] [`Span`][77], returning an `Instrumented` wrapper. Read more

[Source][78]§

### impl<T, U> [Into][79]<U> for T

where U: [From][71]<T>,

[Source][80]§

#### fn [into][81](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][71]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][51],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][82]§

### impl<T> [Same][83] for T

[Source][84]§

#### type [Output][85] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][51],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][51],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][86]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][51],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][51],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][51],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][51],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][51],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][51],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][51],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][87] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][51],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][51],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][88]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][51],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][89] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][51],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][51], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][51], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][51],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][51],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][51],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][90]) -> RequestBodyLimit<Self>

where Self: [Sized][51],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][51],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][51],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][91]§

### impl<T> [ToOwned][92] for T

where T: [Clone][31],

[Source][93]§

#### type [Owned][94] = T

The resulting type after obtaining ownership.

[Source][95]§

#### fn [to_owned][96](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][96]

[Source][97]§

#### fn [clone_into][98](&self, target: [&mut T][59])

Uses borrowed data to replace owned data, usually by cloning. [Read more][98]

[Source][99]§

### impl<T, U> [TryFrom][100]<U> for T

where U: [Into][79]<T>,

[Source][101]§

#### type [Error][102] = [Infallible][103]

The type returned in the event of a conversion error.

[Source][104]§

#### fn [try_from][105](value: U) -> [Result][19]<T, <T as [TryFrom][100]<U>>::[Error][106]>

Performs the conversion.

[Source][107]§

### impl<T, U> [TryInto][108]<U> for T

where U: [TryFrom][100]<T>,

[Source][109]§

#### type [Error][110] = <U as [TryFrom][100]<T>>::[Error][106]

The type returned in the event of a conversion error.

[Source][111]§

#### fn [try_into][112](self) -> [Result][19]<U, <U as [TryFrom][100]<T>>::[Error][106]>

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

where S: [Into][79]<Dispatch>,

Attaches the provided [`Subscriber`][113] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][114] [`Subscriber`][113] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/response/sse.rs.html#174-177
   [6]: ../../../src/axum/response/sse.rs.html#200-423
   [7]: struct.Event.html (struct axum::response::sse::Event)
   [8]: ../../../src/axum/response/sse.rs.html#202
   [9]: ../../../src/axum/response/sse.rs.html#217-222
   [10]: struct.EventDataWriter.html (struct axum::response::sse::EventDataWriter)
   [11]: struct.Event.html#method.data (method axum::response::sse::Event::data)
   [12]: struct.Event.html#method.json_data (method axum::response::sse::Event::json_data)
   [13]: struct.EventDataWriter.html#method.into_event (method axum::response::sse::EventDataWriter::into_event)
   [14]: ../../../src/axum/response/sse.rs.html#237-244
   [15]: https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html (trait core::convert::AsRef)
   [16]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [17]: https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data
   [18]: ../../../src/axum/response/sse.rs.html#256-277
   [19]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [20]: ../../struct.Error.html (struct axum::Error)
   [21]: https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html (trait serde_core::ser::Serialize)
   [22]: ../../../src/axum/response/sse.rs.html#289-295
   [23]: ../../../src/axum/response/sse.rs.html#311-323
   [24]: https://developer.mozilla.org/en-US/docs/Web/API/EventSource
   [25]: https://developer.mozilla.org/en-US/docs/Web/API/EventSource/message_event
   [26]: ../../../src/axum/response/sse.rs.html#334-364
   [27]: https://doc.rust-lang.org/nightly/core/time/struct.Duration.html (struct core::time::Duration)
   [28]: ../../../src/axum/response/sse.rs.html#378-396
   [29]: https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId
   [30]: ../../../src/axum/response/sse.rs.html#172
   [31]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [32]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [33]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [34]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [35]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [36]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [37]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [38]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [39]: ../../../src/axum/response/sse.rs.html#479-486
   [40]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [41]: ../../../src/axum/response/sse.rs.html#480-485
   [42]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [44]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [46]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [47]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [48]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [49]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [50]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [51]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [52]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [53]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [54]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [57]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [58]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [59]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [60]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [61]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [62]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [63]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [64]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [65]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [66]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [67]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [68]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [69]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [70]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [71]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [72]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [73]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [74]: ../../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [75]: ../../extract/trait.FromRef.html#tymethod.from_ref
   [76]: super::Span::current()
   [77]: crate::Span
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [82]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [83]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [84]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [85]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [86]: https://docs.rs/http/latest/http/struct.Extensions.html
   [87]: crate::follow_redirect::policy::Standard
   [88]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [89]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [90]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [91]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [92]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [93]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [94]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [95]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [96]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [97]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [98]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
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

