<!-- Generated from rustdoc HTML: extract/struct.State.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## State

## [axum][1]0.8.8

## State

### Sections

  * With `Router`
    * Combining stateful routers
  * With `MethodRouter`
  * With `Handler`
  * Substates
  * For library authors
  * Shared mutable state
    * Example



### Tuple Fields

  * 0



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * Default
  * Deref
  * DerefMut
  * FromRequestParts<OuterState>



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

# Struct State Copy item path

[Source][4]
``` 
pub struct State<S>(pub S);
```

Expand description

Extractor for state.

See [“Accessing state in middleware”][5] for how to access state in middleware.

State is global and used in every request a router with state receives. For accessing data derived from requests, such as authorization data, see [`Extension`][6].

## §With `Router`
``` 
use axum::{Router, routing::get, extract::State};

// the application state
//
// here you can put configuration, database connection pools, or whatever
// state you need
#[derive(Clone)]
struct AppState {}

let state = AppState {};

// create a `Router` that holds our state
let app = Router::new()
    .route("/", get(handler))
    // provide the state so the router can access it
    .with_state(state);

async fn handler(
    // access the state via the `State` extractor
    // extracting a state of the wrong type results in a compile error
    State(state): State<AppState>,
) {
    // use `state`...
}
```

Note that `State` is an extractor, so be sure to put it before any body extractors, see [“the order of extractors”][7].

### §Combining stateful routers

Multiple [`Router`][8]s can be combined with [`Router::nest`][9] or [`Router::merge`][10] When combining [`Router`][8]s with one of these methods, the [`Router`][8]s must have the same state type. Generally, this can be inferred automatically:
``` 
use axum::{Router, routing::get, extract::State};

#[derive(Clone)]
struct AppState {}

let state = AppState {};

// create a `Router` that will be nested within another
let api = Router::new()
    .route("/posts", get(posts_handler));

let app = Router::new()
    .nest("/api", api)
    .with_state(state);

async fn posts_handler(State(state): State<AppState>) {
    // use `state`...
}
```

However, if you are composing [`Router`][8]s that are defined in separate scopes, you may need to annotate the [`State`][11] type explicitly:
``` 
use axum::{Router, routing::get, extract::State};

#[derive(Clone)]
struct AppState {}

fn make_app() -> Router {
    let state = AppState {};

    Router::new()
        .nest("/api", make_api())
        .with_state(state) // the outer Router's state is inferred
}

// the inner Router must specify its state type to compose with the
// outer router
fn make_api() -> Router<AppState> {
    Router::new()
        .route("/posts", get(posts_handler))
}

async fn posts_handler(State(state): State<AppState>) {
    // use `state`...
}
```

In short, a [`Router`][8]’s generic state type defaults to `()` (no state) unless [`Router::with_state`][12] is called or the value of the generic type is given explicitly.

## §With `MethodRouter`
``` 
use axum::{routing::get, extract::State};

#[derive(Clone)]
struct AppState {}

let state = AppState {};

let method_router_with_state = get(handler)
    // provide the state so the handler can access it
    .with_state(state);

async fn handler(State(state): State<AppState>) {
    // use `state`...
}
```

## §With `Handler`
``` 
use axum::{routing::get, handler::Handler, extract::State};

#[derive(Clone)]
struct AppState {}

let state = AppState {};

async fn handler(State(state): State<AppState>) {
    // use `state`...
}

// provide the state so the handler can access it
let handler_with_state = handler.with_state(state);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, handler_with_state.into_make_service()).await;
```

## §Substates

[`State`][11] only allows a single state type but you can use [`FromRef`][13] to extract “substates”:
``` 
use axum::{Router, routing::get, extract::{State, FromRef}};

// the application state
#[derive(Clone)]
struct AppState {
    // that holds some api specific state
    api_state: ApiState,
}

// the api specific state
#[derive(Clone)]
struct ApiState {}

// support converting an `AppState` in an `ApiState`
impl FromRef<AppState> for ApiState {
    fn from_ref(app_state: &AppState) -> ApiState {
        app_state.api_state.clone()
    }
}

let state = AppState {
    api_state: ApiState {},
};

let app = Router::new()
    .route("/", get(handler))
    .route("/api/users", get(api_users))
    .with_state(state);

async fn api_users(
    // access the api specific state
    State(api_state): State<ApiState>,
) {
}

async fn handler(
    // we can still access to top level state
    State(state): State<AppState>,
) {
}
```

