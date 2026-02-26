<!-- Generated from rustdoc HTML: struct.Json.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Json

## [axum][1]0.8.8

## Json

### Sections

  * Extractor example
  * Response example



### Tuple Fields

  * 0



### Methods

  * from_bytes



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * Default
  * Deref
  * DerefMut
  * From<T>
  * FromRequest<S>
  * IntoResponse
  * OptionalFromRequest<S>



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
  * From<!>
  * From<T>
  * FromRef<T>
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

# Struct Json Copy item path

[Source][3]
``` 
pub struct Json<T>(pub T);
```

Available on **crate feature`json`** only.

Expand description

JSON Extractor / Response.

When used as an extractor, it can deserialize request bodies into some type that implements [`serde::de::DeserializeOwned`][4]. The request will be rejected (and a [`JsonRejection`][5] will be returned) if:

  * The request doesn’t have a `Content-Type: application/json` (or similar) header.
  * The body doesn’t contain syntactically valid JSON.
  * The body contains syntactically valid JSON, but it couldn’t be deserialized into the target type.
  * Buffering the request body fails.



⚠️ Since parsing JSON requires consuming the request body, the `Json` extractor must be _last_ if there are multiple extractors in a handler. See [“the order of extractors”][6]

See [`JsonRejection`][5] for more details.

## §Extractor example
``` 
use axum::{
    extract,
    routing::post,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn create_user(extract::Json(payload): extract::Json<CreateUser>) {
    // payload is a `CreateUser`
}

let app = Router::new().route("/users", post(create_user));
```

When used as a response, it can serialize any type that implements [`serde::Serialize`][7] to `JSON`, and will automatically set `Content-Type: application/json` header.

If the [`Serialize`][7] implementation decides to fail or if a map with non-string keys is used, a 500 response will be issued whose body is the error message in UTF-8.

## §Response example
``` 
use axum::{
    extract::Path,
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct User {
    id: Uuid,
    username: String,
}

async fn get_user(Path(user_id) : Path<Uuid>) -> Json<User> {
    let user = find_user(user_id).await;
    Json(user)
}

async fn find_user(user_id: Uuid) -> User {
    // ...
}

let app = Router::new().route("/users/{id}", get(get_user));
```

## Tuple Fields§

§`0: T`

## Implementations§

[Source][8]§

### impl<T> [Json][9]<T>

where T: [DeserializeOwned][4],

[Source][10]

#### pub fn from_bytes(bytes: &[[u8][11]]) -> [Result][12]<Self, [JsonRejection][5]>

Construct a `Json<T>` from a byte slice. Most users should prefer to use the `FromRequest` impl but special cases may require first extracting a `Request` into `Bytes` then optionally constructing a `Json<T>`.

## Trait Implementations§

[Source][13]§

### impl<T: [Clone][14]> [Clone][14] for [Json][9]<T>

[Source][13]§

#### fn [clone][15](&self) -> [Json][9]<T>

Returns a duplicate of the value. [Read more][15]

1.0.0 · [Source][16]§

#### fn [clone_from][17](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][17]

[Source][13]§

### impl<T: [Debug][18]> [Debug][18] for [Json][9]<T>

[Source][13]§

#### fn [fmt][19](&self, f: &mut [Formatter][20]<'_>) -> [Result][21]

Formats the value using the given formatter. [Read more][19]

[Source][13]§

### impl<T: [Default][22]> [Default][22] for [Json][9]<T>

[Source][13]§

#### fn [default][23]() -> [Json][9]<T>

Returns the “default value” for a type. [Read more][23]

[Source][24]§

### impl<T> [Deref][25] for [Json][9]<T>

[Source][24]§

#### type [Target][26] = T

The resulting type after dereferencing.

[Source][24]§

#### fn [deref][27](&self) -> &Self::[Target][28]

Dereferences the value.

[Source][24]§

### impl<T> [DerefMut][29] for [Json][9]<T>

[Source][24]§

#### fn [deref_mut][30](&mut self) -> &mut Self::[Target][28]

Mutably dereferences the value.

[Source][31]§

### impl<T> [From][32]<T> for [Json][9]<T>

[Source][33]§

#### fn [from][34](inner: T) -> Self

Converts to this type from the input type.

[Source][35]§

### impl<T, S> [FromRequest][36]<S> for [Json][9]<T>

where T: [DeserializeOwned][4], S: [Send][37] \+ [Sync][38],

[Source][39]§

#### type [Rejection][40] = [JsonRejection][5]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][41]§

#### async fn [from_request][42](req: [Request][43], state: [&S][44]) -> [Result][12]<Self, Self::[Rejection][45]>

Perform the extraction.

[Source][46]§

### impl<T> [IntoResponse][47] for [Json][9]<T>

where T: [Serialize][7],

[Source][48]§

