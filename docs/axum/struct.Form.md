<!-- Generated from rustdoc HTML: struct.Form.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Form

## [axum][1]0.8.8

## Form

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
  * FromRequest<S>
  * IntoResponse



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

# Struct Form Copy item path

[Source][3]
``` 
pub struct Form<T>(pub T);
```

Available on **crate feature`form`** only.

Expand description

URL encoded extractor and response.

## §As extractor

If used as an extractor, `Form` will deserialize form data from the request, specifically:

  * If the request has a method of `GET` or `HEAD`, the form data will be read from the query string (same as with [`Query`][4])
  * If the request has a different method, the form will be read from the body of the request. It must have a `content-type` of `application/x-www-form-urlencoded` for this to work. If you want to parse `multipart/form-data` request bodies, use [`Multipart`][5] instead.



This matches how HTML forms are sent by browsers by default. In both cases, the inner type `T` must implement [`serde::Deserialize`][6].

⚠️ Since parsing form data might require consuming the request body, the `Form` extractor must be _last_ if there are multiple extractors in a handler. See [“the order of extractors”][7]
``` 
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
struct SignUp {
    username: String,
    password: String,
}

async fn accept_form(Form(sign_up): Form<SignUp>) {
    // ...
}
```

## §As response

`Form` can also be used to encode any type that implements [`serde::Serialize`][8] as `application/x-www-form-urlencoded`
``` 
use axum::Form;
use serde::Serialize;

#[derive(Serialize)]
struct Payload {
    value: String,
}

async fn handler() -> Form<Payload> {
    Form(Payload { value: "foo".to_owned() })
}
```

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][9]§

### impl<T: [Clone][10]> [Clone][10] for [Form][11]<T>

[Source][9]§

#### fn [clone][12](&self) -> [Form][11]<T>

Returns a duplicate of the value. [Read more][12]

1.0.0 · [Source][13]§

#### fn [clone_from][14](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][14]

[Source][9]§

### impl<T: [Debug][15]> [Debug][15] for [Form][11]<T>

[Source][9]§

#### fn [fmt][16](&self, f: &mut [Formatter][17]<'_>) -> [Result][18]

Formats the value using the given formatter. [Read more][16]

[Source][9]§

### impl<T: [Default][19]> [Default][19] for [Form][11]<T>

[Source][9]§

#### fn [default][20]() -> [Form][11]<T>

Returns the “default value” for a type. [Read more][20]

[Source][21]§

### impl<T> [Deref][22] for [Form][11]<T>

[Source][21]§

#### type [Target][23] = T

The resulting type after dereferencing.

[Source][21]§

#### fn [deref][24](&self) -> &Self::[Target][25]

Dereferences the value.

[Source][21]§

### impl<T> [DerefMut][26] for [Form][11]<T>

[Source][21]§

#### fn [deref_mut][27](&mut self) -> &mut Self::[Target][25]

Mutably dereferences the value.

[Source][28]§

### impl<T, S> [FromRequest][29]<S> for [Form][11]<T>

where T: [DeserializeOwned][30], S: [Send][31] \+ [Sync][32],

[Source][33]§

#### type [Rejection][34] = [FormRejection][35]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][36]§

#### async fn [from_request][37](req: [Request][38], _state: [&S][39]) -> [Result][40]<Self, Self::[Rejection][41]>

Perform the extraction.

[Source][42]§

### impl<T> [IntoResponse][43] for [Form][11]<T>

where T: [Serialize][8],

[Source][44]§

#### fn [into_response][45](self) -> [Response][46]

Create a response.

[Source][9]§

### impl<T: [Copy][47]> [Copy][47] for [Form][11]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][48] for [Form][11]<T>

where T: [Freeze][48],

§

### impl<T> [RefUnwindSafe][49] for [Form][11]<T>

where T: [RefUnwindSafe][49],

§

### impl<T> [Send][31] for [Form][11]<T>

where T: [Send][31],

§

### impl<T> [Sync][32] for [Form][11]<T>

where T: [Sync][32],

§

### impl<T> [Unpin][50] for [Form][11]<T>

where T: [Unpin][50],

§

### impl<T> [UnwindSafe][51] for [Form][11]<T>

where T: [UnwindSafe][51],

## Blanket Implementations§

