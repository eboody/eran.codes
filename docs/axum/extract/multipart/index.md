<!-- Generated from rustdoc HTML: extract/multipart/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module multipart

## [axum][1]0.8.8

## Module multipart

### Module Items

  * Structs
  * Enums



## [In axum::extract][2]

[axum][3]::[extract][2]

# Module multipart Copy item path

[Source][4]

Available on **crate feature`multipart`** only.

Expand description

Extractor that parses `multipart/form-data` requests commonly used with file uploads.

See [`Multipart`][5] for more details.

## Structs§

[Field][6]
    A single field in a multipart stream.
[InvalidBoundary][7]
    Rejection type used if the `boundary` in a `multipart/form-data` is missing or invalid.
[Multipart][8]
    Extractor that parses `multipart/form-data` requests (commonly used with file uploads).
[MultipartError][9]
    Errors associated with parsing `multipart/form-data` requests.

## Enums§

[MultipartRejection][10]
    Rejection used for [`Multipart`][5].

   [1]: ../../../axum/index.html
   [2]: ../index.html
   [3]: ../../index.html
   [4]: ../../../src/axum/extract/multipart.rs.html#1-459
   [5]: ../struct.Multipart.html (struct axum::extract::Multipart)
   [6]: struct.Field.html (struct axum::extract::multipart::Field)
   [7]: struct.InvalidBoundary.html (struct axum::extract::multipart::InvalidBoundary)
   [8]: struct.Multipart.html (struct axum::extract::multipart::Multipart)
   [9]: struct.MultipartError.html (struct axum::extract::multipart::MultipartError)
   [10]: enum.MultipartRejection.html (enum axum::extract::multipart::MultipartRejection)

