<!-- Generated from rustdoc HTML: extract/struct.Path.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Path

## [axum][1]0.8.8

## Path

### Sections

  * `Option<Path<T>>` behavior
  * Example
  * Providing detailed rejection output



### Tuple Fields

  * 0



### Trait Implementations

  * Debug
  * Deref
  * DerefMut
  * FromRequestParts<S>
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
  * CryptoRng
  * From<T>
  * FromRequest<S, ViaParts>
  * Instrument
  * Into<U>
  * PolicyExt
  * Receiver
  * Rng
  * RngCore
  * Same
  * ServiceExt
  * TryCryptoRng
  * TryFrom<U>
  * TryInto<U>
  * TryRngCore
  * VZip<V>
  * WithSubscriber



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct Path Copy item path

[Source][4]
``` 
pub struct Path<T>(pub T);
```

Expand description

Extractor that will get captures from the URL and parse them using [`serde`][5].

Any percent encoded parameters will be automatically decoded. The decoded parameters must be valid UTF-8, otherwise `Path` will fail and return a `400 Bad Request` response.

## §`Option<Path<T>>` behavior

You can use `Option<Path<T>>` as an extractor to allow the same handler to be used in a route with parameters that deserialize to `T`, and another route with no parameters at all.

## §Example

These examples assume the `serde` feature of the [`uuid`][6] crate is enabled.

One `Path` can extract multiple captures. It is not necessary (and does not work) to give a handler more than one `Path` argument.
``` 
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use uuid::Uuid;

async fn users_teams_show(
    Path((user_id, team_id)): Path<(Uuid, Uuid)>,
) {
    // ...
}

let app = Router::new().route("/users/{user_id}/team/{team_id}", get(users_teams_show));
```

If the path contains only one parameter, then you can omit the tuple.
``` 
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use uuid::Uuid;

async fn user_info(Path(user_id): Path<Uuid>) {
    // ...
}

let app = Router::new().route("/users/{user_id}", get(user_info));
```

Path segments also can be deserialized into any type that implements [`serde::Deserialize`][7]. This includes tuples and structs:
``` 
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

// Path segment labels will be matched with struct field names
#[derive(Deserialize)]
struct Params {
    user_id: Uuid,
    team_id: Uuid,
}

async fn users_teams_show(
    Path(Params { user_id, team_id }): Path<Params>,
) {
    // ...
}

// When using tuples the path segments will be matched by their position in the route
async fn users_teams_create(
    Path((user_id, team_id)): Path<(String, String)>,
) {
    // ...
}

let app = Router::new().route(
    "/users/{user_id}/team/{team_id}",
    get(users_teams_show).post(users_teams_create),
);
```

If you wish to capture all path parameters you can use `HashMap` or `Vec`:
``` 
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use std::collections::HashMap;

async fn params_map(
    Path(params): Path<HashMap<String, String>>,
) {
    // ...
}

async fn params_vec(
    Path(params): Path<Vec<(String, String)>>,
) {
    // ...
}

let app = Router::new()
    .route("/users/{user_id}/team/{team_id}", get(params_map).post(params_vec));
```

## §Providing detailed rejection output

If the URI cannot be deserialized into the target type the request will be rejected and an error response will be returned. See [`customize-path-rejection`][8] for an example of how to customize that error.

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][9]§

### impl<T: [Debug][10]> [Debug][10] for [Path][11]<T>

[Source][9]§

#### fn [fmt][12](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]

Formats the value using the given formatter. [Read more][12]

[Source][15]§

### impl<T> [Deref][16] for [Path][11]<T>

[Source][15]§

#### type [Target][17] = T

The resulting type after dereferencing.

[Source][15]§

#### fn [deref][18](&self) -> &Self::[Target][19]

Dereferences the value.

[Source][15]§

### impl<T> [DerefMut][20] for [Path][11]<T>

[Source][15]§

#### fn [deref_mut][21](&mut self) -> &mut Self::[Target][19]

Mutably dereferences the value.

[Source][22]§

### impl<T, S> [FromRequestParts][23]<S> for [Path][11]<T>

where T: [DeserializeOwned][24] \+ [Send][25], S: [Send][25] \+ [Sync][26],

[Source][27]§

