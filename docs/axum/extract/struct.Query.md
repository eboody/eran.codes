<!-- Generated from rustdoc HTML: extract/struct.Query.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Query

## [axum][1]0.8.8

## Query

### Sections

  * Examples



### Tuple Fields

  * 0



### Methods

  * try_from_uri



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * Default
  * Deref
  * DerefMut
  * FromRequestParts<S>



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



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct Query Copy item path

[Source][4]
``` 
pub struct Query<T>(pub T);
```

Available on **crate feature`query`** only.

Expand description

Extractor that deserializes query strings into some type.

`T` is expected to implement [`serde::Deserialize`][5].

## §Examples
``` 
use axum::{
    extract::Query,
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

// This will parse query strings like `?page=2&per_page=30` into `Pagination`
// structs.
async fn list_things(pagination: Query<Pagination>) {
    let pagination: Pagination = pagination.0;

    // ...
}

let app = Router::new().route("/list_things", get(list_things));
```

If the query string cannot be parsed it will reject the request with a `400 Bad Request` response.

## Tuple Fields§

§`0: T`

## Implementations§

[Source][6]§

### impl<T> [Query][7]<T>

where T: [DeserializeOwned][8],

[Source][9]

#### pub fn try_from_uri(value: &Uri) -> [Result][10]<Self, [QueryRejection][11]>

Attempts to construct a [`Query`][7] from a reference to a [`Uri`].

##### §Example
``` 
use axum::extract::Query;
use http::Uri;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExampleParams {
    foo: String,
    bar: u32,
}

let uri: Uri = "http://example.com/path?foo=hello&bar=42".parse().unwrap();
let result: Query<ExampleParams> = Query::try_from_uri(&uri).unwrap();
assert_eq!(result.foo, String::from("hello"));
assert_eq!(result.bar, 42);
```

## Trait Implementations§

[Source][12]§

### impl<T: [Clone][13]> [Clone][13] for [Query][7]<T>

[Source][12]§

#### fn [clone][14](&self) -> [Query][7]<T>

Returns a duplicate of the value. [Read more][14]

1.0.0 · [Source][15]§

#### fn [clone_from][16](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][16]

[Source][12]§

### impl<T: [Debug][17]> [Debug][17] for [Query][7]<T>

[Source][12]§

#### fn [fmt][18](&self, f: &mut [Formatter][19]<'_>) -> [Result][20]

Formats the value using the given formatter. [Read more][18]

[Source][12]§

### impl<T: [Default][21]> [Default][21] for [Query][7]<T>

[Source][12]§

#### fn [default][22]() -> [Query][7]<T>

Returns the “default value” for a type. [Read more][22]

[Source][23]§

### impl<T> [Deref][24] for [Query][7]<T>

[Source][23]§

#### type [Target][25] = T

The resulting type after dereferencing.

[Source][23]§

#### fn [deref][26](&self) -> &Self::[Target][27]

Dereferences the value.

[Source][23]§

### impl<T> [DerefMut][28] for [Query][7]<T>

[Source][23]§

#### fn [deref_mut][29](&mut self) -> &mut Self::[Target][27]

Mutably dereferences the value.

[Source][30]§

### impl<T, S> [FromRequestParts][31]<S> for [Query][7]<T>

where T: [DeserializeOwned][8], S: [Send][32] \+ [Sync][33],

[Source][34]§

#### type [Rejection][35] = [QueryRejection][11]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][36]§

#### async fn [from_request_parts][37]( parts: &mut Parts, _state: [&S][38], ) -> [Result][10]<Self, Self::[Rejection][39]>

Perform the extraction.

[Source][12]§

### impl<T: [Copy][40]> [Copy][40] for [Query][7]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][41] for [Query][7]<T>

where T: [Freeze][41],

§

### impl<T> [RefUnwindSafe][42] for [Query][7]<T>

where T: [RefUnwindSafe][42],

§

### impl<T> [Send][32] for [Query][7]<T>

