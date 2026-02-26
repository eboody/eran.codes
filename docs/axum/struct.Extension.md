<!-- Generated from rustdoc HTML: struct.Extension.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Extension

## [axum][1]0.8.8

## Extension

### Sections

  * As extractor
  * As response



### Tuple Fields

  * 0



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * Default
  * Deref
  * DerefMut
  * FromRequestParts<S>
  * IntoResponse
  * IntoResponseParts
  * Layer<S>
  * OptionalFromRequestParts<S>



### Auto Trait Implementations

  * Freeze
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
  * CryptoRng
  * From<T>
  * FromRef<T>
  * FromRequest<S, ViaParts>
  * HandlerWithoutStateExt<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Receiver
  * Rng
  * RngCore
  * Same
  * ServiceExt
  * ToOwned
  * TryCryptoRng
  * TryFrom<U>
  * TryInto<U>
  * TryRngCore
  * VZip<V>
  * WithSubscriber



## [In crate axum][2]

[axum][2]

# Struct Extension Copy item path

[Source][3]
``` 
pub struct Extension<T>(pub T);
```

Expand description

Extractor and response for extensions.

## §As extractor

This is commonly used to share state across handlers.
``` 
use axum::{
    Router,
    Extension,
    routing::get,
};
use std::sync::Arc;

// Some shared state used throughout our application
struct State {
    // ...
}

async fn handler(state: Extension<Arc<State>>) {
    // ...
}

let state = Arc::new(State { /* ... */ });

let app = Router::new().route("/", get(handler))
    // Add middleware that inserts the state into all incoming request's
    // extensions.
    .layer(Extension(state));
```

If the extension is missing it will reject the request with a `500 Internal Server Error` response. Alternatively, you can use `Option<Extension<T>>` to make the extension extractor optional.

## §As response

Response extensions can be used to share state with middleware.
``` 
use axum::{
    Extension,
    response::IntoResponse,
};

async fn handler() -> (Extension<Foo>, &'static str) {
    (
        Extension(Foo("foo")),
        "Hello, World!"
    )
}

#[derive(Clone)]
struct Foo(&'static str);
```

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][4]§

### impl<T: [Clone][5]> [Clone][5] for [Extension][6]<T>

[Source][4]§

#### fn [clone][7](&self) -> [Extension][6]<T>

Returns a duplicate of the value. [Read more][7]

1.0.0 · [Source][8]§

#### fn [clone_from][9](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][9]

[Source][4]§

### impl<T: [Debug][10]> [Debug][10] for [Extension][6]<T>

[Source][4]§

#### fn [fmt][11](&self, f: &mut [Formatter][12]<'_>) -> [Result][13]

Formats the value using the given formatter. [Read more][11]

[Source][4]§

### impl<T: [Default][14]> [Default][14] for [Extension][6]<T>

[Source][4]§

#### fn [default][15]() -> [Extension][6]<T>

Returns the “default value” for a type. [Read more][15]

[Source][16]§

### impl<T> [Deref][17] for [Extension][6]<T>

[Source][16]§

#### type [Target][18] = T

The resulting type after dereferencing.

[Source][16]§

#### fn [deref][19](&self) -> &Self::[Target][20]

Dereferences the value.

[Source][16]§

### impl<T> [DerefMut][21] for [Extension][6]<T>

[Source][16]§

#### fn [deref_mut][22](&mut self) -> &mut Self::[Target][20]

Mutably dereferences the value.

[Source][23]§

### impl<T, S> [FromRequestParts][24]<S> for [Extension][6]<T>

where T: [Clone][5] \+ [Send][25] \+ [Sync][26] \+ 'static, S: [Send][25] \+ [Sync][26],

[Source][27]§

#### type [Rejection][28] = [ExtensionRejection][29]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][30]§

#### async fn [from_request_parts][31]( req: &mut Parts, _state: [&S][32], ) -> [Result][33]<Self, Self::[Rejection][34]>

Perform the extraction.

[Source][35]§

### impl<T> [IntoResponse][36] for [Extension][6]<T>

where T: [Clone][5] \+ [Send][25] \+ [Sync][26] \+ 'static,

[Source][37]§

#### fn [into_response][38](self) -> [Response][39]