For convenience `FromRef` can also be derived using `#[derive(FromRef)]`.

## §For library authors

If you’re writing a library that has an extractor that needs state, this is the recommended way to do it:
``` 
use axum_core::extract::{FromRequestParts, FromRef};
use http::request::Parts;
use std::convert::Infallible;

// the extractor your library provides
struct MyLibraryExtractor;

impl<S> FromRequestParts<S> for MyLibraryExtractor
where
    // keep `S` generic but require that it can produce a `MyLibraryState`
    // this means users will have to implement `FromRef<UserState> for MyLibraryState`
    MyLibraryState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // get a `MyLibraryState` from a reference to the state
        let state = MyLibraryState::from_ref(state);

        // ...
    }
}

// the state your library needs
struct MyLibraryState {
    // ...
}
```

## §Shared mutable state

[As state is global within a `Router`][12] you can’t directly get a mutable reference to the state.

The most basic solution is to use an `Arc<Mutex<_>>`. Which kind of mutex you need depends on your use case. See [the tokio docs][14] for more details.

Note that holding a locked `std::sync::Mutex` across `.await` points will result in `!Send` futures which are incompatible with axum. If you need to hold a mutex across `.await` points, consider using a `tokio::sync::Mutex` instead.

### §Example
``` 
use axum::{Router, routing::get, extract::State};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<String>>,
}

async fn handler(State(state): State<AppState>) {
    {
        let mut data = state.data.lock().expect("mutex was poisoned");
        *data = "updated foo".to_owned();
    }

    // ...
}

let state = AppState {
    data: Arc::new(Mutex::new("foo".to_owned())),
};

let app = Router::new()
    .route("/", get(handler))
    .with_state(state);
```

## Tuple Fields§

§`0: S`

## Trait Implementations§

[Source][15]§

### impl<S: [Clone][16]> [Clone][16] for [State][11]<S>

[Source][15]§

#### fn [clone][17](&self) -> [State][11]<S>

Returns a duplicate of the value. [Read more][17]

1.0.0 · [Source][18]§

#### fn [clone_from][19](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][19]

[Source][15]§

### impl<S: [Debug][20]> [Debug][20] for [State][11]<S>

[Source][15]§

#### fn [fmt][21](&self, f: &mut [Formatter][22]<'_>) -> [Result][23]

Formats the value using the given formatter. [Read more][21]

[Source][15]§

### impl<S: [Default][24]> [Default][24] for [State][11]<S>

[Source][15]§

#### fn [default][25]() -> [State][11]<S>

Returns the “default value” for a type. [Read more][25]

[Source][26]§

### impl<S> [Deref][27] for [State][11]<S>

[Source][28]§

#### type [Target][29] = S

The resulting type after dereferencing.

[Source][30]§

#### fn [deref][31](&self) -> &Self::[Target][32]

Dereferences the value.

[Source][33]§

### impl<S> [DerefMut][34] for [State][11]<S>

[Source][35]§

#### fn [deref_mut][36](&mut self) -> &mut Self::[Target][32]

Mutably dereferences the value.

[Source][37]§

### impl<OuterState, InnerState> [FromRequestParts][38]<OuterState> for [State][11]<InnerState>

where InnerState: [FromRef][13]<OuterState>, OuterState: [Send][39] \+ [Sync][40],

[Source][41]§

#### type [Rejection][42] = [Infallible][43]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][44]§

#### async fn [from_request_parts][45]( _parts: &mut Parts, state: [&OuterState][46], ) -> [Result][47]<Self, Self::[Rejection][48]>

Perform the extraction.

[Source][15]§

### impl<S: [Copy][49]> [Copy][49] for [State][11]<S>

## Auto Trait Implementations§

§

### impl<S> [Freeze][50] for [State][11]<S>

where S: [Freeze][50],

§

### impl<S> [RefUnwindSafe][51] for [State][11]<S>

where S: [RefUnwindSafe][51],

§

