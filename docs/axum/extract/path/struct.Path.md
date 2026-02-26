<!-- Generated from rustdoc HTML: extract/path/struct.Path.html -->
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



## [In axum::extract::path][2]

[axum][3]::[extract][4]::[path][2]

# Struct Path Copy item path

[Source][5]
``` 
pub struct Path<T>(pub T);
```

Expand description

Extractor that will get captures from the URL and parse them using [`serde`][6].

Any percent encoded parameters will be automatically decoded. The decoded parameters must be valid UTF-8, otherwise `Path` will fail and return a `400 Bad Request` response.

## §`Option<Path<T>>` behavior

You can use `Option<Path<T>>` as an extractor to allow the same handler to be used in a route with parameters that deserialize to `T`, and another route with no parameters at all.

## §Example

These examples assume the `serde` feature of the [`uuid`][7] crate is enabled.

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

Path segments also can be deserialized into any type that implements [`serde::Deserialize`][8]. This includes tuples and structs:
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

If the URI cannot be deserialized into the target type the request will be rejected and an error response will be returned. See [`customize-path-rejection`][9] for an example of how to customize that error.

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][10]§

### impl<T: [Debug][11]> [Debug][11] for [Path][12]<T>

[Source][10]§

#### fn [fmt][13](&self, f: &mut [Formatter][14]<'_>) -> [Result][15]

Formats the value using the given formatter. [Read more][13]

[Source][16]§

### impl<T> [Deref][17] for [Path][12]<T>

[Source][16]§

#### type [Target][18] = T

The resulting type after dereferencing.

[Source][16]§

#### fn [deref][19](&self) -> &Self::[Target][20]

Dereferences the value.

[Source][16]§

### impl<T> [DerefMut][21] for [Path][12]<T>

[Source][16]§

#### fn [deref_mut][22](&mut self) -> &mut Self::[Target][20]

Mutably dereferences the value.

[Source][23]§

### impl<T, S> [FromRequestParts][24]<S> for [Path][12]<T>

where T: [DeserializeOwned][25] \+ [Send][26], S: [Send][26] \+ [Sync][27],

[Source][28]§

#### type [Rejection][29] = [PathRejection][30]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][31]§

#### async fn [from_request_parts][32]( parts: &mut Parts, _state: [&S][33], ) -> [Result][34]<Self, Self::[Rejection][35]>

Perform the extraction.

[Source][36]§

### impl<T, S> [OptionalFromRequestParts][37]<S> for [Path][12]<T>

where T: [DeserializeOwned][25] \+ [Send][26] \+ 'static, S: [Send][26] \+ [Sync][27],

[Source][38]§

#### type [Rejection][39] = [PathRejection][30]

If the extractor fails, it will use this “rejection” type. [Read more][39]

[Source][40]§

#### async fn [from_request_parts][41]( parts: &mut Parts, _state: [&S][33], ) -> [Result][34]<[Option][42]<Self>, Self::[Rejection][43]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl<T> [Freeze][44] for [Path][12]<T>

where T: [Freeze][44],

§

### impl<T> [RefUnwindSafe][45] for [Path][12]<T>

where T: [RefUnwindSafe][45],

§

### impl<T> [Send][26] for [Path][12]<T>

where T: [Send][26],

§

### impl<T> [Sync][27] for [Path][12]<T>

where T: [Sync][27],

§

### impl<T> [Unpin][46] for [Path][12]<T>

where T: [Unpin][46],

§

### impl<T> [UnwindSafe][47] for [Path][12]<T>

where T: [UnwindSafe][47],

## Blanket Implementations§

[Source][48]§

### impl<T> [Any][49] for T

where T: 'static + ?[Sized][50],

[Source][51]§

#### fn [type_id][52](&self) -> [TypeId][53]

Gets the `TypeId` of `self`. [Read more][52]

[Source][54]§

### impl<T> [Borrow][55]<T> for T

where T: ?[Sized][50],

[Source][56]§

#### fn [borrow][57](&self) -> [&T][33]

Immutably borrows from an owned value. [Read more][57]

[Source][58]§

### impl<T> [BorrowMut][59]<T> for T

where T: ?[Sized][50],

[Source][60]§

#### fn [borrow_mut][61](&mut self) -> [&mut T][33]

Mutably borrows from an owned value. [Read more][61]

[Source][62]§

### impl<T> [From][63]<T> for T

[Source][64]§

#### fn [from][65](t: T) -> T

Returns the argument unchanged.

§

### impl<S, T> [FromRequest][66]<S, ViaParts> for T

where S: [Send][26] \+ [Sync][27], T: [FromRequestParts][24]<S>,

§

#### type [Rejection][67] = <T as [FromRequestParts][24]<S>>::[Rejection][35]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][68]( req: Request<[Body][69]>, state: [&S][33], ) -> impl [Future][70]<Output = [Result][34]<T, <T as [FromRequest][66]<S, ViaParts>>::[Rejection][71]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][72] [`Span`][73], returning an `Instrumented` wrapper. Read more

[Source][74]§

