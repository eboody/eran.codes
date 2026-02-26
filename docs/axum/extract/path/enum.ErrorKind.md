<!-- Generated from rustdoc HTML: extract/path/enum.ErrorKind.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## ErrorKind

## [axum][1]0.8.8

## ErrorKind

### Variants

  * DeserializeError
  * InvalidUtf8InPathParam
  * Message
  * ParseError
  * ParseErrorAtIndex
  * ParseErrorAtKey
  * UnsupportedType
  * WrongNumberOfParameters



### Trait Implementations

  * Debug
  * Display
  * Eq
  * PartialEq
  * StructuralPartialEq



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
  * Equivalent<K>
  * Equivalent<K>
  * From<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Same
  * ServiceExt
  * ToString
  * ToStringFallible
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract::path][2]

[axum][3]::[extract][4]::[path][2]

# Enum ErrorKind Copy item path

[Source][5]
``` 
#[non_exhaustive]

pub enum ErrorKind {
    WrongNumberOfParameters {
        got: [usize][6],
        expected: [usize][6],
    },
    ParseErrorAtKey {
        key: [String][7],
        value: [String][7],
        expected_type: &'static [str][8],
    },
    ParseErrorAtIndex {
        index: [usize][6],
        value: [String][7],
        expected_type: &'static [str][8],
    },
    ParseError {
        value: [String][7],
        expected_type: &'static [str][8],
    },
    InvalidUtf8InPathParam {
        key: [String][7],
    },
    UnsupportedType {
        name: &'static [str][8],
    },
    DeserializeError {
        key: [String][7],
        value: [String][7],
        message: [String][7],
    },
    Message([String][7]),
}
```

Expand description

The kinds of errors that can happen we deserializing into a [`Path`][9].

This type is obtained through [`FailedToDeserializePathParams::kind`][10] or [`FailedToDeserializePathParams::into_kind`][11] and is useful for building more precise error messages.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

§

### WrongNumberOfParameters

The URI contained the wrong number of parameters.

#### Fields

§`got: [usize][6]`

The number of actual parameters in the URI.

§`expected: [usize][6]`

The number of expected parameters.

§

### ParseErrorAtKey

Failed to parse the value at a specific key into the expected type.

This variant is used when deserializing into types that have named fields, such as structs.

#### Fields

§`key: [String][7]`

The key at which the value was located.

§`value: [String][7]`

The value from the URI.

§`expected_type: &'static [str][8]`

The expected type of the value.

§

### ParseErrorAtIndex

Failed to parse the value at a specific index into the expected type.

This variant is used when deserializing into sequence types, such as tuples.

#### Fields

§`index: [usize][6]`

The index at which the value was located.

§`value: [String][7]`

The value from the URI.

§`expected_type: &'static [str][8]`

The expected type of the value.

§

### ParseError

Failed to parse a value into the expected type.

This variant is used when deserializing into a primitive type (such as `String` and `u32`).

#### Fields

§`value: [String][7]`

The value from the URI.

§`expected_type: &'static [str][8]`

The expected type of the value.

§

### InvalidUtf8InPathParam

A parameter contained text that, once percent decoded, wasn’t valid UTF-8.

#### Fields

§`key: [String][7]`

The key at which the invalid value was located.

§

### UnsupportedType

Tried to serialize into an unsupported type such as nested maps.

This error kind is caused by programmer errors and thus gets converted into a `500 Internal Server Error` response.

#### Fields

§`name: &'static [str][8]`

The name of the unsupported type.

§

### DeserializeError

Failed to deserialize the value with a custom deserialization error.

#### Fields

§`key: [String][7]`

The key at which the invalid value was located.

§`value: [String][7]`

The value that failed to deserialize.

§`message: [String][7]`

The deserialization failure message.

§

### Message([String][7])

Catch-all variant for errors that don’t fit any other variant.

## Trait Implementations§

[Source][12]§

### impl [Debug][13] for [ErrorKind][14]

[Source][12]§

#### fn [fmt][15](&self, f: &mut [Formatter][16]<'_>) -> [Result][17]

Formats the value using the given formatter. [Read more][15]

[Source][18]§

### impl [Display][19] for [ErrorKind][14]

[Source][20]§

#### fn [fmt][21](&self, f: &mut [Formatter][16]<'_>) -> [Result][17]

Formats the value using the given formatter. [Read more][21]

[Source][12]§

### impl [PartialEq][22] for [ErrorKind][14]

[Source][12]§

#### fn [eq][23](&self, other: &[ErrorKind][14]) -> [bool][24]

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source][25]§

#### fn [ne][26](&self, other: [&Rhs][27]) -> [bool][24]

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source][12]§

### impl [Eq][28] for [ErrorKind][14]

[Source][12]§

### impl [StructuralPartialEq][29] for [ErrorKind][14]

## Auto Trait Implementations§

§

### impl [Freeze][30] for [ErrorKind][14]

§

### impl [RefUnwindSafe][31] for [ErrorKind][14]

§

### impl [Send][32] for [ErrorKind][14]

§

### impl [Sync][33] for [ErrorKind][14]

§

### impl [Unpin][34] for [ErrorKind][14]

§

### impl [UnwindSafe][35] for [ErrorKind][14]

## Blanket Implementations§

[Source][36]§

### impl<T> [Any][37] for T

