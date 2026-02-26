<!-- Generated from rustdoc HTML: response/struct.AppendHeaders.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## AppendHeaders

## [axum][1]0.8.8

## AppendHeaders

### Tuple Fields

  * 0



### Trait Implementations

  * Clone
  * Copy
  * Debug
  * IntoResponse
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
  * HandlerWithoutStateExt<T>
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

# Struct AppendHeaders Copy item path
```
pub struct AppendHeaders<I>(pub I);
```

Expand description

Append headers to a response.

Returning something like `[("content-type", "foo=bar")]` from a handler will override any existing `content-type` headers. If instead you want to append headers, use `AppendHeaders`:
``` 
use axum::{
    response::{AppendHeaders, IntoResponse},
    http::header::SET_COOKIE,
};

async fn handler() -> impl IntoResponse {
    // something that sets the `set-cookie` header
    let set_some_cookies = /* ... */

    (
        set_some_cookies,
        // append two `set-cookie` headers to the response
        // without overriding the ones added by `set_some_cookies`
        AppendHeaders([
            (SET_COOKIE, "foo=bar"),
            (SET_COOKIE, "baz=qux"),
        ])
    )
}
```

## Tuple Fields§

§`0: I`

## Trait Implementations§

§

### impl<I> [Clone][4] for [AppendHeaders][5]<I>

where I: [Clone][4],

§

#### fn [clone][6](&self) -> [AppendHeaders][5]<I>

Returns a duplicate of the value. [Read more][6]

1.0.0 · [Source][7]§

#### fn [clone_from][8](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][8]

§

### impl<I> [Debug][9] for [AppendHeaders][5]<I>

where I: [Debug][9],

§

#### fn [fmt][10](&self, f: &mut [Formatter][11]<'_>) -> [Result][12]<[()][13], [Error][14]>

Formats the value using the given formatter. [Read more][10]

§

### impl<I, K, V> [IntoResponse][15] for [AppendHeaders][5]<I>

where I: [IntoIterator][16]<Item = [(K, V)][17]>, K: [TryInto][18]<HeaderName>, <K as [TryInto][18]<HeaderName>>::[Error][19]: [Display][20], V: [TryInto][18]<HeaderValue>, <V as [TryInto][18]<HeaderValue>>::[Error][19]: [Display][20],

§

#### fn [into_response][21](self) -> Response<[Body][22]>

Create a response.

§

### impl<I, K, V> [IntoResponseParts][23] for [AppendHeaders][5]<I>

where I: [IntoIterator][16]<Item = [(K, V)][17]>, K: [TryInto][18]<HeaderName>, <K as [TryInto][18]<HeaderName>>::[Error][19]: [Display][20], V: [TryInto][18]<HeaderValue>, <V as [TryInto][18]<HeaderValue>>::[Error][19]: [Display][20],

§

#### type [Error][24] = TryIntoHeaderError<<K as [TryInto][18]<HeaderName>>::[Error][19], <V as [TryInto][18]<HeaderValue>>::[Error][19]>

The type returned in the event of an error. [Read more][24]

§

#### fn [into_response_parts][25]( self, res: [ResponseParts][26], ) -> [Result][12]<[ResponseParts][26], <[AppendHeaders][5]<I> as [IntoResponseParts][23]>::[Error][27]>

Set parts of the response

§

### impl<I> [Copy][28] for [AppendHeaders][5]<I>

where I: [Copy][28],

## Auto Trait Implementations§

§

### impl<I> [Freeze][29] for [AppendHeaders][5]<I>

where I: [Freeze][29],

§

### impl<I> [RefUnwindSafe][30] for [AppendHeaders][5]<I>

where I: [RefUnwindSafe][30],

§

### impl<I> [Send][31] for [AppendHeaders][5]<I>

where I: [Send][31],

§

### impl<I> [Sync][32] for [AppendHeaders][5]<I>

where I: [Sync][32],

§

### impl<I> [Unpin][33] for [AppendHeaders][5]<I>

where I: [Unpin][33],

§

### impl<I> [UnwindSafe][34] for [AppendHeaders][5]<I>

where I: [UnwindSafe][34],

## Blanket Implementations§

[Source][35]§

### impl<T> [Any][36] for T

where T: 'static + ?[Sized][37],

[Source][38]§

#### fn [type_id][39](&self) -> [TypeId][40]

Gets the `TypeId` of `self`. [Read more][39]

[Source][41]§

### impl<T> [Borrow][42]<T> for T

where T: ?[Sized][37],

[Source][43]§

#### fn [borrow][44](&self) -> [&T][45]

Immutably borrows from an owned value. [Read more][44]

[Source][46]§

### impl<T> [BorrowMut][47]<T> for T

where T: ?[Sized][37],

[Source][48]§

#### fn [borrow_mut][49](&mut self) -> [&mut T][45]

Mutably borrows from an owned value. [Read more][49]

[Source][50]§

### impl<T> [CloneToUninit][51] for T

where T: [Clone][4],

[Source][52]§

#### unsafe fn [clone_to_uninit][53](&self, dest: [*mut ][54][u8][55])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][53]

[Source][56]§

### impl<T> [From][57]<T> for T

[Source][58]§

#### fn [from][59](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][60]<T> for T

where T: [Clone][4],

§

#### fn [from_ref][61](input: [&T][45]) -> T

Converts to this type from a reference to the input type.

[Source][62]§

### impl<H, T> [HandlerWithoutStateExt][63]<T> for H

where H: [Handler][64]<T, [()][13]>,