### impl<T, U> [Into][75]<U> for T

where U: [From][63]<T>,

[Source][76]§

#### fn [into][77](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][63]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][50],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][78]§

### impl<P, T> [Receiver][79] for P

where P: [Deref][17]<Target = T> \+ ?[Sized][50], T: ?[Sized][50],

[Source][80]§

#### type [Target][81] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][82]§

### impl<R> [Rng][83] for R

where R: [RngCore][84] \+ ?[Sized][50],

[Source][85]§

#### fn [random][86]<T>(&mut self) -> T

where [StandardUniform][87]: [Distribution][88]<T>,

Return a random value via the [`StandardUniform`][87] distribution. [Read more][86]

[Source][89]§

#### fn [random_iter][90]<T>(self) -> [Iter][91]<[StandardUniform][87], Self, T>

where Self: [Sized][50], [StandardUniform][87]: [Distribution][88]<T>,

Return an iterator over [`random`][92] variates [Read more][90]

[Source][93]§

#### fn [random_range][94]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][95], R: [SampleRange][96]<T>,

Generate a random value in the given range. [Read more][94]

[Source][97]§

#### fn [random_bool][98](&mut self, p: [f64][99]) -> [bool][100]

Return a bool with a probability `p` of being true. [Read more][98]

[Source][101]§

#### fn [random_ratio][102](&mut self, numerator: [u32][103], denominator: [u32][103]) -> [bool][100]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][102]

[Source][104]§

#### fn [sample][105]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][88]<T>,

Sample a new value, using the given distribution. [Read more][105]

[Source][106]§

#### fn [sample_iter][107]<T, D>(self, distr: D) -> [Iter][91]<D, Self, T>

where D: [Distribution][88]<T>, Self: [Sized][50],

Create an iterator that generates values using the given distribution. [Read more][107]

[Source][108]§

#### fn [fill][109]<T>(&mut self, dest: [&mut T][33])

where T: [Fill][110] \+ ?[Sized][50],

Fill any type implementing [`Fill`][110] with random data [Read more][109]

[Source][111]§

#### fn [gen][112]<T>(&mut self) -> T

where [StandardUniform][87]: [Distribution][88]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][92].

[Source][113]§

#### fn [gen_range][114]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][95], R: [SampleRange][96]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][115].

[Source][116]§

#### fn [gen_bool][117](&mut self, p: [f64][99]) -> [bool][100]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][118].

[Source][119]§

#### fn [gen_ratio][120](&mut self, numerator: [u32][103], denominator: [u32][103]) -> [bool][100]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][121].

[Source][122]§

### impl<T> [RngCore][84] for T

where T: [DerefMut][21], <T as [Deref][17]>::[Target][20]: [RngCore][84],

[Source][123]§

#### fn [next_u32][124](&mut self) -> [u32][103]

Return the next random `u32`. [Read more][124]

[Source][125]§

#### fn [next_u64][126](&mut self) -> [u64][127]

Return the next random `u64`. [Read more][126]

[Source][128]§

#### fn [fill_bytes][129](&mut self, dst: &mut [[u8][130]])

Fill `dest` with random data. [Read more][129]

[Source][131]§

### impl<T> [Same][132] for T

[Source][133]§

#### type [Output][134] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][50],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][50],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][135]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][50],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][50],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][50],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][50],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][50],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][50],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][50],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][136] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][137]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][50],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][138] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][137]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][50],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][138] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][137]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][50],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][138] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][50],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][50],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][50],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][50],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][50],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][50],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][50], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][50], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][50],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][50],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][50],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][139]) -> RequestBodyLimit<Self>

where Self: [Sized][50],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][50],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][50],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][140]§

### impl<T, U> [TryFrom][141]<U> for T

where U: [Into][75]<T>,

[Source][142]§

#### type [Error][143] = [Infallible][144]

The type returned in the event of a conversion error.

[Source][145]§

#### fn [try_from][146](value: U) -> [Result][34]<T, <T as [TryFrom][141]<U>>::[Error][147]>

Performs the conversion.

[Source][148]§

### impl<T, U> [TryInto][149]<U> for T

where U: [TryFrom][141]<T>,

[Source][150]§

#### type [Error][151] = <U as [TryFrom][141]<T>>::[Error][147]

The type returned in the event of a conversion error.

[Source][152]§

#### fn [try_into][153](self) -> [Result][34]<U, <U as [TryFrom][141]<T>>::[Error][147]>

Performs the conversion.

[Source][154]§

### impl<R> [TryRngCore][155] for R

where R: [RngCore][84] \+ ?[Sized][50],

[Source][156]§

#### type [Error][157] = [Infallible][144]

The type returned in the event of a RNG error.

[Source][158]§

#### fn [try_next_u32][159](&mut self) -> [Result][34]<[u32][103], <R as [TryRngCore][155]>::[Error][160]>

Return the next random `u32`.

[Source][161]§

#### fn [try_next_u64][162](&mut self) -> [Result][34]<[u64][127], <R as [TryRngCore][155]>::[Error][160]>

