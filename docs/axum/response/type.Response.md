<!-- Generated from rustdoc HTML: response/type.Response.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Response

## [axum][1]0.8.8

## Response

### Aliased Type

## [In axum::response][2]

[axum][3]::[response][2]

# Type Alias Response Copy item path
```
pub type Response<T = [Body][4]> = Response<T>;
```

Expand description

Type alias for [`http::Response`] whose body type defaults to [`Body`][4], the most common body type used with axum.

## Aliased Type§
```
pub struct Response<T = [Body][4]> { /* private fields */ }
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../body/struct.Body.html (struct axum::body::Body)