where T: 'static + ?[Sized][38],

[Source][39]§

#### fn [type_id][40](&self) -> [TypeId][41]

Gets the `TypeId` of `self`. [Read more][40]

[Source][42]§

### impl<T> [Borrow][43]<T> for T

where T: ?[Sized][38],

[Source][44]§

#### fn [borrow][45](&self) -> [&T][27]

Immutably borrows from an owned value. [Read more][45]

[Source][46]§

### impl<T> [BorrowMut][47]<T> for T

where T: ?[Sized][38],

[Source][48]§

#### fn [borrow_mut][49](&mut self) -> [&mut T][27]

Mutably borrows from an owned value. [Read more][49]

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][28] \+ ?[Sized][38], K: [Borrow][43]<Q> \+ ?[Sized][38],

§

#### fn equivalent(&self, key: [&K][27]) -> [bool][24]

Checks if this value is equivalent to the given key. Read more

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][28] \+ ?[Sized][38], K: [Borrow][43]<Q> \+ ?[Sized][38],

§

#### fn equivalent(&self, key: [&K][27]) -> [bool][24]

Compare self to `key` and return `true` if they are equal.

[Source][50]§

### impl<T> [From][51]<T> for T

[Source][52]§

#### fn [from][53](t: T) -> T

Returns the argument unchanged.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][54] [`Span`][55], returning an `Instrumented` wrapper. Read more

[Source][56]§

### impl<T, U> [Into][57]<U> for T

where U: [From][51]<T>,

[Source][58]§

#### fn [into][59](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][51]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][38],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][60]§

### impl<T> [Same][61] for T

[Source][62]§

#### type [Output][63] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][38],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][38],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][64]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][38],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][38],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][38],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][38],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][38],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][38],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][38],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][65] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][66]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][38],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][67] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][66]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][38],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][67] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][66]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][38],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][67] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][38],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][38], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][38], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][38],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][38],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][38],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][6]) -> RequestBodyLimit<Self>

where Self: [Sized][38],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][38],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][38],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][68]§

### impl<T> [ToString][69] for T

where T: [Display][19] \+ ?[Sized][38],

[Source][70]§

#### fn [to_string][71](&self) -> [String][7]

Converts the given value to a `String`. [Read more][71]

§

### impl<T> ToStringFallible for T

where T: [Display][19],

§

#### fn try_to_string(&self) -> [Result][72]<[String][7], [TryReserveError][73]>

[`ToString::to_string`][74], but without panic on OOM.

[Source][75]§

### impl<T, U> [TryFrom][76]<U> for T

where U: [Into][57]<T>,

[Source][77]§

#### type [Error][78] = [Infallible][79]

The type returned in the event of a conversion error.

[Source][80]§

#### fn [try_from][81](value: U) -> [Result][72]<T, <T as [TryFrom][76]<U>>::[Error][82]>

Performs the conversion.

[Source][83]§

### impl<T, U> [TryInto][84]<U> for T

where U: [TryFrom][76]<T>,

[Source][85]§

#### type [Error][86] = <U as [TryFrom][76]<T>>::[Error][82]

The type returned in the event of a conversion error.

[Source][87]§

#### fn [try_into][88](self) -> [Result][72]<U, <U as [TryFrom][76]<T>>::[Error][82]>

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

where S: [Into][57]<Dispatch>,

Attaches the provided [`Subscriber`][89] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][90] [`Subscriber`][89] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/path/mod.rs.html#285-355
   [6]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [7]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [8]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [9]: ../struct.Path.html (struct axum::extract::Path)
   [10]: struct.FailedToDeserializePathParams.html#method.kind (method axum::extract::path::FailedToDeserializePathParams::kind)
   [11]: struct.FailedToDeserializePathParams.html#method.into_kind (method axum::extract::path::FailedToDeserializePathParams::into_kind)
   [12]: ../../../src/axum/extract/path/mod.rs.html#283
   [13]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [14]: enum.ErrorKind.html (enum axum::extract::path::ErrorKind)
   [15]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [16]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [17]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [18]: ../../../src/axum/extract/path/mod.rs.html#357-402
   [19]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [20]: ../../../src/axum/extract/path/mod.rs.html#358-401
   [21]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [22]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html (trait core::cmp::PartialEq)
   [23]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq
   [24]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [25]: https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264
   [26]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne
   [27]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [28]: https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html (trait core::cmp::Eq)
   [29]: https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html (trait core::marker::StructuralPartialEq)
   [30]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [31]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [32]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [33]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [34]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [35]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [36]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [37]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [38]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [39]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [40]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [41]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [42]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [43]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [44]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [45]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [46]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [47]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [48]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [49]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [50]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [51]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [52]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [53]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [54]: super::Span::current()
   [55]: crate::Span
   [56]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [57]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [58]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [59]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [60]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [61]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [62]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [63]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [64]: https://docs.rs/http/latest/http/struct.Extensions.html
   [65]: crate::follow_redirect::policy::Standard
   [66]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [67]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [68]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [69]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [70]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [71]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [72]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [73]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [74]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [75]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [76]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [77]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [78]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [79]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [80]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [81]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [82]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [83]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [84]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [85]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [86]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [87]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [88]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [89]: super::Subscriber
   [90]: dispatcher#setting-the-default-subscriber