[Source][65]§

#### fn [into_service][66](self) -> [HandlerService][67]<H, T, [()][13]>

Convert the handler into a [`Service`] and no state.

[Source][68]§

#### fn [into_make_service][69](self) -> [IntoMakeService][70]<[HandlerService][67]<H, T, [()][13]>>

Convert the handler into a [`MakeService`][71] and no state. [Read more][69]

[Source][72]§

#### fn [into_make_service_with_connect_info][73]<C>( self, ) -> [IntoMakeServiceWithConnectInfo][74]<[HandlerService][67]<H, T, [()][13]>, C>

Available on **crate feature`tokio`** only.

Convert the handler into a [`MakeService`][71] which stores information about the incoming connection and has no state. [Read more][73]

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

where U: [From][57]<T>,

[Source][79]§

#### fn [into][80](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][57]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][37],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][81]§

### impl<T> [Same][82] for T

[Source][83]§

#### type [Output][84] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][37],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][37],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][85]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][37],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][37],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][37],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][37],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][37],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][37],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][37],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][86] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][16]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][37],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][87] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][16]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][37],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][87] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][16]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][37],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][87] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][37],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][37], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][37], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][37],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][37],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][37],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][88]) -> RequestBodyLimit<Self>

where Self: [Sized][37],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][37],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][37],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][89]§

### impl<T> [ToOwned][90] for T

where T: [Clone][4],

[Source][91]§

#### type [Owned][92] = T

The resulting type after obtaining ownership.

[Source][93]§

#### fn [to_owned][94](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][94]

[Source][95]§

#### fn [clone_into][96](&self, target: [&mut T][45])

Uses borrowed data to replace owned data, usually by cloning. [Read more][96]

[Source][97]§

### impl<T, U> [TryFrom][98]<U> for T

where U: [Into][78]<T>,

[Source][99]§

#### type [Error][100] = [Infallible][101]

The type returned in the event of a conversion error.

[Source][102]§

#### fn [try_from][103](value: U) -> [Result][12]<T, <T as [TryFrom][98]<U>>::[Error][104]>

Performs the conversion.

[Source][105]§

### impl<T, U> [TryInto][18]<U> for T

where U: [TryFrom][98]<T>,

[Source][106]§

#### type [Error][107] = <U as [TryFrom][98]<T>>::[Error][104]

The type returned in the event of a conversion error.

[Source][108]§

#### fn [try_into][109](self) -> [Result][12]<U, <U as [TryFrom][98]<T>>::[Error][104]>

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

Attaches the provided [`Subscriber`][110] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][111] [`Subscriber`][110] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [5]: struct.AppendHeaders.html (struct axum::response::AppendHeaders)
   [6]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [7]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [8]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [9]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [10]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [11]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [12]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [13]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [14]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [15]: trait.IntoResponse.html (trait axum::response::IntoResponse)
   [16]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [17]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
   [18]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [19]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error (type core::convert::TryInto::Error)
   [20]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [21]: trait.IntoResponse.html#tymethod.into_response
   [22]: ../body/struct.Body.html (struct axum::body::Body)
   [23]: trait.IntoResponseParts.html (trait axum::response::IntoResponseParts)
   [24]: trait.IntoResponseParts.html#associatedtype.Error
   [25]: trait.IntoResponseParts.html#tymethod.into_response_parts
   [26]: struct.ResponseParts.html (struct axum::response::ResponseParts)
   [27]: trait.IntoResponseParts.html#associatedtype.Error (type axum::response::IntoResponseParts::Error)
   [28]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [30]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [31]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [34]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [35]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [36]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [37]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [38]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [39]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [40]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [41]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [42]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [43]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [44]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [45]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [50]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [51]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [52]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [53]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [54]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [55]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [57]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [60]: ../extract/trait.FromRef.html (trait axum::extract::FromRef)
   [61]: ../extract/trait.FromRef.html#tymethod.from_ref
   [62]: ../../src/axum/handler/mod.rs.html#380-398
   [63]: ../handler/trait.HandlerWithoutStateExt.html (trait axum::handler::HandlerWithoutStateExt)
   [64]: ../handler/trait.Handler.html (trait axum::handler::Handler)
   [65]: ../../src/axum/handler/mod.rs.html#384-386
   [66]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_service
   [67]: ../handler/struct.HandlerService.html (struct axum::handler::HandlerService)
   [68]: ../../src/axum/handler/mod.rs.html#388-390
   [69]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service
   [70]: ../routing/struct.IntoMakeService.html (struct axum::routing::IntoMakeService)
   [71]: tower::make::MakeService
   [72]: ../../src/axum/handler/mod.rs.html#393-397
   [73]: ../handler/trait.HandlerWithoutStateExt.html#tymethod.into_make_service_with_connect_info
   [74]: ../extract/connect_info/struct.IntoMakeServiceWithConnectInfo.html (struct axum::extract::connect_info::IntoMakeServiceWithConnectInfo)
   [75]: super::Span::current()
   [76]: crate::Span
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [79]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [80]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [81]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [82]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [83]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [84]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [85]: https://docs.rs/http/latest/http/struct.Extensions.html
   [86]: crate::follow_redirect::policy::Standard
   [87]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [88]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [89]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [90]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [91]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [92]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [93]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [94]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [95]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [96]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [97]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [98]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [99]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [100]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [101]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [102]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [103]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [104]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [105]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [106]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [107]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [108]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [109]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [110]: super::Subscriber
   [111]: dispatcher#setting-the-default-subscriber