where T: [Send][32],

§

### impl<T> [Sync][33] for [Query][7]<T>

where T: [Sync][33],

§

### impl<T> [Unpin][43] for [Query][7]<T>

where T: [Unpin][43],

§

### impl<T> [UnwindSafe][44] for [Query][7]<T>

where T: [UnwindSafe][44],

## Blanket Implementations§

[Source][45]§

### impl<T> [Any][46] for T

where T: 'static + ?[Sized][47],

[Source][48]§

#### fn [type_id][49](&self) -> [TypeId][50]

Gets the `TypeId` of `self`. [Read more][49]

[Source][51]§

### impl<T> [Borrow][52]<T> for T

where T: ?[Sized][47],

[Source][53]§

#### fn [borrow][54](&self) -> [&T][38]

Immutably borrows from an owned value. [Read more][54]

[Source][55]§

### impl<T> [BorrowMut][56]<T> for T

where T: ?[Sized][47],

[Source][57]§

#### fn [borrow_mut][58](&mut self) -> [&mut T][38]

Mutably borrows from an owned value. [Read more][58]

[Source][59]§

### impl<T> [CloneToUninit][60] for T

where T: [Clone][13],

[Source][61]§

#### unsafe fn [clone_to_uninit][62](&self, dest: [*mut ][63][u8][64])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][62]

[Source][65]§

### impl<T> [From][66]<T> for T

[Source][67]§

#### fn [from][68](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][69]<T> for T

where T: [Clone][13],

§

#### fn [from_ref][70](input: [&T][38]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][71]<S, ViaParts> for T

where S: [Send][32] \+ [Sync][33], T: [FromRequestParts][31]<S>,

§

#### type [Rejection][72] = <T as [FromRequestParts][31]<S>>::[Rejection][39]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][73]( req: Request<[Body][74]>, state: [&S][38], ) -> impl [Future][75]<Output = [Result][10]<T, <T as [FromRequest][71]<S, ViaParts>>::[Rejection][76]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][77] [`Span`][78], returning an `Instrumented` wrapper. Read more

[Source][79]§

### impl<T, U> [Into][80]<U> for T

where U: [From][66]<T>,

[Source][81]§

#### fn [into][82](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][66]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][47],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][83]§

### impl<P, T> [Receiver][84] for P

where P: [Deref][24]<Target = T> \+ ?[Sized][47], T: ?[Sized][47],

[Source][85]§

#### type [Target][86] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][87]§

### impl<R> [Rng][88] for R

where R: [RngCore][89] \+ ?[Sized][47],

[Source][90]§

#### fn [random][91]<T>(&mut self) -> T

where [StandardUniform][92]: [Distribution][93]<T>,

Return a random value via the [`StandardUniform`][92] distribution. [Read more][91]

[Source][94]§

#### fn [random_iter][95]<T>(self) -> [Iter][96]<[StandardUniform][92], Self, T>

where Self: [Sized][47], [StandardUniform][92]: [Distribution][93]<T>,

Return an iterator over [`random`][97] variates [Read more][95]

[Source][98]§

#### fn [random_range][99]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][100], R: [SampleRange][101]<T>,

Generate a random value in the given range. [Read more][99]

[Source][102]§

#### fn [random_bool][103](&mut self, p: [f64][104]) -> [bool][105]

Return a bool with a probability `p` of being true. [Read more][103]

[Source][106]§

#### fn [random_ratio][107](&mut self, numerator: [u32][108], denominator: [u32][108]) -> [bool][105]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][107]

[Source][109]§

#### fn [sample][110]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][93]<T>,

Sample a new value, using the given distribution. [Read more][110]

[Source][111]§

#### fn [sample_iter][112]<T, D>(self, distr: D) -> [Iter][96]<D, Self, T>

where D: [Distribution][93]<T>, Self: [Sized][47],

Create an iterator that generates values using the given distribution. [Read more][112]

[Source][113]§

