<!-- Generated from rustdoc HTML: extract/connect_info/struct.ConnectInfo.html -->
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



## [In axum::extract::connect_info][2]

[axum][3]::[extract][4]::[connect_info][2]

# Struct ConnectInfo Copy item path

[Source][5]
``` 
pub struct ConnectInfo<T>(pub T);
```

Available on **crate feature`tokio`** only.

Expand description

Extractor for getting connection information produced by a [`Connected`][6].

Note this extractor requires you to use [`Router::into_make_service_with_connect_info`][7] to run your app otherwise it will fail at runtime.

See [`Router::into_make_service_with_connect_info`][7] for more details.

## Tuple Fields§

§`0: T`

## Trait Implementations§

[Source][8]§

### impl<T: [Clone][9]> [Clone][9] for [ConnectInfo][10]<T>

[Source][8]§

#### fn [clone][11](&self) -> [ConnectInfo][10]<T>

Returns a duplicate of the value. [Read more][11]

1.0.0 · [Source][12]§

#### fn [clone_from][13](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][13]

[Source][8]§

### impl<T: [Debug][14]> [Debug][14] for [ConnectInfo][10]<T>

[Source][8]§

#### fn [fmt][15](&self, f: &mut [Formatter][16]<'_>) -> [Result][17]

Formats the value using the given formatter. [Read more][15]

[Source][18]§

### impl<T> [Deref][19] for [ConnectInfo][10]<T>

[Source][18]§

#### type [Target][20] = T

The resulting type after dereferencing.

[Source][18]§

#### fn [deref][21](&self) -> &Self::[Target][22]

Dereferences the value.

[Source][18]§

### impl<T> [DerefMut][23] for [ConnectInfo][10]<T>

[Source][18]§

#### fn [deref_mut][24](&mut self) -> &mut Self::[Target][22]

Mutably dereferences the value.

[Source][25]§

### impl<S, T> [FromRequestParts][26]<S> for [ConnectInfo][10]<T>

where S: [Send][27] \+ [Sync][28], T: [Clone][9] \+ [Send][27] \+ [Sync][28] \+ 'static,

[Source][29]§

#### type [Rejection][30] = <[Extension][31]<[ConnectInfo][10]<T>> as [FromRequestParts][26]<S>>::[Rejection][32]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][33]§

#### async fn [from_request_parts][34]( parts: &mut Parts, state: [&S][35], ) -> [Result][36]<Self, Self::[Rejection][32]>

Perform the extraction.

[Source][8]§

### impl<T: [Copy][37]> [Copy][37] for [ConnectInfo][10]<T>

## Auto Trait Implementations§

§

### impl<T> [Freeze][38] for [ConnectInfo][10]<T>

where T: [Freeze][38],

§

### impl<T> [RefUnwindSafe][39] for [ConnectInfo][10]<T>

where T: [RefUnwindSafe][39],

§

### impl<T> [Send][27] for [ConnectInfo][10]<T>

where T: [Send][27],

§

### impl<T> [Sync][28] for [ConnectInfo][10]<T>

where T: [Sync][28],

§

### impl<T> [Unpin][40] for [ConnectInfo][10]<T>

where T: [Unpin][40],

§

### impl<T> [UnwindSafe][41] for [ConnectInfo][10]<T>

where T: [UnwindSafe][41],

## Blanket Implementations§

[Source][42]§

### impl<T> [Any][43] for T

where T: 'static + ?[Sized][44],

[Source][45]§

#### fn [type_id][46](&self) -> [TypeId][47]

Gets the `TypeId` of `self`. [Read more][46]

[Source][48]§

### impl<T> [Borrow][49]<T> for T

where T: ?[Sized][44],

[Source][50]§

#### fn [borrow][51](&self) -> [&T][35]

Immutably borrows from an owned value. [Read more][51]

[Source][52]§

### impl<T> [BorrowMut][53]<T> for T

where T: ?[Sized][44],

[Source][54]§

#### fn [borrow_mut][55](&mut self) -> [&mut T][35]

Mutably borrows from an owned value. [Read more][55]

[Source][56]§

### impl<T> [CloneToUninit][57] for T

where T: [Clone][9],

[Source][58]§

#### unsafe fn [clone_to_uninit][59](&self, dest: [*mut ][60][u8][61])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][59]

[Source][62]§

### impl<T> [From][63]<T> for T

[Source][64]§

#### fn [from][65](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][66]<T> for T

where T: [Clone][9],

§

#### fn [from_ref][67](input: [&T][35]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][68]<S, ViaParts> for T

where S: [Send][27] \+ [Sync][28], T: [FromRequestParts][26]<S>,

§

#### type [Rejection][69] = <T as [FromRequestParts][26]<S>>::[Rejection][32]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][70]( req: Request<[Body][71]>, state: [&S][35], ) -> impl [Future][72]<Output = [Result][36]<T, <T as [FromRequest][68]<S, ViaParts>>::[Rejection][73]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][74] [`Span`][75], returning an `Instrumented` wrapper. Read more

