<!-- Generated from rustdoc HTML: trait.RequestPartsExt.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## RequestPartsExt

## [axum][1]0.8.8

## RequestPartsExt

### Required Methods

  * extract
  * extract_with_state



### Implementations on Foreign Types

  * Parts



### Dyn Compatibility

### Implementors

## [In crate axum][2]

[axum][2]

# Trait RequestPartsExt Copy item path
```
pub trait RequestPartsExt: [Sized][3] + Sealed {
    // Required methods
    fn extract<E>(
        &mut self,
    ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][6]<[()][7]>>::[Rejection][8]>> + [Send][9]
       where E: [FromRequestParts][6]<[()][7]> + 'static;
    fn extract_with_state<'a, E, S>(
        &'a mut self,
        state: [&'a S][10],
    ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][6]<S>>::[Rejection][8]>> + [Send][9] + 'a
       where E: [FromRequestParts][6]<S> + 'static,
             S: [Send][9] + [Sync][11];
}
```

Expand description

Extension trait that adds additional methods to [`Parts`].

## Required Methods§

#### fn extract<E>( &mut self, ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][6]<[()][7]>>::[Rejection][8]>> \+ [Send][9]

where E: [FromRequestParts][6]<[()][7]> \+ 'static,

Apply an extractor to this `Parts`.

This is just a convenience for `E::from_request_parts(parts, &())`.

##### §Example
``` 
use axum::{
    extract::{Query, Path, FromRequestParts},
    response::{Response, IntoResponse},
    http::request::Parts,
    RequestPartsExt,
};
use std::collections::HashMap;

struct MyExtractor {
    path_params: HashMap<String, String>,
    query_params: HashMap<String, String>,
}

impl<S> FromRequestParts<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path_params = parts
            .extract::<Path<HashMap<String, String>>>()
            .await
            .map(|Path(path_params)| path_params)
            .map_err(|err| err.into_response())?;

        let query_params = parts
            .extract::<Query<HashMap<String, String>>>()
            .await
            .map(|Query(params)| params)
            .map_err(|err| err.into_response())?;

        Ok(MyExtractor { path_params, query_params })
    }
}
```

#### fn extract_with_state<'a, E, S>( &'a mut self, state: [&'a S][10], ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][6]<S>>::[Rejection][8]>> \+ [Send][9] \+ 'a

where E: [FromRequestParts][6]<S> \+ 'static, S: [Send][9] \+ [Sync][11],

Apply an extractor that requires some state to this `Parts`.

This is just a convenience for `E::from_request_parts(parts, state)`.

##### §Example
``` 
use axum::{
    extract::{FromRef, FromRequestParts},
    response::{Response, IntoResponse},
    http::request::Parts,
    RequestPartsExt,
};

struct MyExtractor {
    requires_state: RequiresState,
}

impl<S> FromRequestParts<S> for MyExtractor
where
    String: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let requires_state = parts
            .extract_with_state::<RequiresState, _>(state)
            .await?;

        Ok(MyExtractor { requires_state })
    }
}

struct RequiresState { /* ... */ }

// some extractor that requires a `String` in the state
impl<S> FromRequestParts<S> for RequiresState
where
    String: FromRef<S>,
    S: Send + Sync,
{
    // ...
}
```

## Dyn Compatibility§

This trait is **not** [dyn compatible][12].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementations on Foreign Types§

§

### impl [RequestPartsExt][13] for Parts

§

#### fn extract<E>( &mut self, ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][6]<[()][7]>>::[Rejection][8]>> \+ [Send][9]

where E: [FromRequestParts][6]<[()][7]> \+ 'static,

§

#### fn extract_with_state<'a, E, S>( &'a mut self, state: [&'a S][10], ) -> impl [Future][4]<Output = [Result][5]<E, <E as [FromRequestParts][6]<S>>::[Rejection][8]>> \+ [Send][9] \+ 'a

where E: [FromRequestParts][6]<S> \+ 'static, S: [Send][9] \+ [Sync][11],

## Implementors§

   [1]: ../axum/index.html
   [2]: index.html
   [3]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [4]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [5]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [6]: extract/trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [7]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [8]: extract/trait.FromRequestParts.html#associatedtype.Rejection (type axum::extract::FromRequestParts::Rejection)
   [9]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [10]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [12]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [13]: trait.RequestPartsExt.html (trait axum::RequestPartsExt)