#### fn [fill][114]<T>(&mut self, dest: [&mut T][38])

where T: [Fill][115] \+ ?[Sized][47],

Fill any type implementing [`Fill`][115] with random data [Read more][114]

[Source][116]§

#### fn [gen][117]<T>(&mut self) -> T

where [StandardUniform][92]: [Distribution][93]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][97].

[Source][118]§

#### fn [gen_range][119]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][100], R: [SampleRange][101]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][120].

[Source][121]§

#### fn [gen_bool][122](&mut self, p: [f64][104]) -> [bool][105]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][123].

[Source][124]§

#### fn [gen_ratio][125](&mut self, numerator: [u32][108], denominator: [u32][108]) -> [bool][105]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][126].

[Source][127]§

### impl<T> [RngCore][89] for T

where T: [DerefMut][28], <T as [Deref][24]>::[Target][27]: [RngCore][89],

[Source][128]§

#### fn [next_u32][129](&mut self) -> [u32][108]

Return the next random `u32`. [Read more][129]

[Source][130]§

#### fn [next_u64][131](&mut self) -> [u64][132]

Return the next random `u64`. [Read more][131]

[Source][133]§

#### fn [fill_bytes][134](&mut self, dst: &mut [[u8][64]])

Fill `dest` with random data. [Read more][134]

[Source][135]§

### impl<T> [Same][136] for T

[Source][137]§

#### type [Output][138] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][47],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][47],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][139]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][47],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][47],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][47],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][47],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][47],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][47],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][47],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][140] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][141]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][142] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][141]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][142] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][141]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][47],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][142] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][47],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][47], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][47], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][47],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][47],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][47],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][143]) -> RequestBodyLimit<Self>

where Self: [Sized][47],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][47],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][47],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][144]§

### impl<T> [ToOwned][145] for T

where T: [Clone][13],

[Source][146]§

#### type [Owned][147] = T

The resulting type after obtaining ownership.

[Source][148]§

#### fn [to_owned][149](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][149]

[Source][150]§

#### fn [clone_into][151](&self, target: [&mut T][38])

Uses borrowed data to replace owned data, usually by cloning. [Read more][151]

[Source][152]§

### impl<T, U> [TryFrom][153]<U> for T

where U: [Into][80]<T>,

[Source][154]§

#### type [Error][155] = [Infallible][156]

The type returned in the event of a conversion error.

[Source][157]§

#### fn [try_from][158](value: U) -> [Result][10]<T, <T as [TryFrom][153]<U>>::[Error][159]>

Performs the conversion.

[Source][160]§

### impl<T, U> [TryInto][161]<U> for T

where U: [TryFrom][153]<T>,

[Source][162]§

#### type [Error][163] = <U as [TryFrom][153]<T>>::[Error][159]

The type returned in the event of a conversion error.

[Source][164]§

#### fn [try_into][165](self) -> [Result][10]<U, <U as [TryFrom][153]<T>>::[Error][159]>

Performs the conversion.

[Source][166]§

### impl<R> [TryRngCore][167] for R

where R: [RngCore][89] \+ ?[Sized][47],

[Source][168]§

#### type [Error][169] = [Infallible][156]

The type returned in the event of a RNG error.

[Source][170]§

#### fn [try_next_u32][171](&mut self) -> [Result][10]<[u32][108], <R as [TryRngCore][167]>::[Error][172]>

Return the next random `u32`.

[Source][173]§

#### fn [try_next_u64][174](&mut self) -> [Result][10]<[u64][132], <R as [TryRngCore][167]>::[Error][172]>

Return the next random `u64`.

[Source][175]§

#### fn [try_fill_bytes][176]( &mut self, dst: &mut [[u8][64]], ) -> [Result][10]<[()][177], <R as [TryRngCore][167]>::[Error][172]>

Fill `dest` entirely with random data.

[Source][178]§

#### fn [unwrap_err][179](self) -> [UnwrapErr][180]<Self>