#### fn [into_response][49](self) -> [Response][50]

Create a response.

[Source][51]§

### impl<T, S> [OptionalFromRequest][52]<S> for [Json][9]<T>

where T: [DeserializeOwned][4], S: [Send][37] \+ [Sync][38],

[Source][53]§

#### type [Rejection][54] = [JsonRejection][5]

If the extractor fails, it will use this “rejection” type. [Read more][54]

[Source][55]§

#### async fn [from_request][56]( req: [Request][43], state: [&S][44], ) -> [Result][12]<[Option][57]<Self>, Self::[Rejection][58]>

Perform the extraction.

[Source][13]§

### impl<T: [Copy][59]> [Copy][59] for [Json][9]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][60] for [Json][9]<T>

where T: [Freeze][60],

§

### impl<T> [RefUnwindSafe][61] for [Json][9]<T>

where T: [RefUnwindSafe][61],

§

### impl<T> [Send][37] for [Json][9]<T>

where T: [Send][37],

§

### impl<T> [Sync][38] for [Json][9]<T>

where T: [Sync][38],

§

### impl<T> [Unpin][62] for [Json][9]<T>

where T: [Unpin][62],

§

### impl<T> [UnwindSafe][63] for [Json][9]<T>

where T: [UnwindSafe][63],

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

#### fn [borrow][73](&self) -> [&T][44]

Immutably borrows from an owned value. [Read more][73]

[Source][74]§

### impl<T> [BorrowMut][75]<T> for T

where T: ?[Sized][66],

[Source][76]§

#### fn [borrow_mut][77](&mut self) -> [&mut T][44]

Mutably borrows from an owned value. [Read more][77]

[Source][78]§

### impl<T> [CloneToUninit][79] for T

where T: [Clone][14],

[Source][80]§

#### unsafe fn [clone_to_uninit][81](&self, dest: [*mut ][82][u8][11])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][81]

[Source][83]§

### impl<T> [From][32]<[!][84]> for T

[Source][85]§

#### fn [from][34](t: [!][84]) -> T

Converts to this type from the input type.

[Source][86]§

### impl<T> [From][32]<T> for T

[Source][87]§

#### fn [from][34](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][88]<T> for T

where T: [Clone][14],

§

#### fn [from_ref][89](input: [&T][44]) -> T

Converts to this type from a reference to the input type.

[Source][90]§

### impl<H, T> [HandlerWithoutStateExt][91]<T> for H

where H: [Handler][92]<T, [()][93]>,

[Source][94]§

#### fn [into_service][95](self) -> [HandlerService][96]<H, T, [()][93]>

Convert the handler into a [`Service`] and no state.

[Source][97]§

#### fn [into_make_service][98](self) -> [IntoMakeService][99]<[HandlerService][96]<H, T, [()][93]>>

Convert the handler into a [`MakeService`][100] and no state. [Read more][98]

[Source][101]§

#### fn [into_make_service_with_connect_info][102]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][103]<[HandlerService][96]<H, T, [()][93]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][100] which stores information about the incoming connection and has no state. [Read more][102]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][104] [`Span`][105], returning an `Instrumented` wrapper. Read more

[Source][106]§

### impl<T, U> [Into][107]<U> for T

where U: [From][32]<T>,

[Source][108]§

#### fn [into][109](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][32]<T> for U` chooses to do.

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

[Source][110]§

### impl<P, T> [Receiver][111] for P

where P: [Deref][25]<Target = T> \+ ?[Sized][66], T: ?[Sized][66],

[Source][112]§

#### type [Target][113] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][114]§

### impl<R> [Rng][115] for R

where R: [RngCore][116] \+ ?[Sized][66],

[Source][117]§

#### fn [random][118]<T>(&mut self) -> T

where [StandardUniform][119]: [Distribution][120]<T>,

Return a random value via the [`StandardUniform`][119] distribution. [Read more][118]

[Source][121]§

#### fn [random_iter][122]<T>(self) -> [Iter][123]<[StandardUniform][119], Self, T>

where Self: [Sized][66], [StandardUniform][119]: [Distribution][120]<T>,

Return an iterator over [`random`][124] variates [Read more][122]

[Source][125]§

#### fn [random_range][126]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][127], R: [SampleRange][128]<T>,

Generate a random value in the given range. [Read more][126]

[Source][129]§

#### fn [random_bool][130](&mut self, p: [f64][131]) -> [bool][132]

Return a bool with a probability `p` of being true. [Read more][130]

[Source][133]§

#### fn [random_ratio][134](&mut self, numerator: [u32][135], denominator: [u32][135]) -> [bool][132]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][134]

[Source][136]§

#### fn [sample][137]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][120]<T>,

Sample a new value, using the given distribution. [Read more][137]

[Source][138]§