Create a response.

[Source][40]§

### impl<T> [IntoResponseParts][41] for [Extension][6]<T>

where T: [Clone][5] \+ [Send][25] \+ [Sync][26] \+ 'static,

[Source][42]§

#### type [Error][43] = [Infallible][44]

The type returned in the event of an error. [Read more][43]

[Source][45]§

#### fn [into_response_parts][46]( self, res: [ResponseParts][47], ) -> [Result][33]<[ResponseParts][47], Self::[Error][48]>

Set parts of the response

[Source][49]§

### impl<S, T> Layer<S> for [Extension][6]<T>

where T: [Clone][5] \+ [Send][25] \+ [Sync][26] \+ 'static,

[Source][50]§

#### type Service = [AddExtension][51]<S, T>

The wrapped service

[Source][52]§

#### fn layer(&self, inner: S) -> Self::Service

Wrap the given service with the middleware, returning a new service that has been decorated with the middleware.

[Source][53]§

### impl<T, S> [OptionalFromRequestParts][54]<S> for [Extension][6]<T>

where T: [Clone][5] \+ [Send][25] \+ [Sync][26] \+ 'static, S: [Send][25] \+ [Sync][26],

[Source][55]§

#### type [Rejection][56] = [Infallible][44]

If the extractor fails, it will use this “rejection” type. [Read more][56]

[Source][57]§

#### async fn [from_request_parts][58]( req: &mut Parts, _state: [&S][32], ) -> [Result][33]<[Option][59]<Self>, Self::[Rejection][60]>

Perform the extraction.

[Source][4]§

### impl<T: [Copy][61]> [Copy][61] for [Extension][6]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][62] for [Extension][6]<T>

where T: [Freeze][62],

§

### impl<T> [RefUnwindSafe][63] for [Extension][6]<T>

where T: [RefUnwindSafe][63],

§

### impl<T> [Send][25] for [Extension][6]<T>

where T: [Send][25],

§

### impl<T> [Sync][26] for [Extension][6]<T>

where T: [Sync][26],

§

### impl<T> [Unpin][64] for [Extension][6]<T>

where T: [Unpin][64],

§

### impl<T> [UnwindSafe][65] for [Extension][6]<T>

where T: [UnwindSafe][65],

## Blanket Implementations§

[Source][66]§

### impl<T> [Any][67] for T

where T: 'static + ?[Sized][68],

[Source][69]§

#### fn [type_id][70](&self) -> [TypeId][71]

Gets the `TypeId` of `self`. [Read more][70]

[Source][72]§

### impl<T> [Borrow][73]<T> for T

where T: ?[Sized][68],

[Source][74]§

#### fn [borrow][75](&self) -> [&T][32]

Immutably borrows from an owned value. [Read more][75]

[Source][76]§

### impl<T> [BorrowMut][77]<T> for T

where T: ?[Sized][68],

[Source][78]§

#### fn [borrow_mut][79](&mut self) -> [&mut T][32]

Mutably borrows from an owned value. [Read more][79]

[Source][80]§

### impl<T> [CloneToUninit][81] for T

where T: [Clone][5],

[Source][82]§

#### unsafe fn [clone_to_uninit][83](&self, dest: [*mut ][84][u8][85])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][83]

[Source][86]§

### impl<T> [From][87]<T> for T

[Source][88]§

#### fn [from][89](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][90]<T> for T

where T: [Clone][5],

§

#### fn [from_ref][91](input: [&T][32]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][92]<S, ViaParts> for T

where S: [Send][25] \+ [Sync][26], T: [FromRequestParts][24]<S>,

§

#### type [Rejection][93] = <T as [FromRequestParts][24]<S>>::[Rejection][34]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][94]( req: Request<[Body][95]>, state: [&S][32], ) -> impl [Future][96]<Output = [Result][33]<T, <T as [FromRequest][92]<S, ViaParts>>::[Rejection][97]>>

Perform the extraction.

[Source][98]§

### impl<H, T> [HandlerWithoutStateExt][99]<T> for H

where H: [Handler][100]<T, [()][101]>,

[Source][102]§

#### fn [into_service][103](self) -> [HandlerService][104]<H, T, [()][101]>

Convert the handler into a [`Service`] and no state.