where Self: [Sized][47],

Wrap RNG with the [`UnwrapErr`][180] wrapper.

[Source][181]§

#### fn [unwrap_mut][182](&mut self) -> [UnwrapMut][183]<'_, Self>

Wrap RNG with the [`UnwrapMut`][183] wrapper.

[Source][184]§

#### fn [read_adapter][185](&mut self) -> [RngReadAdapter][186]<'_, Self>

where Self: [Sized][47],

Available on **crate feature`std`** only.

Convert an [`RngCore`][89] to a [`RngReadAdapter`][186].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][80]<Dispatch>,

Attaches the provided [`Subscriber`][187] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][188] [`Subscriber`][187] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][189]§

### impl<T> [CryptoRng][190] for T

where T: [DerefMut][28], <T as [Deref][24]>::[Target][27]: [CryptoRng][190],

[Source][191]§

### impl<R> [TryCryptoRng][192] for R

where R: [CryptoRng][190] \+ ?[Sized][47],

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/query.rs.html#41
   [5]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html (trait serde_core::de::Deserialize)
   [6]: ../../src/axum/extract/query.rs.html#55-86
   [7]: struct.Query.html (struct axum::extract::Query)
   [8]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [9]: ../../src/axum/extract/query.rs.html#78-85
   [10]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [11]: rejection/enum.QueryRejection.html (enum axum::extract::rejection::QueryRejection)
   [12]: ../../src/axum/extract/query.rs.html#40
   [13]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [15]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [16]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [17]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [19]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [20]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [21]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [22]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [23]: ../../src/axum/extract/query.rs.html#88
   [24]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [25]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [26]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [27]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [28]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [29]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [30]: ../../src/axum/extract/query.rs.html#43-53
   [31]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [34]: ../../src/axum/extract/query.rs.html#48
   [35]: trait.FromRequestParts.html#associatedtype.Rejection
   [36]: ../../src/axum/extract/query.rs.html#50-52
   [37]: trait.FromRequestParts.html#tymethod.from_request_parts
   [38]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [39]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [42]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [44]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [45]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [46]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [47]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [48]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [49]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [50]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [57]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [58]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [59]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [60]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [61]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [62]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [63]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [64]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [67]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [68]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [69]: trait.FromRef.html (trait axum::extract::FromRef)
   [70]: trait.FromRef.html#tymethod.from_ref
   [71]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [72]: trait.FromRequest.html#associatedtype.Rejection
   [73]: trait.FromRequest.html#tymethod.from_request
   [74]: ../body/struct.Body.html (struct axum::body::Body)
   [75]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [76]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [77]: super::Span::current()
   [78]: crate::Span
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [83]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [84]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [85]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [86]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [87]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [88]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [89]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [90]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [91]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [92]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [93]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [94]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [95]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [96]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [97]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [98]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [99]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [100]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [101]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [102]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [103]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [104]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [105]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [106]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [107]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [108]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [109]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [110]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [111]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [112]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [113]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [114]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [115]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [116]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [117]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [118]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [119]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [120]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [121]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [122]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [123]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [124]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [125]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [126]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [127]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [128]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [129]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [130]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [131]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [132]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [133]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [134]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [135]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [136]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [137]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [138]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [139]: https://docs.rs/http/latest/http/struct.Extensions.html
   [140]: crate::follow_redirect::policy::Standard
   [141]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [142]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [143]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [144]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [145]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [146]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [147]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [148]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [149]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [150]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [151]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [152]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [153]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [154]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [155]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [156]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [157]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [158]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [159]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [160]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [161]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [162]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [163]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [164]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [165]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [166]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [167]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [168]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [169]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [170]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [171]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [172]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [173]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [174]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [175]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [176]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [177]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [178]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [179]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [180]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [181]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [182]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [183]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [184]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [185]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [186]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [187]: super::Subscriber
   [188]: dispatcher#setting-the-default-subscriber
   [189]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [190]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [191]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [192]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

