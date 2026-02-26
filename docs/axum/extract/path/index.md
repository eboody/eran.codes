<!-- Generated from rustdoc HTML: extract/path/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module path

## [axum][1]0.8.8

## Module path

### Module Items

  * Structs
  * Enums



## [In axum::extract][2]

[axum][3]::[extract][2]

# Module path Copy item path

[Source][4]

Expand description

Extractor that will get captures from the URL and parse them using [`serde`][5].

## Structs§

[FailedToDeserializePathParams][6]
    Rejection type for [`Path`][7] if the captured routes params couldn’t be deserialized into the expected type.
[InvalidUtf8InPathParam][8]
    Rejection used by [`RawPathParams`][9] if a parameter contained text that, once percent decoded, wasn’t valid UTF-8.
[Path][10]
    Extractor that will get captures from the URL and parse them using [`serde`][11].
[RawPathParams][12]
    Extractor that will get captures from the URL without deserializing them.
[RawPathParamsIter][13]
    An iterator over raw path parameters.

## Enums§

[ErrorKind][14]
    The kinds of errors that can happen we deserializing into a [`Path`][7].

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/extract/path/mod.rs.html#1-1039
   [5]: https://docs.rs/serde/1.0.228/serde/index.html (mod serde)
   [6]: struct.FailedToDeserializePathParams.html (struct axum::extract::path::FailedToDeserializePathParams)
   [7]: ../struct.Path.html (struct axum::extract::Path)
   [8]: struct.InvalidUtf8InPathParam.html (struct axum::extract::path::InvalidUtf8InPathParam)
   [9]: ../struct.RawPathParams.html (struct axum::extract::RawPathParams)
   [10]: struct.Path.html (struct axum::extract::path::Path)
   [11]: https://crates.io/crates/serde
   [12]: struct.RawPathParams.html (struct axum::extract::path::RawPathParams)
   [13]: struct.RawPathParamsIter.html (struct axum::extract::path::RawPathParamsIter)
   [14]: enum.ErrorKind.html (enum axum::extract::path::ErrorKind)

