<!-- Generated from rustdoc HTML: extract/derive.FromRequestParts.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## FromRequestParts

## [axum][1]0.8.8

## FromRequestParts

### Sections

  * Example
  * Cannot extract the body



## [In axum::extract][2]

[axum][3]::[extract][2]

# Derive Macro FromRequestParts Copy item path
```
#[derive(FromRequestParts)]
{
    // Attributes available to this derive:
    #[from_request]
}

```

Available on **crate feature`macros`** only.

Expand description

Derive an implementation of [`FromRequestParts`][4].

This works similarly to `#[derive(FromRequest)]` except it uses [`FromRequestParts`][4]. All the same options are supported.

## §Example
``` 
use axum_macros::FromRequestParts;
use axum::{
    extract::Query,
};
use axum_extra::{
    TypedHeader,
    headers::ContentType,
};
use std::collections::HashMap;

#[derive(FromRequestParts)]
struct MyExtractor {
    #[from_request(via(Query))]
    query_params: HashMap<String, String>,
    content_type: TypedHeader<ContentType>,
}

async fn handler(extractor: MyExtractor) {}
```

## §Cannot extract the body

[`FromRequestParts`][4] cannot extract the request body:

ⓘ
```
use axum_macros::FromRequestParts;

#[derive(FromRequestParts)]
struct MyExtractor {
    body: String,
}
```

Use `#[derive(FromRequest)]` for that.

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html

