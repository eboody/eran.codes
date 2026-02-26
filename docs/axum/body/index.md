<!-- Generated from rustdoc HTML: body/index.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Module body

## [axum][1]0.8.8

## Module body

### Module Items

  * Re-exports
  * Structs
  * Functions



## [In crate axum][2]

[axum][2]

# Module body Copy item path

[Source][3]

Expand description

HTTP body utilities.

## Re-exports§

`pub use http_body::Body as HttpBody;`
`pub use bytes::Bytes;`

## Structs§

[Body][4]
    The body type used in axum requests and responses.
[BodyDataStream][5]
    A stream of data frames.

## Functions§

[to_bytes][6]
    Converts [`Body`][4] into [`Bytes`] and limits the maximum size of the body.

   [1]: ../../axum/index.html
   [2]: ../index.html
   [3]: ../../src/axum/body/mod.rs.html#1-54
   [4]: struct.Body.html (struct axum::body::Body)
   [5]: struct.BodyDataStream.html (struct axum::body::BodyDataStream)
   [6]: fn.to_bytes.html (fn axum::body::to_bytes)