Return the next random `u64`.

[Source][163]§

#### fn [try_fill_bytes][164]( &mut self, dst: &mut [[u8][130]], ) -> [Result][34]<[()][165], <R as [TryRngCore][155]>::[Error][160]>

Fill `dest` entirely with random data.

[Source][166]§

#### fn [unwrap_err][167](self) -> [UnwrapErr][168]<Self>

where Self: [Sized][50],

Wrap RNG with the [`UnwrapErr`][168] wrapper.

[Source][169]§

#### fn [unwrap_mut][170](&mut self) -> [UnwrapMut][171]<'_, Self>

Wrap RNG with the [`UnwrapMut`][171] wrapper.

[Source][172]§

#### fn [read_adapter][173](&mut self) -> [RngReadAdapter][174]<'_, Self>

where Self: [Sized][50],

Available on **crate feature`std`** only.

Convert an [`RngCore`][84] to a [`RngReadAdapter`][174].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][75]<Dispatch>,

Attaches the provided [`Subscriber`][175] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][176] [`Subscriber`][175] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][177]§

### impl<T> [CryptoRng][178] for T

where T: [DerefMut][21], <T as [Deref][17]>::[Target][20]: [CryptoRng][178],

[Source][179]§

### impl<R> [TryCryptoRng][180] for R

where R: [CryptoRng][178] \+ ?[Sized][50],

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/path/mod.rs.html#153
   [6]: https://crates.io/crates/serde
   [7]: https://crates.io/crates/uuid
   [8]: https://docs.rs/serde/1.0.127/serde/trait.Deserialize.html
   [9]: https://github.com/tokio-rs/axum/blob/main/examples/customize-path-rejection/src/main.rs
   [10]: ../../../src/axum/extract/path/mod.rs.html#152
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [12]: ../struct.Path.html (struct axum::extract::Path)
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [16]: ../../../src/axum/extract/path/mod.rs.html#155
   [17]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [18]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [19]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [20]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [21]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [22]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [23]: ../../../src/axum/extract/path/mod.rs.html#157-190
   [24]: ../trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [25]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [28]: ../../../src/axum/extract/path/mod.rs.html#162
   [29]: ../trait.FromRequestParts.html#associatedtype.Rejection
   [30]: ../rejection/enum.PathRejection.html (enum axum::extract::rejection::PathRejection)
   [31]: ../../../src/axum/extract/path/mod.rs.html#164-189
   [32]: ../trait.FromRequestParts.html#tymethod.from_request_parts
   [33]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [34]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [35]: ../trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [36]: ../../../src/axum/extract/path/mod.rs.html#192-213
   [37]: ../trait.OptionalFromRequestParts.html (trait axum::extract::OptionalFromRequestParts)
   [38]: ../../../src/axum/extract/path/mod.rs.html#197
   [39]: ../trait.OptionalFromRequestParts.html#associatedtype.Rejection
   [40]: ../../../src/axum/extract/path/mod.rs.html#199-212
   [41]: ../trait.OptionalFromRequestParts.html#tymethod.from_request_parts
   [42]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [43]: ../trait.OptionalFromRequestParts.html#associatedtype.Rejection (type axum::extract::OptionalFromRequestParts::Rejection)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [45]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [46]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [47]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [48]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [49]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [50]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [51]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [52]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [53]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [56]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [57]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [58]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [59]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [60]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [61]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [66]: ../trait.FromRequest.html (trait axum::extract::FromRequest)
   [67]: ../trait.FromRequest.html#associatedtype.Rejection
   [68]: ../trait.FromRequest.html#tymethod.from_request
   [69]: ../../body/struct.Body.html (struct axum::body::Body)
   [70]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [71]: ../trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [72]: super::Span::current()
   [73]: crate::Span
   [74]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [75]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [76]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [77]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [78]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [79]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [80]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [81]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [82]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [83]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [84]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [85]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [86]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [87]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [88]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [89]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [90]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [91]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [92]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [93]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [94]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [95]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [96]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [97]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [98]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [99]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [100]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [101]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [102]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [103]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [104]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [105]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [106]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [107]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [108]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [109]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [110]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [111]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [112]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [113]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [114]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [115]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [116]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [117]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [118]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [119]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [120]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [121]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [122]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [123]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [124]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [125]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [126]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [127]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [128]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [129]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [130]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [131]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [132]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [133]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [134]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [135]: https://docs.rs/http/latest/http/struct.Extensions.html
   [136]: crate::follow_redirect::policy::Standard
   [137]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [138]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [139]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [140]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [141]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [142]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [143]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [144]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [145]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [146]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [147]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [148]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [149]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [150]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [151]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [152]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [153]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [154]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [155]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [156]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [157]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [158]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [159]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [160]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [161]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [162]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [163]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [164]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [165]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [166]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [167]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [168]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [169]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [170]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [171]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [172]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [173]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [174]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [175]: super::Subscriber
   [176]: dispatcher#setting-the-default-subscriber
   [177]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [178]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [179]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [180]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

