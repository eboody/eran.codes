<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/derive.EnumDiscriminants.html -->
<!-- Crawl timestamp: 2026-02-26T18:14:00Z -->

[ Docs.rs ][1]

  * [ strum-0.28.0 ][2]

    * strum 0.28.0 
    * [ Permalink ][3]
    * [ Docs.rs crate page ][4]
    * [MIT][5]

    * Links
    * [ Homepage ][6]
    * [ Repository ][6]
    * [ crates.io ][7]
    * [ Source ][8]

    * Owners
    * [ Peternator7 ][9]

    * Dependencies
    *       * [ phf ^0.13 _normal_ _optional_ ][10]
      * [ strum_macros ^0.28 _normal_ _optional_ ][11]

    * Versions
    *     * [ **41.18%** of the crate is documented ][12]

  * [ Platform ][13]
    * [aarch64-apple-darwin][14]
    * [aarch64-unknown-linux-gnu][15]
    * [i686-pc-windows-msvc][16]
    * [x86_64-pc-windows-msvc][17]
    * [x86_64-unknown-linux-gnu][18]
  * [ Feature flags ][19]



  * [docs.rs][13]
    * [ About docs.rs][20]
    * [ Badges][21]
    * [ Builds][22]
    * [ Metadata][23]
    * [ Shorthand URLs][24]
    * [ Download][25]
    * [ Rustdoc JSON][26]
    * [ Build queue][27]
    * [ Privacy policy][28]


  * [Rust][13]
    * [Rust website][29]
    * [The Book][30]
    * [Standard Library API Reference][31]
    * [Rust by Example][32]
    * [The Cargo Guide][33]
    * [Clippy Documentation][34]



[Skip to main content][35]

## [EnumDiscriminants][13]

## [strum][36]0.28.0

[strum][36]

# Derive Macro EnumDiscriminants Copy item path

[Source][37]
``` 
#[derive(EnumDiscriminants)]
{
    // Attributes available to this derive:
    #[strum]
    #[strum_discriminants]
}

```

Expand description

Generate a new type with only the discriminant names.

Given an enum named `MyEnum`, generates another enum called `MyEnumDiscriminants` with the same variants but without any data fields. This is useful when you wish to determine the variant of an `enum` but one or more of the variants contains a non-`Default` field. `From` implementations are generated so that you can easily convert from `MyEnum` to `MyEnumDiscriminants`.

By default, the generated enum has the following derives: `Clone, Copy, Debug, PartialEq, Eq`. If your enum derives `Default` and has a `#[default]` variant, that will also be copied onto the discriminant enum. You can add additional derives using the `#[strum_discriminants(derive(AdditionalDerive))]` attribute.

Note, the variant attributes passed to the discriminant enum are filtered to avoid compilation errors due to the derives mismatches, thus only `#[doc]`, `#[cfg]`, `#[allow]`, and `#[deny]` are passed through by default. If you want to specify a custom attribute on the discriminant variant, wrap it with `#[strum_discriminants(...)]` attribute.
``` 
// Bring trait into scope
use std::str::FromStr;
use strum::{IntoEnumIterator, EnumMessage as _};
use strum_macros::{EnumDiscriminants, EnumIter, EnumString, EnumMessage};

#[derive(Debug)]
struct NonDefault;

// simple example
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString, EnumMessage))]
#[strum_discriminants(doc = "This is the docstring on the generated type.")]
enum MyEnum {
    #[strum_discriminants(strum(message = "Variant zero"))]
    Variant0(NonDefault),
    Variant1 { a: NonDefault },
}

// You can rename the generated enum using the `#[strum_discriminants(name(OtherName))]` attribute:
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(name(MyVariants))]
enum MyEnumR {
    Variant0(bool),
    Variant1 { a: bool },
}

// test simple example
assert_eq!(
    MyEnumDiscriminants::Variant0,
    MyEnumDiscriminants::from_str("Variant0").unwrap()
);
// test rename example combined with EnumIter
assert_eq!(
    vec![MyVariants::Variant0, MyVariants::Variant1],
    MyVariants::iter().collect::<Vec<_>>()
);

// Make use of the auto-From conversion to check whether an instance of `MyEnum` matches a
// `MyEnumDiscriminants` discriminant.
assert_eq!(
    MyEnumDiscriminants::Variant0,
    MyEnum::Variant0(NonDefault).into()
);
assert_eq!(
    MyEnumDiscriminants::Variant0,
    MyEnumDiscriminants::from(MyEnum::Variant0(NonDefault))
);

// Make use of the EnumMessage on the `MyEnumDiscriminants` discriminant.
assert_eq!(
    MyEnumDiscriminants::Variant0.get_message(),
    Some("Variant zero")
);
```

It is also possible to specify the visibility (e.g. `pub`/`pub(crate)`/etc.) of the generated enum. By default, the generated enum inherits the visibility of the parent enum it was generated from.
``` 
use strum_macros::EnumDiscriminants;

// You can set the visibility of the generated enum using the `#[strum_discriminants(vis(..))]` attribute:
mod inner {
    use strum_macros::EnumDiscriminants;

    #[derive(Debug, EnumDiscriminants)]
    #[strum_discriminants(vis(pub))]
    #[strum_discriminants(name(PubDiscriminants))]
    enum PrivateEnum {
        Variant0(bool),
        Variant1 { a: bool },
    }
}

// test visibility example, `PrivateEnum` should not be accessible here
assert_ne!(
    inner::PubDiscriminants::Variant0,
    inner::PubDiscriminants::Variant1,
);
```

   [1]: https://docs.rs/
   [2]: derive.EnumDiscriminants.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/derive.EnumDiscriminants.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: derive.EnumDiscriminants.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/derive.EnumDiscriminants.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/derive.EnumDiscriminants.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/derive.EnumDiscriminants.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/derive.EnumDiscriminants.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/derive.EnumDiscriminants.html
   [19]: https://docs.rs/crate/strum/latest/features (Browse available feature flags of strum-0.28.0)
   [20]: https://docs.rs/about
   [21]: https://docs.rs/about/badges
   [22]: https://docs.rs/about/builds
   [23]: https://docs.rs/about/metadata
   [24]: https://docs.rs/about/redirections
   [25]: https://docs.rs/about/download
   [26]: https://docs.rs/about/rustdoc-json
   [27]: https://docs.rs/releases/queue
   [28]: https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs
   [29]: https://www.rust-lang.org/
   [30]: https://doc.rust-lang.org/book/
   [31]: https://doc.rust-lang.org/std/
   [32]: https://doc.rust-lang.org/rust-by-example/
   [33]: https://doc.rust-lang.org/cargo/guide/
   [34]: https://doc.rust-lang.org/nightly/clippy
   [35]: derive.EnumDiscriminants.html#main-content
   [36]: index.html
   [37]: https://docs.rs/strum_macros/0.28.0/x86_64-unknown-linux-gnu/src/strum_macros/lib.rs.html#951

