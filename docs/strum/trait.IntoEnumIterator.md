<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/trait.IntoEnumIterator.html -->
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

## [IntoEnumIterator][13]

## [strum][36]0.28.0

## [IntoEnumIterator][13]

### [Sections][13]

  * [Example][37]



### [Required Associated Types][38]

  * [Iterator][39]



### [Required Methods][40]

  * [iter][41]



### [Dyn Compatibility][42]

### [Implementors][43]

## [In crate strum][36]

[strum][36]

# Trait IntoEnumIterator Copy item path

[Source][44]
``` 
pub trait IntoEnumIterator: [Sized][45] {
    type [Iterator][46]: [Iterator][47]<Item = Self> + [Clone][48] + [DoubleEndedIterator][49] + [ExactSizeIterator][50] + [FusedIterator][51];

    // Required method
    fn [iter][52]() -> Self::[Iterator][53];
}
```

Expand description

This trait designates that an `Enum` can be iterated over. It can be auto generated using the [`EnumIter`][54] derive macro.

## [§][55]Example
``` 
// You need to bring the type into scope to use it!!!
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug)]
enum Color {
    Red,
    Green { range: usize },
    Blue(usize),
    Yellow,
}

// Iterate over the items in an enum and perform some function on them.
fn generic_iterator<E, F>(pred: F)
where
    E: IntoEnumIterator,
    F: Fn(E),
{
    for e in E::iter() {
        pred(e)
    }
}

generic_iterator::<Color, _>(|color| println!("{:?}", color));
```

## Required Associated Types[§][38]

[Source][56]

#### type [Iterator][46]: [Iterator][47]<Item = Self> \+ [Clone][48] \+ [DoubleEndedIterator][49] \+ [ExactSizeIterator][50] \+ [FusedIterator][51]

## Required Methods[§][40]

[Source][57]

#### fn [iter][52]() -> Self::[Iterator][53]

## Dyn Compatibility[§][42]

This trait is **not** [dyn compatible][58].

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._

## Implementors[§][43]

   [1]: https://docs.rs/
   [2]: trait.IntoEnumIterator.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/trait.IntoEnumIterator.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: trait.IntoEnumIterator.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/trait.IntoEnumIterator.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/trait.IntoEnumIterator.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/trait.IntoEnumIterator.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/trait.IntoEnumIterator.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/trait.IntoEnumIterator.html
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
   [35]: trait.IntoEnumIterator.html#main-content
   [36]: index.html
   [37]: trait.IntoEnumIterator.html#example (Example)
   [38]: trait.IntoEnumIterator.html#required-associated-types
   [39]: trait.IntoEnumIterator.html#associatedtype.Iterator (Iterator)
   [40]: trait.IntoEnumIterator.html#required-methods
   [41]: trait.IntoEnumIterator.html#tymethod.iter (iter)
   [42]: trait.IntoEnumIterator.html#dyn-compatibility
   [43]: trait.IntoEnumIterator.html#implementors
   [44]: https://docs.rs/strum/latest/src/strum/lib.rs.html#99-107
   [45]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [46]: trait.IntoEnumIterator.html#associatedtype.Iterator
   [47]: https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html (trait core::iter::traits::iterator::Iterator)
   [48]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [49]: https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html (trait core::iter::traits::double_ended::DoubleEndedIterator)
   [50]: https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html (trait core::iter::traits::exact_size::ExactSizeIterator)
   [51]: https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html (trait core::iter::traits::marker::FusedIterator)
   [52]: trait.IntoEnumIterator.html#tymethod.iter
   [53]: trait.IntoEnumIterator.html#associatedtype.Iterator (type strum::IntoEnumIterator::Iterator)
   [54]: derive.EnumIter.html
   [55]: trait.IntoEnumIterator.html#example
   [56]: https://docs.rs/strum/latest/src/strum/lib.rs.html#100-104
   [57]: https://docs.rs/strum/latest/src/strum/lib.rs.html#106
   [58]: https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility

