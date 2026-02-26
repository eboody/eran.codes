<!-- Generated from rustdoc HTML: extract/type.Request.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Request

## [axum][1]0.8.8

## Request

### Aliased Type

## [In axum::extract][2]

[axum][3]::[extract][2]

# Type Alias Request Copy item path
```
pub type Request<T = [Body][4]> = Request<T>;
```

Expand description

Type alias for [`http::Request`] whose body type defaults to [`Body`][4], the most common body type used with axum.

## Aliased Type§
```
pub struct Request<T = [Body][4]> { /* private fields */ }
```

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: ../body/struct.Body.html (struct axum::body::Body)

