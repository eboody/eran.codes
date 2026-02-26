<!-- Generated from rustdoc HTML: extract/rejection/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module rejection

## [axum][1]0.8.8

## Module rejection

### Module Items

  * Re-exports
  * Structs
  * Enums



## [In axum::extract][2]

[axum][3]::[extract][2]

# Module rejection Copy item path

[Source][4]

Expand description

Rejection response types.

## Re-exports§

`pub use crate::extract::path::[FailedToDeserializePathParams][5];`
`pub use crate::extract::path::[InvalidUtf8InPathParam][6];`

## Structs§

[FailedToDeserializeForm][7]
    Rejection type used if the [`Form`][8] extractor is unable to deserialize the form into the target type.
[FailedToDeserializeFormBody][9]
    Rejection type used if the [`Form`][8] extractor is unable to deserialize the form body into the target type.
[FailedToDeserializeQueryString][10]
    Rejection type used if the [`Query`][11] extractor is unable to deserialize the query string into the target type.
[InvalidFormContentType][12]
    Rejection type for [`Form`][8] or [`RawForm`][13] used if the `Content-Type` header is missing or its value is not `application/x-www-form-urlencoded`.
[InvalidUtf8][14]
    Rejection type used when buffering the request into a [`String`][15] if the body doesn’t contain valid UTF-8.
[JsonDataError][16]`json`
    Rejection type for [`Json`][17].
[JsonSyntaxError][18]`json`
    Rejection type for [`Json`][17].
[LengthLimitError][19]
    Encountered some other error when buffering the body.
[MatchedPathMissing][20]`matched-path`
    Rejection if no matched path could be found.
[MissingExtension][21]
    Rejection type for [`Extension`][22] if an expected request extension was not found.
[MissingJsonContentType][23]`json`
    Rejection type for [`Json`][17] used if the `Content-Type` header is missing.
[MissingPathParams][24]
    Rejection type used if axum’s internal representation of path parameters is missing. This is commonly caused by extracting `Request<_>`. `Path` must be extracted first.
[NestedPathRejection][25]
    Rejection type for [`NestedPath`][26].
[UnknownBodyError][27]
    Encountered an unknown error when buffering the body.

## Enums§

[BytesRejection][28]
    Rejection used for [`Bytes`][29].
[ExtensionRejection][30]
    Rejection used for [`Extension`][22].
[FailedToBufferBody][31]
    Rejection type for extractors that buffer the request body. Used if the request body cannot be buffered due to an error.
[FormRejection][32]
    Rejection used for [`Form`][8].
[JsonRejection][33]`json`
    Rejection used for [`Json`][17].
[MatchedPathRejection][34]`matched-path`
    Rejection used for [`MatchedPath`][35].
[PathRejection][36]
    Rejection used for [`Path`][37].
[QueryRejection][38]
    Rejection used for [`Query`][11].
[RawFormRejection][39]
    Rejection used for [`RawForm`][13].
[RawPathParamsRejection][40]
    Rejection used for [`RawPathParams`][41].
[StringRejection][42]
    Rejection used for [`String`][15].

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/extract/rejection.rs.html#1-200
   [5]: ../path/struct.FailedToDeserializePathParams.html (struct axum::extract::path::FailedToDeserializePathParams)
   [6]: ../path/struct.InvalidUtf8InPathParam.html (struct axum::extract::path::InvalidUtf8InPathParam)
   [7]: struct.FailedToDeserializeForm.html (struct axum::extract::rejection::FailedToDeserializeForm)
   [8]: ../../struct.Form.html (struct axum::Form)
   [9]: struct.FailedToDeserializeFormBody.html (struct axum::extract::rejection::FailedToDeserializeFormBody)
   [10]: struct.FailedToDeserializeQueryString.html (struct axum::extract::rejection::FailedToDeserializeQueryString)
   [11]: ../struct.Query.html (struct axum::extract::Query)
   [12]: struct.InvalidFormContentType.html (struct axum::extract::rejection::InvalidFormContentType)
   [13]: ../struct.RawForm.html (struct axum::extract::RawForm)
   [14]: struct.InvalidUtf8.html (struct axum::extract::rejection::InvalidUtf8)
   [15]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [16]: struct.JsonDataError.html (struct axum::extract::rejection::JsonDataError)
   [17]: ../../struct.Json.html (struct axum::Json)
   [18]: struct.JsonSyntaxError.html (struct axum::extract::rejection::JsonSyntaxError)
   [19]: struct.LengthLimitError.html (struct axum::extract::rejection::LengthLimitError)
   [20]: struct.MatchedPathMissing.html (struct axum::extract::rejection::MatchedPathMissing)
   [21]: struct.MissingExtension.html (struct axum::extract::rejection::MissingExtension)
   [22]: ../../struct.Extension.html (struct axum::Extension)
   [23]: struct.MissingJsonContentType.html (struct axum::extract::rejection::MissingJsonContentType)
   [24]: struct.MissingPathParams.html (struct axum::extract::rejection::MissingPathParams)
   [25]: struct.NestedPathRejection.html (struct axum::extract::rejection::NestedPathRejection)
   [26]: ../struct.NestedPath.html (struct axum::extract::NestedPath)
   [27]: struct.UnknownBodyError.html (struct axum::extract::rejection::UnknownBodyError)
   [28]: enum.BytesRejection.html (enum axum::extract::rejection::BytesRejection)
   [29]: bytes::Bytes
   [30]: enum.ExtensionRejection.html (enum axum::extract::rejection::ExtensionRejection)
   [31]: enum.FailedToBufferBody.html (enum axum::extract::rejection::FailedToBufferBody)
   [32]: enum.FormRejection.html (enum axum::extract::rejection::FormRejection)
   [33]: enum.JsonRejection.html (enum axum::extract::rejection::JsonRejection)
   [34]: enum.MatchedPathRejection.html (enum axum::extract::rejection::MatchedPathRejection)
   [35]: ../struct.MatchedPath.html (struct axum::extract::MatchedPath)
   [36]: enum.PathRejection.html (enum axum::extract::rejection::PathRejection)
   [37]: ../struct.Path.html (struct axum::extract::Path)
   [38]: enum.QueryRejection.html (enum axum::extract::rejection::QueryRejection)
   [39]: enum.RawFormRejection.html (enum axum::extract::rejection::RawFormRejection)
   [40]: enum.RawPathParamsRejection.html (enum axum::extract::rejection::RawPathParamsRejection)
   [41]: ../struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [42]: enum.StringRejection.html (enum axum::extract::rejection::StringRejection)

