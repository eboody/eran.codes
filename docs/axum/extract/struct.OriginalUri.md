<!-- Generated from rustdoc HTML: extract/struct.OriginalUri.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## OriginalUri

## [axum][1]0.8.8

## OriginalUri

### Sections

  * Example
  * Extracting via request extensions



### Tuple Fields

  * 0



### Methods from Deref<Target=Uri>

  * authority
  * host
  * path
  * path_and_query
  * port
  * port_u16
  * query
  * scheme
  * scheme_str



### Trait Implementations

  * Clone
  * Debug
  * Deref
  * DerefMut
  * FromRequestParts<S>



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
  * FromRequest<S, ViaParts>
  * Instrument
  * Into<U>
  * PolicyExt
  * Receiver
  * Same
  * ServiceExt
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract][2]

[axum][3]::[extract][2]

# Struct OriginalUri Copy item path

[Source][4]
``` 
pub struct OriginalUri(pub Uri);
```

Available on **crate feature`original-uri`** only.

Expand description

Extractor that gets the original request URI regardless of nesting.

This is necessary since [`Uri`][5], when used as an extractor, will have the prefix stripped if used in a nested service.

## §Example
``` 
use axum::{
    routing::get,
    Router,
    extract::OriginalUri,
    http::Uri
};

let api_routes = Router::new()
    .route(
        "/users",
        get(|uri: Uri, OriginalUri(original_uri): OriginalUri| async {
            // `uri` is `/users`
            // `original_uri` is `/api/users`
        }),
    );

let app = Router::new().nest("/api", api_routes);
```

## §Extracting via request extensions

`OriginalUri` can also be accessed from middleware via request extensions. This is useful for example with [`Trace`][6] to create a span that contains the full path, if your service might be nested:
``` 
use axum::{
    Router,
    extract::OriginalUri,
    http::Request,
    routing::get,
};
use tower_http::trace::TraceLayer;

let api_routes = Router::new()
    .route("/users/{id}", get(|| async { /* ... */ }))
    .layer(
        TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
            let path = if let Some(path) = req.extensions().get::<OriginalUri>() {
                // This will include `/api`
                path.0.path().to_owned()
            } else {
                // The `OriginalUri` extension will always be present if using
                // `Router` unless another extractor or middleware has removed it
                req.uri().path().to_owned()
            };
            tracing::info_span!("http-request", %path)
        }),
    );

let app = Router::new().nest("/api", api_routes);
```

## Tuple Fields§

§`0: Uri`

## Methods from [Deref][7]<Target = Uri>§

#### pub fn path_and_query(&self) -> [Option][8]<&PathAndQuery>

Returns the path & query components of the Uri

#### pub fn path(&self) -> &[str][9]

Get the path of this `Uri`.

Both relative and absolute URIs contain a path component, though it might be the empty string. The path component is **case sensitive**.
``` 
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
                                       |--------|
                                            |
                                          path
```

If the URI is `*` then the path component is equal to `*`.

##### §Examples

A relative URI
``` 
let uri: Uri = "/hello/world".parse().unwrap();

assert_eq!(uri.path(), "/hello/world");
```

An absolute URI
``` 
let uri: Uri = "http://example.org/hello/world".parse().unwrap();

assert_eq!(uri.path(), "/hello/world");
```

#### pub fn scheme(&self) -> [Option][8]<&Scheme>

Get the scheme of this `Uri`.

The URI scheme refers to a specification for assigning identifiers within that scheme. Only absolute URIs contain a scheme component, but not all absolute URIs will contain a scheme component. Although scheme names are case-insensitive, the canonical form is lowercase.
``` 
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
|-|
 |
scheme
```

##### §Examples

Absolute URI
``` 
use http::uri::{Scheme, Uri};

let uri: Uri = "http://example.org/hello/world".parse().unwrap();

assert_eq!(uri.scheme(), Some(&Scheme::HTTP));
```

Relative URI
``` 
let uri: Uri = "/hello/world".parse().unwrap();

assert!(uri.scheme().is_none());
```

