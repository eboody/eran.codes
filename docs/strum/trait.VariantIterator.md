<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/trait.VariantIterator.html -->
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

## [VariantIterator][13]

## [strum][36]0.28.0

## [VariantIterator][13]

### [Required Associated Types][37]

  * [Iterator][38]



### [Required Methods][39]

  * [iter][40]



### [Dyn Compatibility][41]

### [Implementors][42]

## [In crate strum][36]

[strum][36]

# Trait VariantIterator Copy item path

[Source][43]
``` 
pub trait VariantIterator: [Sized][44] {
    type [Iterator][45]: [Iterator][46]<Item = Self>;

    // Required method
    fn [iter][47]() -> Self::[Iterator][48];
}
```

## Required Associated Types[§][37]

[Source][49]

#### type [Iterator][45]: [Iterator][46]<Item = Self>

## Required Methods[§][39]

[Source][50]

#### fn [iter][47]() -> Self::[Iterator][48]

## Dyn Compatibility[§][41]

This trait is **not** [dyn compatible][51].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors[§][42]

   [1]: https://docs.rs/
   [2]: trait.VariantIterator.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/trait.VariantIterator.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: trait.VariantIterator.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/trait.VariantIterator.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/trait.VariantIterator.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/trait.VariantIterator.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/trait.VariantIterator.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/trait.VariantIterator.html
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
   [35]: trait.VariantIterator.html#main-content
   [36]: index.html
   [37]: trait.VariantIterator.html#required-associated-types
   [38]: trait.VariantIterator.html#associatedtype.Iterator (Iterator)
   [39]: trait.VariantIterator.html#required-methods
   [40]: trait.VariantIterator.html#tymethod.iter (iter)
   [41]: trait.VariantIterator.html#dyn-compatibility
   [42]: trait.VariantIterator.html#implementors
   [43]: https://docs.rs/strum/latest/src/strum/lib.rs.html#109-113
   [44]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [45]: trait.VariantIterator.html#associatedtype.Iterator
   [46]: https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html (trait core::iter::traits::iterator::Iterator)
   [47]: trait.VariantIterator.html#tymethod.iter
   [48]: trait.VariantIterator.html#associatedtype.Iterator (type strum::VariantIterator::Iterator)
   [49]: https://docs.rs/strum/latest/src/strum/lib.rs.html#110
   [50]: https://docs.rs/strum/latest/src/strum/lib.rs.html#112
   [51]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility

