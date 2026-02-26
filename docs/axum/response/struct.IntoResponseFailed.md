<!-- Generated from rustdoc HTML: response/struct.IntoResponseFailed.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IntoResponseFailed

## [axum][1]0.8.8

## IntoResponseFailed

### Trait Implementations

  * Clone
  * Copy
  * Debug
  * IntoResponseParts



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
  * From<T>
  * FromRef<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ToOwned
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::response][2]

[axum][3]::[response][2]

# Struct IntoResponseFailed Copy item path
```
pub struct IntoResponseFailed;
```

Expand description

Response part that stops status code overrides.

This type should be used by types implementing [`IntoResponseParts`][4] or [`IntoResponse`][5] when they fail to produce the response usually expected of them and return some sort of error response instead.

It is checked used by the tuple impls of [`IntoResponse`][5] that have a [`StatusCode`] as their first element to ignore that status code. Consider the following example:
``` 
fn my_handler(/* ... */) -> (StatusCode, Json<CreatedResponse>) {
    // This response type's serialization may fail
    let response = CreatedResponse { /* ... */ };
    (StatusCode::CREATED, Json(response))
}
```

When `response` serialization succeeds, the server responds with a status code of 201 Created (overwriting `Json`s default status code of 200 OK), and the expected JSON payload.

When `response` serialization fails hoewever, `impl IntoResponse for Json` return a response with status code 500 Internal Server Error, and `IntoResponseFailed` as a response extension, and the 201 Created override is ignored.

This is a behavior introduced with axum 0.9.  
To force a status code override even when an inner [`IntoResponseParts`][4] / [`IntoResponse`][5] failed, use [`ForceStatusCode`].

## Trait Implementations§

§

### impl [Clone][6] for [IntoResponseFailed][7]

§

#### fn [clone][8](&self) -> [IntoResponseFailed][7]

Returns a duplicate of the value. [Read more][8]

1.0.0 · [Source][9]§

#### fn [clone_from][10](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][10]

§

### impl [Debug][11] for [IntoResponseFailed][7]

§

#### fn [fmt][12](&self, f: &mut [Formatter][13]<'_>) -> [Result][14]<[()][15], [Error][16]>

Formats the value using the given formatter. [Read more][12]

§

### impl [IntoResponseParts][4] for [IntoResponseFailed][7]

§

#### type [Error][17] = [Infallible][18]

The type returned in the event of an error. [Read more][17]

§

#### fn [into_response_parts][19]( self, res: [ResponseParts][20], ) -> [Result][14]<[ResponseParts][20], <[IntoResponseFailed][7] as [IntoResponseParts][4]>::[Error][21]>

Set parts of the response

§

### impl [Copy][22] for [IntoResponseFailed][7]

## Auto Trait Implementations§

§

### impl [Freeze][23] for [IntoResponseFailed][7]

§

### impl [RefUnwindSafe][24] for [IntoResponseFailed][7]

§

### impl [Send][25] for [IntoResponseFailed][7]

§

### impl [Sync][26] for [IntoResponseFailed][7]

§

### impl [Unpin][27] for [IntoResponseFailed][7]

§

### impl [UnwindSafe][28] for [IntoResponseFailed][7]

## Blanket Implementations§

[Source][29]§

### impl<T> [Any][30] for T

where T: 'static + ?[Sized][31],

[Source][32]§

#### fn [type_id][33](&self) -> [TypeId][34]

Gets the `TypeId` of `self`. [Read more][33]

[Source][35]§

### impl<T> [Borrow][36]<T> for T

where T: ?[Sized][31],

[Source][37]§

#### fn [borrow][38](&self) -> [&T][39]

Immutably borrows from an owned value. [Read more][38]

[Source][40]§

### impl<T> [BorrowMut][41]<T> for T

where T: ?[Sized][31],

[Source][42]§

#### fn [borrow_mut][43](&mut self) -> [&mut T][39]

Mutably borrows from an owned value. [Read more][43]