[Source][76]§

### impl<T, U> [Into][77]<U> for T

where U: [From][63]<T>,

[Source][78]§

#### fn [into][79](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][63]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][44],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][80]§

### impl<P, T> [Receiver][81] for P

where P: [Deref][19]<Target = T> \+ ?[Sized][44], T: ?[Sized][44],

[Source][82]§

#### type [Target][83] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][84]§

### impl<R> [Rng][85] for R

where R: [RngCore][86] \+ ?[Sized][44],

[Source][87]§

#### fn [random][88]<T>(&mut self) -> T

where [StandardUniform][89]: [Distribution][90]<T>,

Return a random value via the [`StandardUniform`][89] distribution. [Read more][88]

[Source][91]§

#### fn [random_iter][92]<T>(self) -> [Iter][93]<[StandardUniform][89], Self, T>

where Self: [Sized][44], [StandardUniform][89]: [Distribution][90]<T>,

Return an iterator over [`random`][94] variates [Read more][92]

[Source][95]§

#### fn [random_range][96]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][97], R: [SampleRange][98]<T>,

Generate a random value in the given range. [Read more][96]

[Source][99]§

#### fn [random_bool][100](&mut self, p: [f64][101]) -> [bool][102]

Return a bool with a probability `p` of being true. [Read more][100]

[Source][103]§

#### fn [random_ratio][104](&mut self, numerator: [u32][105], denominator: [u32][105]) -> [bool][102]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][104]

[Source][106]§

#### fn [sample][107]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][90]<T>,

Sample a new value, using the given distribution. [Read more][107]

[Source][108]§

#### fn [sample_iter][109]<T, D>(self, distr: D) -> [Iter][93]<D, Self, T>

where D: [Distribution][90]<T>, Self: [Sized][44],

Create an iterator that generates values using the given distribution. [Read more][109]

[Source][110]§

#### fn [fill][111]<T>(&mut self, dest: [&mut T][35])

where T: [Fill][112] \+ ?[Sized][44],

Fill any type implementing [`Fill`][112] with random data [Read more][111]

[Source][113]§

#### fn [gen][114]<T>(&mut self) -> T

where [StandardUniform][89]: [Distribution][90]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][94].

[Source][115]§

#### fn [gen_range][116]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][97], R: [SampleRange][98]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][117].

[Source][118]§

#### fn [gen_bool][119](&mut self, p: [f64][101]) -> [bool][102]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][120].

[Source][121]§

#### fn [gen_ratio][122](&mut self, numerator: [u32][105], denominator: [u32][105]) -> [bool][102]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][123].

[Source][124]§

### impl<T> [RngCore][86] for T

where T: [DerefMut][23], <T as [Deref][19]>::[Target][22]: [RngCore][86],

[Source][125]§

#### fn [next_u32][126](&mut self) -> [u32][105]

Return the next random `u32`. [Read more][126]

[Source][127]§

#### fn [next_u64][128](&mut self) -> [u64][129]

Return the next random `u64`. [Read more][128]

[Source][130]§

#### fn [fill_bytes][131](&mut self, dst: &mut [[u8][61]])

Fill `dest` with random data. [Read more][131]

[Source][132]§

### impl<T> [Same][133] for T

[Source][134]§

#### type [Output][135] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][44],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][44],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][136]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][44],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][44],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][44],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][44],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][44],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][137] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][138]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][139] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][138]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][139] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][138]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][44],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][139] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][44],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][44], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][44],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][44],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][140]) -> RequestBodyLimit<Self>

where Self: [Sized][44],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][44],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][141]§

### impl<T> [ToOwned][142] for T

where T: [Clone][9],

[Source][143]§

#### type [Owned][144] = T

The resulting type after obtaining ownership.

[Source][145]§

#### fn [to_owned][146](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][146]

[Source][147]§

#### fn [clone_into][148](&self, target: [&mut T][35])

Uses borrowed data to replace owned data, usually by cloning. [Read more][148]

[Source][149]§

### impl<T, U> [TryFrom][150]<U> for T

where U: [Into][77]<T>,

[Source][151]§

#### type [Error][152] = [Infallible][153]

The type returned in the event of a conversion error.

[Source][154]§

#### fn [try_from][155](value: U) -> [Result][36]<T, <T as [TryFrom][150]<U>>::[Error][156]>

Performs the conversion.

[Source][157]§

### impl<T, U> [TryInto][158]<U> for T

where U: [TryFrom][150]<T>,

[Source][159]§

#### type [Error][160] = <U as [TryFrom][150]<T>>::[Error][156]

The type returned in the event of a conversion error.

[Source][161]§

#### fn [try_into][162](self) -> [Result][36]<U, <U as [TryFrom][150]<T>>::[Error][156]>

Performs the conversion.

[Source][163]§

### impl<R> [TryRngCore][164] for R

where R: [RngCore][86] \+ ?[Sized][44],

[Source][165]§

#### type [Error][166] = [Infallible][153]

