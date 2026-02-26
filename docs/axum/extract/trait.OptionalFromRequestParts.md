<!-- Generated from rustdoc HTML: extract/trait.OptionalFromRequestParts.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## OptionalFromRequestParts

## [axum][1]0.8.8

## OptionalFromRequestParts

### Required Associated Types

  * Rejection



### Required Methods

  * from_request_parts



### Dyn Compatibility

### Implementors

## [In axum::extract][2]

[axum][3]::[extract][2]

# Trait OptionalFromRequestParts Copy item path
```
pub trait OptionalFromRequestParts<S>: [Sized][4] {
    type Rejection: [IntoResponse][5];

    // Required method
    fn from_request_parts(
        parts: &mut Parts,
        state: [&S][6],
    ) -> impl [Future][7]<Output = [Result][8]<[Option][9]<Self>, Self::[Rejection][10]>> + [Send][11];
}
```

Expand description

Customize the behavior of `Option<Self>` as a [`FromRequestParts`][12] extractor.

## Required Associated Types§

#### type Rejection: [IntoResponse][5]

If the extractor fails, it will use this “rejection” type.

A rejection is a kind of error that can be converted into a response.

## Required Methods§

#### fn from_request_parts( parts: &mut Parts, state: [&S][6], ) -> impl [Future][7]<Output = [Result][8]<[Option][9]<Self>, Self::[Rejection][10]>> \+ [Send][11]

Perform the extraction.

## Dyn Compatibility§

This trait is **not** [dyn compatible][13].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source][14]§

### impl<S> [OptionalFromRequestParts][15]<S> for [MatchedPath][16]

where S: [Send][11] \+ [Sync][17],

Available on **crate feature`matched-path`** only.

[Source][18]§

#### type Rejection = [Infallible][19]

[Source][20]§

### impl<T, S> [OptionalFromRequestParts][15]<S> for [Extension][21]<T>

where T: [Clone][22] \+ [Send][11] \+ [Sync][17] \+ 'static, S: [Send][11] \+ [Sync][17],

[Source][23]§

#### type Rejection = [Infallible][19]

[Source][24]§

### impl<T, S> [OptionalFromRequestParts][15]<S> for [Path][25]<T>

where T: [DeserializeOwned][26] \+ [Send][11] \+ 'static, S: [Send][11] \+ [Sync][17],

[Source][27]§

#### type Rejection = [PathRejection][28]

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [5]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [6]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [7]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [8]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [9]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [10]: trait.OptionalFromRequestParts.html#associatedtype.Rejection (type axum::extract::OptionalFromRequestParts::Rejection)
   [11]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [12]: trait.FromRequestParts.html (trait axum::extract::FromRequestParts)
   [13]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [14]: ../../src/axum/extract/matched_path.rs.html#84-96
   [15]: trait.OptionalFromRequestParts.html (trait axum::extract::OptionalFromRequestParts)
   [16]: struct.MatchedPath.html (struct axum::extract::MatchedPath)
   [17]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [18]: ../../src/axum/extract/matched_path.rs.html#88
   [19]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [20]: ../../src/axum/extension.rs.html#100-113
   [21]: ../struct.Extension.html (struct axum::Extension)
   [22]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [23]: ../../src/axum/extension.rs.html#105
   [24]: ../../src/axum/extract/path/mod.rs.html#192-213
   [25]: struct.Path.html (struct axum::extract::Path)
   [26]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [27]: ../../src/axum/extract/path/mod.rs.html#197
   [28]: rejection/enum.PathRejection.html (enum axum::extract::rejection::PathRejection)