#### pub fn scheme_str(&self) -> [Option][8]<&[str][9]>

Get the scheme of this `Uri` as a `&str`.

##### §Example
``` 
let uri: Uri = "http://example.org/hello/world".parse().unwrap();

assert_eq!(uri.scheme_str(), Some("http"));
```

#### pub fn authority(&self) -> [Option][8]<&Authority>

Get the authority of this `Uri`.

The authority is a hierarchical element for naming authority such that the remainder of the URI is delegated to that authority. For HTTP, the authority consists of the host and port. The host portion of the authority is **case-insensitive**.

The authority also includes a `username:password` component, however the use of this is deprecated and should be avoided.
``` 
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
      |-------------------------------|
                    |
                authority
```

##### §Examples

Absolute URI
``` 
let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();

assert_eq!(uri.authority().map(|a| a.as_str()), Some("example.org:80"));
```

Relative URI
``` 
let uri: Uri = "/hello/world".parse().unwrap();

assert!(uri.authority().is_none());
```

#### pub fn host(&self) -> [Option][8]<&[str][9]>

Get the host of this `Uri`.

The host subcomponent of authority is identified by an IP literal encapsulated within square brackets, an IPv4 address in dotted- decimal form, or a registered name. The host subcomponent is **case-insensitive**.
``` 
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
                        |---------|
                             |
                            host
```

##### §Examples

Absolute URI
``` 
let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();

assert_eq!(uri.host(), Some("example.org"));
```

Relative URI
``` 
let uri: Uri = "/hello/world".parse().unwrap();

assert!(uri.host().is_none());
```

#### pub fn port(&self) -> [Option][8]<Port<&[str][9]>>

Get the port part of this `Uri`.

The port subcomponent of authority is designated by an optional port number following the host and delimited from it by a single colon (“:”) character. It can be turned into a decimal port number with the `as_u16` method or as a `str` with the `as_str` method.
``` 
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
                                    |-|
                                     |
                                    port
```

##### §Examples

Absolute URI with port
``` 
let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();

let port = uri.port().unwrap();
assert_eq!(port.as_u16(), 80);
```

Absolute URI without port
``` 
let uri: Uri = "http://example.org/hello/world".parse().unwrap();

assert!(uri.port().is_none());
```

Relative URI
``` 
let uri: Uri = "/hello/world".parse().unwrap();

assert!(uri.port().is_none());
```

#### pub fn port_u16(&self) -> [Option][8]<[u16][10]>

Get the port of this `Uri` as a `u16`.

##### §Example
``` 
let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();

assert_eq!(uri.port_u16(), Some(80));
```

#### pub fn query(&self) -> [Option][8]<&[str][9]>

Get the query string of this `Uri`, starting after the `?`.

