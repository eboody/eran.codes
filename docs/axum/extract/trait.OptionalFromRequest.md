<!-- Generated from rustdoc HTML: extract/trait.OptionalFromRequest.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## OptionalFromRequest

## [axum][1]0.8.8

## OptionalFromRequest

### Required Associated Types

  * Rejection



### Required Methods

  * from_request



### Dyn Compatibility

### Implementors

## [In axum::extract][2]

[axum][3]::[extract][2]

# Trait OptionalFromRequest Copy item path
```
pub trait OptionalFromRequest<S, M = ViaRequest>: [Sized][4] {
    type Rejection: [IntoResponse][5];

    // Required method
    fn from_request(
        req: Request<[Body][6]>,
        state: [&S][7],
    ) -> impl [Future][8]<Output = [Result][9]<[Option][10]<Self>, Self::[Rejection][11]>> + [Send][12];
}
```

Expand description

Customize the behavior of `Option<Self>` as a [`FromRequest`][13] extractor.

## Required Associated Types§

#### type Rejection: [IntoResponse][5]

If the extractor fails, it will use this “rejection” type.

A rejection is a kind of error that can be converted into a response.

## Required Methods§

#### fn from_request( req: Request<[Body][6]>, state: [&S][7], ) -> impl [Future][8]<Output = [Result][9]<[Option][10]<Self>, Self::[Rejection][11]>> \+ [Send][12]

Perform the extraction.

## Dyn Compatibility§

This trait is **not** [dyn compatible][14].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors§

[Source][15]§

### impl<S> [OptionalFromRequest][16]<S> for [Multipart][17]

where S: [Send][12] \+ [Sync][18],

Available on **crate feature`multipart`** only.

[Source][19]§

#### type Rejection = [MultipartRejection][20]

[Source][21]§

### impl<T, S> [OptionalFromRequest][16]<S> for [Json][22]<T>

where T: [DeserializeOwned][23], S: [Send][12] \+ [Sync][18],

Available on **crate feature`json`** only.

[Source][24]§

#### type Rejection = [JsonRejection][25]

   [1]: ../../axum/index.html
   [2]: index.html
   [3]: ../index.html
   [4]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [5]: ../response/trait.IntoResponse.html (trait axum::response::IntoResponse)
   [6]: ../body/struct.Body.html (struct axum::body::Body)
   [7]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [8]: https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html (trait core::future::future::Future)
   [9]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [10]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [11]: trait.OptionalFromRequest.html#associatedtype.Rejection (type axum::extract::OptionalFromRequest::Rejection)
   [12]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [13]: trait.FromRequest.html (trait axum::extract::FromRequest)
   [14]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility
   [15]: ../../src/axum/extract/multipart.rs.html#84-104
   [16]: trait.OptionalFromRequest.html (trait axum::extract::OptionalFromRequest)
   [17]: struct.Multipart.html (struct axum::extract::Multipart)
   [18]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [19]: ../../src/axum/extract/multipart.rs.html#88
   [20]: multipart/enum.MultipartRejection.html (enum axum::extract::multipart::MultipartRejection)
   [21]: ../../src/axum/json.rs.html#116-136
   [22]: ../struct.Json.html (struct axum::Json)
   [23]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.DeserializeOwned.html (trait serde_core::de::DeserializeOwned)
   [24]: ../../src/axum/json.rs.html#121
   [25]: rejection/enum.JsonRejection.html (enum axum::extract::rejection::JsonRejection)