### impl<S> [Send][39] for [State][11]<S>

where S: [Send][39],

§

### impl<S> [Sync][40] for [State][11]<S>

where S: [Sync][40],

§

### impl<S> [Unpin][52] for [State][11]<S>

where S: [Unpin][52],

§

### impl<S> [UnwindSafe][53] for [State][11]<S>

where S: [UnwindSafe][53],

## Blanket Implementations§

[Source][54]§

### impl<T> [Any][55] for T

where T: 'static + ?[Sized][56],

[Source][57]§

#### fn [type_id][58](&self) -> [TypeId][59]

Gets the `TypeId` of `self`. [Read more][58]

[Source][60]§

### impl<T> [Borrow][61]<T> for T

where T: ?[Sized][56],

[Source][62]§

#### fn [borrow][63](&self) -> [&T][46]

Immutably borrows from an owned value. [Read more][63]

[Source][64]§

### impl<T> [BorrowMut][65]<T> for T

where T: ?[Sized][56],

[Source][66]§

#### fn [borrow_mut][67](&mut self) -> [&mut T][46]

Mutably borrows from an owned value. [Read more][67]

[Source][68]§

### impl<T> [CloneToUninit][69] for T

where T: [Clone][16],

[Source][70]§

#### unsafe fn [clone_to_uninit][71](&self, dest: [*mut ][72][u8][73])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][71]

[Source][74]§

### impl<T> [From][75]<T> for T

[Source][76]§

#### fn [from][77](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][13]<T> for T

where T: [Clone][16],

§

#### fn [from_ref][78](input: [&T][46]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][79]<S, ViaParts> for T

where S: [Send][39] \+ [Sync][40], T: [FromRequestParts][38]<S>,

§

#### type [Rejection][80] = <T as [FromRequestParts][38]<S>>::[Rejection][48]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][81]( req: Request<[Body][82]>, state: [&S][46], ) -> impl [Future][83]<Output = [Result][47]<T, <T as [FromRequest][79]<S, ViaParts>>::[Rejection][84]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][85] [`Span`][86], returning an `Instrumented` wrapper. Read more

[Source][87]§

### impl<T, U> [Into][88]<U> for T

where U: [From][75]<T>,

[Source][89]§

#### fn [into][90](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][75]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][56],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][91]§

### impl<P, T> [Receiver][92] for P

where P: [Deref][27]<Target = T> \+ ?[Sized][56], T: ?[Sized][56],

[Source][93]§

#### type [Target][94] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][95]§

### impl<R> [Rng][96] for R

where R: [RngCore][97] \+ ?[Sized][56],

[Source][98]§

#### fn [random][99]<T>(&mut self) -> T

where [StandardUniform][100]: [Distribution][101]<T>,

Return a random value via the [`StandardUniform`][100] distribution. [Read more][99]

[Source][102]§

#### fn [random_iter][103]<T>(self) -> [Iter][104]<[StandardUniform][100], Self, T>

where Self: [Sized][56], [StandardUniform][100]: [Distribution][101]<T>,

Return an iterator over [`random`][105] variates [Read more][103]

[Source][106]§

#### fn [random_range][107]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][108], R: [SampleRange][109]<T>,

Generate a random value in the given range. [Read more][107]

[Source][110]§

#### fn [random_bool][111](&mut self, p: [f64][112]) -> [bool][113]

Return a bool with a probability `p` of being true. [Read more][111]

[Source][114]§

#### fn [random_ratio][115](&mut self, numerator: [u32][116], denominator: [u32][116]) -> [bool][113]

Return a bool with a probability of `numerator/denominator` of being true. [Read more][115]

[Source][117]§

#### fn [sample][118]<T, D>(&mut self, distr: D) -> T

where D: [Distribution][101]<T>,

Sample a new value, using the given distribution. [Read more][118]

[Source][119]§

#### fn [sample_iter][120]<T, D>(self, distr: D) -> [Iter][104]<D, Self, T>

where D: [Distribution][101]<T>, Self: [Sized][56],

Create an iterator that generates values using the given distribution. [Read more][120]

[Source][121]§

#### fn [fill][122]<T>(&mut self, dest: [&mut T][46])

where T: [Fill][123] \+ ?[Sized][56],