#### type [Rejection][28] = [PathRejection][29]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][30]§

#### async fn [from_request_parts][31]( parts: &mut Parts, _state: [&S][32], ) -> [Result][33]<Self, Self::[Rejection][34]>

Perform the extraction.

[Source][35]§

### impl<T, S> [OptionalFromRequestParts][36]<S> for [Path][11]<T>

where T: [DeserializeOwned][24] \+ [Send][25] \+ 'static, S: [Send][25] \+ [Sync][26],

[Source][37]§

#### type [Rejection][38] = [PathRejection][29]

If the extractor fails, it will use this “rejection” type. [Read more][38]

[Source][39]§

#### async fn [from_request_parts][40]( parts: &mut Parts, _state: [&S][32], ) -> [Result][33]<[Option][41]<Self>, Self::[Rejection][42]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl<T> [Freeze][43] for [Path][11]<T>

where T: [Freeze][43],

§

### impl<T> [RefUnwindSafe][44] for [Path][11]<T>

where T: [RefUnwindSafe][44],

§

### impl<T> [Send][25] for [Path][11]<T>

where T: [Send][25],

§

### impl<T> [Sync][26] for [Path][11]<T>

where T: [Sync][26],

§

### impl<T> [Unpin][45] for [Path][11]<T>

where T: [Unpin][45],

§

### impl<T> [UnwindSafe][46] for [Path][11]<T>

where T: [UnwindSafe][46],

## Blanket Implementations§

[Source][47]§

### impl<T> [Any][48] for T

where T: 'static + ?[Sized][49],

[Source][50]§

#### fn [type_id][51](&self) -> [TypeId][52]

Gets the `TypeId` of `self`. [Read more][51]

[Source][53]§

### impl<T> [Borrow][54]<T> for T

where T: ?[Sized][49],

[Source][55]§

#### fn [borrow][56](&self) -> [&T][32]

Immutably borrows from an owned value. [Read more][56]

[Source][57]§

### impl<T> [BorrowMut][58]<T> for T

where T: ?[Sized][49],

[Source][59]§

#### fn [borrow_mut][60](&mut self) -> [&mut T][32]

Mutably borrows from an owned value. [Read more][60]

[Source][61]§

### impl<T> [From][62]<T> for T

[Source][63]§

#### fn [from][64](t: T) -> T

Returns the argument unchanged.

§

### impl<S, T> [FromRequest][65]<S, ViaParts> for T

where S: [Send][25] \+ [Sync][26], T: [FromRequestParts][23]<S>,

§

#### type [Rejection][66] = <T as [FromRequestParts][23]<S>>::[Rejection][34]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][67]( req: Request<[Body][68]>, state: [&S][32], ) -> impl [Future][69]<Output = [Result][33]<T, <T as [FromRequest][65]<S, ViaParts>>::[Rejection][70]>>

Perform the extraction.

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

where U: [From][62]<T>,

[Source][75]§

#### fn [into][76](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][62]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][49],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][77]§

### impl<P, T> [Receiver][78] for P

where P: [Deref][16]<Target = T> \+ ?[Sized][49], T: ?[Sized][49],

[Source][79]§

#### type [Target][80] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][81]§

### impl<R> [Rng][82] for R

where R: [RngCore][83] \+ ?[Sized][49],

[Source][84]§

#### fn [random][85]<T>(&mut self) -> T

where [StandardUniform][86]: [Distribution][87]<T>,

Return a random value via the [`StandardUniform`][86] distribution. [Read more][85]

[Source][88]§

#### fn [random_iter][89]<T>(self) -> [Iter][90]<[StandardUniform][86], Self, T>

where Self: [Sized][49], [StandardUniform][86]: [Distribution][87]<T>,

Return an iterator over [`random`][91] variates [Read more][89]

[Source][92]§

#### fn [random_range][93]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][94], R: [SampleRange][95]<T>,

Generate a random value in the given range. [Read more][93]

[Source][96]§

#### fn [random_bool][97](&mut self, p: [f64][98]) -> [bool][99]

Return a bool with a probability `p` of being true. [Read more][97]

[Source][100]§

#### fn [random_ratio][101](&mut self, numerator: [u32][102], denominator: [u32][102]) -> [bool][99]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][101]

