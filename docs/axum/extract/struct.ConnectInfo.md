<!-- Generated from rustdoc HTML: extract/struct.ConnectInfo.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ConnectInfo

## [axum][1]0.8.8

## ConnectInfo

### Tuple Fields

  * 0



### Trait Implementations

  * Clone
  * Copy
  * Debug
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

# Struct ConnectInfo Copy item path

[Source][4]
``` 
pub struct ConnectInfo<T>(pub T);
```

Available on **crate feature`tokio`** only.

Expand description

Extractor for getting connection information produced by a [`Connected`][5].

Note this extractor requires you to use [`Router::into_make_service_with_connect_info`][6] to run your app otherwise it will fail at runtime.

See [`Router::into_make_service_with_connect_info`][6] for more details.

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][7]§

### impl<T: [Clone][8]> [Clone][8] for [ConnectInfo][9]<T>

[Source][7]§

#### fn [clone][10](&self) -> [ConnectInfo][9]<T>

Returns a duplicate of the value. [Read more][10]

1.0.0 · [Source][11]§

#### fn [clone_from][12](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][12]

[Source][7]§

### impl<T: [Debug][13]> [Debug][13] for [ConnectInfo][9]<T>

[Source][7]§

#### fn [fmt][14](&self, f: &mut [Formatter][15]<'_>) -> [Result][16]

Formats the value using the given formatter. [Read more][14]

[Source][17]§

### impl<T> [Deref][18] for [ConnectInfo][9]<T>

[Source][17]§

#### type [Target][19] = T

The resulting type after dereferencing.

[Source][17]§

#### fn [deref][20](&self) -> &Self::[Target][21]

Dereferences the value.

[Source][17]§

### impl<T> [DerefMut][22] for [ConnectInfo][9]<T>

[Source][17]§

#### fn [deref_mut][23](&mut self) -> &mut Self::[Target][21]

Mutably dereferences the value.

[Source][24]§

### impl<S, T> [FromRequestParts][25]<S> for [ConnectInfo][9]<T>

where S: [Send][26] \+ [Sync][27], T: [Clone][8] \+ [Send][26] \+ [Sync][27] \+ 'static,

[Source][28]§

#### type [Rejection][29] = <[Extension][30]<[ConnectInfo][9]<T>> as [FromRequestParts][25]<S>>::[Rejection][31]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][32]§

#### async fn [from_request_parts][33]( parts: &mut Parts, state: [&S][34], ) -> [Result][35]<Self, Self::[Rejection][31]>

Perform the extraction.

[Source][7]§

### impl<T: [Copy][36]> [Copy][36] for [ConnectInfo][9]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][37] for [ConnectInfo][9]<T>

where T: [Freeze][37],

§

### impl<T> [RefUnwindSafe][38] for [ConnectInfo][9]<T>

where T: [RefUnwindSafe][38],

§

### impl<T> [Send][26] for [ConnectInfo][9]<T>

where T: [Send][26],

§

### impl<T> [Sync][27] for [ConnectInfo][9]<T>

where T: [Sync][27],

§

### impl<T> [Unpin][39] for [ConnectInfo][9]<T>

where T: [Unpin][39],

§

### impl<T> [UnwindSafe][40] for [ConnectInfo][9]<T>

where T: [UnwindSafe][40],

## Blanket Implementations§

[Source][41]§

### impl<T> [Any][42] for T

where T: 'static + ?[Sized][43],

[Source][44]§

#### fn [type_id][45](&self) -> [TypeId][46]

Gets the `TypeId` of `self`. [Read more][45]

[Source][47]§

### impl<T> [Borrow][48]<T> for T

where T: ?[Sized][43],

[Source][49]§

#### fn [borrow][50](&self) -> [&T][34]

Immutably borrows from an owned value. [Read more][50]

[Source][51]§

### impl<T> [BorrowMut][52]<T> for T

where T: ?[Sized][43],

[Source][53]§

#### fn [borrow_mut][54](&mut self) -> [&mut T][34]

Mutably borrows from an owned value. [Read more][54]

[Source][55]§

### impl<T> [CloneToUninit][56] for T

where T: [Clone][8],

[Source][57]§

#### unsafe fn [clone_to_uninit][58](&self, dest: [*mut ][59][u8][60])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][58]

[Source][61]§

### impl<T> [From][62]<T> for T

[Source][63]§

#### fn [from][64](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][65]<T> for T

where T: [Clone][8],

§

#### fn [from_ref][66](input: [&T][34]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][67]<S, ViaParts> for T

where S: [Send][26] \+ [Sync][27], T: [FromRequestParts][25]<S>,

§

#### type [Rejection][68] = <T as [FromRequestParts][25]<S>>::[Rejection][31]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][69]( req: Request<[Body][70]>, state: [&S][34], ) -> impl [Future][71]<Output = [Result][35]<T, <T as [FromRequest][67]<S, ViaParts>>::[Rejection][72]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][73] [`Span`][74], returning an `Instrumented` wrapper. Read more