[Source][105]§

#### fn [into_make_service][106](self) -> [IntoMakeService][107]<[HandlerService][104]<H, T, [()][101]>>

Convert the handler into a [`MakeService`][108] and no state. [Read more][106]

[Source][109]§

#### fn [into_make_service_with_connect_info][110]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][111]<[HandlerService][104]<H, T, [()][101]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][108] which stores information about the incoming connection and has no state. [Read more][110]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][112] [`Span`][113], returning an `Instrumented` wrapper. Read more

[Source][114]§

### impl<T, U> [Into][115]<U> for T

where U: [From][87]<T>,

[Source][116]§

#### fn [into][117](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][87]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][68],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][118]§

### impl<P, T> [Receiver][119] for P

where P: [Deref][17]<Target = T> \+ ?[Sized][68], T: ?[Sized][68],

[Source][120]§

#### type [Target][121] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][122]§

### impl<R> [Rng][123] for R

where R: [RngCore][124] \+ ?[Sized][68],

[Source][125]§

#### fn [random][126]<T>(&mut self) -> T

where [StandardUniform][127]: [Distribution][128]<T>,

Return a random value via the [`StandardUniform`][127] distribution. [Read more][126]

[Source][129]§

#### fn [random_iter][130]<T>(self) -> [Iter][131]<[StandardUniform][127], Self, T>

where Self: [Sized][68], [StandardUniform][127]: [Distribution][128]<T>,

Return an iterator over [`random`][132] variates [Read more][130]

[Source][133]§

#### fn [random_range][134]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][135], R: [SampleRange][136]<T>,

Generate a random value in the given range. [Read more][134]

[Source][137]§

#### fn [random_bool][138](&mut self, p: [f64][139]) -> [bool][140]

Return a bool with a probability `p` of being true. [Read more][138]

[Source][141]§

#### fn [random_ratio][142](&mut self, numerator: [u32][143], denominator: [u32][143]) -> [bool][140]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][142]

[Source][144]§

#### fn [sample][145]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][128]<T>,

Sample a new value, using the given distribution. [Read more][145]

[Source][146]§

#### fn [sample_iter][147]<T, D>(self, distr: D) -> [Iter][131]<D, Self, T>

where D: [Distribution][128]<T>, Self: [Sized][68],

Create an iterator that generates values using the given distribution. [Read more][147]

[Source][148]§

#### fn [fill][149]<T>(&mut self, dest: [&mut T][32])

where T: [Fill][150] \+ ?[Sized][68],

Fill any type implementing [`Fill`][150] with random data [Read more][149]

[Source][151]§

#### fn [gen][152]<T>(&mut self) -> T

where [StandardUniform][127]: [Distribution][128]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][132].

[Source][153]§

#### fn [gen_range][154]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][135], R: [SampleRange][136]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][155].

[Source][156]§

#### fn [gen_bool][157](&mut self, p: [f64][139]) -> [bool][140]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][158].

[Source][159]§

#### fn [gen_ratio][160](&mut self, numerator: [u32][143], denominator: [u32][143]) -> [bool][140]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][161].

[Source][162]§

### impl<T> [RngCore][124] for T

where T: [DerefMut][21], <T as [Deref][17]>::[Target][20]: [RngCore][124],

[Source][163]§

#### fn [next_u32][164](&mut self) -> [u32][143]

Return the next random `u32`. [Read more][164]

[Source][165]§

#### fn [next_u64][166](&mut self) -> [u64][167]

Return the next random `u64`. [Read more][166]

[Source][168]§

#### fn [fill_bytes][169](&mut self, dst: &mut [[u8][85]])

Fill `dest` with random data. [Read more][169]

[Source][170]§

### impl<T> [Same][171] for T

[Source][172]§

#### type [Output][173] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][68],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][68],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][174]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][68],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][68],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][68],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][68],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][68],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][68],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][68],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][175] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][176]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][68],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][177] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][176]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][68],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][177] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][176]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][68],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][177] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][68],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][68],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][68],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][68],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][68],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][68],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][68], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][68], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][68],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][68],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][68],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][178]) -> RequestBodyLimit<Self>

where Self: [Sized][68],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][68],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][68],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][179]§