The type returned in the event of a RNG error.

[Source][167]§

#### fn [try_next_u32][168](&mut self) -> [Result][36]<[u32][105], <R as [TryRngCore][164]>::[Error][169]>

Return the next random `u32`.

[Source][170]§

#### fn [try_next_u64][171](&mut self) -> [Result][36]<[u64][129], <R as [TryRngCore][164]>::[Error][169]>

Return the next random `u64`.

[Source][172]§

#### fn [try_fill_bytes][173]( &mut self, dst: &mut [[u8][61]], ) -> [Result][36]<[()][174], <R as [TryRngCore][164]>::[Error][169]>

Fill `dest` entirely with random data.

[Source][175]§

#### fn [unwrap_err][176](self) -> [UnwrapErr][177]<Self>

where Self: [Sized][44],

Wrap RNG with the [`UnwrapErr`][177] wrapper.

[Source][178]§

#### fn [unwrap_mut][179](&mut self) -> [UnwrapMut][180]<'_, Self>

Wrap RNG with the [`UnwrapMut`][180] wrapper.

[Source][181]§

#### fn [read_adapter][182](&mut self) -> [RngReadAdapter][183]<'_, Self>

where Self: [Sized][44],

Available on **crate feature`std`** only.

Convert an [`RngCore`][86] to a [`RngReadAdapter`][183].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][77]<Dispatch>,

Attaches the provided [`Subscriber`][184] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][185] [`Subscriber`][184] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][186]§

### impl<T> [CryptoRng][187] for T

where T: [DerefMut][23], <T as [Deref][19]>::[Target][22]: [CryptoRng][187],

[Source][188]§

### impl<R> [TryCryptoRng][189] for R

where R: [CryptoRng][187] \+ ?[Sized][44],

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/connect_info.rs.html#136
   [6]: trait.Connected.html (trait axum::extract::connect_info::Connected)
   [7]: ../../struct.Router.html#method.into_make_service_with_connect_info (method axum::Router::into_make_service_with_connect_info)
   [8]: ../../../src/axum/extract/connect_info.rs.html#135
   [9]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [10]: ../struct.ConnectInfo.html (struct axum::extract::ConnectInfo)
   [11]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [12]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [13]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [14]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [16]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [18]: ../../../src/axum/extract/connect_info.rs.html#156
   [19]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [20]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [21]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [22]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [23]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [24]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [25]: ../../../src/axum/extract/connect_info.rs.html#138-154
   [26]: ../trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [29]: ../../../src/axum/extract/connect_info.rs.html#143
   [30]: ../trait.FromRequestParts.html#associatedtype.Rejection
   [31]: ../../struct.Extension.html (struct axum::Extension)
   [32]: ../trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [33]: ../../../src/axum/extract/connect_info.rs.html#145-153
   [34]: ../trait.FromRequestParts.html#tymethod.from_request_parts
   [35]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [36]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [39]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [41]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [42]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [43]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [45]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [46]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [47]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [50]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [51]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [52]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [53]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [54]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [55]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [56]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [57]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [58]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [59]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [60]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [61]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [62]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [63]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [64]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [65]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [66]: ../trait.FromRef.html (trait axum::extract::FromRef)
   [67]: ../trait.FromRef.html#tymethod.from_ref
   [68]: ../trait.FromRequest.html (trait axum::extract::FromRequest)
   [69]: ../trait.FromRequest.html#associatedtype.Rejection
   [70]: ../trait.FromRequest.html#tymethod.from_request
   [71]: ../../body/struct.Body.html (struct axum::body::Body)
   [72]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [73]: ../trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [74]: super::Span::current()
   [75]: crate::Span
   [76]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [77]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [78]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [79]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [80]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [81]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [82]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [83]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [84]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [85]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [86]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [87]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [88]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [89]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [90]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [91]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [92]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [93]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [94]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [95]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [96]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [97]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [98]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [99]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [100]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [101]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [102]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [103]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [104]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [105]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [106]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [107]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [108]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [109]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [110]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [111]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [112]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [113]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [114]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [115]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [116]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [117]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [118]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [119]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [120]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [121]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [122]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [123]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [124]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [125]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [126]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [127]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [128]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [129]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [130]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [131]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [132]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [133]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [134]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [135]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [136]: https://docs.rs/http/latest/http/struct.Extensions.html
   [137]: crate::follow_redirect::policy::Standard
   [138]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [139]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [140]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [141]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [142]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [143]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [144]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [145]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [146]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [147]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [148]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [149]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [150]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [151]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [152]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [153]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [154]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [155]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [156]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [157]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [158]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [159]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [160]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [161]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [162]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [163]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [164]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [165]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [166]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [167]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [168]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [169]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [170]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [171]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [172]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [173]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [174]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [175]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [176]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [177]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [178]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [179]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [180]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [181]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [182]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [183]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [184]: super::Subscriber
   [185]: dispatcher#setting-the-default-subscriber
   [186]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [187]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [188]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [189]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