#### fn [sample_iter][139]<T, D>(self, distr: D) -> [Iter][123]<D, Self, T>

where D: [Distribution][120]<T>, Self: [Sized][66],

Create an iterator that generates values using the given distribution. [Read more][139]

[Source][140]§

#### fn [fill][141]<T>(&mut self, dest: [&mut T][44])

where T: [Fill][142] \+ ?[Sized][66],

Fill any type implementing [`Fill`][142] with random data [Read more][141]

[Source][143]§

#### fn [gen][144]<T>(&mut self) -> T

where [StandardUniform][119]: [Distribution][120]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][124].

[Source][145]§

#### fn [gen_range][146]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][127], R: [SampleRange][128]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][147].

[Source][148]§

#### fn [gen_bool][149](&mut self, p: [f64][131]) -> [bool][132]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][150].

[Source][151]§

#### fn [gen_ratio][152](&mut self, numerator: [u32][135], denominator: [u32][135]) -> [bool][132]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][153].

[Source][154]§

### impl<T> [RngCore][116] for T

where T: [DerefMut][29], <T as [Deref][25]>::[Target][28]: [RngCore][116],

[Source][155]§

#### fn [next_u32][156](&mut self) -> [u32][135]

Return the next random `u32`. [Read more][156]

[Source][157]§

#### fn [next_u64][158](&mut self) -> [u64][159]

Return the next random `u64`. [Read more][158]

[Source][160]§

#### fn [fill_bytes][161](&mut self, dst: &mut [[u8][11]])

Fill `dest` with random data. [Read more][161]

[Source][162]§

### impl<T> [Same][163] for T

[Source][164]§

#### type [Output][165] = T

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

Add some shareable value to [request extensions][166]. Read more

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

Follow redirect resposes using the [`Standard`][167] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][168]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][66],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][169] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][168]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][66],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][169] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][168]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][66],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][169] on responses. Read more

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

#### fn request_body_limit(self, limit: [usize][170]) -> RequestBodyLimit<Self>

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

[Source][171]§

### impl<T> [ToOwned][172] for T

where T: [Clone][14],

[Source][173]§

#### type [Owned][174] = T

The resulting type after obtaining ownership.

[Source][175]§

#### fn [to_owned][176](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][176]

[Source][177]§

#### fn [clone_into][178](&self, target: [&mut T][44])

Uses borrowed data to replace owned data, usually by cloning. [Read more][178]

[Source][179]§

### impl<T, U> [TryFrom][180]<U> for T

where U: [Into][107]<T>,

[Source][181]§

#### type [Error][182] = [Infallible][183]

The type returned in the event of a conversion error.

[Source][184]§

#### fn [try_from][185](value: U) -> [Result][12]<T, <T as [TryFrom][180]<U>>::[Error][186]>

Performs the conversion.

[Source][187]§

### impl<T, U> [TryInto][188]<U> for T

where U: [TryFrom][180]<T>,

[Source][189]§

#### type [Error][190] = <U as [TryFrom][180]<T>>::[Error][186]

The type returned in the event of a conversion error.

[Source][191]§

#### fn [try_into][192](self) -> [Result][12]<U, <U as [TryFrom][180]<T>>::[Error][186]>

Performs the conversion.

[Source][193]§

### impl<R> [TryRngCore][194] for R

where R: [RngCore][116] \+ ?[Sized][66],

[Source][195]§

#### type [Error][196] = [Infallible][183]

The type returned in the event of a RNG error.

[Source][197]§

#### fn [try_next_u32][198](&mut self) -> [Result][12]<[u32][135], <R as [TryRngCore][194]>::[Error][199]>

Return the next random `u32`.

[Source][200]§

#### fn [try_next_u64][201](&mut self) -> [Result][12]<[u64][159], <R as [TryRngCore][194]>::[Error][199]>

Return the next random `u64`.

[Source][202]§

#### fn [try_fill_bytes][203]( &mut self, dst: &mut [[u8][11]], ) -> [Result][12]<[()][93], <R as [TryRngCore][194]>::[Error][199]>

Fill `dest` entirely with random data.

[Source][204]§

#### fn [unwrap_err][205](self) -> [UnwrapErr][206]<Self>

where Self: [Sized][66],

Wrap RNG with the [`UnwrapErr`][206] wrapper.

[Source][207]§

#### fn [unwrap_mut][208](&mut self) -> [UnwrapMut][209]<'_, Self>

Wrap RNG with the [`UnwrapMut`][209] wrapper.

[Source][210]§

#### fn [read_adapter][211](&mut self) -> [RngReadAdapter][212]<'_, Self>

where Self: [Sized][66],

Available on **crate feature`std`** only.

Convert an [`RngCore`][116] to a [`RngReadAdapter`][212].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][107]<Dispatch>,