[Source][103]§

#### fn [sample][104]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][87]<T>,

Sample a new value, using the given distribution. [Read more][104]

[Source][105]§

#### fn [sample_iter][106]<T, D>(self, distr: D) -> [Iter][90]<D, Self, T>

where D: [Distribution][87]<T>, Self: [Sized][49],

Create an iterator that generates values using the given distribution. [Read more][106]

[Source][107]§

#### fn [fill][108]<T>(&mut self, dest: [&mut T][32])

where T: [Fill][109] \+ ?[Sized][49],

Fill any type implementing [`Fill`][109] with random data [Read more][108]

[Source][110]§

#### fn [gen][111]<T>(&mut self) -> T

where [StandardUniform][86]: [Distribution][87]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][91].

[Source][112]§

#### fn [gen_range][113]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][94], R: [SampleRange][95]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][114].

[Source][115]§

#### fn [gen_bool][116](&mut self, p: [f64][98]) -> [bool][99]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][117].

[Source][118]§

#### fn [gen_ratio][119](&mut self, numerator: [u32][102], denominator: [u32][102]) -> [bool][99]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][120].

[Source][121]§

### impl<T> [RngCore][83] for T

where T: [DerefMut][20], <T as [Deref][16]>::[Target][19]: [RngCore][83],

[Source][122]§

#### fn [next_u32][123](&mut self) -> [u32][102]

Return the next random `u32`. [Read more][123]

[Source][124]§

#### fn [next_u64][125](&mut self) -> [u64][126]

Return the next random `u64`. [Read more][125]

[Source][127]§

#### fn [fill_bytes][128](&mut self, dst: &mut [[u8][129]])

Fill `dest` with random data. [Read more][128]

[Source][130]§

### impl<T> [Same][131] for T

[Source][132]§

#### type [Output][133] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][49],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][49],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][134]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][49],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][49],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][49],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][49],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][49],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][49],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][49],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][135] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][136]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][49],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][137] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][136]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][49],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][137] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][136]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][49],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][137] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][49],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][49],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][49],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][49],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][49],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][49],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][49], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][49], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][49],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][49],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][49],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][138]) -> RequestBodyLimit<Self>

where Self: [Sized][49],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][49],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][49],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][139]§

### impl<T, U> [TryFrom][140]<U> for T

where U: [Into][74]<T>,

[Source][141]§

#### type [Error][142] = [Infallible][143]

The type returned in the event of a conversion error.

[Source][144]§

#### fn [try_from][145](value: U) -> [Result][33]<T, <T as [TryFrom][140]<U>>::[Error][146]>

Performs the conversion.

[Source][147]§

### impl<T, U> [TryInto][148]<U> for T

where U: [TryFrom][140]<T>,

[Source][149]§

#### type [Error][150] = <U as [TryFrom][140]<T>>::[Error][146]

The type returned in the event of a conversion error.

[Source][151]§

#### fn [try_into][152](self) -> [Result][33]<U, <U as [TryFrom][140]<T>>::[Error][146]>

Performs the conversion.

[Source][153]§

### impl<R> [TryRngCore][154] for R

where R: [RngCore][83] \+ ?[Sized][49],

[Source][155]§

#### type [Error][156] = [Infallible][143]

The type returned in the event of a RNG error.

[Source][157]§

#### fn [try_next_u32][158](&mut self) -> [Result][33]<[u32][102], <R as [TryRngCore][154]>::[Error][159]>

Return the next random `u32`.

[Source][160]§

#### fn [try_next_u64][161](&mut self) -> [Result][33]<[u64][126], <R as [TryRngCore][154]>::[Error][159]>

Return the next random `u64`.

[Source][162]§

#### fn [try_fill_bytes][163]( &mut self, dst: &mut [[u8][129]], ) -> [Result][33]<[()][164], <R as [TryRngCore][154]>::[Error][159]>

Fill `dest` entirely with random data.

[Source][165]§

#### fn [unwrap_err][166](self) -> [UnwrapErr][167]<Self>

where Self: [Sized][49],

Wrap RNG with the [`UnwrapErr`][167] wrapper.

[Source][168]§

#### fn [unwrap_mut][169](&mut self) -> [UnwrapMut][170]<'_, Self>