The query component contains non-hierarchical data that, along with data in the path component, serves to identify a resource within the scope of the URI’s scheme and naming authority (if any). The query component is indicated by the first question mark (“?”) character and terminated by a number sign (“#”) character or by the end of the URI.
``` 
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
                                                  |-------------------|
                                                            |
                                                          query
```

##### §Examples

Absolute URI
``` 
let uri: Uri = "http://example.org/hello/world?key=value".parse().unwrap();

assert_eq!(uri.query(), Some("key=value"));
```

Relative URI with a query string component
``` 
let uri: Uri = "/hello/world?key=value&foo=bar".parse().unwrap();

assert_eq!(uri.query(), Some("key=value&foo=bar"));
```

Relative URI without a query string component
``` 
let uri: Uri = "/hello/world".parse().unwrap();

assert!(uri.query().is_none());
```

## Trait Implementations§

[Source][11]§

### impl [Clone][12] for [OriginalUri][13]

[Source][11]§

#### fn [clone][14](&self) -> [OriginalUri][13]

Returns a duplicate of the value. [Read more][14]

1.0.0 · [Source][15]§

#### fn [clone_from][16](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][16]

[Source][11]§

### impl [Debug][17] for [OriginalUri][13]

[Source][11]§

#### fn [fmt][18](&self, f: &mut [Formatter][19]<'_>) -> [Result][20]

Formats the value using the given formatter. [Read more][18]

[Source][21]§

### impl [Deref][7] for [OriginalUri][13]

[Source][21]§

#### type [Target][22] = Uri

The resulting type after dereferencing.

[Source][21]§

#### fn [deref][23](&self) -> &Self::[Target][24]

Dereferences the value.

[Source][21]§

### impl [DerefMut][25] for [OriginalUri][13]

[Source][21]§

#### fn [deref_mut][26](&mut self) -> &mut Self::[Target][24]

Mutably dereferences the value.

[Source][27]§

### impl<S> [FromRequestParts][28]<S> for [OriginalUri][13]

where S: [Send][29] \+ [Sync][30],

[Source][31]§

#### type [Rejection][32] = [Infallible][33]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

[Source][34]§

#### async fn [from_request_parts][35]( parts: &mut Parts, state: [&S][36], ) -> [Result][37]<Self, Self::[Rejection][38]>

Perform the extraction.

## Auto Trait Implementations§

§

### impl ![Freeze][39] for [OriginalUri][13]

§

### impl [RefUnwindSafe][40] for [OriginalUri][13]

§

### impl [Send][29] for [OriginalUri][13]

§

### impl [Sync][30] for [OriginalUri][13]

§

### impl [Unpin][41] for [OriginalUri][13]

§

### impl [UnwindSafe][42] for [OriginalUri][13]

## Blanket Implementations§

[Source][43]§

### impl<T> [Any][44] for T

where T: 'static + ?[Sized][45],

[Source][46]§

#### fn [type_id][47](&self) -> [TypeId][48]

Gets the `TypeId` of `self`. [Read more][47]

[Source][49]§

### impl<T> [Borrow][50]<T> for T

where T: ?[Sized][45],

[Source][51]§

#### fn [borrow][52](&self) -> [&T][36]

Immutably borrows from an owned value. [Read more][52]

[Source][53]§

### impl<T> [BorrowMut][54]<T> for T

where T: ?[Sized][45],

[Source][55]§

#### fn [borrow_mut][56](&mut self) -> [&mut T][36]

Mutably borrows from an owned value. [Read more][56]

[Source][57]§

### impl<T> [CloneToUninit][58] for T

where T: [Clone][12],

[Source][59]§

#### unsafe fn [clone_to_uninit][60](&self, dest: [*mut ][61][u8][62])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][60]

[Source][63]§

### impl<T> [From][64]<T> for T

[Source][65]§

#### fn [from][66](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][67]<T> for T

where T: [Clone][12],

§

#### fn [from_ref][68](input: [&T][36]) -> T

Converts to this type from a reference to the input type.

§

### impl<S, T> [FromRequest][69]<S, ViaParts> for T

where S: [Send][29] \+ [Sync][30], T: [FromRequestParts][28]<S>,

§

#### type [Rejection][70] = <T as [FromRequestParts][28]<S>>::[Rejection][38]

If the extractor fails it’ll use this “rejection” type. A rejection is a kind of error that can be converted into a response.

§

#### fn [from_request][71]( req: Request<[Body][72]>, state: [&S][36], ) -> impl [Future][73]<Output = [Result][37]<T, <T as [FromRequest][69]<S, ViaParts>>::[Rejection][74]>>

Perform the extraction.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][75] [`Span`][76], returning an `Instrumented` wrapper. Read more

[Source][77]§

### impl<T, U> [Into][78]<U> for T

where U: [From][64]<T>,

[Source][79]§

#### fn [into][80](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][64]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][45],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][81]§

### impl<P, T> [Receiver][82] for P

where P: [Deref][7]<Target = T> \+ ?[Sized][45], T: ?[Sized][45],

[Source][83]§

#### type [Target][84] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][85]§

### impl<T> [Same][86] for T

[Source][87]§

