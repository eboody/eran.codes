<!-- Generated from rustdoc HTML: middleware/trait.IntoMapRequestResult.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## IntoMapRequestResult

## [axum][1]0.8.8

## IntoMapRequestResult

### Required Methods

  * into_map_request_result



### Implementations on Foreign Types

  * Request<B>
  * Result<Request<B>, E>



### Implementors

## [In axum::middleware][2]

[axum][3]::[middleware][2]

# Trait IntoMapRequestResult Copy item path

[Source][4]
``` 
pub trait IntoMapRequestResult<B>: Sealed<B> {
    // Required method
    fn into_map_request_result(self) -> [Result][5]<Request<B>, [Response][6]>;
}
```

Expand description

Trait implemented by types that can be returned from [`map_request`][7], [`map_request_with_state`][8].

This trait is sealed such that it cannot be implemented outside this crate.

## Required Methods§

[Source][9]

#### fn into_map_request_result(self) -> [Result][5]<Request<B>, [Response][6]>

Perform the conversion.

## Implementations on Foreign Types§

[Source][10]§

### impl<B> [IntoMapRequestResult][11]<B> for Request<B>

[Source][12]§

#### fn into_map_request_result(self) -> [Result][5]<Self, [Response][6]>

[Source][13]§

### impl<B, E> [IntoMapRequestResult][11]<B> for [Result][5]<Request<B>, E>

where E: [IntoResponse][14],

[Source][15]§

#### fn into_map_request_result(self) -> [Result][5]<Request<B>, [Response][6]>

## Implementors§

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../../src/axum/middleware/map_request.rs.html#367-371
   [5]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [6]: ../response/type.Response.html (type axum::response::Response)
   [7]: fn.map_request.html (fn axum::middleware::map_request)
   [8]: fn.map_request_with_state.html (fn axum::middleware::map_request_with_state)
   [9]: ../../src/axum/middleware/map_request.rs.html#370
   [10]: ../../src/axum/middleware/map_request.rs.html#382-386
   [11]: trait.IntoMapRequestResult.html (trait axum::middleware::IntoMapRequestResult)
   [12]: ../../src/axum/middleware/map_request.rs.html#383-385
   [13]: ../../src/axum/middleware/map_request.rs.html#373-380
   [14]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [15]: ../../src/axum/middleware/map_request.rs.html#377-379