### impl<T> [ToOwned][180] for T

where T: [Clone][5],

[Source][181]§

#### type [Owned][182] = T

The resulting type after obtaining ownership.

[Source][183]§

#### fn [to_owned][184](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][184]

[Source][185]§

#### fn [clone_into][186](&self, target: [&mut T][32])

Uses borrowed data to replace owned data, usually by cloning. [Read more][186]

[Source][187]§

### impl<T, U> [TryFrom][188]<U> for T

where U: [Into][115]<T>,

[Source][189]§

#### type [Error][190] = [Infallible][44]

The type returned in the event of a conversion error.

[Source][191]§

#### fn [try_from][192](value: U) -> [Result][33]<T, <T as [TryFrom][188]<U>>::[Error][193]>

Performs the conversion.

[Source][194]§

### impl<T, U> [TryInto][195]<U> for T

where U: [TryFrom][188]<T>,

[Source][196]§

#### type [Error][197] = <U as [TryFrom][188]<T>>::[Error][193]

The type returned in the event of a conversion error.

[Source][198]§

#### fn [try_into][199](self) -> [Result][33]<U, <U as [TryFrom][188]<T>>::[Error][193]>

Performs the conversion.

[Source][200]§

### impl<R> [TryRngCore][201] for R

where R: [RngCore][124] \+ ?[Sized][68],

[Source][202]§

#### type [Error][203] = [Infallible][44]

The type returned in the event of a RNG error.

[Source][204]§

#### fn [try_next_u32][205](&mut self) -> [Result][33]<[u32][143], <R as [TryRngCore][201]>::[Error][206]>

Return the next random `u32`.

[Source][207]§

#### fn [try_next_u64][208](&mut self) -> [Result][33]<[u64][167], <R as [TryRngCore][201]>::[Error][206]>

Return the next random `u64`.

[Source][209]§

#### fn [try_fill_bytes][210]( &mut self, dst: &mut [[u8][85]], ) -> [Result][33]<[()][101], <R as [TryRngCore][201]>::[Error][206]>

Fill `dest` entirely with random data.

[Source][211]§

#### fn [unwrap_err][212](self) -> [UnwrapErr][213]<Self>

where Self: [Sized][68],

Wrap RNG with the [`UnwrapErr`][213] wrapper.

[Source][214]§

#### fn [unwrap_mut][215](&mut self) -> [UnwrapMut][216]<'_, Self>

Wrap RNG with the [`UnwrapMut`][216] wrapper.

[Source][217]§

#### fn [read_adapter][218](&mut self) -> [RngReadAdapter][219]<'_, Self>

where Self: [Sized][68],

Available on **crate feature`std`** only.

Convert an [`RngCore`][124] to a [`RngReadAdapter`][219].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][115]<Dispatch>,

Attaches the provided [`Subscriber`][220] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][221] [`Subscriber`][220] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][222]§

### impl<T> [CryptoRng][223] for T

where T: [DerefMut][21], <T as [Deref][17]>::[Target][20]: [CryptoRng][223],

[Source][224]§

### impl<R> [TryCryptoRng][225] for R