[Source][52]§

### impl<T> [Any][53] for T

where T: 'static + ?[Sized][54],

[Source][55]§

#### fn [type_id][56](&self) -> [TypeId][57]

Gets the `TypeId` of `self`. [Read more][56]

[Source][58]§

### impl<T> [Borrow][59]<T> for T

where T: ?[Sized][54],

[Source][60]§

#### fn [borrow][61](&self) -> [&T][39]

Immutably borrows from an owned value. [Read more][61]

[Source][62]§

### impl<T> [BorrowMut][63]<T> for T

where T: ?[Sized][54],

[Source][64]§

#### fn [borrow_mut][65](&mut self) -> [&mut T][39]

Mutably borrows from an owned value. [Read more][65]

[Source][66]§

### impl<T> [CloneToUninit][67] for T

where T: [Clone][10],

[Source][68]§

#### unsafe fn [clone_to_uninit][69](&self, dest: [*mut ][70][u8][71])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][69]

[Source][72]§

### impl<T> [From][73]<T> for T

[Source][74]§

#### fn [from][75](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][76]<T> for T

where T: [Clone][10],

§

#### fn [from_ref][77](input: [&T][39]) -> T

Converts to this type from a reference to the input type.

[Source][78]§

### impl<H, T> [HandlerWithoutStateExt][79]<T> for H

where H: [Handler][80]<T, [()][81]>,

[Source][82]§

#### fn [into_service][83](self) -> [HandlerService][84]<H, T, [()][81]>

Convert the handler into a [`Service`] and no state.

[Source][85]§

#### fn [into_make_service][86](self) -> [IntoMakeService][87]<[HandlerService][84]<H, T, [()][81]>>

Convert the handler into a [`MakeService`][88] and no state. [Read more][86]

[Source][89]§

#### fn [into_make_service_with_connect_info][90]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][91]<[HandlerService][84]<H, T, [()][81]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][88] which stores information about the incoming connection and has no state. [Read more][90]

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][92] [`Span`][93], returning an `Instrumented` wrapper. Read more

[Source][94]§

### impl<T, U> [Into][95]<U> for T

where U: [From][73]<T>,

[Source][96]§

#### fn [into][97](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][73]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][54],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][98]§

### impl<P, T> [Receiver][99] for P

where P: [Deref][22]<Target = T> \+ ?[Sized][54], T: ?[Sized][54],

[Source][100]§

#### type [Target][101] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][102]§

### impl<R> [Rng][103] for R

where R: [RngCore][104] \+ ?[Sized][54],

[Source][105]§

#### fn [random][106]<T>(&mut self) -> T

where [StandardUniform][107]: [Distribution][108]<T>,

Return a random value via the [`StandardUniform`][107] distribution. [Read more][106]

[Source][109]§

#### fn [random_iter][110]<T>(self) -> [Iter][111]<[StandardUniform][107], Self, T>

where Self: [Sized][54], [StandardUniform][107]: [Distribution][108]<T>,

Return an iterator over [`random`][112] variates [Read more][110]

[Source][113]§

#### fn [random_range][114]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][115], R: [SampleRange][116]<T>,

Generate a random value in the given range. [Read more][114]

[Source][117]§

#### fn [random_bool][118](&mut self, p: [f64][119]) -> [bool][120]

Return a bool with a probability `p` of being true. [Read more][118]

[Source][121]§

#### fn [random_ratio][122](&mut self, numerator: [u32][123], denominator: [u32][123]) -> [bool][120]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][122]

[Source][124]§

#### fn [sample][125]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][108]<T>,

Sample a new value, using the given distribution. [Read more][125]

[Source][126]§

#### fn [sample_iter][127]<T, D>(self, distr: D) -> [Iter][111]<D, Self, T>

where D: [Distribution][108]<T>, Self: [Sized][54],

Create an iterator that generates values using the given distribution. [Read more][127]

[Source][128]§

#### fn [fill][129]<T>(&mut self, dest: [&mut T][39])

where T: [Fill][130] \+ ?[Sized][54],

Fill any type implementing [`Fill`][130] with random data [Read more][129]

[Source][131]§

#### fn [gen][132]<T>(&mut self) -> T

where [StandardUniform][107]: [Distribution][108]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][112].

[Source][133]§