Wrap RNG with the [`UnwrapMut`][170] wrapper.

[Source][171]§

#### fn [read_adapter][172](&mut self) -> [RngReadAdapter][173]<'_, Self>

where Self: [Sized][49],

Available on **crate feature`std`** only.

Convert an [`RngCore`][83] to a [`RngReadAdapter`][173].

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

Attaches the provided [`Subscriber`][174] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][175] [`Subscriber`][174] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][176]§

### impl<T> [CryptoRng][177] for T

where T: [DerefMut][20], <T as [Deref][16]>::[Target][19]: [CryptoRng][177],

[Source][178]§

### impl<R> [TryCryptoRng][179] for R

where R: [CryptoRng][177] \+ ?[Sized][49],

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/path/mod.rs.html#153
   [5]: https://crates.io/crates/serde
   [6]: https://crates.io/crates/uuid
   [7]: https://docs.rs/serde/1.0.127/serde/trait.Deserialize.html
   [8]: https://github.com/tokio-rs/axum/blob/main/examples/customize-path-rejection/src/main.rs
   [9]: ../../src/axum/extract/path/mod.rs.html#152
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [11]: struct.Path.html (struct axum::extract::Path)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [13]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [15]: ../../src/axum/extract/path/mod.rs.html#155
   [16]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [17]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [18]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [19]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [20]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [21]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [22]: ../../src/axum/extract/path/mod.rs.html#157-190
   [23]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [24]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [27]: ../../src/axum/extract/path/mod.rs.html#162
   [28]: trait.FromRequestParts.html#associatedtype.Rejection
   [29]: rejection/enum.PathRejection.html (enum axum::extract::rejection::PathRejection)
   [30]: ../../src/axum/extract/path/mod.rs.html#164-189
   [31]: trait.FromRequestParts.html#tymethod.from_request_parts
   [32]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [33]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [34]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [35]: ../../src/axum/extract/path/mod.rs.html#192-213
   [36]: trait.OptionalFromRequestParts.html (trait axum::extract::OptionalFromRequestParts)
   [37]: ../../src/axum/extract/path/mod.rs.html#197
   [38]: trait.OptionalFromRequestParts.html#associatedtype.Rejection
   [39]: ../../src/axum/extract/path/mod.rs.html#199-212
   [40]: trait.OptionalFromRequestParts.html#tymethod.from_request_parts
   [41]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [42]: trait.OptionalFromRequestParts.html#associatedtype.Rejection (type axum::extract::OptionalFromRequestParts::Rejection)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [44]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [46]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [47]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [48]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [49]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [50]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [51]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [52]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [57]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [58]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [59]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [60]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [65]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [66]: trait.FromRequest.html#associatedtype.Rejection
   [67]: trait.FromRequest.html#tymethod.from_request
   [68]: ../body/struct.Body.html (struct axum::body::Body)
   [69]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [70]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [71]: super::Span::current()
   [72]: crate::Span
   [73]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [74]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [77]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [78]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [79]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [80]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [81]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [82]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [83]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [84]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [85]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [86]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [87]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [88]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [89]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [90]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [91]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [92]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [93]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [94]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [95]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [96]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [97]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [98]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [99]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [100]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [101]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [102]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [103]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [104]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [105]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [106]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [107]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [108]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [109]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [110]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [111]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [112]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [113]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [114]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [115]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [116]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [117]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [118]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [119]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [120]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [121]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [122]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [123]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [124]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [125]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [126]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [127]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [128]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [129]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [130]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [131]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [132]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [133]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [134]: https://docs.rs/http/latest/http/struct.Extensions.html
   [135]: crate::follow_redirect::policy::Standard
   [136]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [137]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [138]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [139]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [140]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [141]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [142]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [143]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [144]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [145]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [146]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [147]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [148]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [149]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [150]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [151]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [152]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [153]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [154]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [155]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [156]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [157]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [158]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [159]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [160]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [161]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [162]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [163]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [164]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [165]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [166]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [167]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [168]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [169]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [170]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [171]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [172]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [173]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [174]: super::Subscriber
   [175]: dispatcher#setting-the-default-subscriber
   [176]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [177]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [178]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [179]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