[Source][75]§

### impl<T, U> [Into][76]<U> for T

where U: [From][62]<T>,

[Source][77]§

#### fn [into][78](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][62]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][43],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][79]§

### impl<P, T> [Receiver][80] for P

where P: [Deref][18]<Target = T> \+ ?[Sized][43], T: ?[Sized][43],

[Source][81]§

#### type [Target][82] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][83]§

### impl<R> [Rng][84] for R

where R: [RngCore][85] \+ ?[Sized][43],

[Source][86]§

#### fn [random][87]<T>(&mut self) -> T

where [StandardUniform][88]: [Distribution][89]<T>,

Return a random value via the [`StandardUniform`][88] distribution. [Read more][87]

[Source][90]§

#### fn [random_iter][91]<T>(self) -> [Iter][92]<[StandardUniform][88], Self, T>

where Self: [Sized][43], [StandardUniform][88]: [Distribution][89]<T>,

Return an iterator over [`random`][93] variates [Read more][91]

[Source][94]§

#### fn [random_range][95]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][96], R: [SampleRange][97]<T>,

Generate a random value in the given range. [Read more][95]

[Source][98]§

#### fn [random_bool][99](&mut self, p: [f64][100]) -> [bool][101]

Return a bool with a probability `p` of being true. [Read more][99]

[Source][102]§

#### fn [random_ratio][103](&mut self, numerator: [u32][104], denominator: [u32][104]) -> [bool][101]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][103]

[Source][105]§

#### fn [sample][106]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][89]<T>,

Sample a new value, using the given distribution. [Read more][106]

[Source][107]§

#### fn [sample_iter][108]<T, D>(self, distr: D) -> [Iter][92]<D, Self, T>

where D: [Distribution][89]<T>, Self: [Sized][43],

Create an iterator that generates values using the given distribution. [Read more][108]

[Source][109]§

#### fn [fill][110]<T>(&mut self, dest: [&mut T][34])

where T: [Fill][111] \+ ?[Sized][43],

Fill any type implementing [`Fill`][111] with random data [Read more][110]

[Source][112]§

#### fn [gen][113]<T>(&mut self) -> T

where [StandardUniform][88]: [Distribution][89]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][93].

[Source][114]§

#### fn [gen_range][115]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][96], R: [SampleRange][97]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][116].

[Source][117]§

#### fn [gen_bool][118](&mut self, p: [f64][100]) -> [bool][101]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][119].

[Source][120]§

#### fn [gen_ratio][121](&mut self, numerator: [u32][104], denominator: [u32][104]) -> [bool][101]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][122].

[Source][123]§

### impl<T> [RngCore][85] for T

where T: [DerefMut][22], <T as [Deref][18]>::[Target][21]: [RngCore][85],

[Source][124]§

#### fn [next_u32][125](&mut self) -> [u32][104]

Return the next random `u32`. [Read more][125]

[Source][126]§

#### fn [next_u64][127](&mut self) -> [u64][128]

Return the next random `u64`. [Read more][127]

[Source][129]§

#### fn [fill_bytes][130](&mut self, dst: &mut [[u8][60]])

Fill `dest` with random data. [Read more][130]

[Source][131]§

### impl<T> [Same][132] for T

[Source][133]§

#### type [Output][134] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][43],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][43],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][135]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][43],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][43],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][43],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][43],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][43],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][136] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][137]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][138] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][137]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][138] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][137]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][43],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][138] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][43],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][43], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][43],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][43],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][139]) -> RequestBodyLimit<Self>

where Self: [Sized][43],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][43],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][140]§

### impl<T> [ToOwned][141] for T

where T: [Clone][8],

[Source][142]§

#### type [Owned][143] = T

The resulting type after obtaining ownership.

[Source][144]§

#### fn [to_owned][145](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][145]

[Source][146]§

#### fn [clone_into][147](&self, target: [&mut T][34])

Uses borrowed data to replace owned data, usually by cloning. [Read more][147]

[Source][148]§

### impl<T, U> [TryFrom][149]<U> for T

where U: [Into][76]<T>,

[Source][150]§

#### type [Error][151] = [Infallible][152]

The type returned in the event of a conversion error.

[Source][153]§

#### fn [try_from][154](value: U) -> [Result][35]<T, <T as [TryFrom][149]<U>>::[Error][155]>

Performs the conversion.

[Source][156]§

### impl<T, U> [TryInto][157]<U> for T

where U: [TryFrom][149]<T>,

[Source][158]§

#### type [Error][159] = <U as [TryFrom][149]<T>>::[Error][155]

The type returned in the event of a conversion error.

[Source][160]§