#### fn [gen_range][134]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][115], R: [SampleRange][116]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][135].

[Source][136]§

#### fn [gen_bool][137](&mut self, p: [f64][119]) -> [bool][120]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][138].

[Source][139]§

#### fn [gen_ratio][140](&mut self, numerator: [u32][123], denominator: [u32][123]) -> [bool][120]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][141].

[Source][142]§

### impl<T> [RngCore][104] for T

where T: [DerefMut][26], <T as [Deref][22]>::[Target][25]: [RngCore][104],

[Source][143]§

#### fn [next_u32][144](&mut self) -> [u32][123]

Return the next random `u32`. [Read more][144]

[Source][145]§

#### fn [next_u64][146](&mut self) -> [u64][147]

Return the next random `u64`. [Read more][146]

[Source][148]§

#### fn [fill_bytes][149](&mut self, dst: &mut [[u8][71]])

Fill `dest` with random data. [Read more][149]

[Source][150]§

### impl<T> [Same][151] for T

[Source][152]§

#### type [Output][153] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][54],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][54],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][154]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][54],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][54],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][54],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][54],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][54],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][54],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][54],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][155] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][156]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][54],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][157] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][156]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][54],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][157] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][156]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][54],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][157] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][54],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][54], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][54], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][54],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][54],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][54],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][158]) -> RequestBodyLimit<Self>

where Self: [Sized][54],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][54],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][54],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][159]§

### impl<T> [ToOwned][160] for T

where T: [Clone][10],

[Source][161]§

#### type [Owned][162] = T

The resulting type after obtaining ownership.

[Source][163]§

#### fn [to_owned][164](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][164]

[Source][165]§

#### fn [clone_into][166](&self, target: [&mut T][39])

Uses borrowed data to replace owned data, usually by cloning. [Read more][166]

[Source][167]§

### impl<T, U> [TryFrom][168]<U> for T

where U: [Into][95]<T>,

[Source][169]§

#### type [Error][170] = [Infallible][171]

The type returned in the event of a conversion error.

[Source][172]§

#### fn [try_from][173](value: U) -> [Result][40]<T, <T as [TryFrom][168]<U>>::[Error][174]>

Performs the conversion.

[Source][175]§

### impl<T, U> [TryInto][176]<U> for T

where U: [TryFrom][168]<T>,

[Source][177]§

#### type [Error][178] = <U as [TryFrom][168]<T>>::[Error][174]

The type returned in the event of a conversion error.

[Source][179]§

#### fn [try_into][180](self) -> [Result][40]<U, <U as [TryFrom][168]<T>>::[Error][174]>

Performs the conversion.

[Source][181]§

### impl<R> [TryRngCore][182] for R

where R: [RngCore][104] \+ ?[Sized][54],

[Source][183]§

#### type [Error][184] = [Infallible][171]

The type returned in the event of a RNG error.

[Source][185]§

#### fn [try_next_u32][186](&mut self) -> [Result][40]<[u32][123], <R as [TryRngCore][182]>::[Error][187]>

Return the next random `u32`.

[Source][188]§

#### fn [try_next_u64][189](&mut self) -> [Result][40]<[u64][147], <R as [TryRngCore][182]>::[Error][187]>

Return the next random `u64`.

[Source][190]§

#### fn [try_fill_bytes][191]( &mut self, dst: &mut [[u8][71]], ) -> [Result][40]<[()][81], <R as [TryRngCore][182]>::[Error][187]>

Fill `dest` entirely with random data.

[Source][192]§

#### fn [unwrap_err][193](self) -> [UnwrapErr][194]<Self>

where Self: [Sized][54],

Wrap RNG with the [`UnwrapErr`][194] wrapper.

[Source][195]§

#### fn [unwrap_mut][196](&mut self) -> [UnwrapMut][197]<'_, Self>

Wrap RNG with the [`UnwrapMut`][197] wrapper.

[Source][198]§

#### fn [read_adapter][199](&mut self) -> [RngReadAdapter][200]<'_, Self>

where Self: [Sized][54],

Available on **crate feature`std`** only.

Convert an [`RngCore`][104] to a [`RngReadAdapter`][200].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][95]<Dispatch>,

Attaches the provided [`Subscriber`][201] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][202] [`Subscriber`][201] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][203]§