Attaches the provided [`Subscriber`][213] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][214] [`Subscriber`][213] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][215]§

### impl<T> [CryptoRng][216] for T

where T: [DerefMut][29], <T as [Deref][25]>::[Target][28]: [CryptoRng][216],

[Source][217]§

### impl<R> [TryCryptoRng][218] for R

where R: [CryptoRng][216] \+ ?[Sized][66],

   [1]: ../axum/index.html
   [2]: index.html
   [3]: ../src/axum/json.rs.html#97
   [4]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [5]: extract/rejection/enum.JsonRejection.html (enum axum::extract::rejection::JsonRejection)
   [6]: extract/index.html#the-order-of-extractors (mod axum::extract)
   [7]: https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html (trait serde_core::ser::Serialize)
   [8]: ../src/axum/json.rs.html#157-195
   [9]: struct.Json.html (struct axum::Json)
   [10]: ../src/axum/json.rs.html#164-194
   [11]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [12]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [13]: ../src/axum/json.rs.html#94
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [15]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [16]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [17]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [19]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [20]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [21]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [22]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [23]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [24]: ../src/axum/json.rs.html#149
   [25]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [26]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [27]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [28]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [29]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [30]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [31]: ../src/axum/json.rs.html#151-155
   [32]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [33]: ../src/axum/json.rs.html#152-154
   [34]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [35]: ../src/axum/json.rs.html#99-114
   [36]: extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [39]: ../src/axum/json.rs.html#104
   [40]: extract/trait.FromRequest.html#associatedtype.Rejection
   [41]: ../src/axum/json.rs.html#106-113
   [42]: extract/trait.FromRequest.html#tymethod.from_request
   [43]: extract/type.Request.html (type axum::extract::Request)
   [44]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [45]: extract/trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [46]: ../src/axum/json.rs.html#197-232
   [47]: response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [48]: ../src/axum/json.rs.html#201-231
   [49]: response/trait.IntoResponse.html#tymethod.into_response
   [50]: response/type.Response.html (type axum::response::Response)
   [51]: ../src/axum/json.rs.html#116-136
   [52]: extract/trait.OptionalFromRequest.html (trait axum::extract::OptionalFromRequest)
   [53]: ../src/axum/json.rs.html#121
   [54]: extract/trait.OptionalFromRequest.html#associatedtype.Rejection
   [55]: ../src/axum/json.rs.html#123-135
   [56]: extract/trait.OptionalFromRequest.html#tymethod.from_request
   [57]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [58]: extract/trait.OptionalFromRequest.html#associatedtype.Rejection (type axum::extract::OptionalFromRequest::Rejection)
   [59]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
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
   [78]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [79]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [80]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [81]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [82]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#802
   [84]: https://doc.rust-lang.org/nightly/std/primitive.never.html
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#803
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [88]: extract/trait.FromRef.html (trait axum::extract::FromRef)
   [89]: extract/trait.FromRef.html#tymethod.from_ref
   [90]: ../src/axum/handler/mod.rs.html#380-398
   [91]: handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [92]: handler/trait.Handler.html (trait axum::handler::Handler)
   [93]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [94]: ../src/axum/handler/mod.rs.html#384-386
   [95]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [96]: handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [97]: ../src/axum/handler/mod.rs.html#388-390
   [98]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [99]: routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [100]: tower::make::MakeService
   [101]: ../src/axum/handler/mod.rs.html#393-397
   [102]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [103]: extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [104]: super::Span::current()
   [105]: crate::Span
   [106]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [107]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [108]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [109]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [110]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [111]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [112]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [113]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [114]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [115]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [116]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [117]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [118]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [119]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [120]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [121]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [122]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [123]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [124]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [125]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [126]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [127]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [128]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [129]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [130]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [131]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [132]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [133]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [134]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [135]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [136]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [137]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [138]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [139]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [140]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [141]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [142]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [143]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [144]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [145]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [146]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [147]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [148]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [149]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [150]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [151]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [152]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [153]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [154]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [155]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [156]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [157]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [158]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [159]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [160]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [161]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [162]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [163]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [164]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [165]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [166]: https://docs.rs/http/latest/http/struct.Extensions.html
   [167]: crate::follow_redirect::policy::Standard
   [168]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [169]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [170]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [171]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [172]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [173]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [174]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [175]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [176]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [177]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [178]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [179]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [180]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [181]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [182]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [183]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [184]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [185]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [186]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [187]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [188]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [189]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [190]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [191]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [192]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [193]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [194]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [195]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [196]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [197]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [198]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [199]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [200]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [201]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [202]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [203]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [204]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [205]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [206]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [207]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [208]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [209]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [210]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [211]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [212]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [213]: super::Subscriber
   [214]: dispatcher#setting-the-default-subscriber
   [215]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [216]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [217]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [218]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