where R: [CryptoRng][223] \+ ?[Sized][68],

   [1]: ../axum/index.html
   [2]: index.html
   [3]: ../src/axum/extension.rs.html#72
   [4]: ../src/axum/extension.rs.html#70
   [5]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [6]: struct.Extension.html (struct axum::Extension)
   [7]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [8]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [12]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [14]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [15]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [16]: ../src/axum/extension.rs.html#115
   [17]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [18]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [19]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [20]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [21]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [22]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [23]: ../src/axum/extension.rs.html#83-98
   [24]: extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [27]: ../src/axum/extension.rs.html#88
   [28]: extract/trait.FromRequestParts.html#associatedtype.Rejection
   [29]: extract/rejection/enum.ExtensionRejection.html (enum axum::extract::rejection::ExtensionRejection)
   [30]: ../src/axum/extension.rs.html#90-97
   [31]: extract/trait.FromRequestParts.html#tymethod.from_request_parts
   [32]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [33]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [34]: extract/trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [35]: ../src/axum/extension.rs.html#129-138
   [36]: response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [37]: ../src/axum/extension.rs.html#133-137
   [38]: response/trait.IntoResponse.html#tymethod.into_response
   [39]: response/type.Response.html (type axum::response::Response)
   [40]: ../src/axum/extension.rs.html#117-127
   [41]: response/trait.IntoResponseParts.html (trait axum::response::IntoResponseParts)
   [42]: ../src/axum/extension.rs.html#121
   [43]: response/trait.IntoResponseParts.html#associatedtype.Error
   [44]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [45]: ../src/axum/extension.rs.html#123-126
   [46]: response/trait.IntoResponseParts.html#tymethod.into_response_parts
   [47]: response/struct.ResponseParts.html (struct axum::response::ResponseParts)
   [48]: response/trait.IntoResponseParts.html#associatedtype.Error (type axum::response::IntoResponseParts::Error)
   [49]: ../src/axum/extension.rs.html#140-152
   [50]: ../src/axum/extension.rs.html#144
   [51]: middleware/struct.AddExtension.html (struct axum::middleware::AddExtension)
   [52]: ../src/axum/extension.rs.html#146-151
   [53]: ../src/axum/extension.rs.html#100-113
   [54]: extract/trait.OptionalFromRequestParts.html (trait axum::extract::OptionalFromRequestParts)
   [55]: ../src/axum/extension.rs.html#105
   [56]: extract/trait.OptionalFromRequestParts.html#associatedtype.Rejection
   [57]: ../src/axum/extension.rs.html#107-112
   [58]: extract/trait.OptionalFromRequestParts.html#tymethod.from_request_parts
   [59]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [60]: extract/trait.OptionalFromRequestParts.html#associatedtype.Rejection (type axum::extract::OptionalFromRequestParts::Rejection)
   [61]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [62]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [63]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [64]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [65]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [66]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [67]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [68]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [69]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [70]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [71]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [72]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [73]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [74]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [75]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [76]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [77]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [78]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [79]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [80]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [81]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [82]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [83]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [84]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [85]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [90]: extract/trait.FromRef.html (trait axum::extract::FromRef)
   [91]: extract/trait.FromRef.html#tymethod.from_ref
   [92]: extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [93]: extract/trait.FromRequest.html#associatedtype.Rejection
   [94]: extract/trait.FromRequest.html#tymethod.from_request
   [95]: body/struct.Body.html (struct axum::body::Body)
   [96]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [97]: extract/trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [98]: ../src/axum/handler/mod.rs.html#380-398
   [99]: handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [100]: handler/trait.Handler.html (trait axum::handler::Handler)
   [101]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [102]: ../src/axum/handler/mod.rs.html#384-386
   [103]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [104]: handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [105]: ../src/axum/handler/mod.rs.html#388-390
   [106]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [107]: routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [108]: tower::make::MakeService
   [109]: ../src/axum/handler/mod.rs.html#393-397
   [110]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [111]: extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [112]: super::Span::current()
   [113]: crate::Span
   [114]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [115]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [116]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [117]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [118]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [119]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [120]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [121]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [122]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [123]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [124]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [125]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [126]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [127]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [128]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [129]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [130]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [131]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [132]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [133]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [134]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [135]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [136]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [137]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [138]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [139]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [140]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [141]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [142]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [143]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [144]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [145]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [146]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [147]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [148]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [149]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [150]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [151]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [152]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [153]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [154]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [155]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [156]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [157]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [158]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [159]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [160]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [161]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [162]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [163]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [164]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [165]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [166]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [167]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [168]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [169]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [170]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [171]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [172]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [173]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [174]: https://docs.rs/http/latest/http/struct.Extensions.html
   [175]: crate::follow_redirect::policy::Standard
   [176]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [177]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [178]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [179]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [180]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [181]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [182]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [183]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [184]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [185]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [186]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [187]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [188]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [189]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [190]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [191]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [192]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [193]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [194]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [195]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [196]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [197]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [198]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [199]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [200]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [201]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [202]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [203]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [204]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [205]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [206]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [207]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [208]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [209]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [210]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [211]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [212]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [213]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [214]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [215]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [216]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [217]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [218]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [219]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [220]: super::Subscriber
   [221]: dispatcher#setting-the-default-subscriber
   [222]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [223]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [224]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [225]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