### impl<T> [CryptoRng][204] for T

where T: [DerefMut][26], <T as [Deref][22]>::[Target][25]: [CryptoRng][204],

[Source][205]§

### impl<R> [TryCryptoRng][206] for R

where R: [CryptoRng][204] \+ ?[Sized][54],

   [1]: ../axum/index.html
   [2]: index.html
   [3]: ../src/axum/form.rs.html#71
   [4]: extract/struct.Query.html (struct axum::extract::Query)
   [5]: extract/struct.Multipart.html (struct axum::extract::Multipart)
   [6]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html (trait serde_core::de::Deserialize)
   [7]: extract/index.html#the-order-of-extractors (mod axum::extract)
   [8]: https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html (trait serde_core::ser::Serialize)
   [9]: ../src/axum/form.rs.html#69
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [11]: struct.Form.html (struct axum::Form)
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [13]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [17]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [19]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [20]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [21]: ../src/axum/form.rs.html#132
   [22]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [23]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [24]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [25]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [26]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [27]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [28]: ../src/axum/form.rs.html#73-105
   [29]: extract/trait.FromRequest.html (trait axum::extract::FromRequest)
   [30]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [33]: ../src/axum/form.rs.html#78
   [34]: extract/trait.FromRequest.html#associatedtype.Rejection
   [35]: extract/rejection/enum.FormRejection.html (enum axum::extract::rejection::FormRejection)
   [36]: ../src/axum/form.rs.html#80-104
   [37]: extract/trait.FromRequest.html#tymethod.from_request
   [38]: extract/type.Request.html (type axum::extract::Request)
   [39]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [40]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [41]: extract/trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [42]: ../src/axum/form.rs.html#107-131
   [43]: response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [44]: ../src/axum/form.rs.html#111-130
   [45]: response/trait.IntoResponse.html#tymethod.into_response
   [46]: response/type.Response.html (type axum::response::Response)
   [47]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [48]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [49]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [50]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [51]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [52]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [53]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [54]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [55]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [56]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [57]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [58]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [59]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [60]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [61]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [62]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [63]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [64]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [65]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [66]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [67]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [68]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [69]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [70]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [71]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [72]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [73]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [74]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [75]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [76]: extract/trait.FromRef.html (trait axum::extract::FromRef)
   [77]: extract/trait.FromRef.html#tymethod.from_ref
   [78]: ../src/axum/handler/mod.rs.html#380-398
   [79]: handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [80]: handler/trait.Handler.html (trait axum::handler::Handler)
   [81]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [82]: ../src/axum/handler/mod.rs.html#384-386
   [83]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [84]: handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [85]: ../src/axum/handler/mod.rs.html#388-390
   [86]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [87]: routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [88]: tower::make::MakeService
   [89]: ../src/axum/handler/mod.rs.html#393-397
   [90]: handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [91]: extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [92]: super::Span::current()
   [93]: crate::Span
   [94]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [95]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [96]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [97]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [98]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [99]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [100]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [101]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [102]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [103]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [104]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [105]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [106]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [107]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [108]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [109]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [110]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [111]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [112]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [113]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [114]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [115]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [116]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [117]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [118]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [119]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [120]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [121]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [122]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [123]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [124]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [125]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [126]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [127]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [128]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [129]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [130]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [131]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [132]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [133]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [134]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [135]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [136]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [137]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [138]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [139]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [140]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [141]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [142]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [143]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [144]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [145]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [146]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [147]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [148]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [149]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [150]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [151]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [152]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [153]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [154]: https://docs.rs/http/latest/http/struct.Extensions.html
   [155]: crate::follow_redirect::policy::Standard
   [156]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [157]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [158]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [159]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [160]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [161]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [162]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [163]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [164]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [165]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [166]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [167]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [168]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [169]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [170]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [171]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [172]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [173]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [174]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [175]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [176]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [177]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [178]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [179]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [180]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [181]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [182]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [183]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [184]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [185]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [186]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [187]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [188]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [189]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [190]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [191]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [192]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [193]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [194]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [195]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [196]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [197]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [198]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [199]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [200]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [201]: super::Subscriber
   [202]: dispatcher#setting-the-default-subscriber
   [203]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [204]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [205]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [206]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