[Source][44]§

### impl<T> [CloneToUninit][45] for T

where T: [Clone][6],

[Source][46]§

#### unsafe fn [clone_to_uninit][47](&self, dest: [*mut ][48][u8][49])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][47]

[Source][50]§

### impl<T> [From][51]<T> for T

[Source][52]§

#### fn [from][53](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][54]<T> for T

where T: [Clone][6],

§

#### fn [from_ref][55](input: [&T][39]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][56] [`Span`][57], returning an `Instrumented` wrapper. Read more

[Source][58]§

### impl<T, U> [Into][59]<U> for T

where U: [From][51]<T>,

[Source][60]§

#### fn [into][61](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][51]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][31],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][62]§

### impl<T> [Same][63] for T

[Source][64]§

#### type [Output][65] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][31],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][31],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][66]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][31],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][31],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][31],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][31],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][31],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][31],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][31],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][67] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][68]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][31],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][69] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][31],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][31], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][31], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][31],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][31],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][31],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][70]) -> RequestBodyLimit<Self>

where Self: [Sized][31],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][31],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][31],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][71]§

### impl<T> [ToOwned][72] for T

where T: [Clone][6],

[Source][73]§

#### type [Owned][74] = T

The resulting type after obtaining ownership.

[Source][75]§

#### fn [to_owned][76](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][76]

[Source][77]§

#### fn [clone_into][78](&self, target: [&mut T][39])

Uses borrowed data to replace owned data, usually by cloning. [Read more][78]

[Source][79]§

### impl<T, U> [TryFrom][80]<U> for T

where U: [Into][59]<T>,

[Source][81]§

#### type [Error][82] = [Infallible][18]

The type returned in the event of a conversion error.

[Source][83]§

#### fn [try_from][84](value: U) -> [Result][14]<T, <T as [TryFrom][80]<U>>::[Error][85]>

Performs the conversion.

[Source][86]§

### impl<T, U> [TryInto][87]<U> for T

where U: [TryFrom][80]<T>,

[Source][88]§

#### type [Error][89] = <U as [TryFrom][80]<T>>::[Error][85]

The type returned in the event of a conversion error.

[Source][90]§

#### fn [try_into][91](self) -> [Result][14]<U, <U as [TryFrom][80]<T>>::[Error][85]>

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

where S: [Into][59]<Dispatch>,

Attaches the provided [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][93] [`Subscriber`][92] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: trait.IntoResponseParts.html (trait axum::response::IntoResponseParts)
   [5]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [6]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [7]: struct.IntoResponseFailed.html (struct axum::response::IntoResponseFailed)
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [9]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [10]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [11]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [12]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [13]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [14]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [15]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [16]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [17]: trait.IntoResponseParts.html#associatedtype.Error
   [18]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [19]: trait.IntoResponseParts.html#tymethod.into_response_parts
   [20]: struct.ResponseParts.html (struct axum::response::ResponseParts)
   [21]: trait.IntoResponseParts.html#associatedtype.Error (type axum::response::IntoResponseParts::Error)
   [22]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [23]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [24]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [25]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [26]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [27]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [28]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [29]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [30]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [32]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [33]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [34]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [35]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [36]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [37]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [38]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [39]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [40]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [41]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [44]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [45]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [46]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [47]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [48]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [49]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [50]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [51]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [54]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [55]: ../extract/trait.FromRef.html#tymethod.from_ref
   [56]: super::Span::current()
   [57]: crate::Span
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [60]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [61]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [62]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [63]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [64]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [65]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [66]: https://docs.rs/http/latest/http/struct.Extensions.html
   [67]: crate::follow_redirect::policy::Standard
   [68]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [69]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [70]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [71]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [72]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [73]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [74]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [75]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [76]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [77]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [78]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [81]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [85]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [86]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [87]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [88]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [89]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [90]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [91]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [92]: super::Subscriber
   [93]: dispatcher#setting-the-default-subscriber