#### fn [try_into][161](self) -> [Result][35]<U, <U as [TryFrom][149]<T>>::[Error][155]>

Performs the conversion.

[Source][162]§

### impl<R> [TryRngCore][163] for R

where R: [RngCore][85] \+ ?[Sized][43],

[Source][164]§

#### type [Error][165] = [Infallible][152]

The type returned in the event of a RNG error.

[Source][166]§

#### fn [try_next_u32][167](&mut self) -> [Result][35]<[u32][104], <R as [TryRngCore][163]>::[Error][168]>

Return the next random `u32`.

[Source][169]§

#### fn [try_next_u64][170](&mut self) -> [Result][35]<[u64][128], <R as [TryRngCore][163]>::[Error][168]>

Return the next random `u64`.

[Source][171]§

#### fn [try_fill_bytes][172]( &mut self, dst: &mut [[u8][60]], ) -> [Result][35]<[()][173], <R as [TryRngCore][163]>::[Error][168]>

Fill `dest` entirely with random data.

[Source][174]§

#### fn [unwrap_err][175](self) -> [UnwrapErr][176]<Self>

where Self: [Sized][43],

Wrap RNG with the [`UnwrapErr`][176] wrapper.

[Source][177]§

#### fn [unwrap_mut][178](&mut self) -> [UnwrapMut][179]<'_, Self>

Wrap RNG with the [`UnwrapMut`][179] wrapper.

[Source][180]§

#### fn [read_adapter][181](&mut self) -> [RngReadAdapter][182]<'_, Self>

where Self: [Sized][43],

Available on **crate feature`std`** only.

Convert an [`RngCore`][85] to a [`RngReadAdapter`][182].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][76]<Dispatch>,

Attaches the provided [`Subscriber`][183] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][184] [`Subscriber`][183] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][185]§

### impl<T> [CryptoRng][186] for T

where T: [DerefMut][22], <T as [Deref][18]>::[Target][21]: [CryptoRng][186],

[Source][187]§

### impl<R> [TryCryptoRng][188] for R

where R: [CryptoRng][186] \+ ?[Sized][43],

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/connect_info.rs.html#136
   [5]: connect_info/trait.Connected.html (trait axum::extract::connect_info::Connected)
   [6]: ../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [7]: ../../src/axum/extract/connect_info.rs.html#135
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [9]: struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [11]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [15]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [16]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [17]: ../../src/axum/extract/connect_info.rs.html#156
   [18]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [19]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [20]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [21]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [22]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [23]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [24]: ../../src/axum/extract/connect_info.rs.html#138-154
   [25]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [28]: ../../src/axum/extract/connect_info.rs.html#143
   [29]: trait.FromRequestParts.html#associatedtype.Rejection
   [30]: ../struct.Extension.html (struct axum::Extension)
   [31]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [32]: ../../src/axum/extract/connect_info.rs.html#145-153
   [33]: trait.FromRequestParts.html#tymethod.from_request_parts
   [34]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [35]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [36]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [38]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [41]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [42]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [43]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [44]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [45]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [46]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [47]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [48]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [55]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [56]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [57]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [58]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [59]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [60]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [61]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [62]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [65]: trait.FromRef.html (trait axum::extract::FromRef)
   [66]: trait.FromRef.html#tymethod.from_ref
   [67]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [68]: trait.FromRequest.html#associatedtype.Rejection
   [69]: trait.FromRequest.html#tymethod.from_request
   [70]: ../body/struct.Body.html (struct axum::body::Body)
   [71]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [72]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [73]: super::Span::current()
   [74]: crate::Span
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [79]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [80]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [81]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [82]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [83]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [84]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [85]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [86]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [87]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [88]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [89]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [90]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [91]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [92]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [93]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [94]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [95]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [96]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [97]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [98]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [99]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [100]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [101]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [102]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [103]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [104]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [105]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [106]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [107]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [108]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [109]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [110]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [111]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [112]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [113]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [114]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [115]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [116]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [117]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [118]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [119]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [120]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [121]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [122]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [123]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [124]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [125]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [126]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [127]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [128]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [129]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [130]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [131]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [132]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [133]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [134]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [135]: https://docs.rs/http/latest/http/struct.Extensions.html
   [136]: crate::follow_redirect::policy::Standard
   [137]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [138]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [139]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [140]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [141]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [142]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [143]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [144]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [145]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [146]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [147]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [148]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [149]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [150]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [151]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [152]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [153]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [154]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [155]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [156]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [157]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [158]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [159]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [160]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [161]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [162]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [163]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [164]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [165]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [166]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [167]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [168]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [169]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [170]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [171]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [172]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [173]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [174]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [175]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [176]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [177]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [178]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [179]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [180]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [181]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [182]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [183]: super::Subscriber
   [184]: dispatcher#setting-the-default-subscriber
   [185]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [186]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [187]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [188]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