#### type [Output][88] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][45],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][45],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][89]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][45],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][45],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][45],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][45],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][45],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][45],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][45],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][90] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][91]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][92] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][91]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][92] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][91]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][45],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][92] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][45],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][45], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][45], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][45],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][45],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][45],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][93]) -> RequestBodyLimit<Self>

where Self: [Sized][45],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][45],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][45],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][94]§

### impl<T> [ToOwned][95] for T

where T: [Clone][12],

[Source][96]§

#### type [Owned][97] = T

The resulting type after obtaining ownership.

[Source][98]§

#### fn [to_owned][99](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][99]

[Source][100]§

#### fn [clone_into][101](&self, target: [&mut T][36])

Uses borrowed data to replace owned data, usually by cloning. [Read more][101]

[Source][102]§

### impl<T, U> [TryFrom][103]<U> for T

where U: [Into][78]<T>,

[Source][104]§

#### type [Error][105] = [Infallible][33]

The type returned in the event of a conversion error.

[Source][106]§

#### fn [try_from][107](value: U) -> [Result][37]<T, <T as [TryFrom][103]<U>>::[Error][108]>

Performs the conversion.

[Source][109]§

### impl<T, U> [TryInto][110]<U> for T

where U: [TryFrom][103]<T>,

[Source][111]§

#### type [Error][112] = <U as [TryFrom][103]<T>>::[Error][108]

The type returned in the event of a conversion error.

[Source][113]§

#### fn [try_into][114](self) -> [Result][37]<U, <U as [TryFrom][103]<T>>::[Error][108]>

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

where S: [Into][78]<Dispatch>,

Attaches the provided [`Subscriber`][115] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][116] [`Subscriber`][115] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/extract/original_uri.rs.html#68
   [5]: http::Uri
   [6]: tower_http::trace::Trace
   [7]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [8]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [9]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [10]: https://doc.rust-lang.org/nightly/std/primitive.u16.html
   [11]: ../../src/axum/extract/original_uri.rs.html#67
   [12]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [13]: struct.OriginalUri.html (struct axum::extract::OriginalUri)
   [14]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [15]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [16]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [17]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [18]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [19]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [20]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [21]: ../../src/axum/extract/original_uri.rs.html#85
   [22]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [23]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [24]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [25]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html (trait core::ops::deref::DerefMut)
   [26]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut
   [27]: ../../src/axum/extract/original_uri.rs.html#70-83
   [28]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [31]: ../../src/axum/extract/original_uri.rs.html#74
   [32]: trait.FromRequestParts.html#associatedtype.Rejection
   [33]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [34]: ../../src/axum/extract/original_uri.rs.html#76-82
   [35]: trait.FromRequestParts.html#tymethod.from_request_parts
   [36]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [37]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [38]: trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [39]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [40]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [41]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [42]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [43]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [44]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [46]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [47]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [48]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [49]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [50]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [51]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [52]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [53]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [54]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [55]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [56]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [57]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [58]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [59]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [60]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [61]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [62]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [63]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [64]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [65]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [66]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [67]: trait.FromRef.html (trait axum::extract::FromRef)
   [68]: trait.FromRef.html#tymethod.from_ref
   [69]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [70]: trait.FromRequest.html#associatedtype.Rejection
   [71]: trait.FromRequest.html#tymethod.from_request
   [72]: ../body/struct.Body.html (struct axum::body::Body)
   [73]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [74]: trait.FromRequest.html#associatedtype.Rejection (type axum::extract::FromRequest::Rejection)
   [75]: super::Span::current()
   [76]: crate::Span
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [81]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [82]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [83]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [84]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [85]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [86]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [87]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [88]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [89]: https://docs.rs/http/latest/http/struct.Extensions.html
   [90]: crate::follow_redirect::policy::Standard
   [91]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [92]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [93]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [94]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [95]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [96]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [97]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [98]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [99]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [100]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [101]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [102]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [103]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [104]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [105]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [106]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [107]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [108]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [109]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [110]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [111]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [112]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [113]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [114]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [115]: super::Subscriber
   [116]: dispatcher#setting-the-default-subscriber