Fill any type implementing [`Fill`][123] with random data [Read more][122]

[Source][124]§

#### fn [gen][125]<T>(&mut self) -> T

where [StandardUniform][100]: [Distribution][101]<T>,

👎Deprecated since 0.9.0: Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`][105].

[Source][126]§

#### fn [gen_range][127]<T, R>(&mut self, range: R) -> T

where T: [SampleUniform][108], R: [SampleRange][109]<T>,

👎Deprecated since 0.9.0: Renamed to `random_range`

Alias for [`Rng::random_range`][128].

[Source][129]§

#### fn [gen_bool][130](&mut self, p: [f64][112]) -> [bool][113]

👎Deprecated since 0.9.0: Renamed to `random_bool`

Alias for [`Rng::random_bool`][131].

[Source][132]§

#### fn [gen_ratio][133](&mut self, numerator: [u32][116], denominator: [u32][116]) -> [bool][113]

👎Deprecated since 0.9.0: Renamed to `random_ratio`

Alias for [`Rng::random_ratio`][134].

[Source][135]§

### impl<T> [RngCore][97] for T

where T: [DerefMut][34], <T as [Deref][27]>::[Target][32]: [RngCore][97],

[Source][136]§

#### fn [next_u32][137](&mut self) -> [u32][116]

Return the next random `u32`. [Read more][137]

[Source][138]§

#### fn [next_u64][139](&mut self) -> [u64][140]

Return the next random `u64`. [Read more][139]

[Source][141]§

#### fn [fill_bytes][142](&mut self, dst: &mut [[u8][73]])

Fill `dest` with random data. [Read more][142]

[Source][143]§

### impl<T> [Same][144] for T

[Source][145]§

#### type [Output][146] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][56],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][56],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][147]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][56],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][56],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][56],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][56],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][56],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][56],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][56],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][148] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][149]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][56],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][150] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][149]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][56],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][150] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][149]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][56],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][150] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][56],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][56],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][56],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][56],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][56],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][56],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][56], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][56], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][56],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][56],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][56],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][151]) -> RequestBodyLimit<Self>

where Self: [Sized][56],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][56],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][56],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][152]§

### impl<T> [ToOwned][153] for T

where T: [Clone][16],

[Source][154]§

#### type [Owned][155] = T

The resulting type after obtaining ownership.

[Source][156]§

#### fn [to_owned][157](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][157]

[Source][158]§

#### fn [clone_into][159](&self, target: [&mut T][46])

Uses borrowed data to replace owned data, usually by cloning. [Read more][159]

[Source][160]§

### impl<T, U> [TryFrom][161]<U> for T

where U: [Into][88]<T>,

[Source][162]§

#### type [Error][163] = [Infallible][43]

The type returned in the event of a conversion error.

[Source][164]§

#### fn [try_from][165](value: U) -> [Result][47]<T, <T as [TryFrom][161]<U>>::[Error][166]>

Performs the conversion.

[Source][167]§

### impl<T, U> [TryInto][168]<U> for T

where U: [TryFrom][161]<T>,

[Source][169]§

#### type [Error][170] = <U as [TryFrom][161]<T>>::[Error][166]

The type returned in the event of a conversion error.

[Source][171]§

#### fn [try_into][172](self) -> [Result][47]<U, <U as [TryFrom][161]<T>>::[Error][166]>

Performs the conversion.

[Source][173]§

### impl<R> [TryRngCore][174] for R

where R: [RngCore][97] \+ ?[Sized][56],

[Source][175]§

#### type [Error][176] = [Infallible][43]

The type returned in the event of a RNG error.

[Source][177]§

#### fn [try_next_u32][178](&mut self) -> [Result][47]<[u32][116], <R as [TryRngCore][174]>::[Error][179]>

Return the next random `u32`.

[Source][180]§

#### fn [try_next_u64][181](&mut self) -> [Result][47]<[u64][140], <R as [TryRngCore][174]>::[Error][179]>

Return the next random `u64`.

[Source][182]§

#### fn [try_fill_bytes][183]( &mut self, dst: &mut [[u8][73]], ) -> [Result][47]<[()][184], <R as [TryRngCore][174]>::[Error][179]>

Fill `dest` entirely with random data.

[Source][185]§

#### fn [unwrap_err][186](self) -> [UnwrapErr][187]<Self>

where Self: [Sized][56],

Wrap RNG with the [`UnwrapErr`][187] wrapper.

[Source][188]§

#### fn [unwrap_mut][189](&mut self) -> [UnwrapMut][190]<'_, Self>

Wrap RNG with the [`UnwrapMut`][190] wrapper.

[Source][191]§

#### fn [read_adapter][192](&mut self) -> [RngReadAdapter][193]<'_, Self>

where Self: [Sized][56],

Available on **crate feature`std`** only.

Convert an [`RngCore`][97] to a [`RngReadAdapter`][193].

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][88]<Dispatch>,

Attaches the provided [`Subscriber`][194] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][195] [`Subscriber`][194] to this type, returning a [`WithDispatch`] wrapper. Read more

[Source][196]§

### impl<T> [CryptoRng][197] for T

where T: [DerefMut][34], <T as [Deref][27]>::[Target][32]: [CryptoRng][197],

[Source][198]§

### impl<R> [TryCryptoRng][199] for R

where R: [CryptoRng][197] \+ ?[Sized][56],

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/state.rs.html#296
   [5]: ../middleware/index.html#accessing-state-in-middleware (mod axum::middleware)
   [6]: ../struct.Extension.html (struct axum::Extension)
   [7]: index.html#the-order-of-extractors (mod axum::extract)
   [8]: ../struct.Router.html (struct axum::Router)
   [9]: ../struct.Router.html#method.nest (method axum::Router::nest)
   [10]: ../struct.Router.html#method.merge (method axum::Router::merge)
   [11]: struct.State.html (struct axum::extract::State)
   [12]: ../struct.Router.html#method.with_state (method axum::Router::with_state)
   [13]: trait.FromRef.html (trait axum::extract::FromRef)
   [14]: https://docs.rs/tokio/1.25.0/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use
   [15]: ../../src/axum/extract/state.rs.html#295
   [16]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [17]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [18]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [19]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [20]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [21]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [22]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [23]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [24]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [25]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [26]: ../../src/axum/extract/state.rs.html#314-320
   [27]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [28]: ../../src/axum/extract/state.rs.html#315
   [29]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [30]: ../../src/axum/extract/state.rs.html#317-319
   [31]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [32]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [33]: ../../src/axum/extract/state.rs.html#322-326
   [34]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [35]: ../../src/axum/extract/state.rs.html#323-325
   [36]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [37]: ../../src/axum/extract/state.rs.html#298-312
   [38]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [40]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [41]: ../../src/axum/extract/state.rs.html#303
   [42]: trait.FromRequestParts.html#associatedtype.Rejection
   [43]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [44]: ../../src/axum/extract/state.rs.html#305-311
   [45]: trait.FromRequestParts.html#tymethod.from_request_parts
   [46]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [47]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [48]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [49]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [50]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [51]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [52]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [53]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [54]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [55]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [56]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [57]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [58]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [59]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [60]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [61]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [62]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [63]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [64]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [65]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [66]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [67]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [68]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [69]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [70]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [71]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [72]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [73]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [74]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [75]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [76]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [77]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [78]: trait.FromRef.html#tymethod.from_ref
   [79]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [80]: trait.FromRequest.html#associatedtype.Rejection
   [81]: trait.FromRequest.html#tymethod.from_request
   [82]: ../body/struct.Body.html (struct axum::body::Body)
   [83]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [84]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [85]: super::Span::current()
   [86]: crate::Span
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [89]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [90]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [91]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [92]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [93]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [94]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [95]: https://rust-random.github.io/rand/src/rand/rng.rs.html#357
   [96]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html (trait rand::rng::Rng)
   [97]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html (trait rand_core::RngCore)
   [98]: https://rust-random.github.io/rand/src/rand/rng.rs.html#95-97
   [99]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random
   [100]: https://rust-random.github.io/rand/rand/distr/struct.StandardUniform.html (struct rand::distr::StandardUniform)
   [101]: https://rust-random.github.io/rand/rand/distr/distribution/trait.Distribution.html (trait rand::distr::distribution::Distribution)
   [102]: https://rust-random.github.io/rand/src/rand/rng.rs.html#120-123
   [103]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_iter
   [104]: https://rust-random.github.io/rand/rand/distr/distribution/struct.Iter.html (struct rand::distr::distribution::Iter)
   [105]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random (method rand::rng::Rng::random)
   [106]: https://rust-random.github.io/rand/src/rand/rng.rs.html#161-164
   [107]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range
   [108]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleUniform.html (trait rand::distr::uniform::SampleUniform)
   [109]: https://rust-random.github.io/rand/rand/distr/uniform/trait.SampleRange.html (trait rand::distr::uniform::SampleRange)
   [110]: https://rust-random.github.io/rand/src/rand/rng.rs.html#191
   [111]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool
   [112]: https://doc.rust-lang.org/nightly/std/primitive.f64.html
   [113]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [114]: https://rust-random.github.io/rand/src/rand/rng.rs.html#225
   [115]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio
   [116]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
   [117]: https://rust-random.github.io/rand/src/rand/rng.rs.html#249
   [118]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample
   [119]: https://rust-random.github.io/rand/src/rand/rng.rs.html#286-289
   [120]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.sample_iter
   [121]: https://rust-random.github.io/rand/src/rand/rng.rs.html#314
   [122]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.fill
   [123]: https://rust-random.github.io/rand/rand/rng/trait.Fill.html (trait rand::rng::Fill)
   [124]: https://rust-random.github.io/rand/src/rand/rng.rs.html#324-326
   [125]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen
   [126]: https://rust-random.github.io/rand/src/rand/rng.rs.html#334-337
   [127]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_range
   [128]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_range (method rand::rng::Rng::random_range)
   [129]: https://rust-random.github.io/rand/src/rand/rng.rs.html#345
   [130]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_bool
   [131]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_bool (method rand::rng::Rng::random_bool)
   [132]: https://rust-random.github.io/rand/src/rand/rng.rs.html#352
   [133]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.gen_ratio
   [134]: https://rust-random.github.io/rand/rand/rng/trait.Rng.html#method.random_ratio (method rand::rng::Rng::random_ratio)
   [135]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#158-160
   [136]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#163
   [137]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u32
   [138]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#168
   [139]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.next_u64
   [140]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
   [141]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#173
   [142]: https://rust-random.github.io/rand/rand_core/trait.RngCore.html#tymethod.fill_bytes
   [143]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [144]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [145]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [146]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [147]: https://docs.rs/http/latest/http/struct.Extensions.html
   [148]: crate::follow_redirect::policy::Standard
   [149]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [150]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [151]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [152]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [153]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [154]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [155]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [156]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [157]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [158]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [159]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [160]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [161]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [162]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [163]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [164]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [165]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [166]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [167]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [168]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [169]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [170]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [171]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [172]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [173]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#257
   [174]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html (trait rand_core::TryRngCore)
   [175]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#258
   [176]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error
   [177]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#261
   [178]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u32
   [179]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#associatedtype.Error (type rand_core::TryRngCore::Error)
   [180]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#266
   [181]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_next_u64
   [182]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#271
   [183]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes
   [184]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [185]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#232-234
   [186]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_err
   [187]: https://rust-random.github.io/rand/rand_core/struct.UnwrapErr.html (struct rand_core::UnwrapErr)
   [188]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#240
   [189]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.unwrap_mut
   [190]: https://rust-random.github.io/rand/rand_core/struct.UnwrapMut.html (struct rand_core::UnwrapMut)
   [191]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#246-248
   [192]: https://rust-random.github.io/rand/rand_core/trait.TryRngCore.html#method.read_adapter
   [193]: https://rust-random.github.io/rand/rand_core/struct.RngReadAdapter.html (struct rand_core::RngReadAdapter)
   [194]: super::Subscriber
   [195]: dispatcher#setting-the-default-subscriber
   [196]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#206
   [197]: https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html (trait rand_core::CryptoRng)
   [198]: https://rust-random.github.io/rand/src/rand_core/lib.rs.html#293
   [199]: https://rust-random.github.io/rand/rand_core/trait.TryCryptoRng.html (trait rand_core::TryCryptoRng)

